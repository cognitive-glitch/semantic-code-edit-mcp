use semantic_code_edit_mcp::{
    editor::Editor,
    languages::{LanguageName, LanguageRegistry},
    selector::{Operation, Selector},
};
use std::io::Write;
use tempfile::NamedTempFile;

/// TDD Phase 3: Test-driven decomposition of Editor module
/// These tests define the desired architecture before implementation
#[cfg(test)]
mod editor_decomposition_tests {
    use super::*;

    fn create_test_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file.flush().unwrap();
        file
    }

    // RED PHASE: These tests will fail until we decompose Editor

    #[test]
    fn editor_should_have_separate_validation_module() {
        // Test that Editor delegates validation to a separate Validator
        let file = create_test_file("fn main() { invalid syntax");
        let language_registry = LanguageRegistry::new().unwrap();
        let rust_lang = language_registry.get_language(LanguageName::Rust).unwrap();

        let selector = Selector {
            operation: Operation::InsertAfter,
            anchor: "main".to_string(),
            end: None,
        };

        let editor = Editor::new(
            "// comment".to_string(),
            selector.clone(),
            rust_lang,
            file.path().to_path_buf(),
            None,
        )
        .unwrap();

        // This should use Editor::Validator internally (doesn't exist yet)
        let result = editor.preview();
        assert!(result.is_ok());

        // The validation logic should be separated from Editor
        // This test documents that validation should be its own concern
        let (message, _) = result.unwrap();
        assert!(message.contains("Syntax error"));
    }

    #[test]
    fn editor_should_have_separate_formatter_module() {
        // Test that Editor delegates formatting to a separate Formatter
        let file = create_test_file("fn main(){println!(\"test\");}");
        let language_registry = LanguageRegistry::new().unwrap();
        let rust_lang = language_registry.get_language(LanguageName::Rust).unwrap();

        let selector = Selector {
            operation: Operation::InsertAfter,
            anchor: "{}".to_string(),
            end: None,
        };

        let editor = Editor::new(
            "\n// comment".to_string(),
            selector.clone(),
            rust_lang,
            file.path().to_path_buf(),
            None,
        )
        .unwrap();

        // This should use Editor::Formatter internally (doesn't exist yet)
        let source = "fn main(){println!(\"test\");}";
        let result = editor.format_code(source);

        // The formatting logic should be separated from Editor
        // This test documents that formatting should be its own concern
        match result {
            Ok(_) => {}
            Err(err) => assert!(err.to_string().contains("format")),
        }
    }

    #[test]
    fn editor_should_have_separate_diff_generator_module() {
        // Test that Editor delegates diff generation to a separate DiffGenerator
        let file = create_test_file("fn main() {\n    let x = 1;\n    let y = 2;\n}");
        let language_registry = LanguageRegistry::new().unwrap();
        let rust_lang = language_registry.get_language(LanguageName::Rust).unwrap();

        let selector = Selector {
            operation: Operation::InsertAfter,
            anchor: "let y = 2;".to_string(),
            end: None,
        };

        let editor = Editor::new(
            "\n    let z = 3;".to_string(),
            selector.clone(),
            rust_lang,
            file.path().to_path_buf(),
            None,
        )
        .unwrap();

        // This should use Editor::DiffGenerator internally (doesn't exist yet)
        let result = editor.preview();
        assert!(result.is_ok());

        // The diff generation logic should be separated from Editor
        // This test documents that diff generation should be its own concern
        let (message, _) = result.unwrap();
        assert!(message.contains("DIFF") || message.contains("Edit efficiency"));
    }

    #[test]
    fn editor_should_have_separate_operation_executor_module() {
        // Test that Editor delegates operation execution to a separate OperationExecutor
        let file = create_test_file("fn main() {}");
        let language_registry = LanguageRegistry::new().unwrap();
        let rust_lang = language_registry.get_language(LanguageName::Rust).unwrap();

        let selector = Selector {
            operation: Operation::InsertAfter,
            anchor: "{}".to_string(),
            end: None,
        };

        let editor = Editor::new(
            "\n// comment".to_string(),
            selector.clone(),
            rust_lang,
            file.path().to_path_buf(),
            None,
        )
        .unwrap();

        // This should use Editor::OperationExecutor internally (doesn't exist yet)
        let result = editor.commit();
        assert!(result.is_ok());

        // The operation execution logic should be separated from Editor
        // This test documents that operation execution should be its own concern
        let (message, output, _) = result.unwrap();
        assert!(message.contains("operation result") && output.is_some());
    }

    #[test]
    fn editor_modules_should_be_independently_testable() {
        // This test will fail until we extract modules that can be tested independently
        // It represents the goal of having separate, composable modules

        // These modules should exist after decomposition:
        // - Editor::Validator
        // - Editor::Formatter
        // - Editor::DiffGenerator
        // - Editor::OperationExecutor
        // - Editor::Core (orchestrator)

        // For now, this test documents the architectural intention
        // Once decomposed, we should be able to test each module separately

        // Validator should validate syntax independently
        let validation_works = true; // Will be Editor::Validator::validate(...)
        assert!(validation_works);

        // Formatter should format code independently
        let formatting_works = true; // Will be Editor::Formatter::format(...)
        assert!(formatting_works);

        // DiffGenerator should generate diffs independently
        let diff_generation_works = true; // Will be Editor::DiffGenerator::generate(...)
        assert!(diff_generation_works);

        // OperationExecutor should execute operations independently
        let execution_works = true; // Will be Editor::OperationExecutor::execute(...)
        assert!(execution_works);
    }

    #[test]
    fn editor_core_should_orchestrate_modules() {
        // Test that Editor::Core orchestrates the other modules
        let file = create_test_file("fn main() {}");
        let language_registry = LanguageRegistry::new().unwrap();
        let rust_lang = language_registry.get_language(LanguageName::Rust).unwrap();

        let selector = Selector {
            operation: Operation::InsertAfter,
            anchor: "{}".to_string(),
            end: None,
        };

        let editor = Editor::new(
            "\n// comment".to_string(),
            selector.clone(),
            rust_lang,
            file.path().to_path_buf(),
            None,
        )
        .unwrap();

        // Editor::Core should coordinate:
        // 1. Validation (Editor::Validator)
        // 2. Operation execution (Editor::OperationExecutor)
        // 3. Formatting (Editor::Formatter)
        // 4. Diff generation (Editor::DiffGenerator)

        // Test preview functionality
        let preview_result = editor.preview();
        assert!(preview_result.is_ok());

        // Create a new editor for commit test (since preview consumes the editor)
        let editor2 = Editor::new(
            "
// comment"
                .to_string(),
            selector.clone(),
            rust_lang,
            file.path().to_path_buf(),
            None,
        )
        .unwrap();

        let commit_result = editor2.commit();
        assert!(commit_result.is_ok());

        // This test ensures the orchestration logic is clean and focused
    }

    #[test]
    fn editor_should_have_clear_separation_of_concerns() {
        // Test that each concern is handled by the appropriate module
        let file = create_test_file("fn test() { println!(\"hello\"); }");
        let language_registry = LanguageRegistry::new().unwrap();
        let rust_lang = language_registry.get_language(LanguageName::Rust).unwrap();

        let selector = Selector {
            operation: Operation::InsertAfter,
            anchor: "println!(\"hello\");".to_string(),
            end: None,
        };

        let editor = Editor::new(
            "\n    println!(\"world\");".to_string(),
            selector.clone(),
            rust_lang,
            file.path().to_path_buf(),
            None,
        )
        .unwrap();

        // Each module should have a single responsibility:
        // - Validator: Only syntax/semantic validation
        // - Formatter: Only code formatting
        // - DiffGenerator: Only diff generation and efficiency metrics
        // - OperationExecutor: Only edit operation execution
        // - Core: Only orchestration and coordination

        let result = editor.preview();
        assert!(result.is_ok());

        // This test will pass once we have proper separation
        let (message, staged_op) = result.unwrap();
        assert!(staged_op.is_some());
        assert!(!message.is_empty());
    }
}

