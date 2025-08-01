//! Context validation for semantic code editing operations.
//!
//! This module provides language-specific semantic validation using tree-sitter queries.
//! It validates that code edits maintain semantic correctness beyond just syntax.
//! Features include:
//! - Tree-sitter query-based validation rules
//! - Language-specific context checking
//! - Violation reporting with node information
//! - Integration with the broader validation system

use tree_sitter::{Node, Query, QueryCursor, StreamingIterator, Tree};

/// Tree-sitter based context validator for semantic code editing
pub struct ContextValidator;

#[derive(Debug)]
pub struct ValidationResult<'tree, 'source> {
    pub is_valid: bool,
    pub violations: Vec<ContextViolation<'tree>>,
    pub source_code: &'source str,
}

#[derive(Debug)]
pub struct ContextViolation<'tree> {
    pub node: Node<'tree>,
    pub message: String, // Human-readable error
    pub suggestion: &'static str,
}

impl ContextValidator {
    /// Validate if content can be safely inserted at the target location
    pub fn validate_tree<'tree, 'source>(
        tree: &'tree Tree,
        query: &Query,
        source_code: &'source str,
    ) -> ValidationResult<'tree, 'source> {
        // Run validation queries against the temporary tree
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(query, tree.root_node(), source_code.as_bytes());

        let mut violations = Vec::new();

        while let Some(m) = matches.next() {
            for capture in m.captures {
                let node = capture.node;

                // Extract violation type from capture name
                if let Some(violation_type) = Self::extract_violation_type(capture.index, query) {
                    // Only process "invalid" captures
                    if violation_type.starts_with("invalid.") {
                        violations.push(ContextViolation {
                            node,
                            message: Self::get_violation_message(&violation_type),
                            suggestion: Self::get_violation_suggestion(&violation_type),
                        });
                    }
                }
            }
        }

        ValidationResult {
            is_valid: violations.is_empty(),
            source_code,
            violations,
        }
    }

    fn extract_violation_type(capture_index: u32, query: &Query) -> Option<String> {
        query
            .capture_names()
            .get(capture_index as usize)
            .map(|s| s.to_string())
    }

    fn get_violation_message(violation_type: &str) -> String {
        match violation_type {
            "invalid.function.in.struct.fields" => {
                "Functions cannot be defined inside struct field lists".to_string()
            }
            "invalid.function.in.enum.variants" => {
                "Functions cannot be defined inside enum variant lists".to_string()
            }
            "invalid.type.in.function.body" => {
                "Type definitions cannot be placed inside function bodies".to_string()
            }
            "invalid.impl.in.function.body" => {
                "Impl blocks cannot be placed inside function bodies".to_string()
            }
            "invalid.trait.in.function.body" => {
                "Trait definitions cannot be placed inside function bodies".to_string()
            }
            "invalid.impl.nested" => "Impl blocks can only be defined at module level".to_string(),
            "invalid.trait.nested" => {
                "Trait definitions can only be defined at module level".to_string()
            }
            "invalid.use.in.item.body" => "Use declarations should be at module level".to_string(),
            "invalid.const.in.function.body" => {
                "Const/static items should be at module level".to_string()
            }
            "invalid.mod.in.function.body" => {
                "Module declarations cannot be inside function bodies".to_string()
            }
            "invalid.item.nested.in.item" => {
                "Items cannot be nested inside other items".to_string()
            }
            "invalid.expression.as.type" => "Expressions cannot be used as types".to_string(),
            _ => format!(
                "Invalid placement: {}",
                violation_type
                    .strip_prefix("invalid.")
                    .unwrap_or(violation_type)
            ),
        }
    }

    fn get_violation_suggestion(violation_type: &str) -> &'static str {
        match violation_type {
            "invalid.function.in.struct.fields" | "invalid.function.in.enum.variants" => {
                "Place the function after the type definition"
            }
            "invalid.type.in.function.body"
            | "invalid.impl.in.function.body"
            | "invalid.trait.in.function.body" => "Move this to module level",

            "invalid.use.in.item.body" => "Move use declarations to the top of the file",
            _ => "Consider placing this construct in an appropriate context",
        }
    }
}

impl ValidationResult<'_, '_> {
    /// Find the nearest UTF-8 character boundary
    fn find_utf8_boundary(&self, byte_pos: usize, search_backward: bool) -> usize {
        let bytes = self.source_code.as_bytes();
        let mut pos = byte_pos.min(bytes.len());

        if search_backward {
            // Search backwards for valid UTF-8 start
            while pos > 0 && !self.source_code.is_char_boundary(pos) {
                pos -= 1;
            }
        } else {
            // Search forwards for valid UTF-8 boundary
            while pos < bytes.len() && !self.source_code.is_char_boundary(pos) {
                pos += 1;
            }
        }

        pos
    }

    pub fn format_errors(&self) -> String {
        if self.is_valid {
            return "✅ All validations passed".to_string();
        }

        let mut response = String::new();
        response.push_str("❌ Invalid placement detected:\n\n");

        for violation in &self.violations {
            response.push_str(&format!("• {}:\n", violation.message));
            let parent = violation.node.parent().unwrap_or(violation.node);

            // Safe UTF-8 string slicing using byte_range()
            let range = parent.byte_range();
            let source_slice = if range.end <= self.source_code.len() {
                // Ensure we don't slice in the middle of UTF-8 characters
                match self.source_code.get(range.clone()) {
                    Some(slice) => slice,
                    None => {
                        // Fallback: find nearest valid UTF-8 boundaries
                        let start = self.find_utf8_boundary(range.start, true);
                        let end =
                            self.find_utf8_boundary(range.end.min(self.source_code.len()), false);
                        self.source_code
                            .get(start..end)
                            .unwrap_or("<invalid UTF-8 range>")
                    }
                }
            } else {
                "<range out of bounds>"
            };

            response.push_str(source_slice);
            response.push_str("\n\n");
            response.push_str(&format!("  💡 Suggestion: {}\n", violation.suggestion));
        }

        response
    }
}
