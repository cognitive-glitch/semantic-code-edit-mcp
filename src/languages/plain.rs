//! Plain text language support with basic parsing.
//!
//! This module provides fallback language support for unrecognized file types:
//! - Basic tree-sitter parsing for text operations
//! - No specific file extensions (used as fallback)
//! - Minimal editor for plain text editing
//! - Standardized language configuration using LanguageBuilder

use super::{LanguageBuilder, LanguageName};
use anyhow::Result;

pub fn language() -> Result<super::LanguageCommon> {
    LanguageBuilder::new(LanguageName::Other, &[], tree_sitter_plain::LANGUAGE.into()).build()
}
