//! JavaScript language support with tree-sitter parsing.
//!
//! This module provides JavaScript-specific editing capabilities including:
//! - Tree-sitter parsing for AST-aware operations
//! - Support for .js and .mjs files
//! - Standardized language configuration using LanguageBuilder
//! - Default editor for basic operations

use crate::languages::{LanguageBuilder, LanguageCommon, LanguageName};
use anyhow::Result;

pub fn language() -> Result<LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::Javascript,
        &["js", "jsx", "mjs", "cjs"],
        tree_sitter_javascript::LANGUAGE.into(),
    )
    .with_validation_query(include_str!("../../queries/javascript/validation.scm"))
    .build()
}
