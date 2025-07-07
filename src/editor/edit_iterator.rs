//! Iterator for handling multiple potential edit locations.
//!
//! This module provides the `EditIterator` which handles cases where a selector
//! might match multiple locations in the source code. Features include:
//! - Iterate through potential edit positions
//! - Smart location selection and ranking
//! - Fallback handling for ambiguous matches
//! - Integration with selector targeting system
//! - Support for all operation types

use std::iter::Iterator;

use tree_sitter::Tree;

use crate::{
    editor::EditPosition,
    selector::{Operation, Selector},
};

use super::{Edit, Editor};

pub(super) struct EditIterator<'editor, 'language> {
    editor: &'editor Editor<'language>,
    selector: &'editor Selector,
    source_code: &'editor str,
    tree: &'editor Tree,
    staged_edit: Option<&'editor EditPosition>,
    edits: Option<Vec<Edit<'editor, 'language>>>,
    current_index: usize,
}

impl<'editor, 'language> EditIterator<'editor, 'language> {
    pub(crate) fn new(editor: &'editor Editor<'language>) -> Self {
        let Editor {
            selector,
            source_code,
            tree,
            staged_edit,
            ..
        } = &editor;
        Self {
            editor,
            selector,
            source_code,
            tree,
            staged_edit: staged_edit.as_ref(),
            edits: None,
            current_index: 0,
        }
    }

    fn find_edits(&self) -> Result<Vec<Edit<'editor, 'language>>, String> {
        let source_code: &str = self.source_code;
        let tree: &Tree = self.tree;
        self.selector.validate()?;
        let Selector {
            operation,
            anchor,
            end,
        } = &self.selector;

        match operation {
            Operation::InsertBefore => self.find_insert_positions(anchor, true, source_code),
            Operation::InsertAfter => self.find_insert_positions(anchor, false, source_code),
            Operation::InsertAfterNode => {
                self.find_after_ast_insert_positions(anchor, source_code, tree)
            }
            Operation::ReplaceRange => self.find_range_matches(anchor, end.as_deref(), source_code),
            Operation::ReplaceExact => self.find_exact_matches(anchor, source_code),
            Operation::ReplaceNode => self.select_ast_node(anchor, source_code, tree),
        }
    }

    fn ensure_text_ranges_loaded(&mut self) -> Result<(), String> {
        if self.edits.is_none() {
            self.edits = Some(self.find_edits()?);
        }
        Ok(())
    }

    fn build_edit(&self, start_byte: usize) -> Edit<'editor, 'language> {
        Edit::new(
            self.editor,
            EditPosition {
                start_byte,
                end_byte: None,
            },
        )
    }

    /// Add spacing and newline variations to a list of edits
    ///
    /// This consolidates the common pattern of generating additional edit variations
    /// with space and newline prefixes/suffixes for better placement options.
    fn add_spacing_variations(&self, edits: &mut Vec<Edit<'editor, 'language>>, before: bool) {
        let original_edits = edits.clone();
        for edit in &original_edits {
            if before {
                edits.push(edit.clone().with_content(format!("{} ", &edit.content)));
                edits.push(edit.clone().with_content(format!("{}\n", &edit.content)));
            } else {
                edits.push(edit.clone().with_content(format!(" {}", &edit.content)));
                edits.push(edit.clone().with_content(format!("\n{}", &edit.content)));
            }
        }
    }

    fn find_after_ast_insert_positions(
        &self,
        anchor: &str,
        source_code: &str,
        tree: &Tree,
    ) -> Result<Vec<Edit<'editor, 'language>>, String> {
        let mut edits = self
            .select_ast_node(anchor, source_code, tree)?
            .into_iter()
            .filter_map(|edit| {
                edit.position
                    .end_byte
                    .map(|start_byte| self.build_edit(start_byte))
            })
            .collect::<Vec<_>>();

        self.add_spacing_variations(&mut edits, false);
        Ok(edits)
    }

