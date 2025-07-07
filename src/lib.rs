//! # Semantic Code Edit MCP
//!
//! A Model Context Protocol (MCP) server that provides semantic code editing capabilities
//! using tree-sitter for AST-aware code transformations. This crate enables precise,
//! syntax-aware code modifications across multiple programming languages.
//!
//! ## Features
//!
//! - **AST-aware editing**: Uses tree-sitter for precise code transformations
//! - **Multi-language support**: Supports 17+ programming languages
//! - **Staged operations**: Preview changes before applying them
//! - **Validation**: Syntax and semantic validation before modifications
//! - **Session management**: Context and state management across operations
//! - **Performance optimization**: Configurable caching and statistics
//!
//! ## Core Components
//!
//! - [`editor`]: Main editing engine with validation and formatting
//! - [`languages`]: Language-specific parsers and editors
//! - [`selector`]: Code targeting system for precise edits
//! - [`tools`]: MCP tools for code operations
//! - [`validation`]: Syntax and semantic validation
//! - [`state`]: Session and cache management
//! - [`error`]: Comprehensive error handling
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use semantic_code_edit_mcp::state::SemanticEditTools;
//! use semantic_code_edit_mcp::filesystem::StdFileOperations;
//!
//! // Create tools instance with default configuration
//! let tools = SemanticEditTools::with_standard_operations(None)?;
//!
//! // Tools are used via MCP protocol for code editing operations
//! # Ok::<(), anyhow::Error>(())
//! ```

#![allow(clippy::collapsible_if)]
#![deny(dead_code)]

pub mod editor;
pub mod error;
pub mod filesystem;
pub mod languages;
pub mod selector;
pub mod state;
pub mod tools;
pub mod validation;
