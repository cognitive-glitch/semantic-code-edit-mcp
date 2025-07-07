//! Validation system for semantic code editing operations.
//!
//! This module provides comprehensive validation for code editing operations,
//! ensuring that modifications maintain syntactic and semantic correctness.
//! It includes both syntax validation (using tree-sitter) and context validation
//! (using language-specific rules).
//!
//! ## Key Components
//!
//! - [`ContextValidator`]: Validates edits against language-specific semantic rules
//!
//! ## Validation Types
//!
//! - **Syntax validation**: Tree-sitter parsing validation for all languages
//! - **Context validation**: Language-specific semantic rules (e.g., no functions in struct fields)
//! - **UTF-8 validation**: Ensures text boundary safety
//!
//! ## Features
//!
//! - **Pre-edit validation**: Prevents invalid operations before applying changes
//! - **Custom validation queries**: Tree-sitter queries for semantic rules
//! - **Error reporting**: Detailed error messages with line numbers

mod context_validator;
pub use context_validator::ContextValidator;
