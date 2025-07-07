//! Tests for the shared LineConverter utility
//!
//! These tests define the expected behavior before extracting the common pattern
//! from python.rs and toml.rs

#[cfg(test)]
mod line_converter_tests {
    use semantic_code_edit_mcp::languages::utils::LineConverter;

    #[test]
    fn empty_string_has_no_lines() {
        let converter = LineConverter::new("");
        assert_eq!(converter.offset_to_line(0), 0);
    }

    #[test]
    fn single_line_without_newline() {
        let converter = LineConverter::new("hello world");
        assert_eq!(converter.offset_to_line(0), 0);
        assert_eq!(converter.offset_to_line(5), 0);
        assert_eq!(converter.offset_to_line(11), 0);
    }

    #[test]
    fn single_line_with_newline() {
        let converter = LineConverter::new("hello world\n");
        assert_eq!(converter.offset_to_line(0), 0);
        assert_eq!(converter.offset_to_line(11), 0);
        assert_eq!(converter.offset_to_line(12), 1); // After newline
    }

    #[test]
    fn multiple_lines() {
        let text = "line one\nline two\nline three";
        let converter = LineConverter::new(text);

        // First line
        assert_eq!(converter.offset_to_line(0), 0);
        assert_eq!(converter.offset_to_line(7), 0);

        // Second line
        assert_eq!(converter.offset_to_line(9), 1);
        assert_eq!(converter.offset_to_line(16), 1);

        // Third line
        assert_eq!(converter.offset_to_line(18), 2);
        assert_eq!(converter.offset_to_line(28), 2);
    }

    #[test]
    fn empty_lines() {
        let text = "line one\n\n\nline four";
        let converter = LineConverter::new(text);

        assert_eq!(converter.offset_to_line(0), 0); // "line one"
        assert_eq!(converter.offset_to_line(9), 1); // First empty line
        assert_eq!(converter.offset_to_line(10), 2); // Second empty line
        assert_eq!(converter.offset_to_line(11), 3); // "line four"
    }

    #[test]
    fn offset_beyond_text_returns_last_line() {
        let text = "line one\nline two";
        let converter = LineConverter::new(text);

        assert_eq!(converter.offset_to_line(1000), 1);
    }

    #[test]
    fn carriage_return_line_endings() {
        let text = "line one\r\nline two\r\nline three";
        let converter = LineConverter::new(text);

        assert_eq!(converter.offset_to_line(0), 0);
        assert_eq!(converter.offset_to_line(10), 1); // After \r\n
        assert_eq!(converter.offset_to_line(20), 2);
    }

    #[test]
    fn mixed_line_endings() {
        let text = "line one\nline two\r\nline three\rline four";
        let converter = LineConverter::new(text);

        assert_eq!(converter.offset_to_line(0), 0);
        assert_eq!(converter.offset_to_line(9), 1); // After \n
        assert_eq!(converter.offset_to_line(19), 2); // After \r\n
        assert_eq!(converter.offset_to_line(30), 3); // After \r
    }

    #[test]
    fn unicode_text() {
        let text = "첫째 줄\n둘째 줄\n세째 줄";
        let converter = LineConverter::new(text);

        // Note: offsets are byte positions, not character positions
        assert_eq!(converter.offset_to_line(0), 0);
        assert_eq!(converter.offset_to_line(11), 1); // After first line + \n
        assert_eq!(converter.offset_to_line(23), 2); // After second line + \n
    }

    #[test]
    fn trailing_newlines() {
        let text = "line one\n\n\n";
        let converter = LineConverter::new(text);

        assert_eq!(converter.offset_to_line(0), 0);
        assert_eq!(converter.offset_to_line(9), 1);
        assert_eq!(converter.offset_to_line(10), 2);
        assert_eq!(converter.offset_to_line(11), 3);
    }
}
