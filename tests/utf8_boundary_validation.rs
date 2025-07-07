use semantic_code_edit_mcp::validation::ContextValidator;
use tree_sitter::Query;

/// Test UTF-8 boundary safety in context validation
///
/// This test ensures that context validation properly handles UTF-8 boundaries
/// when extracting source code ranges, preventing panics from invalid slicing.
#[test]
fn test_utf8_boundary_safety_in_validation() {
    // Create source code with multi-byte UTF-8 characters at critical positions
    let source_code =
        "fn 测试() {\n    // 这是测试 - multi-byte UTF-8 characters\n    let x = 42;\n}";

    // Parse the code with Rust tree-sitter
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(source_code, None).unwrap();

    // Create a query that might match problematic byte ranges
    let query = Query::new(
        &tree_sitter_rust::LANGUAGE.into(),
        "(function_item) @function",
    )
    .unwrap();

    // This should not panic even with UTF-8 characters
    let result = ContextValidator::validate_tree(&tree, &query, source_code);

    // The validation should complete without panicking
    assert!(result.is_valid || !result.is_valid); // Just ensure it doesn't panic
}

#[test]
fn test_utf8_boundary_panic_reproduction() {
    // Source with specific UTF-8 positioning that could cause byte boundary issues
    let source_code = "fn test() {\n    let 变量 = 42; // 中文注释\n}";

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(source_code, None).unwrap();

    // Create query that will match and potentially create violations
    let query = Query::new(
        &tree_sitter_rust::LANGUAGE.into(),
        "(function_item) @invalid.function",
    )
    .unwrap();

    // This test should demonstrate the UTF-8 boundary issue in format_errors()
    let result = ContextValidator::validate_tree(&tree, &query, source_code);

    // This call to format_errors() may panic with current implementation
    // due to UTF-8 boundary issues when slicing source_code[parent.byte_range()]
    let error_message = result.format_errors();

    // If we reach here without panic, the issue is fixed or not triggered
    println!("Error message: {error_message}");
}

#[test]
fn test_edge_case_utf8_boundaries() {
    // Test edge cases where byte ranges might split UTF-8 characters
    let test_cases = vec![
        "fn 测试() {}",
        "// 这是一个测试\nfn test() {}",
        "fn test() { /* 中文 */ }",
        "fn 名前() { let 変数 = \"文字列\"; }",
    ];

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();

    for source_code in test_cases {
        let tree = parser.parse(source_code, None).unwrap();

        let query = Query::new(
            &tree_sitter_rust::LANGUAGE.into(),
            "(function_item) @function",
        )
        .unwrap();

        // Each case should handle UTF-8 safely
        let result = ContextValidator::validate_tree(&tree, &query, source_code);
        let _formatted = result.format_errors(); // Should not panic
    }
}
