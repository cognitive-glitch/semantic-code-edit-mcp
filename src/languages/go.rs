//! Go language support with tree-sitter parsing.
//!
//! This module provides Go-specific editing capabilities including:
//! - Tree-sitter parsing for AST-aware operations
//! - Support for .go files
//! - Standardized language configuration using LanguageBuilder
//! - Default editor for basic operations

use super::{LanguageBuilder, LanguageName};
use anyhow::Result;

pub fn language() -> Result<super::LanguageCommon> {
    LanguageBuilder::new(LanguageName::Go, &["go"], tree_sitter_go::LANGUAGE.into()).build()
}
