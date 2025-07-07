//! PHP language support with tree-sitter parsing.
//!
//! This module provides PHP-specific editing capabilities including:
//! - Tree-sitter parsing for AST-aware operations
//! - Support for .php files
//! - Standardized language configuration using LanguageBuilder
//! - Default editor for basic operations

use super::{LanguageBuilder, LanguageName};
use anyhow::Result;

pub fn language() -> Result<super::LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::Php,
        &["php"],
        tree_sitter_php::LANGUAGE_PHP.into(),
    )
    .build()
}