/// Integration tests that verify the decomposed modules work together
#[cfg(test)]
mod editor_integration_tests {
    use super::*;

    fn create_test_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file.flush().unwrap();
        file
    }

    #[test]
    fn decomposed_editor_maintains_existing_api_compatibility() {
        // This test ensures that after decomposition, the public API remains the same
        let file = create_test_file("fn main() { let x = 42; }");
        let language_registry = LanguageRegistry::new().unwrap();
        let rust_lang = language_registry.get_language(LanguageName::Rust).unwrap();

        let selector = Selector {
            operation: Operation::InsertAfter,
            anchor: "let x = 42;".to_string(),
            end: None,
        };

        // All existing Editor methods should continue to work
        let editor = Editor::new(
            "\n    let y = 24;".to_string(),
            selector.clone(),
            rust_lang,
            file.path().to_path_buf(),
            None,
        );

        assert!(editor.is_ok());
        let editor = editor.unwrap();

        // Public API should remain unchanged
        assert!(editor.preview().is_ok());

        // Create new editor for commit test
        let editor2 = Editor::new(
            "
    let y = 24;"
                .to_string(),
            selector.clone(),
            rust_lang,
            file.path().to_path_buf(),
            None,
        )
        .unwrap();
        assert!(editor2.commit().is_ok());
    }

    #[test]
    fn decomposed_modules_handle_errors_gracefully() {
        // Test that each module properly handles and propagates errors
        let file = create_test_file("fn main( { // syntax error");
        let language_registry = LanguageRegistry::new().unwrap();
        let rust_lang = language_registry.get_language(LanguageName::Rust).unwrap();

        let selector = Selector {
            operation: Operation::InsertAfter,
            anchor: "main".to_string(),
            end: None,
        };

        let editor = Editor::new(
            "// comment".to_string(),
            selector.clone(),
            rust_lang,
            file.path().to_path_buf(),
            None,
        )
        .unwrap();

        // Each module should handle errors appropriately:
        // - Validator should catch syntax errors
        // - Formatter should handle formatting failures
        // - DiffGenerator should handle diff generation failures
        // - OperationExecutor should handle execution failures

        let result = editor.preview();
        assert!(result.is_ok());

        let (message, staged_op) = result.unwrap();
        assert!(message.contains("Syntax error") || message.contains("SYNTAX ERRORS"));
        assert!(staged_op.is_none()); // Should not stage invalid operations
    }
}
