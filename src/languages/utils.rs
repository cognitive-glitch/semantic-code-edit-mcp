//! # Utilities Module
//!
//! Common utilities for language processing, including text position handling.

/// Converts byte offsets to line numbers in text
///
/// This utility is used by language editors that need to convert error positions
/// from parsers (which typically report byte offsets) to line numbers for error reporting.
///
/// ## Example
///
/// ```ignore
/// use semantic_code_edit_mcp::languages::utils::LineConverter;
///
/// let text = "line one\nline two\nline three";
/// let converter = LineConverter::new(text);
///
/// assert_eq!(converter.offset_to_line(0), 0);   // First line
/// assert_eq!(converter.offset_to_line(9), 1);   // Second line
/// assert_eq!(converter.offset_to_line(18), 2);  // Third line
/// ```
pub struct LineConverter {
    newline_positions: Vec<usize>,
}

impl LineConverter {
    /// Create a new LineConverter for the given text
    pub fn new(text: &str) -> Self {
        // Start with position 0 (beginning of first line)
        let mut newline_positions = vec![0];

        // Find all newline positions, adding position after each newline
        let bytes = text.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            match bytes[i] {
                b'\n' => {
                    newline_positions.push(i + 1); // Position after the newline
                    i += 1;
                }
                b'\r' => {
                    // Handle \r\n as a single newline
                    if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                        newline_positions.push(i + 2); // Position after \r\n
                        i += 2;
                    } else {
                        newline_positions.push(i + 1); // Position after \r
                        i += 1;
                    }
                }
                _ => i += 1,
            }
        }

        Self { newline_positions }
    }

    /// Convert a byte offset to a line number (0-indexed)
    ///
    /// Returns the line number containing the given byte offset.
    /// If the offset is beyond the text, returns the last line number.
    pub fn offset_to_line(&self, offset: usize) -> usize {
        // Binary search to find the line containing this offset
        match self.newline_positions.binary_search(&offset) {
            Ok(line) => {
                // Exact match - if this is position 0, it's line 0
                // If it's a position after a newline, it's that line
                if line == 0 { 0 } else { line }
            }
            Err(line) => {
                // Insert position - the line number is one less than insert position
                if line == 0 { 0 } else { line - 1 }
            }
        }
    }

    /// Convert a text range to a line range (0-indexed)
    ///
    /// Returns a `Range<usize>` containing the start and end line numbers.
    pub fn range_to_lines(&self, start_offset: usize, end_offset: usize) -> std::ops::Range<usize> {
        std::ops::Range {
            start: self.offset_to_line(start_offset),
            end: self.offset_to_line(end_offset),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let converter = LineConverter::new("");
        assert_eq!(converter.offset_to_line(0), 0);
    }

    #[test]
    fn single_line() {
        let converter = LineConverter::new("hello");
        assert_eq!(converter.offset_to_line(0), 0);
        assert_eq!(converter.offset_to_line(2), 0);
        assert_eq!(converter.offset_to_line(5), 0);
    }

    #[test]
    fn multiple_lines() {
        let text = "first\nsecond\nthird";
        let converter = LineConverter::new(text);

        assert_eq!(converter.offset_to_line(0), 0); // "first"
        assert_eq!(converter.offset_to_line(5), 0); // end of "first"
        assert_eq!(converter.offset_to_line(6), 1); // "second"
        assert_eq!(converter.offset_to_line(12), 1); // end of "second"
        assert_eq!(converter.offset_to_line(13), 2); // "third"
    }

    #[test]
    fn windows_line_endings() {
        let text = "first\r\nsecond\r\nthird";
        let converter = LineConverter::new(text);

        assert_eq!(converter.offset_to_line(0), 0); // "first"
        assert_eq!(converter.offset_to_line(7), 1); // "second" (after \r\n)
        assert_eq!(converter.offset_to_line(15), 2); // "third" (after \r\n)
    }
}
