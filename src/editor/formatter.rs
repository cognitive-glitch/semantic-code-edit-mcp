//! # Formatter Module
//!
//! This module provides language-specific code formatting capabilities.
//! It delegates to each language's formatter implementation (e.g., rustfmt for Rust, prettier for JS).
//!
//! ## Features
//!
//! - Supports all languages with custom formatters (Rust, Python, TOML, etc.)
//! - Falls back to no-op formatting for languages without formatters
//! - Provides clear error messages when formatting fails
//!
//! ## Example
//!
//! ```ignore
//! use semantic_code_edit_mcp::editor::formatter::Formatter;
//!
//! // Format code using language-specific formatter
//! match Formatter::format_code(language, source_code) {
//!     Ok(formatted) => println!("Formatted code: {}", formatted),
//!     Err(e) => eprintln!("Formatting failed: {}", e),
//! }
//! ```

use crate::languages::LanguageCommon;
use anyhow::{Result, anyhow};

/// Handles code formatting for different languages
pub struct Formatter;

impl Formatter {
    /// Formats source code using language-specific formatter
    pub fn format_code(language: &LanguageCommon, source: &str) -> Result<String> {
        language.editor().format_code(source).map_err(|e| {
            anyhow!(
                "The formatter has encountered the following error making \
                 that change, so the file has not been modified. The tool has \
                 prevented what it believes to be an unsafe edit. Please try a \
                 different edit.\n\n\
                 {e}"
            )
        })
    }
}
