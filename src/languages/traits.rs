//! Language editor traits and default implementations.
//!
//! This module defines the core traits for language-specific editing operations
//! and provides default implementations. Features include:
//! - LanguageEditor trait for custom language support
//! - DefaultEditor providing basic tree-sitter validation
//! - Formatting and error collection interfaces
//! - Extensible design for adding new languages

use anyhow::Result;
use tree_sitter::{Node, Tree};

/// Default editor implementation with basic tree-sitter validation
#[derive(Debug, Clone)]
pub struct DefaultEditor;

impl DefaultEditor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for language-specific operations like validation and formatting
pub trait LanguageEditor: Send + Sync {
    /// Collect syntax error line numbers from a tree-sitter parse tree
    fn collect_errors(&self, tree: &Tree, _content: &str) -> Vec<usize> {
        collect_errors(tree)
            .into_iter()
            .map(|node| node.start_position().row)
            .collect()
    }

    /// Format code according to language conventions
    fn format_code(&self, source: &str) -> Result<String> {
        Ok(source.to_string())
    }
}

impl LanguageEditor for DefaultEditor {
    // Uses all default implementations
}

pub fn collect_errors<'tree>(tree: &'tree Tree) -> Vec<Node<'tree>> {
    let mut errors = vec![];
    collect_errors_recursive(tree.root_node(), &mut errors);
    errors
}

fn collect_errors_recursive<'tree>(node: Node<'tree>, errors: &mut Vec<Node<'tree>>) {
    // Check if this node is an error
    if node.is_error() || node.is_missing() {
        errors.push(node);
    }

    // Recursively check all children
    for child in node.children(&mut node.walk()) {
        collect_errors_recursive(child, errors);
    }
}
