//! TOML language support with taplo formatting and validation.
//!
//! This module provides TOML-specific editing capabilities including:
//! - Taplo formatter integration for code formatting
//! - Validation using taplo for syntax correctness
//! - Tree-sitter parsing for AST-aware operations
//! - Line-based conversion utilities for editing

use crate::languages::{
    LanguageBuilder, LanguageCommon, LanguageName, traits::LanguageEditor, utils::LineConverter,
};
use anyhow::Result;
use tree_sitter::Tree;

pub fn language() -> Result<LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::Toml,
        &["toml"],
        tree_sitter_toml_ng::LANGUAGE.into(),
    )
    .with_editor(Box::new(TomlEditor::new()))
    .build()
}

pub struct TomlEditor;

impl Default for TomlEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl TomlEditor {
    pub fn new() -> Self {
        Self
    }
}

impl LanguageEditor for TomlEditor {
    fn format_code(&self, source: &str) -> Result<String> {
        Ok(taplo::formatter::format(
            source,
            taplo::formatter::Options::default(),
        ))
    }

    fn collect_errors(&self, _tree: &Tree, content: &str) -> Vec<usize> {
        let converter = LineConverter::new(content);

        taplo::parser::parse(content)
            .errors
            .into_iter()
            .flat_map(|error| {
                let start_offset = usize::from(error.range.start());
                let end_offset = usize::from(error.range.end());
                converter.range_to_lines(start_offset, end_offset)
            })
            .collect()
    }
}
