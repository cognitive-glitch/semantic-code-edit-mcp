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
