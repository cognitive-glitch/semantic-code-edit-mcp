//! # Filesystem Operations
//!
//! This module provides an abstraction layer for filesystem operations,
//! enabling dependency injection for testing and different deployment scenarios.

use anyhow::Result;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Abstraction for file system operations
///
/// This trait allows injecting different file system implementations
/// for production vs testing scenarios, following the dependency inversion principle.
pub trait FileOperations: Send + Sync {
    /// Write content to a file at the given path
    fn write_file(&self, path: PathBuf, content: String) -> Result<()>;
}

/// Standard filesystem operations using std::fs
///
/// This is the production implementation that writes to the actual filesystem.
#[derive(Debug, Default)]
pub struct StdFileOperations;

impl FileOperations for StdFileOperations {
    fn write_file(&self, path: PathBuf, content: String) -> Result<()> {
        std::fs::write(path, content).map_err(Into::into)
    }
}

/// Test filesystem operations that capture writes in memory
///
/// This implementation captures all write operations for testing purposes,
/// allowing tests to verify what would be written without side effects.
#[derive(Debug, Default)]
pub struct TestFileOperations {
    captured_writes: Arc<Mutex<Vec<(PathBuf, String)>>>,
}

impl TestFileOperations {
    /// Create a new test file operations instance
    pub fn new() -> Self {
        Self {
            captured_writes: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Get all captured write operations
    pub fn get_captured_writes(&self) -> Vec<(PathBuf, String)> {
        self.captured_writes
            .lock()
            .expect("Mutex not poisoned")
            .clone()
    }

    /// Get the content of the last write operation, if any
    pub fn get_last_write_content(&self) -> Option<String> {
        self.captured_writes
            .lock()
            .expect("Mutex not poisoned")
            .last()
            .map(|(_, content)| content.clone())
    }

    /// Clear all captured writes
    pub fn clear_captures(&self) {
        self.captured_writes
            .lock()
            .expect("Mutex not poisoned")
            .clear();
    }

    /// Get the number of captured writes
    pub fn write_count(&self) -> usize {
        self.captured_writes
            .lock()
            .expect("Mutex not poisoned")
            .len()
    }
}

impl FileOperations for TestFileOperations {
    fn write_file(&self, path: PathBuf, content: String) -> Result<()> {
        self.captured_writes
            .lock()
            .expect("Mutex not poisoned")
            .push((path, content));
        Ok(())
    }
}

// Implement FileOperations for Arc<TestFileOperations> to support shared ownership in tests
impl<T: FileOperations + ?Sized> FileOperations for std::sync::Arc<T> {
    fn write_file(&self, path: PathBuf, content: String) -> Result<()> {
        (**self).write_file(path, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_file_operations_captures_writes() {
        let ops = TestFileOperations::new();
        let path = PathBuf::from("test.txt");
        let content = "hello world".to_string();

        ops.write_file(path.clone(), content.clone()).unwrap();

        let captures = ops.get_captured_writes();
        assert_eq!(captures.len(), 1);
        assert_eq!(captures[0].0, path);
        assert_eq!(captures[0].1, content);
    }

    #[test]
    fn test_file_operations_multiple_writes() {
        let ops = TestFileOperations::new();

        ops.write_file(PathBuf::from("file1.txt"), "content1".to_string())
            .unwrap();
        ops.write_file(PathBuf::from("file2.txt"), "content2".to_string())
            .unwrap();

        assert_eq!(ops.write_count(), 2);
        assert_eq!(ops.get_last_write_content(), Some("content2".to_string()));
    }

    #[test]
    fn test_file_operations_clear_captures() {
        let ops = TestFileOperations::new();
        ops.write_file(PathBuf::from("test.txt"), "content".to_string())
            .unwrap();

        assert_eq!(ops.write_count(), 1);
        ops.clear_captures();
        assert_eq!(ops.write_count(), 0);
    }
}
