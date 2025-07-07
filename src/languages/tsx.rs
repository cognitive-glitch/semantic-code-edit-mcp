//! TSX (TypeScript JSX) language support with tree-sitter parsing.
//!
//! This module provides TSX-specific editing capabilities including:
//! - Tree-sitter parsing for AST-aware operations
//! - Support for .tsx files
//! - Specialized TSX editor for React/JSX syntax
//! - Standardized language configuration using LanguageBuilder

use crate::languages::{LanguageBuilder, LanguageCommon, LanguageName, traits::LanguageEditor};
use anyhow::Result;

pub fn language() -> Result<LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::Tsx,
        &["tsx"],
        tree_sitter_typescript::LANGUAGE_TSX.into(),
    )
    .with_editor(Box::new(TypescriptEditor::new()))
    .build()
}

pub struct TypescriptEditor;

impl Default for TypescriptEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl TypescriptEditor {
    pub fn new() -> Self {
        Self
    }
}

impl LanguageEditor for TypescriptEditor {}
