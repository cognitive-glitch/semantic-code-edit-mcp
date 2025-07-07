//! Error types for semantic code editing
//!
//! This module defines a comprehensive error hierarchy that replaces all
//! Result<T, String> usages throughout the codebase with proper typed errors.

use std::sync::PoisonError;
use thiserror::Error;

/// The comprehensive error type for all semantic code editing operations
#[derive(Error, Debug)]
pub enum SemanticEditError {
    /// Language-related errors
    #[error("unsupported language: {language}")]
    UnsupportedLanguage { language: String },

    #[error("language parser not available for {language}")]
    ParserUnavailable { language: String },

    /// File and I/O errors
    #[error("file not found: {path}")]
    FileNotFound { path: String },

    #[error("invalid file encoding: {path}")]
    InvalidEncoding { path: String },

    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Edit and validation errors
    #[error("no valid edit locations found for selector")]
    NoValidEditLocations,

    #[error("syntax validation failed: {details}")]
    SyntaxValidationFailed { details: String },

    #[error("edit would create invalid syntax at line {line}")]
    InvalidSyntaxResult { line: usize },

    #[error("edit iterator in invalid state")]
    EditIteratorInvalidState,

    /// Tree-sitter parsing errors
    #[error("failed to parse syntax tree")]
    TreeSitterParseError,

    #[error("invalid tree-sitter query: {query}")]
    InvalidTreeSitterQuery { query: String },

    /// Cache and state errors
    #[error("file cache is poisoned")]
    FileCachePoisoned,

    #[error("session state is invalid")]
    InvalidSessionState,

    /// UTF-8 and text boundary errors
    #[error("invalid UTF-8 boundary at byte position {position}")]
    InvalidUtf8Boundary { position: usize },

    #[error("text range is out of bounds: {start}..{end} (max: {max})")]
    TextRangeOutOfBounds {
        start: usize,
        end: usize,
        max: usize,
    },
}

impl<T> From<PoisonError<T>> for SemanticEditError {
    fn from(_: PoisonError<T>) -> Self {
        SemanticEditError::FileCachePoisoned
    }
}

/// Type alias for Results using our error type
pub type Result<T> = std::result::Result<T, SemanticEditError>;
