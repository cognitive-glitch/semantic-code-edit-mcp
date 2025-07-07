//! Multi-language support system for semantic code editing.
//!
//! This module provides support for 17+ programming languages through a unified
//! interface. Each language can have custom parsing, formatting, and validation
//! capabilities while sharing common functionality.
//!
//! ## Supported Languages
//!
//! - **Systems**: Rust, C, C++, Go
//! - **Web**: JavaScript, TypeScript, TSX
//! - **Enterprise**: Java, C#, PHP
//! - **Scripting**: Python, Ruby
//! - **Data**: JSON, TOML
//! - **Generic**: Plain text
//!
//! ## Key Components
//!
//! - [`LanguageRegistry`]: Central registry for all supported languages
//! - [`LanguageBuilder`]: Fluent API for creating language configurations
//! - [`LanguageCommon`]: Common interface for all languages
//! - [`LanguageEditor`]: Trait for language-specific editing operations
//!
//! ## Features
//!
//! - **Standardized configuration**: Consistent setup across all languages
//! - **Custom editors**: Language-specific formatting and validation
//! - **Validation queries**: Tree-sitter queries for semantic validation
//! - **Auto-detection**: File extension-based language detection
//! - **Performance**: Shared utilities and optimizations

pub mod c;
pub mod cpp;
pub mod csharp;
pub mod go;
pub mod java;
pub mod javascript;
pub mod json;
pub mod php;
pub mod plain;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod toml;
pub mod traits;
pub mod tsx;
pub mod typescript;
pub mod utils;

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    path::Path,
};
use tree_sitter::{Language, Parser, Query};

use crate::error::SemanticEditError;

use crate::languages::traits::{DefaultEditor, LanguageEditor};

/// Registry to manage all supported languages
#[derive(Debug)]
pub struct LanguageRegistry {
    languages: HashMap<LanguageName, LanguageCommon>,
    extensions: HashMap<&'static str, LanguageName>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
pub struct LanguageCommon {
    #[fieldwork(get(copy))]
    name: LanguageName,
    file_extensions: &'static [&'static str],
    #[fieldwork(rename = tree_sitter_language)]
    language: Language,
    editor: Box<dyn LanguageEditor>,
    validation_query: Option<Query>,
}

impl fmt::Debug for LanguageCommon {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("LanguageCommon")
            .field("name", &self.name)
            .field("file_extensions", &self.file_extensions)
            .field("language", &self.language)
            .field("validation_query", &self.validation_query)
            .finish()
    }
}
impl Display for LanguageCommon {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.name.as_str())
    }
}

/// Builder for creating standardized language configurations
pub struct LanguageBuilder {
    name: LanguageName,
    file_extensions: &'static [&'static str],
    language: Language,
    editor: Option<Box<dyn LanguageEditor>>,
    validation_query_content: Option<&'static str>,
}

impl std::fmt::Debug for LanguageBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LanguageBuilder")
            .field("name", &self.name)
            .field("file_extensions", &self.file_extensions)
            .field("language", &self.language)
            .field("editor", &"<Box<dyn LanguageEditor>>")
            .field("validation_query_content", &self.validation_query_content)
            .finish()
    }
}

impl LanguageBuilder {
    /// Create a new language builder with required parameters
    pub fn new(
        name: LanguageName,
        file_extensions: &'static [&'static str],
        language: Language,
    ) -> Self {
        Self {
            name,
            file_extensions,
            language,
            editor: None,
            validation_query_content: None,
        }
    }

    /// Set a custom editor implementation
    pub fn with_editor(mut self, editor: Box<dyn LanguageEditor>) -> Self {
        self.editor = Some(editor);
        self
    }

    /// Add a validation query from embedded content
    pub fn with_validation_query(mut self, query_content: &'static str) -> Self {
        self.validation_query_content = Some(query_content);
        self
    }

    /// Build the final LanguageCommon configuration
    pub fn build(self) -> Result<LanguageCommon> {
        let validation_query = if let Some(content) = self.validation_query_content {
            Some(tree_sitter::Query::new(&self.language, content)?)
        } else {
            None
        };

        Ok(LanguageCommon {
            name: self.name,
            file_extensions: self.file_extensions,
            language: self.language,
            editor: self
                .editor
                .unwrap_or_else(|| Box::new(DefaultEditor::new())),
            validation_query,
        })
    }
}

