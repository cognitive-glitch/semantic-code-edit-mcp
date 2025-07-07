//! # Validator Module
//!
//! This module provides syntax and context validation for code edits across all supported languages.
//! It implements a two-layer validation approach:
//!
//! 1. **Syntax Validation**: Uses tree-sitter to detect syntax errors in the parsed AST
//! 2. **Context Validation**: Language-specific semantic rules (e.g., no functions in struct fields)
//!
//! ## Example
//!
//! ```ignore
//! use semantic_code_edit_mcp::editor::validator::Validator;
//!
//! // Validate code before applying edits
//! if let Some(error_msg) = Validator::validate(language, tree, content) {
//!     println!("Validation failed: {}", error_msg);
//! }
//! ```

use crate::{languages::LanguageCommon, validation::ContextValidator};
use std::collections::BTreeSet;
use tree_sitter::Tree;

/// Handles syntax and context validation for code edits
pub struct Validator;

impl Validator {
    /// Validates a tree against language-specific rules
    /// Returns None if valid, Some(error_message) if invalid
    pub fn validate(language: &LanguageCommon, tree: &Tree, content: &str) -> Option<String> {
        let errors = language.editor().collect_errors(tree, content);
        if errors.is_empty() {
            if let Some(query) = language.validation_query() {
                let validation_result = ContextValidator::validate_tree(tree, query, content);

                if !validation_result.is_valid {
                    return Some(validation_result.format_errors());
                }
            }

            return None;
        }

        let context_lines = 3;
        let lines_with_errors = errors.into_iter().collect::<BTreeSet<_>>();
        let context_lines = lines_with_errors
            .iter()
            .copied()
            .flat_map(|line| line.saturating_sub(context_lines)..line + context_lines)
            .collect::<BTreeSet<_>>();
        Some(
            std::iter::once(String::from("===SYNTAX ERRORS===\n"))
                .chain(
                    content
                        .lines()
                        .enumerate()
                        .filter(|(index, _)| context_lines.contains(index))
                        .map(|(index, line)| {
                            let display_index = index + 1;
                            if lines_with_errors.contains(&index) {
                                format!("{display_index:>4} ->⎸{line}\n")
                            } else {
                                format!("{display_index:>4}   ⎸{line}\n")
                            }
                        }),
                )
                .collect(),
        )
    }
}
