//! Ruby language support with tree-sitter parsing.
//!
//! This module provides Ruby-specific editing capabilities including:
//! - Tree-sitter parsing for AST-aware operations
//! - Support for .rb files
//! - Standardized language configuration using LanguageBuilder
//! - Default editor for basic operations

use super::{LanguageBuilder, LanguageName};
use anyhow::Result;

pub fn language() -> Result<super::LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::Ruby,
        &["rb"],
        tree_sitter_ruby::LANGUAGE.into(),
    )
    .build()
}
