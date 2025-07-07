//! # Diff Generator Module
//!
//! This module provides diff generation and formatting capabilities for code edits.
//! It creates human-readable diffs and calculates edit efficiency metrics.
//!
//! ## Features
//!
//! - Generates clean diffs without unnecessary headers
//! - Calculates edit efficiency (percentage of lines changed)
//! - Provides helpful tips for large edits with low efficiency
//! - Optimized for AI consumption with clear formatting
//!
//! ## Edit Efficiency
//!
//! The module calculates what percentage of the content was actually changed:
//! - High efficiency (>30%): Most of the content is being modified
//! - Low efficiency (<30%): Only a small portion is changed, suggests using targeted edits
//!
//! ## Example
//!
//! ```ignore
//! use semantic_code_edit_mcp::editor::diff_generator::DiffGenerator;
//!
//! let diff = DiffGenerator::generate_diff(original, modified, content_patch);
//! println!("{}", diff);
//! // Output:
//! // Edit efficiency: 15%
//! // 💡 TIP: For focused changes like this, you might try targeted insert/replace operations
//! //
//! // ===DIFF===
//! // -old line
//! // +new line
//! ```

use diffy::{DiffOptions, Patch, PatchFormatter};
use std::collections::BTreeSet;

/// Handles diff generation and formatting
pub struct DiffGenerator;

impl DiffGenerator {
    /// Generates a formatted diff between source and output
    pub fn generate_diff(source_code: &str, output: &str, content_patch: &str) -> String {
        let diff_patch = DiffOptions::new().create_patch(source_code, output);
        let formatter = PatchFormatter::new().missing_newline_message(false);

        // Get the diff string and clean it up for AI consumption
        let diff_output = formatter.fmt_patch(&diff_patch).to_string();
        let lines: Vec<&str> = diff_output.lines().collect();
        let mut cleaned_diff = String::new();

        let content_line_count = content_patch.lines().count();
        if content_line_count > 10 {
            let changed_lines = Self::calculate_changed_lines(&diff_patch, content_line_count);

            let changed_fraction = (changed_lines * 100) / content_line_count;

            cleaned_diff.push_str(&format!("Edit efficiency: {changed_fraction}%\n",));
            if changed_fraction < 30 {
                cleaned_diff.push_str("💡 TIP: For focused changes like this, you might try targeted insert/replace operations for easier review and iteration\n");
            };
            cleaned_diff.push('\n');
        }

        cleaned_diff.push_str("===DIFF===\n");
        for line in lines {
            // Skip ALL diff headers: file headers, hunk headers (line numbers), and any metadata
            if line.starts_with("---") || line.starts_with("+++") || line.starts_with("@@") {
                // Skip "\ No newline at end of file" messages
                continue;
            }
            cleaned_diff.push_str(line);
            cleaned_diff.push('\n');
        }

        // Remove trailing newline to avoid extra spacing
        if cleaned_diff.ends_with('\n') {
            cleaned_diff.pop();
        }
        cleaned_diff
    }

    /// Calculates the number of changed lines in a patch
    pub fn calculate_changed_lines(patch: &Patch<'_, str>, content_line_count: usize) -> usize {
        let mut changed_line_numbers = BTreeSet::new();

        for hunk in patch.hunks() {
            // old_range().range() returns a std::ops::Range<usize> that's properly 0-indexed
            for line_num in hunk.old_range().range() {
                if line_num < content_line_count {
                    changed_line_numbers.insert(line_num);
                }
            }
        }
        changed_line_numbers.len()
    }
}
