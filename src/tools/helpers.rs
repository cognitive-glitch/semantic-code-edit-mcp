//! Helper utilities for MCP tools.
//!
//! This module provides common functionality shared across different MCP tools.
//! It centralizes patterns and reduces code duplication between tools. Features include:
//! - ToolHelpers trait for common operations
//! - Editor creation from staged operations
//! - Shared validation and error handling
//! - Centralized operation management patterns

use crate::editor::Editor;
use crate::error::SemanticEditError;
use crate::state::{SemanticEditTools, StagedOperation};
use anyhow::Result;

/// Helper trait providing common functionality across tools
pub trait ToolHelpers {
    /// Create an Editor from a staged operation, centralizing the common pattern
    /// used in commit_staged.rs and retarget_staged.rs
    fn create_editor_from_staged(&mut self, session_id: Option<&str>) -> Result<Editor>;

    /// Create an Editor from a taken staged operation
    fn create_editor_from_operation(&self, staged_operation: StagedOperation) -> Result<Editor>;
}

impl ToolHelpers for SemanticEditTools {
    fn create_editor_from_staged(&mut self, session_id: Option<&str>) -> Result<Editor> {
        let staged_operation = self
            .get_staged_operation(session_id)?
            .ok_or_else(|| anyhow::Error::from(SemanticEditError::OperationNotStaged))?;

        Editor::from_staged_operation(staged_operation, self.language_registry())
    }

    fn create_editor_from_operation(&self, staged_operation: StagedOperation) -> Result<Editor> {
        Editor::from_staged_operation(staged_operation, self.language_registry())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filesystem::TestFileOperations;

    fn create_test_state() -> Result<SemanticEditTools> {
        let file_ops = Box::new(TestFileOperations::new());
        SemanticEditTools::with_file_operations(None, file_ops)
    }

    #[test]
    fn create_editor_from_staged_returns_error_when_no_operation_staged() -> Result<()> {
        let mut state = create_test_state()?;

        let result = state.create_editor_from_staged(None);

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("no operation is currently staged"));
        }
        Ok(())
    }

    #[test]
    fn create_editor_from_operation_creates_editor_successfully() -> Result<()> {
        use crate::selector::{Operation, Selector};
        use crate::state::StagedOperation;
        use std::io::Write;
        use tempfile::NamedTempFile;

        let state = create_test_state()?;

        // Create a temporary file with test content
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "fn test() {{}}")?;
        let test_path = temp_file.path().to_path_buf();

        let language = state
            .language_registry()
            .get_language_with_hint(&test_path, None)?;

        let staged_op = StagedOperation {
            selector: Selector {
                anchor: "fn test".to_string(),
                operation: Operation::InsertAfter,
                end: None,
            },
            content: "\n    println!(\"Added!\");".to_string(),
            file_path: test_path,
            language_name: language.name(),
            edit_position: None,
        };

        let editor = state.create_editor_from_operation(staged_op)?;

        // Verify editor was created successfully by calling preview
        assert!(editor.preview().is_ok());
        Ok(())
    }
}
