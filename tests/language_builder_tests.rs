use anyhow::Result;
use semantic_code_edit_mcp::languages::{
    LanguageBuilder, LanguageName, LanguageRegistry, simple_language,
};

#[test]
fn test_language_builder_simple() -> Result<()> {
    let lang =
        LanguageBuilder::new(LanguageName::Go, &["go"], tree_sitter_go::LANGUAGE.into()).build()?;

    assert_eq!(lang.name(), LanguageName::Go);
    assert_eq!(lang.file_extensions(), &["go"]);
    assert!(lang.validation_query().is_none());
    Ok(())
}

#[test]
fn test_language_builder_with_validation() -> Result<()> {
    let lang = LanguageBuilder::new(
        LanguageName::Python,
        &["py"],
        tree_sitter_python::LANGUAGE.into(),
    )
    .with_validation_query(";; Test validation query\n")
    .build()?;

    assert_eq!(lang.name(), LanguageName::Python);
    assert_eq!(lang.file_extensions(), &["py"]);
    assert!(lang.validation_query().is_some());
    Ok(())
}

#[test]
fn test_language_builder_with_custom_editor() -> Result<()> {
    struct TestEditor;
    impl semantic_code_edit_mcp::languages::traits::LanguageEditor for TestEditor {}

    let lang = LanguageBuilder::new(
        LanguageName::Json,
        &["json"],
        tree_sitter_json::LANGUAGE.into(),
    )
    .with_editor(Box::new(TestEditor))
    .build()?;

    assert_eq!(lang.name(), LanguageName::Json);
    assert_eq!(lang.file_extensions(), &["json"]);
    Ok(())
}

#[test]
fn test_language_builder_full_configuration() -> Result<()> {
    struct TestEditor;
    impl semantic_code_edit_mcp::languages::traits::LanguageEditor for TestEditor {}

    let lang = LanguageBuilder::new(
        LanguageName::Rust,
        &["rs"],
        tree_sitter_rust::LANGUAGE.into(),
    )
    .with_editor(Box::new(TestEditor))
    .with_validation_query(";; Test validation query\n")
    .build()?;

    assert_eq!(lang.name(), LanguageName::Rust);
    assert_eq!(lang.file_extensions(), &["rs"]);
    assert!(lang.validation_query().is_some());
    Ok(())
}

#[test]
fn test_simple_language_backward_compatibility() -> Result<()> {
    let lang = simple_language(LanguageName::Go, &["go"], tree_sitter_go::LANGUAGE.into())?;

    assert_eq!(lang.name(), LanguageName::Go);
    assert_eq!(lang.file_extensions(), &["go"]);
    assert!(lang.validation_query().is_none());
    Ok(())
}

#[test]
fn test_all_languages_can_be_registered() -> Result<()> {
    // This test ensures all 17 language modules work with the standardized pattern
    let registry = LanguageRegistry::new()?;

    // Verify all expected languages are registered
    let expected_languages = [
        LanguageName::Rust,
        LanguageName::Json,
        LanguageName::Toml,
        LanguageName::Javascript,
        LanguageName::Typescript,
        LanguageName::Tsx,
        LanguageName::Python,
        LanguageName::Go,
        LanguageName::Cpp,
        LanguageName::C,
        LanguageName::Java,
        LanguageName::Php,
        LanguageName::CSharp,
        LanguageName::Ruby,
    ];

    for language_name in expected_languages {
        let lang = registry.get_language(language_name)?;
        assert_eq!(lang.name(), language_name);
        assert!(!lang.file_extensions().is_empty());
    }

    Ok(())
}

#[test]
fn test_validation_query_languages() -> Result<()> {
    let registry = LanguageRegistry::new()?;

    // Languages that should have validation queries
    let validation_languages = [
        LanguageName::Rust,
        LanguageName::Python,
        LanguageName::Javascript,
        LanguageName::Typescript,
        LanguageName::Json,
    ];

    for language_name in validation_languages {
        let lang = registry.get_language(language_name)?;
        assert!(
            lang.validation_query().is_some(),
            "Language {language_name:?} should have validation query"
        );
    }

    Ok(())
}

#[test]
fn test_docs_method_works() -> Result<()> {
    let lang =
        LanguageBuilder::new(LanguageName::Go, &["go"], tree_sitter_go::LANGUAGE.into()).build()?;

    let docs = lang.docs();
    assert!(docs.contains("go"));
    assert!(docs.contains("Language: go"));
    assert!(docs.contains("File extensions: go"));
    Ok(())
}
