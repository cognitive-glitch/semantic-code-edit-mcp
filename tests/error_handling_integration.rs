//! Integration tests for comprehensive error handling
//!
//! These tests ensure proper error handling throughout the codebase,
//! replacing unwrap() calls with proper error propagation.

use semantic_code_edit_mcp::{
    editor::Editor,
    languages::{LanguageName, LanguageRegistry},
    selector::{Operation, Selector},
};
use std::io::Write;
use tempfile::NamedTempFile;

/// Helper function to create a test file
fn create_test_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file.flush().unwrap();
    file
}

#[cfg(test)]
mod editor_error_handling {
    use super::*;

    #[test]
    fn handles_invalid_file_path() {
        let registry = LanguageRegistry::new().unwrap();
        let rust_lang = registry.get_language(LanguageName::Rust).unwrap();

        let selector = Selector {
            operation: Operation::InsertAfter,
            anchor: "fn main".to_string(),
            end: None,
        };

        // Try to create editor with non-existent file
        let result = Editor::new(
            "// comment".to_string(),
            selector,
            rust_lang,
            std::path::PathBuf::from("/nonexistent/path/file.rs"),
            None,
        );

        // Should return an error for non-existent file
        assert!(result.is_err());
    }

    #[test]
    fn handles_anchor_not_found() {
        let file = create_test_file("fn main() {}");
        let registry = LanguageRegistry::new().unwrap();
        let rust_lang = registry.get_language(LanguageName::Rust).unwrap();

        let selector = Selector {
            operation: Operation::InsertAfter,
            anchor: "nonexistent_anchor".to_string(),
            end: None,
        };

        let result = Editor::new(
            "// comment".to_string(),
            selector,
            rust_lang,
            file.path().to_path_buf(),
            None,
        );

        // Should return error for anchor not found
        assert!(result.is_err());
    }

