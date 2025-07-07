//! TDD Phase 1 (Red): Failing tests for standardizing TestFileOperations error handling
//!
//! These tests define the expected behavior for standardizing all methods to use
//! safe error handling patterns consistently.

use semantic_code_edit_mcp::filesystem::{
    FileOperations, TestFileOperations, TestFileOperationsError,
};
use std::path::PathBuf;

#[cfg(test)]
mod filesystem_standardization_tdd {
    use super::*;

    /// TDD RED: Test that we can use a consistent error handling pattern
    /// All methods should have both unsafe (legacy) and safe versions
    #[test]
    fn test_consistent_error_handling_api() {
        let ops = TestFileOperations::new();

        // Write some test data
        ops.write_file(PathBuf::from("test.txt"), "content".to_string())
            .unwrap();

        // All methods should have both safe and unsafe versions available
        // This drives the design for consistent API

        // Legacy methods (with expect) - should still work for backward compatibility
        let _count = ops.write_count();
        let _captures = ops.get_captured_writes();
        let _last = ops.get_last_write_content();

        // Safe methods (return Result) - should be available
        let _safe_count = ops.write_count_safe().unwrap();
        let _safe_captures = ops.get_captured_writes_safe().unwrap();
        let _safe_last = ops.get_last_write_content_safe().unwrap();
        let _safe_clear = ops.clear_captures_safe();

        // Both APIs should return the same values
        assert_eq!(ops.write_count(), ops.write_count_safe().unwrap());
        assert_eq!(
            ops.get_captured_writes().len(),
            ops.get_captured_writes_safe().unwrap().len()
        );
        assert_eq!(
            ops.get_last_write_content(),
            ops.get_last_write_content_safe().unwrap()
        );
    }

    /// TDD RED: Test that legacy methods can eventually be deprecated
    /// This test defines a future where legacy expect() calls are removed
    #[test]
    fn test_legacy_methods_eventually_deprecated() {
        let ops = TestFileOperations::new();

        // Future design: all methods return Result by default
        // This test will fail until we implement this pattern

        // For now, test that safe methods exist and work
        assert!(ops.write_count_safe().is_ok());
        assert!(ops.get_captured_writes_safe().is_ok());
        assert!(ops.get_last_write_content_safe().is_ok());
        assert!(ops.clear_captures_safe().is_ok());

        // Future: we might want to add a compilation feature flag
        // that makes the unsafe methods unavailable for new code
        // This drives the standardization effort
    }

    /// TDD RED: Test for creating a trait that standardizes error handling
    /// This test defines what a standardized error handling trait should look like
    #[test]
    fn test_safe_operations_trait_design() {
        let ops = TestFileOperations::new();

        // We want to be able to treat this as a safe operations provider
        // This test will fail until we implement the trait
        fn use_safe_operations<T: SafeFileTestOperations>(ops: &T) {
            let _ = ops.safe_write_count();
            let _ = ops.safe_get_captured_writes();
            let _ = ops.safe_get_last_write_content();
            let _ = ops.safe_clear_captures();
        }

        // This will fail to compile until we implement the trait
        use_safe_operations(&ops);
    }

    /// TDD RED: Test for error type standardization
    /// All safe methods should use the same error type
    #[test]
    fn test_standardized_error_type() {
        let ops = TestFileOperations::new();

        // All safe methods should return the same error type
        // This drives error type standardization
        let write_count_result = ops.write_count_safe();
        let captures_result = ops.get_captured_writes_safe();
        let last_content_result = ops.get_last_write_content_safe();
        let clear_result = ops.clear_captures_safe();

        // Check that error types are consistent (they should be)
        // This is more of a compile-time check but useful for documentation
        match (
            write_count_result,
            captures_result,
            last_content_result,
            clear_result,
        ) {
            (Err(_), Err(_), Err(_), Err(_)) => {
                // All errors should be the same type - this test documents that expectation
            }
            _ => {
                // Success cases or mixed results - both are fine for this test
            }
        }
    }
}

/// TDD RED: Define the standardized trait we want to implement
/// This trait represents the desired API for safe file test operations
pub trait SafeFileTestOperations {
    type Error;

    fn safe_write_count(&self) -> Result<usize, Self::Error>;
    fn safe_get_captured_writes(&self) -> Result<Vec<(PathBuf, String)>, Self::Error>;
    fn safe_get_last_write_content(&self) -> Result<Option<String>, Self::Error>;
    fn safe_clear_captures(&self) -> Result<(), Self::Error>;
}

// Implementation of the standardized trait
impl SafeFileTestOperations for TestFileOperations {
    type Error = TestFileOperationsError;

    fn safe_write_count(&self) -> Result<usize, Self::Error> {
        // This delegates to the existing safe method
        self.write_count_safe()
    }

    fn safe_get_captured_writes(&self) -> Result<Vec<(PathBuf, String)>, Self::Error> {
        self.get_captured_writes_safe()
    }

    fn safe_get_last_write_content(&self) -> Result<Option<String>, Self::Error> {
        self.get_last_write_content_safe()
    }

    fn safe_clear_captures(&self) -> Result<(), Self::Error> {
        self.clear_captures_safe()
    }
}
