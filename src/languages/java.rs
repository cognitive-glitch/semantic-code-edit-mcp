//! Java language support with tree-sitter parsing.
//!
//! This module provides Java-specific editing capabilities including:
//! - Tree-sitter parsing for AST-aware operations
//! - Support for .java files
//! - Standardized language configuration using LanguageBuilder
//! - Default editor for basic operations

use super::{LanguageBuilder, LanguageName};
use anyhow::Result;

pub fn language() -> Result<super::LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::Java,
        &["java"],
        tree_sitter_java::LANGUAGE.into(),
    )
    .build()
}