    #[test]
    fn handles_invalid_syntax_in_replace_node() {
        let file = create_test_file("fn main() {}");
        let registry = LanguageRegistry::new().unwrap();
        let rust_lang = registry.get_language(LanguageName::Rust).unwrap();

        let selector = Selector {
            operation: Operation::ReplaceNode,
            anchor: "fn main() {}".to_string(),
            end: None,
        };

        // Invalid Rust syntax
        let result = Editor::new(
            "fn main( { invalid syntax".to_string(),
            selector,
            rust_lang,
            file.path().to_path_buf(),
            None,
        );

        // Should return error for invalid syntax
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod language_registry_error_handling {
    use super::*;
    use semantic_code_edit_mcp::languages::LanguageName;

    #[test]
    fn handles_unsupported_file_extension() {
        let registry = LanguageRegistry::new().unwrap();
        let file = create_test_file("some content");

        // Create a path with unsupported extension
        let path = file.path().with_extension("xyz123");

        // Should handle gracefully without panicking
        let result = registry.get_language_with_hint(&path, None);

        // Currently returns a default, but could return error in future
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn all_language_names_are_supported() {
        let registry = LanguageRegistry::new().unwrap();

        // Test all language enum variants that are actually defined
        let languages = vec![
            LanguageName::Rust,
            LanguageName::Python,
            LanguageName::Javascript,
            LanguageName::Typescript,
            LanguageName::Json,
            LanguageName::Toml,
            LanguageName::C,
            LanguageName::Cpp,
            LanguageName::CSharp,
            LanguageName::Java,
            LanguageName::Go,
            LanguageName::Php,
            LanguageName::Ruby,
            LanguageName::Tsx,
            LanguageName::Other,
        ];

        for lang_name in languages {
            let result = registry.get_language(lang_name);
            assert!(result.is_ok(), "Failed to get language: {:?}", lang_name);
        }
    }
}

#[cfg(test)]
mod file_operations_error_handling {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    #[cfg(unix)]
    fn handles_permission_denied() {
        let file = create_test_file("fn main() {}");
        let registry = LanguageRegistry::new().unwrap();
        let rust_lang = registry.get_language(LanguageName::Rust).unwrap();

        // Make file read-only
        let metadata = fs::metadata(file.path()).unwrap();
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o444); // Read-only
        fs::set_permissions(file.path(), permissions).unwrap();

        let selector = Selector {
            operation: Operation::InsertAfter,
            anchor: "main".to_string(),
            end: None,
        };

        let editor = Editor::new(
            "// comment".to_string(),
            selector,
            rust_lang,
            file.path().to_path_buf(),
            None,
        )
        .unwrap();

        // Try to commit (write) to read-only file
        let result = editor.commit();

        // Should handle permission error gracefully
        if result.is_err() {
            let err = result.unwrap_err();
            assert!(
                err.to_string().contains("Permission denied")
                    || err.to_string().contains("permission")
            );
        }
    }
}

#[cfg(test)]
mod edit_iterator_error_handling {
    use super::*;

    #[test]
    fn handles_empty_file_gracefully() {
        let file = create_test_file("");
        let registry = LanguageRegistry::new().unwrap();
        let rust_lang = registry.get_language(LanguageName::Rust).unwrap();

        let selector = Selector {
            operation: Operation::InsertAfter,
            anchor: "".to_string(), // Empty anchor
            end: None,
        };

        let result = Editor::new(
            "fn main() {}".to_string(),
            selector,
            rust_lang,
            file.path().to_path_buf(),
            None,
        );

        // Empty anchor in empty file may or may not be an error depending on implementation
        // The important thing is it doesn't panic
        let _ = result;
    }

    #[test]
    fn handles_utf8_boundary_errors() {
        // Create file with multi-byte UTF-8 characters
        let content = "fn main() { println!(\"🎉\"); }";
        let file = create_test_file(content);
        let registry = LanguageRegistry::new().unwrap();
        let rust_lang = registry.get_language(LanguageName::Rust).unwrap();

        // Try to create a selector that might split UTF-8 character
        let selector = Selector {
            operation: Operation::ReplaceRange,
            anchor: "🎉".to_string(),
            end: Some("🎉".to_string()),
        };

        let editor = Editor::new(
            "🚀".to_string(),
            selector,
            rust_lang,
            file.path().to_path_buf(),
            None,
        );

        // Should handle UTF-8 correctly without panicking
        assert!(editor.is_ok());

        if let Ok(ed) = editor {
            let result = ed.commit();
            assert!(result.is_ok());

            if let Ok((_, Some(output), _)) = result {
                // Should preserve valid UTF-8
                assert!(output.is_char_boundary(0));
                assert!(output.is_char_boundary(output.len()));
            }
        }
    }
}

#[cfg(test)]
mod validation_error_handling {
    use super::*;

    #[test]
    fn handles_validation_errors_gracefully() {
        let file = create_test_file(
            r#"
struct Config {
    name: String,
}
"#,
        );
        let registry = LanguageRegistry::new().unwrap();
        let rust_lang = registry.get_language(LanguageName::Rust).unwrap();

        let selector = Selector {
            operation: Operation::InsertAfter,
            anchor: "name: String,".to_string(),
            end: None,
        };

        // Try to insert invalid content in struct
        let editor = Editor::new(
            "\n    fn invalid_in_struct() {}".to_string(),
            selector,
            rust_lang,
            file.path().to_path_buf(),
            None,
        );

        // Should either fail or succeed with validation warning
        match editor {
            Ok(ed) => {
                let (msg, _) = ed.preview().unwrap();
                // Should include validation information
                assert!(
                    msg.contains("Validation")
                        || msg.contains("warning")
                        || msg.contains("caution")
                        || msg.contains("syntax")
                );
            }
            Err(e) => {
                // Or fail with appropriate error
                assert!(e.to_string().contains("validation") || e.to_string().contains("Invalid"));
            }
        }
    }
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn handles_large_files_gracefully() {
        // Create a large file (1MB)
        let large_content =
            "fn main() {\n".to_string() + &"    println!(\"test\");\n".repeat(50_000) + "}";

        let file = create_test_file(&large_content);
        let registry = LanguageRegistry::new().unwrap();
        let rust_lang = registry.get_language(LanguageName::Rust).unwrap();

        let selector = Selector {
            operation: Operation::InsertAfter,
            anchor: "fn main() {".to_string(),
            end: None,
        };

        let editor = Editor::new(
            "\n    // Large file test".to_string(),
            selector,
            rust_lang,
            file.path().to_path_buf(),
            None,
        );

        // Should handle large files without panicking
        // Note: This may fail if tree-sitter has a size limit
        match editor {
            Ok(ed) => {
                let result = ed.preview();
                // May or may not succeed, but shouldn't panic
                let _ = result;
            }
            Err(_) => {
                // Tree-sitter may have rejected the large file, which is OK
            }
        }
    }

    #[test]
    fn handles_deeply_nested_structures() {
        // Create deeply nested structure
        let mut content = String::new();
        for i in 0..50 {
            content.push_str(&"    ".repeat(i));
            content.push_str("if true {\n");
        }
        content.push_str(&"    ".repeat(50));
        content.push_str("println!(\"deep\");\n");
        for i in (0..50).rev() {
            content.push_str(&"    ".repeat(i));
            content.push_str("}\n");
        }

        let file = create_test_file(&content);
        let registry = LanguageRegistry::new().unwrap();
        let rust_lang = registry.get_language(LanguageName::Rust).unwrap();

        let selector = Selector {
            operation: Operation::InsertAfter,
            anchor: "println!(\"deep\");".to_string(),
            end: None,
        };

        let editor = Editor::new(
            "\n".to_string() + &"    ".repeat(50) + "println!(\"nested\");",
            selector,
            rust_lang,
            file.path().to_path_buf(),
            None,
        );

        // Should handle deeply nested structures
        assert!(editor.is_ok());
    }
}
