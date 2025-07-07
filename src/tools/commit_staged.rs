//! Commit staged operation tool for applying code edits.
//!
//! This module implements the `commit_staged` MCP tool which executes a previously
//! staged operation, applying the changes to the actual file. Features include:
//! - Executes the currently staged operation
//! - Validates the operation exists
//! - Applies changes to the file system
//! - Returns success confirmation
//! - Clears the staged operation after commit

use crate::error::SemanticEditError;
use crate::state::SemanticEditTools;
use crate::tools::ToolHelpers;
use anyhow::Result;
use mcplease::traits::{Tool, WithExamples};
use mcplease::types::Example;
use serde::{Deserialize, Serialize};

/// Execute the currently staged operation
#[derive(Serialize, Deserialize, Debug, schemars::JsonSchema)]
#[serde(rename = "commit_staged")]
pub struct CommitStaged {
    /// Confirm that you want to execute the staged operation
    #[serde(default = "default_acknowledge")]
    pub acknowledge: bool,
    // this is commented out temporarily as an experiment in usability
    // /// Optional session identifier
    // pub session_id: Option<String>,
}

fn default_acknowledge() -> bool {
    true
}

impl WithExamples for CommitStaged {
    fn examples() -> Vec<Example<Self>> {
        vec![Example {
            description: "Commit the currently staged operation",
            item: Self { acknowledge: true },
        }]
    }
}

impl Tool<SemanticEditTools> for CommitStaged {
    fn execute(self, state: &mut SemanticEditTools) -> Result<String> {
        let Self { acknowledge } = self;

        if !acknowledge {
            return Err(anyhow::Error::from(
                SemanticEditError::OperationNotAcknowledged,
            ));
        }

        let staged_operation = state
            .take_staged_operation(None)?
            .ok_or_else(|| anyhow::Error::from(SemanticEditError::OperationNotStaged))?;

        let editor = state.create_editor_from_operation(staged_operation)?;
        let (message, output, output_path) = editor.commit()?;

        if let Some(output) = output {
            state.file_operations().write_file(output_path, output)?;
        }

        Ok(message)
    }
}
