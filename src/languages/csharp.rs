//! C# language support with tree-sitter parsing.
//!
//! This module provides C#-specific editing capabilities including:
//! - Tree-sitter parsing for AST-aware operations
//! - Support for .cs files
//! - Standardized language configuration using LanguageBuilder
//! - Default editor for basic operations

use super::{LanguageBuilder, LanguageName};
use anyhow::Result;

pub fn language() -> Result<super::LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::CSharp,
        &["cs"],
        tree_sitter_c_sharp::LANGUAGE.into(),
    )
    .build()
}
