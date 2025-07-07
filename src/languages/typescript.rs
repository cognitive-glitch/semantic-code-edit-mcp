//! TypeScript language support with tree-sitter parsing.
//!
//! This module provides TypeScript-specific editing capabilities including:
//! - Tree-sitter parsing for AST-aware operations
//! - Support for .ts files
//! - Standardized language configuration using LanguageBuilder
//! - Default editor for basic operations

use crate::languages::{LanguageBuilder, LanguageCommon, LanguageName};
use anyhow::Result;

pub fn language() -> Result<LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::Typescript,
        &["ts"],
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
    )
    .with_validation_query(include_str!("../../queries/typescript/validation.scm"))
    .build()
}
