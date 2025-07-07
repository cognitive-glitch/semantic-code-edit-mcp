//! # Editor Module
//!
//! The core editing engine for semantic code transformations.
//! This module orchestrates AST-aware code edits with comprehensive validation.
//!
//! ## Architecture
//!
//! The Editor is decomposed into focused submodules:
//! - `validator`: Syntax and context validation
//! - `formatter`: Language-specific code formatting
//! - `diff_generator`: Diff generation and efficiency metrics
//! - `edit`: Individual edit operations
//! - `edit_iterator`: Iterator for multiple edit locations
//! - `edit_position`: Edit position tracking
//!
//! ## Features
//!
//! - **Preview Mode**: Test edits safely before applying
//! - **Staged Operations**: Support for multi-step workflows
//! - **Validation**: Two-layer validation prevents file corruption
//! - **Smart Diffs**: Clean diffs with efficiency metrics
//!
//! ## Example
//!
//! ```ignore
//! use semantic_code_edit_mcp::editor::Editor;
//!
//! let editor = Editor::new(content, selector, language, file_path, None)?;
//!
//! // Preview changes
//! let (preview_msg, staged_op) = editor.preview()?;
//!
//! // Or commit directly
//! let (message, output, path) = editor.commit()?;
//! ```

mod diff_generator;
mod edit;
mod edit_iterator;
mod edit_position;
mod formatter;
mod validator;

use std::path::PathBuf;

use crate::error::SemanticEditError;
use anyhow::{Result, anyhow};
use diff_generator::DiffGenerator;
use edit::Edit;
use edit_iterator::EditIterator;
use formatter::Formatter;
use ropey::Rope;
use tree_sitter::Tree;
use validator::Validator;

pub use edit_position::EditPosition;

use crate::{
    languages::{LanguageCommon, LanguageRegistry},
    selector::Selector,
    state::StagedOperation,
};

pub struct Editor<'language> {
    content: String,
    selector: Selector,
    file_path: PathBuf,
    language: &'language LanguageCommon,
    source_code: String,
    tree: Tree,
    rope: Rope,
    staged_edit: Option<EditPosition>,
}

impl<'language> Editor<'language> {
    pub fn new(
        content: String,
        selector: Selector,
        language: &'language LanguageCommon,
        file_path: PathBuf,
        staged_edit: Option<EditPosition>,
    ) -> Result<Self> {
        let source_code = std::fs::read_to_string(&file_path)?;
        let mut parser = language.tree_sitter_parser()?;
        let tree = parser.parse(&source_code, None).ok_or_else(|| {
            anyhow!(
                "Unable to parse {} as {}",
                file_path.display(),
                language.name()
            )
        })?;
        let rope = Rope::from_str(&source_code);

        Ok(Self {
            content,
            selector,
            language,
            tree,
            file_path,
            source_code,
            rope,
            staged_edit,
        })
    }

    pub fn from_staged_operation(
        staged_operation: StagedOperation,
        language_registry: &'language LanguageRegistry,
    ) -> Result<Self> {
        let StagedOperation {
            selector,
            content,
            file_path,
            language_name,
            edit_position,
        } = staged_operation;
        let language = language_registry.get_language(language_name)?;
        Self::new(content, selector, language, file_path, edit_position)
    }

    fn prevalidate(&self) -> Option<String> {
        self.validate_tree(&self.tree, &self.source_code)
            .map(|errors| {
                format!(
                    "Syntax error found prior to edit, not attempting.
Suggestion: Pause and show your human collaborator this context:\n\n{errors}"
                )
            })
    }

    fn validate_tree(&self, tree: &Tree, content: &str) -> Option<String> {
        Validator::validate(self.language, tree, content)
    }

    pub fn validate(language: &LanguageCommon, tree: &Tree, content: &str) -> Option<String> {
        Validator::validate(language, tree, content)
    }

    fn edit_iterator(&self) -> EditIterator<'_, 'language> {
        EditIterator::new(self)
    }

    fn edit(&mut self) -> Result<(String, Option<String>)> {
        if let Some(prevalidation_failure) = self.prevalidate() {
            return Ok((prevalidation_failure, None));
        };

        let mut failed_edits = vec![];
        for edit in self.edit_iterator() {
            match edit {
                Ok(mut edit) => {
                    edit.apply()?;
                    if edit.is_valid() {
                        return Ok((edit.message(), edit.output()));
                    }

                    failed_edits.push(edit);
                }

                Err(message) => return Ok((message, None)),
            }
        }

        failed_edits
            .first_mut()
            .map(|edit| (edit.message(), None))
            .ok_or_else(|| anyhow::Error::from(SemanticEditError::NoValidEditLocations))
    }

    pub fn preview(mut self) -> Result<(String, Option<StagedOperation>)> {
        let (message, output) = self.edit()?;
        if let Some(output) = &output {
            let mut preview = String::new();

            preview.push_str(&format!("STAGED: {}\n\n", self.selector.operation_name()));
            preview.push_str(&self.diff(output));

            Ok((preview, Some(self.into())))
        } else {
            Ok((message, None))
        }
    }

    fn diff(&self, output: &str) -> String {
        DiffGenerator::generate_diff(&self.source_code, output, &self.content)
    }

    pub fn format_code(&self, source: &str) -> Result<String> {
        Formatter::format_code(self.language, source)
    }

    pub fn commit(mut self) -> Result<(String, Option<String>, PathBuf)> {
        let (mut message, output) = self.edit()?;
        if let Some(output) = &output {
            let diff = self.diff(output);

            message = format!(
                "{} operation result:\n{}\n\n{diff}",
                self.selector.operation_name(),
                message,
            );
        }
        Ok((message, output, self.file_path))
    }

    fn parse(&self, output: &str, old_tree: Option<&Tree>) -> Option<Tree> {
        let mut parser = match self.language.tree_sitter_parser() {
            Ok(parser) => parser,
            Err(_) => return None, // Cannot parse without a valid parser
        };
        parser.parse(output, old_tree)
    }
}

impl From<Editor<'_>> for StagedOperation {
    fn from(value: Editor) -> Self {
        let Editor {
            content,
            selector,
            file_path,
            language,
            staged_edit,
            ..
        } = value;
        Self {
            selector,
            content,
            file_path,
            language_name: language.name(),
            edit_position: staged_edit,
        }
    }
}
