//! Semantic Code Edit MCP Server
//!
//! This is the main entry point for the semantic code editing MCP server.
//! It provides tree-sitter based AST-aware code editing capabilities through
//! the Model Context Protocol (MCP).
//!
//! ## Usage
//!
//! The server communicates via JSON-RPC over stdin/stdout and provides tools for:
//! - Staging code operations with preview
//! - Retargeting operations
//! - Committing changes to files
//! - Setting working directory context
//! - Opening and reading files with diff support

#![allow(clippy::collapsible_if)]

use mcplease::server_info;
use semantic_code_edit_mcp::{state::SemanticEditTools, tools::Tools};
use std::env;

const INSTRUCTIONS: &str = "Semantic code editing with tree-sitter. Use stage_operation to preview changes, retarget_staged to adjust targeting, and commit_staged to apply.";

fn main() {
    let mut state = SemanticEditTools::with_standard_operations(
        env::var("MCP_SESSION_STORAGE_PATH")
            .ok()
            .as_deref()
            .or(Some("~/.ai-tools/sessions/semantic-edit.json")),
    )
    .expect("Failed to initialize SemanticEditTools");

    mcplease::run::<Tools, _>(&mut state, server_info!(), Some(INSTRUCTIONS))
        .expect("Failed to run MCP server")
}
