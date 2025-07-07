//! MCP tools for semantic code editing operations.
//!
//! This module defines the Model Context Protocol (MCP) tools that provide the main
//! interface for semantic code editing operations. Each tool implements a specific
//! aspect of the editing workflow.
//!
//! ## Available Tools
//!
//! - [`StageOperation`]: Stage a code editing operation for preview
//! - [`RetargetStaged`]: Modify the targeting of a staged operation
//! - [`CommitStaged`]: Execute a staged operation
//! - [`SetContext`]: Set the working directory context for relative paths
//! - [`OpenFiles`]: Read files with optional diff support
//!
//! ## Workflow
//!
//! 1. **Stage**: Use `stage_operation` to preview changes
//! 2. **Retarget** (optional): Use `retarget_staged` to adjust targeting
//! 3. **Commit**: Use `commit_staged` to apply changes
//!
//! ## Helper Traits
//!
//! - [`ToolHelpers`]: Common functionality shared across tools

use crate::state::SemanticEditTools;

// Load helper module from tools/ directory
#[path = "tools/helpers.rs"]
pub mod helpers;

// Re-export ToolHelpers trait
pub use helpers::ToolHelpers;

mcplease::tools!(
    SemanticEditTools,
    (StageOperation, stage_operation, "stage_operation"),
    (RetargetStaged, retarget_staged, "retarget_staged"),
    (CommitStaged, commit_staged, "commit_staged"),
    (SetContext, set_context, "set_context"),
    (OpenFiles, open_files, "open_files")
);
