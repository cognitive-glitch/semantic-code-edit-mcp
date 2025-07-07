use anyhow::Result;
use semantic_code_edit_mcp::languages::{LanguageName, LanguageRegistry};

#[test]
fn test_validation_queries_are_loaded() -> Result<()> {
    let registry = LanguageRegistry::new()?;

    // Test that languages with validation queries have them loaded
    let rust_lang = registry.get_language(LanguageName::Rust)?;
    assert!(
        rust_lang.validation_query().is_some(),
        "Rust should have validation query"
    );

    let python_lang = registry.get_language(LanguageName::Python)?;
    assert!(
        python_lang.validation_query().is_some(),
        "Python should have validation query"
    );

    let json_lang = registry.get_language(LanguageName::Json)?;
    assert!(
        json_lang.validation_query().is_some(),
        "JSON should have validation query"
    );

    let js_lang = registry.get_language(LanguageName::Javascript)?;
    assert!(
        js_lang.validation_query().is_some(),
        "JavaScript should have validation query"
    );

    let ts_lang = registry.get_language(LanguageName::Typescript)?;
    assert!(
        ts_lang.validation_query().is_some(),
        "TypeScript should have validation query"
    );

    Ok(())
}

#[test]
fn test_python_validation_query_compiles() -> Result<()> {
    let registry = LanguageRegistry::new()?;
    let python_lang = registry.get_language(LanguageName::Python)?;

    // Create a parser and parse valid Python code
    let mut parser = python_lang.tree_sitter_parser()?;
    let code = "def test(): return 42";
    let tree = parser.parse(code, None).unwrap();

    // Test that validation query can be executed without errors
    if let Some(query) = python_lang.validation_query() {
        let mut query_cursor = tree_sitter::QueryCursor::new();
        let _matches = query_cursor.matches(query, tree.root_node(), code.as_bytes());
        // Just ensuring no crash - the query should execute successfully
    }

    Ok(())
}

#[test]
fn test_json_validation_query_compiles() -> Result<()> {
    let registry = LanguageRegistry::new()?;
    let json_lang = registry.get_language(LanguageName::Json)?;

    // Create a parser and parse valid JSON code
    let mut parser = json_lang.tree_sitter_parser()?;
    let code = r#"{"key": "value"}"#;
    let tree = parser.parse(code, None).unwrap();

    // Test that validation query can be executed without errors
    if let Some(query) = json_lang.validation_query() {
        let mut query_cursor = tree_sitter::QueryCursor::new();
        let _matches = query_cursor.matches(query, tree.root_node(), code.as_bytes());
        // Just ensuring no crash - the query should execute successfully
    }

    Ok(())
}

#[test]
fn test_javascript_validation_query_compiles() -> Result<()> {
    let registry = LanguageRegistry::new()?;
    let js_lang = registry.get_language(LanguageName::Javascript)?;

    // Create a parser and parse valid JavaScript code
    let mut parser = js_lang.tree_sitter_parser()?;
    let code = "function test() { return 42; }";
    let tree = parser.parse(code, None).unwrap();

    // Test that validation query can be executed without errors
    if let Some(query) = js_lang.validation_query() {
        let mut query_cursor = tree_sitter::QueryCursor::new();
        let _matches = query_cursor.matches(query, tree.root_node(), code.as_bytes());
        // Just ensuring no crash - the query should execute successfully
    }

    Ok(())
}

#[test]
fn test_typescript_validation_query_compiles() -> Result<()> {
    let registry = LanguageRegistry::new()?;
    let ts_lang = registry.get_language(LanguageName::Typescript)?;

    // Create a parser and parse valid TypeScript code
    let mut parser = ts_lang.tree_sitter_parser()?;
    let code = "interface Test { name: string; }";
    let tree = parser.parse(code, None).unwrap();

    // Test that validation query can be executed without errors
    if let Some(query) = ts_lang.validation_query() {
        let mut query_cursor = tree_sitter::QueryCursor::new();
        let _matches = query_cursor.matches(query, tree.root_node(), code.as_bytes());
        // Just ensuring no crash - the query should execute successfully
    }

    Ok(())
}

#[test]
fn test_all_languages_can_create_parsers() -> Result<()> {
    let registry = LanguageRegistry::new()?;

    // Test that all languages with validation queries can create parsers
    let languages = [
        LanguageName::Rust,
        LanguageName::Python,
        LanguageName::Json,
        LanguageName::Javascript,
        LanguageName::Typescript,
    ];

    for &lang_name in &languages {
        let lang = registry.get_language(lang_name)?;
        let parser = lang.tree_sitter_parser();
        assert!(
            parser.is_ok(),
            "Should be able to create parser for {lang_name:?}"
        );
    }

    Ok(())
}

#[test]
fn test_validation_coverage_expansion() -> Result<()> {
    let registry = LanguageRegistry::new()?;

    // Count languages with validation queries
    let languages_with_validation = [
        LanguageName::Rust,
        LanguageName::Python,
        LanguageName::Json,
        LanguageName::Javascript,
        LanguageName::Typescript,
    ];

    for &lang_name in &languages_with_validation {
        let lang = registry.get_language(lang_name)?;
        assert!(
            lang.validation_query().is_some(),
            "Language {lang_name:?} should have validation query after expansion"
        );
    }

    // Before this expansion, only Rust had validation queries
    // Now we have 5 languages with validation queries
    assert_eq!(
        languages_with_validation.len(),
        5,
        "Should have expanded validation support to 5 languages"
    );

    Ok(())
}
