//! C++ language support with tree-sitter parsing.
//!
//! This module provides C++-specific editing capabilities including:
//! - Tree-sitter parsing for AST-aware operations
//! - Support for .cpp, .cc, .cxx, .c++, .hpp, .hh, .hxx files
//! - Standardized language configuration using LanguageBuilder
//! - Default editor for basic operations

use super::{LanguageBuilder, LanguageName};
use anyhow::Result;

pub fn language() -> Result<super::LanguageCommon> {
    LanguageBuilder::new(
        LanguageName::Cpp,
        &["cpp", "cxx", "cc", "c++", "hpp", "hxx", "h++"],
        tree_sitter_cpp::LANGUAGE.into(),
    )
    .build()
}
