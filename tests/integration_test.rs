//! Integration tests for semantic-code-edit-mcp
//!
//! Tests the complete workflow through the public API

use semantic_code_edit_mcp::{
    editor::Editor,
    languages::LanguageRegistry,
    selector::{Operation, Selector},
};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to create a test file with content
fn create_test_file(dir: &TempDir, filename: &str, content: &str) -> PathBuf {
    let file_path = dir.path().join(filename);
    fs::write(&file_path, content).unwrap();
    file_path
}

#[test]
fn test_editor_workflow_rust_insert_after() {
    let temp_dir = TempDir::new().unwrap();

    // Create a Rust file
    let content = r#"fn main() {
    println!("Hello, world!");
}"#;
    let file_path = create_test_file(&temp_dir, "main.rs", content);

    // Create language registry
    let registry = LanguageRegistry::new().unwrap();
    let language = registry.get_language_with_hint(&file_path, None).unwrap();

    // Create selector
    let selector = Selector {
        operation: Operation::InsertAfter,
        anchor: r#"println!("Hello, world!");"#.to_string(),
        end: None,
    };

    // Create editor with new content
    let new_content = r#"
    println!("This is a new line!");"#;

    let editor = Editor::new(
        new_content.to_string(),
        selector,
        language,
        file_path.clone(),
        None,
    )
    .unwrap();

    // Test commit (preview is tested separately)
    let (_msg, output, path) = editor.commit().unwrap();
    assert_eq!(path, file_path);

    // The output contains the new content
    let output_str = output.as_ref().unwrap();
    assert!(output_str.contains("This is a new line!"));
    assert!(output_str.contains("Hello, world!"));

    // Write the output to the file to verify the content is correct
    fs::write(&file_path, output_str).unwrap();

    // Verify file was updated
    let updated_content = fs::read_to_string(&file_path).unwrap();
    assert!(updated_content.contains("This is a new line!"));
    assert!(updated_content.contains("Hello, world!"));
}

#[test]
fn test_editor_workflow_python_replace_node() {
    let temp_dir = TempDir::new().unwrap();

    // Create a Python file
    let content = r#"def greet(name):
    print(f"Hello, {name}!")

def main():
    greet("World")
"#;
    let file_path = create_test_file(&temp_dir, "greet.py", content);

    let registry = LanguageRegistry::new().unwrap();
    let language = registry.get_language_with_hint(&file_path, None).unwrap();

    let selector = Selector {
        operation: Operation::ReplaceNode,
        anchor: "def greet(name):".to_string(),
        end: None,
    };

    let new_content = r#"def greet(name, greeting="Hello"):
    print(f"{greeting}, {name}!")"#;

    let editor = Editor::new(
        new_content.to_string(),
        selector,
        language,
        file_path.clone(),
        None,
    )
    .unwrap();

    // Commit directly
    let (_msg, output, _path) = editor.commit().unwrap();

    // Verify
    assert!(output.as_ref().unwrap().contains(r#"greeting="Hello""#));
    assert!(
        !output
            .as_ref()
            .unwrap()
            .contains(r#"print(f"Hello, {name}!")"#)
    );
}

#[test]
fn test_editor_workflow_javascript_insert_before() {
    let temp_dir = TempDir::new().unwrap();

    let content = r#"function calculate(a, b) {
    return a + b;
}

console.log(calculate(5, 3));
"#;
    let file_path = create_test_file(&temp_dir, "calc.js", content);

    let registry = LanguageRegistry::new().unwrap();
    let language = registry.get_language_with_hint(&file_path, None).unwrap();

    let selector = Selector {
        operation: Operation::InsertBefore,
        anchor: "console.log".to_string(),
        end: None,
    };

    let new_content = "// Test the calculate function\n";

    let editor = Editor::new(
        new_content.to_string(),
        selector,
        language,
        file_path.clone(),
        None,
    )
    .unwrap();

    let (msg, output, _path) = editor.commit().unwrap();

    // For debugging
    println!("Commit message: {}", msg);
    println!("Output: {:?}", output);

    // Check if we have output
    assert!(output.is_some(), "Expected output from commit");

    // Verify order
    let output_str = output.as_ref().unwrap();
    let lines: Vec<&str> = output_str.lines().collect();
    let comment_idx = lines.iter().position(|&l| l.contains("// Test")).unwrap();
    let console_idx = lines
        .iter()
        .position(|&l| l.contains("console.log"))
        .unwrap();
    assert!(comment_idx < console_idx);
}

#[test]
fn test_editor_workflow_replace_range() {
    let temp_dir = TempDir::new().unwrap();

    let content = r#"fn process_data() {
    // START_REPLACE
    let data = vec![1, 2, 3];
    for item in data {
        println!("{}", item);
    }
    // END_REPLACE
}
"#;
    let file_path = create_test_file(&temp_dir, "process.rs", content);

    let registry = LanguageRegistry::new().unwrap();
    let language = registry.get_language_with_hint(&file_path, None).unwrap();

    let selector = Selector {
        operation: Operation::ReplaceRange,
        anchor: "// START_REPLACE".to_string(),
        end: Some("// END_REPLACE".to_string()),
    };

    let new_content = r#"// START_REPLACE
    let data = vec![10, 20, 30, 40];
    let sum: i32 = data.iter().sum();
    println!("Sum: {}", sum);
    // END_REPLACE"#;

    let editor = Editor::new(
        new_content.to_string(),
        selector,
        language,
        file_path.clone(),
        None,
    )
    .unwrap();

    let (_msg, output, _path) = editor.commit().unwrap();

    // Verify
    assert!(output.as_ref().unwrap().contains("Sum: {}"));
    assert!(output.as_ref().unwrap().contains("data.iter().sum()"));
    assert!(!output.as_ref().unwrap().contains("for item in data"));
}

#[test]
fn test_error_handling_invalid_syntax() {
    let temp_dir = TempDir::new().unwrap();

    let content = r#"fn main() {
    println!("Hello");
}"#;
    let file_path = create_test_file(&temp_dir, "syntax.rs", content);

    let registry = LanguageRegistry::new().unwrap();
    let language = registry.get_language_with_hint(&file_path, None).unwrap();

    let selector = Selector {
        operation: Operation::ReplaceNode,
        anchor: "fn main()".to_string(),
        end: None,
    };

    // Invalid syntax
    let new_content = "fn main( { // Invalid syntax";

    let result = Editor::new(new_content.to_string(), selector, language, file_path, None);

    // Should fail validation
    assert!(result.is_err());
}

