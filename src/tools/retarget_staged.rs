//! Retarget staged operation tool for adjusting edit targeting.
//!
//! This module implements the `retarget_staged` MCP tool which allows modifying
//! the targeting of a staged operation without changing the content. Features include:
//! - Modify selector targeting for staged operations
//! - Preview changes with new targeting
//! - Validate new selector configuration
//! - Keep existing content unchanged
//! - Return updated preview with diff

use crate::{selector::Selector, state::SemanticEditTools, tools::ToolHelpers};

use crate::error::SemanticEditError;
use anyhow::Result;
use mcplease::{
    traits::{Tool, WithExamples},
    types::Example,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Change the targeting of an already-staged operation without rewriting the content
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename = "retarget_staged")]
pub struct RetargetStaged {
    #[serde(flatten)]
    pub selector: Selector,
}

impl WithExamples for RetargetStaged {
    fn examples() -> Vec<Example<Self>> {
        vec![
            // Example {
            //     description: "After staging content to add a struct field, retarget from field_declaration to field_declaration_list for better insertion point",
            //     item: Self {
            //         selector: NodeSelector {
            //             anchor_text: "pub created_at:".into(),
            //             ancestor_node_type: Some("field_declaration_list".into()),
            //             position: None,
            //         },
            //     },
            // },
            // Example {
            //     description: "Move JSON insertion from inside an object to after the entire object pair",
            //     item: Self {
            //         selector: NodeSelector {
            //             anchor_text: "\"cache\"".into(),
            //             ancestor_node_type: Some("pair".into()),
            //             position: None,
            //         },
            //     },
            // },
            // Example {
            //     description: "Adjust function insertion from declaration_list to function_item scope",
            //     item: Self {
            //         selector: NodeSelector {
            //             anchor_text: "pub fn validate_email".into(),
            //             ancestor_node_type: Some("function_item".into()),
            //             position: None,
            //         },
            //     },
            // },
            // Example {
            //     description: "Use exploration mode first to see all targeting options before retargeting",
            //     item: Self {
            //         selector: NodeSelector {
            //             anchor_text: "impl User".into(),
            //             ancestor_node_type: None,
            //             position: None,
            //         },
            //     },
            // },
        ]
    }
}

impl Tool<SemanticEditTools> for RetargetStaged {
    fn execute(self, state: &mut SemanticEditTools) -> Result<String> {
        let Self { selector } = self;

        let staged_operation = state
            .modify_staged_operation(None, |op| op.retarget(selector))?
            .ok_or_else(|| anyhow::Error::from(SemanticEditError::OperationNotStaged))?;

        let editor = state.create_editor_from_operation(staged_operation)?;
        let (message, staged_operation) = editor.preview()?;
        if staged_operation.is_some() {
            // leave failed operations in place
            state.stage_operation(None, staged_operation)?;
        }
        Ok(message)
    }
}