/// Helper function to create a simple language configuration using DefaultEditor
/// Maintained for backward compatibility - use LanguageBuilder for new code
pub fn simple_language(
    name: LanguageName,
    file_extensions: &'static [&'static str],
    language: Language,
) -> Result<LanguageCommon> {
    LanguageBuilder::new(name, file_extensions, language).build()
}

impl LanguageCommon {
    pub fn tree_sitter_parser(&self) -> Result<Parser> {
        let mut parser = Parser::new();
        parser.set_language(self.tree_sitter_language())?;
        Ok(parser)
    }

    pub fn docs(&self) -> String {
        format!(
            "Language: {}\nFile extensions: {}\nTree-sitter language available for AST-aware operations",
            self.name.as_str(),
            self.file_extensions.join(", ")
        )
    }
}

#[derive(
    Serialize, Deserialize, Debug, JsonSchema, Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Copy,
)]
#[serde(rename_all = "snake_case")]
pub enum LanguageName {
    Rust,
    Json,
    Toml,
    Javascript,
    Typescript,
    Tsx,
    Python,
    Go,
    Cpp,
    C,
    Java,
    Php,
    CSharp,
    Ruby,
    #[serde(other)]
    Other,
}
impl LanguageName {
    fn as_str(&self) -> &str {
        match self {
            LanguageName::Rust => "rust",
            LanguageName::Json => "json",
            LanguageName::Toml => "toml",
            LanguageName::Javascript => "javascript",
            LanguageName::Typescript => "typescript",
            LanguageName::Tsx => "tsx",
            LanguageName::Python => "python",
            LanguageName::Go => "go",
            LanguageName::Cpp => "cpp",
            LanguageName::C => "c",
            LanguageName::Java => "java",
            LanguageName::Php => "php",
            LanguageName::CSharp => "csharp",
            LanguageName::Ruby => "ruby",
            LanguageName::Other => "other",
        }
    }
}

impl Display for LanguageName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl LanguageRegistry {
    pub fn new() -> Result<Self> {
        let mut registry = Self {
            languages: HashMap::new(),
            extensions: HashMap::new(),
        };

        registry.register_language(json::language()?);
        registry.register_language(rust::language()?);
        registry.register_language(toml::language()?);
        registry.register_language(typescript::language()?);
        registry.register_language(tsx::language()?);
        registry.register_language(javascript::language()?);
        registry.register_language(python::language()?);
        registry.register_language(go::language()?);
        registry.register_language(cpp::language()?);
        registry.register_language(c::language()?);
        registry.register_language(java::language()?);
        registry.register_language(php::language()?);
        registry.register_language(csharp::language()?);
        registry.register_language(ruby::language()?);
        registry.register_language(plain::language()?);

        Ok(registry)
    }

    pub fn register_language(&mut self, language: LanguageCommon) {
        let name = language.name();
        for extension in language.file_extensions() {
            self.extensions.insert(extension, name);
        }
        self.languages.insert(name, language);
    }

    pub fn get_language(&self, name: LanguageName) -> Result<&LanguageCommon, SemanticEditError> {
        self.languages
            .get(&name)
            .ok_or(SemanticEditError::ParserUnavailable {
                language: name.to_string(),
            })
    }

    pub fn get_language_with_hint(
        &self,
        file_path: &Path,
        language_hint: Option<LanguageName>,
    ) -> Result<&LanguageCommon> {
        let language_name = language_hint
            .or_else(|| self.detect_language_from_path(file_path))
            .unwrap_or(LanguageName::Other);
        self.get_language(language_name)
            .map_err(anyhow::Error::from)
    }

    pub fn detect_language_from_path(&self, file_path: &Path) -> Option<LanguageName> {
        let extension = file_path.extension()?.to_str()?;
        self.extensions.get(extension).copied()
    }
}