#[test]
fn test_error_handling_anchor_not_found() {
    let temp_dir = TempDir::new().unwrap();

    let content = "fn main() {}";
    let file_path = create_test_file(&temp_dir, "empty.rs", content);

    let registry = LanguageRegistry::new().unwrap();
    let language = registry.get_language_with_hint(&file_path, None).unwrap();

    let selector = Selector {
        operation: Operation::InsertAfter,
        anchor: "nonexistent anchor".to_string(),
        end: None,
    };

    let result = Editor::new("content".to_string(), selector, language, file_path, None);

    assert!(result.is_err());
}

#[test]
fn test_all_operation_types() {
    let temp_dir = TempDir::new().unwrap();

    // Test InsertAfterNode
    let content = r#"fn first() {
    println!("first");
}

fn second() {
    println!("second");
}"#;
    let file_path = create_test_file(&temp_dir, "ops_test.rs", content);

    let registry = LanguageRegistry::new().unwrap();
    let language = registry.get_language_with_hint(&file_path, None).unwrap();

    // InsertAfterNode
    let selector = Selector {
        operation: Operation::InsertAfterNode,
        anchor: "fn first()".to_string(),
        end: None,
    };

    let new_content = "\nfn between() {\n    println!(\"between\");\n}";

    let editor = Editor::new(
        new_content.to_string(),
        selector,
        language,
        file_path.clone(),
        None,
    )
    .unwrap();

    let (_msg, output, _path) = editor.commit().unwrap();
    assert!(
        output.as_ref().unwrap().contains("between"),
        "InsertAfterNode didn't add 'between' function"
    );
    fs::write(&file_path, output.as_ref().unwrap()).unwrap();

    // ReplaceExact
    let selector = Selector {
        operation: Operation::ReplaceExact,
        anchor: "between".to_string(),
        end: None,
    };

    let editor = Editor::new(
        "middle".to_string(),
        selector,
        language,
        file_path.clone(),
        None,
    )
    .unwrap();

    let (_msg, output, _path) = editor.commit().unwrap();
    assert!(output.as_ref().unwrap().contains("middle"));
    // ReplaceExact only replaces the exact match, not all occurrences
    assert!(output.as_ref().unwrap().contains("println!(\"between\")")); // String inside println should remain
}

#[test]
fn test_multi_language_support() {
    let languages = vec![
        ("test.rs", "fn main() {}", "// Rust comment"),
        ("test.py", "def main():\n    pass", "# Python comment"),
        ("test.js", "function main() {}", "// JS comment"),
        ("test.go", "func main() {}", "// Go comment"),
        ("test.java", "class Test {}", "// Java comment"),
    ];

    let temp_dir = TempDir::new().unwrap();
    let registry = LanguageRegistry::new().unwrap();

    for (filename, content, comment) in languages {
        let file_path = create_test_file(&temp_dir, filename, content);
        let language = registry.get_language_with_hint(&file_path, None).unwrap();

        let selector = Selector {
            operation: Operation::InsertAfter,
            anchor: content.lines().next().unwrap().to_string(),
            end: None,
        };

        let editor = Editor::new(
            format!("\n{}", comment),
            selector,
            language,
            file_path.clone(),
            None,
        );

        assert!(editor.is_ok(), "Failed for {}", filename);

        let editor = editor.unwrap();
        let result = editor.commit();
        assert!(
            result.is_ok(),
            "Failed to commit for {}: {:?}",
            filename,
            result
        );
    }
}