    fn find_explicit_range(
        &self,
        anchor: &str,
        end: &str,
        source_code: &str,
    ) -> Result<Vec<Edit<'editor, 'language>>, String> {
        let mut ranges = Vec::new();

        for (from_byte, _) in from_positions(source_code, anchor)? {
            for (to_byte, _) in to_positions(source_code, end)? {
                if to_byte >= from_byte + anchor.len() {
                    ranges.push(
                        self.build_edit(from_byte)
                            .with_end_byte(to_byte + end.len()),
                    );
                }
            }
        }

        if ranges.is_empty() {
            Err(format!(
                "No valid range found from \"{anchor}\" to \"{end}\""
            ))
        } else {
            Ok(ranges)
        }
    }

    fn find_insert_positions(
        &self,
        anchor: &str,
        before: bool,
        source_code: &str,
    ) -> Result<Vec<Edit<'editor, 'language>>, String> {
        let mut edits = source_code
            .match_indices(anchor)
            .map(|(byte_offset, _)| {
                self.build_edit(if before {
                    byte_offset
                } else {
                    byte_offset + anchor.len()
                })
            })
            .collect::<Vec<_>>();

        if edits.is_empty() {
            Err(format!("Anchor text \"{anchor}\" not found in source"))
        } else {
            self.add_spacing_variations(&mut edits, before);
            Ok(edits)
        }
    }

    fn find_exact_matches(
        &self,
        exact_text: &str,
        source_code: &str,
    ) -> Result<Vec<Edit<'editor, 'language>>, String> {
        let positions = source_code
            .match_indices(exact_text)
            .map(|(start_byte, matched)| {
                self.build_edit(start_byte)
                    .with_end_byte(start_byte + matched.len())
            })
            .collect::<Vec<_>>();

        if positions.is_empty() {
            Err(format!("Exact text \"{exact_text}\" not found in source"))
        } else {
            Ok(positions)
        }
    }

    fn find_range_matches(
        &self,
        anchor: &str,
        end: Option<&str>,
        source_code: &str,
    ) -> Result<Vec<Edit<'editor, 'language>>, String> {
        if let Some(end) = end {
            self.find_explicit_range(anchor, end, source_code)
        } else {
            Err("end is required for range replacement".to_string())
        }
    }

    fn select_ast_node(
        &self,
        anchor: &str,
        source_code: &str,
        tree: &Tree,
    ) -> Result<Vec<Edit<'editor, 'language>>, String> {
        let anchor = anchor.trim().lines().next().unwrap_or_default();

        Ok(from_positions(source_code, anchor.trim())?
            .into_iter()
            .filter_map(|(from, anchor)| {
                let from_end = from + anchor.len();
                tree.root_node()
                    .named_descendant_for_byte_range(from, from_end)
                    .or_else(|| tree.root_node().descendant_for_byte_range(from, from_end))
                    .map(|node| {
                        self.build_edit(node.start_byte())
                            .with_end_byte(node.end_byte())
                    })
            })
            .collect())
    }
}

impl<'editor, 'language> Iterator for EditIterator<'editor, 'language> {
    type Item = Result<Edit<'editor, 'language>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // If we have a staged edit, return it first and only once
        if let Some(edit_position) = self.staged_edit.take() {
            return Some(Ok(Edit::new(self.editor, *edit_position)));
        }

        // Ensure text ranges are loaded
        if let Err(e) = self.ensure_text_ranges_loaded() {
            return Some(Err(e));
        }

        // Get the current text range to try - handle invalid state gracefully
        let text_ranges = match self.edits.as_ref() {
            Some(edits) => edits,
            None => {
                return Some(Err("EditIterator in invalid state".to_string()));
            }
        };

        if self.current_index >= text_ranges.len() {
            return None; // No more ranges to try
        }

        let edit = text_ranges[self.current_index].clone();
        self.current_index += 1;

        Some(Ok(edit))
    }
}

fn from_positions<'a>(source_code: &'a str, anchor: &str) -> Result<Vec<(usize, &'a str)>, String> {
    let from_positions: Vec<_> = source_code.match_indices(anchor).collect();
    if from_positions.is_empty() {
        return Err(format!("From text \"{anchor}\" not found in source"));
    }
    Ok(from_positions)
}

fn to_positions<'a>(source_code: &'a str, end: &str) -> Result<Vec<(usize, &'a str)>, String> {
    let to_positions: Vec<_> = source_code.match_indices(end).collect();
    if to_positions.is_empty() {
        return Err(format!("To text \"{end}\" not found in source"));
    }
    Ok(to_positions)
}