#[test]
fn test_json_formatting_preservation() {
    let temp_dir = TempDir::new().unwrap();

    // Create JSON with specific formatting (2-space indent)
    let content = r#"{
  "name": "test",
  "version": "1.0.0",
  "dependencies": {
    "lodash": "4.17.21"
  }
}"#;
    let file_path = create_test_file(&temp_dir, "package.json", content);

    let registry = LanguageRegistry::new().unwrap();
    let language = registry.get_language_with_hint(&file_path, None).unwrap();

    let selector = Selector {
        operation: Operation::InsertAfter,
        anchor: r#""lodash": "4.17.21""#.to_string(),
        end: None,
    };

    let new_content = r#",
    "axios": "1.0.0""#;

    let editor = Editor::new(
        new_content.to_string(),
        selector,
        language,
        file_path.clone(),
        None,
    )
    .unwrap();

    let (_msg, output, _path) = editor.commit().unwrap();

    // Verify formatting was preserved
    assert!(output.as_ref().unwrap().contains(r#""axios": "1.0.0""#));
    assert!(output.as_ref().unwrap().contains("  ")); // 2-space indentation preserved
}

#[test]
fn test_toml_editing() {
    let temp_dir = TempDir::new().unwrap();

    let content = r#"[package]
name = "test"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#;
    let file_path = create_test_file(&temp_dir, "Cargo.toml", content);

    let registry = LanguageRegistry::new().unwrap();
    let language = registry.get_language_with_hint(&file_path, None).unwrap();

    let selector = Selector {
        operation: Operation::InsertAfter,
        anchor: r#"serde = "1.0""#.to_string(),
        end: None,
    };

    let editor = Editor::new(
        "\ntokio = \"1.0\"".to_string(),
        selector,
        language,
        file_path.clone(),
        None,
    )
    .unwrap();

    let (_msg, output, _path) = editor.commit().unwrap();
    assert!(output.as_ref().unwrap().contains("tokio = \"1.0\""));
}

#[test]
fn test_delete_via_empty_replace() {
    let temp_dir = TempDir::new().unwrap();

    let content = r#"fn keep() {}
fn delete() {}
fn also_keep() {}"#;
    let file_path = create_test_file(&temp_dir, "delete.rs", content);

    let registry = LanguageRegistry::new().unwrap();
    let language = registry.get_language_with_hint(&file_path, None).unwrap();

    let selector = Selector {
        operation: Operation::ReplaceNode,
        anchor: "fn delete()".to_string(),
        end: None,
    };

    // Empty content means delete
    let editor = Editor::new(String::new(), selector, language, file_path.clone(), None).unwrap();

    let (_msg, output, _path) = editor.commit().unwrap();

    assert!(output.as_ref().unwrap().contains("fn keep()"));
    assert!(output.as_ref().unwrap().contains("fn also_keep()"));
    assert!(!output.as_ref().unwrap().contains("fn delete()"));
}

#[test]
fn test_validation_query_rust() {
    let temp_dir = TempDir::new().unwrap();

    // Rust validation should catch functions in struct fields
    let content = r#"struct Config {
    name: String,
}"#;
    let file_path = create_test_file(&temp_dir, "config.rs", content);

    let registry = LanguageRegistry::new().unwrap();
    let language = registry.get_language_with_hint(&file_path, None).unwrap();

    let selector = Selector {
        operation: Operation::InsertAfter,
        anchor: "name: String,".to_string(),
        end: None,
    };

    // Try to add a function in struct fields (should fail validation)
    let new_content = "\n    fn invalid() {}";

    let editor = Editor::new(new_content.to_string(), selector, language, file_path, None);

    // Should either fail or succeed with warning
    match editor {
        Ok(ed) => {
            let (preview, _) = ed.preview().unwrap();
            // If it succeeds, it should show a validation warning
            assert!(
                preview.contains("invalid syntax")
                    || preview.contains("caution")
                    || preview.contains("warning")
                    || preview.contains("Validation")
            );
        }
        Err(e) => {
            // Or it should fail with validation error
            assert!(e.to_string().contains("validation") || e.to_string().contains("Validation"));
        }
    }
}
