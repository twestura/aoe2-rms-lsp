//! Utility functions for .rms file parsing.

use tower_lsp::lsp_types::Position;

/// Returns `true` if the given position falls within a block comment in `text`,
/// `false` otherwise.
///
/// RMS block comments begin with `/*` and end with `*/`. Both delimiters must
/// be preceded by whitespace or the start of the file, and followed by
/// whitespace or the end of the file. Comments may be nested. An unterminated
/// comment that reaches the end of the file is valid.
///
/// Returns `true` if the position is on a comment delimiter itself (`/*` or
/// `*/`).
///
/// See the
/// [LSP specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#position)
/// for position encoding details.
pub fn is_in_comment(text: &str, position: Position) -> bool {
    let Some(offset) = position_to_offset(text, position) else {
        return false; // invalid position, not in a comment
    };
    let mut depth = 0;
    let mut cursor = CommentCursor::new(text);
    for _ in 0..offset {
        if cursor.is_comment_open() {
            depth += 1;
        } else if cursor.is_comment_close() && depth > 0 {
            depth -= 1;
        }
        cursor.next();
    }
    depth > 0 || cursor.is_comment_open() || cursor.is_comment_close()
}

/// A cursor for scanning RMS text that tracks whitespace context
/// and can detect comment delimiters.
///
/// Treats the start of the file as preceded by whitespace.
struct CommentCursor<'a> {
    text: &'a str,
    offset: usize,
    prev_ws: bool,
}

impl<'a> CommentCursor<'a> {
    /// Creates a new cursor at the start of `text`.
    fn new(text: &'a str) -> Self {
        Self {
            text,
            offset: 0,
            prev_ws: true,
        }
    }

    /// Returns the current character without advancing, or `None` if at
    /// end of text.
    fn peek(&self) -> Option<char> {
        self.text[self.offset..].chars().next()
    }

    /// Returns the character after current without advancing, or `None` if
    /// fewer than two characters remain.
    fn peek2(&self) -> Option<char> {
        let mut chars = self.text[self.offset..].chars();
        chars.next()?;
        chars.next()
    }

    /// Advances past the current character and returns it, or `None` if
    /// at end of text.
    fn next(&mut self) -> Option<char> {
        let c = self.text[self.offset..].chars().next()?;
        self.prev_ws = c.is_ascii_whitespace();
        self.offset += c.len_utf8();
        Some(c)
    }

    /// Returns `true` if the current position is the start of a valid
    /// comment opener `/*`, preceded by whitespace or start of file,
    /// and followed by whitespace or EOF.
    fn is_comment_open(&self) -> bool {
        self.peek() == Some('/')
            && self.prev_ws
            && self.peek2() == Some('*')
            && self.text[self.offset..]
                .chars()
                .nth(2)
                .map(|c| c.is_ascii_whitespace())
                .unwrap_or(true)
    }

    /// Returns `true` if the current position is the start of a valid
    /// comment closer `*/`, preceded by whitespace, and followed by
    /// whitespace or EOF.
    fn is_comment_close(&self) -> bool {
        self.peek() == Some('*')
            && self.prev_ws
            && self.peek2() == Some('/')
            && self.text[self.offset..]
                .chars()
                .nth(2)
                .map(|c| c.is_ascii_whitespace())
                .unwrap_or(true)
    }
}

/// Given a UTF-8 encoded string `text` and a `Position` (line, character),
/// returns the byte offset into `text` corresponding to that position.
///
/// - Lines are separated by `\n`. Line numbers are zero-based.
/// - Character offsets are UTF-8 code unit counts within the line, zero-based.
/// - If the character offset exceeds the line length, returns the offset of the
///   end of the line (clamping behavior, per the
///   [LSP spec](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#position)).
/// - If the line number exceeds the number of lines in the document, returns
///   `None`.
/// - A position at the very end of the document (line = last line, character =
///   last line length) returns `Some(text.len())`.
fn position_to_offset(text: &str, position: Position) -> Option<usize> {
    let mut current_lineno = 0;
    let mut current_col = 0;
    for (i, c) in text.char_indices() {
        let is_line = current_lineno == position.line;
        debug_assert!(
            current_lineno < position.line || is_line && current_col <= position.character
        );
        if is_line && current_col == position.character {
            return Some(i);
        }
        if c == '\n' {
            if is_line {
                return Some(i);
            }
            current_lineno += 1;
            current_col = 0;
        } else {
            current_col += 1;
        }
    }
    debug_assert!(current_lineno <= position.line);
    (current_lineno == position.line).then_some(text.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower_lsp::lsp_types::Position;

    mod is_in_comment {
        use super::*;

        /// A position in plain text is not in a comment.
        #[test]
        fn test_not_in_comment() {
            let text = "create_land { }";
            assert!(!is_in_comment(text, Position::new(0, 0)));
        }

        /// A position inside a block comment is in a comment.
        #[test]
        fn test_in_comment() {
            let text = "/* this is a comment */";
            assert!(is_in_comment(text, Position::new(0, 5)));
        }

        /// A position on the opening delimiter is in a comment.
        #[test]
        fn test_on_opening_delimiter() {
            let text = "/* comment */";
            assert!(is_in_comment(text, Position::new(0, 0)));
        }

        /// A position on the closing delimiter is in a comment.
        #[test]
        fn test_on_closing_delimiter() {
            let text = "/* comment */";
            assert!(is_in_comment(text, Position::new(0, 11)));
        }

        /// A position inside a nested comment is in a comment.
        #[test]
        fn test_nested_comment() {
            let text = "/* outer /* inner */ still outer */";
            assert!(is_in_comment(text, Position::new(0, 25)));
        }

        /// A position after a closed comment is not in a comment.
        #[test]
        fn test_after_comment() {
            let text = "/* comment */ <PLAYER_SETUP>";
            assert!(!is_in_comment(text, Position::new(0, 14)));
        }

        /// An unterminated comment that reaches end of file is valid.
        #[test]
        fn test_unterminated_comment() {
            let text = "/* unterminated";
            assert!(is_in_comment(text, Position::new(0, 5)));
        }

        /// A position in a comment spanning multiple lines is in a comment.
        #[test]
        fn test_multiline_comment() {
            let text = "/* comment\nstill comment */";
            assert!(is_in_comment(text, Position::new(1, 0)));
        }

        /// A position after a multi-byte Unicode character correctly detects
        /// a comment.
        #[test]
        fn test_comment_unicode_preceding() {
            let text = "é /* comment */";
            assert!(is_in_comment(text, Position::new(0, 4)));
        }
    }

    mod comment_cursor {
        use super::*;

        /// A cursor over an empty string has offset 0 and all peeks return None.
        #[test]
        fn test_empty_string() {
            let mut cursor = CommentCursor::new("");
            assert_eq!(cursor.peek(), None);
            assert_eq!(cursor.peek2(), None);
            assert_eq!(cursor.next(), None);
        }

        /// A cursor over a single character can peek and advance once.
        #[test]
        fn test_single_char() {
            let mut cursor = CommentCursor::new("a");
            assert_eq!(cursor.peek(), Some('a'));
            assert_eq!(cursor.peek2(), None);
            assert_eq!(cursor.next(), Some('a'));
            assert_eq!(cursor.peek(), None);
            assert_eq!(cursor.peek2(), None);
            assert_eq!(cursor.next(), None);
        }

        /// A cursor over two characters can peek ahead by two and advance twice.
        #[test]
        fn test_two_chars() {
            let mut cursor = CommentCursor::new("ab");
            assert_eq!(cursor.peek(), Some('a'));
            assert_eq!(cursor.peek2(), Some('b'));
            assert_eq!(cursor.next(), Some('a'));
            assert_eq!(cursor.peek(), Some('b'));
            assert_eq!(cursor.peek2(), None);
            assert_eq!(cursor.next(), Some('b'));
            assert_eq!(cursor.next(), None);
        }

        /// A cursor over three characters correctly transitions through all states.
        #[test]
        fn test_three_chars() {
            let mut cursor = CommentCursor::new("abc");
            assert_eq!(cursor.peek(), Some('a'));
            assert_eq!(cursor.peek2(), Some('b'));
            assert_eq!(cursor.next(), Some('a'));
            assert_eq!(cursor.peek(), Some('b'));
            assert_eq!(cursor.peek2(), Some('c'));
            assert_eq!(cursor.next(), Some('b'));
            assert_eq!(cursor.peek(), Some('c'));
            assert_eq!(cursor.peek2(), None);
            assert_eq!(cursor.next(), Some('c'));
            assert_eq!(cursor.next(), None);
        }

        /// A cursor over a longer string has correct state at initialization.
        #[test]
        fn test_longer_string_init() {
            let cursor = CommentCursor::new("hello world");
            assert_eq!(cursor.peek(), Some('h'));
            assert_eq!(cursor.peek2(), Some('e'));
        }

        /// A cursor over a longer string has correct state in the middle.
        #[test]
        fn test_longer_string_middle() {
            let mut cursor = CommentCursor::new("hello world");
            for _ in 0..5 {
                cursor.next();
            }
            assert_eq!(cursor.peek(), Some(' '));
            assert_eq!(cursor.peek2(), Some('w'));
        }

        /// A cursor over a longer string has correct state at the end.
        #[test]
        fn test_longer_string_end() {
            let mut cursor = CommentCursor::new("hello");
            for _ in 0..4 {
                cursor.next();
            }
            assert_eq!(cursor.peek(), Some('o'));
            assert_eq!(cursor.peek2(), None);
            cursor.next();
            assert_eq!(cursor.peek(), None);
            assert_eq!(cursor.peek2(), None);
            assert_eq!(cursor.next(), None);
        }

        /// A cursor over a string with a multi-byte Unicode character has the correct
        /// byte offset after advancing past it.
        #[test]
        fn test_unicode_char_offset() {
            let mut cursor = CommentCursor::new("é");
            assert_eq!(cursor.peek(), Some('é'));
            assert_eq!(cursor.next(), Some('é'));
            assert_eq!(cursor.next(), None);
        }

        /// A cursor correctly peeks past a multi-byte Unicode character.
        #[test]
        fn test_unicode_peek2() {
            let cursor = CommentCursor::new("éa");
            assert_eq!(cursor.peek(), Some('é'));
            assert_eq!(cursor.peek2(), Some('a'));
        }

        /// A cursor over a string with a multi-byte Unicode character followed by
        /// ASCII has correct offsets throughout.
        #[test]
        fn test_unicode_then_ascii() {
            let mut cursor = CommentCursor::new("héllo");
            assert_eq!(cursor.next(), Some('h'));
            assert_eq!(cursor.next(), Some('é'));
            assert_eq!(cursor.next(), Some('l'));
        }

        /// A cursor at the start of `/* comment */` detects a comment opener.
        #[test]
        fn test_comment_open_basic() {
            let cursor = CommentCursor::new("/* comment */");
            assert!(cursor.is_comment_open());
        }

        /// A comment opener not preceded by whitespace is not detected.
        #[test]
        fn test_comment_open_no_preceding_whitespace() {
            let mut cursor = CommentCursor::new("a/* comment */");
            cursor.next();
            assert!(!cursor.is_comment_open());
        }

        /// A comment opener not followed by whitespace is not detected.
        #[test]
        fn test_comment_open_no_following_whitespace() {
            let cursor = CommentCursor::new("/*comment */");
            assert!(!cursor.is_comment_open());
        }

        /// A comment opener at end of file is detected.
        #[test]
        fn test_comment_open_at_eof() {
            let cursor = CommentCursor::new("/*");
            assert!(cursor.is_comment_open());
        }

        /// A comment opener preceded by whitespace mid-text is detected.
        #[test]
        fn test_comment_open_after_whitespace() {
            let mut cursor = CommentCursor::new(" /* comment */");
            cursor.next();
            assert!(cursor.is_comment_open());
        }

        /// A plain `/` not followed by `*` is not a comment opener.
        #[test]
        fn test_comment_open_plain_slash() {
            let cursor = CommentCursor::new("/ something");
            assert!(!cursor.is_comment_open());
        }

        /// A cursor at `*/` preceded by whitespace detects a comment closer.
        #[test]
        fn test_comment_close_basic() {
            let mut cursor = CommentCursor::new("/* x */ rest");
            cursor.next(); // /
            cursor.next(); // *
            cursor.next(); // ' '
            cursor.next(); // x
            cursor.next(); // ' '
            assert!(cursor.is_comment_close());
        }

        /// A comment closer not preceded by whitespace is not detected.
        #[test]
        fn test_comment_close_no_preceding_whitespace() {
            let mut cursor = CommentCursor::new("/* x*/ rest");
            cursor.next(); // /
            cursor.next(); // *
            cursor.next(); // ' '
            cursor.next(); // x
            assert!(!cursor.is_comment_close());
        }

        /// A comment closer not followed by whitespace is not detected.
        #[test]
        fn test_comment_close_no_following_whitespace() {
            let mut cursor = CommentCursor::new("/* x */rest");
            cursor.next(); // /
            cursor.next(); // *
            cursor.next(); // ' '
            cursor.next(); // x
            cursor.next(); // ' '
            assert!(!cursor.is_comment_close());
        }

        /// A comment closer at end of file is detected.
        #[test]
        fn test_comment_close_at_eof() {
            let mut cursor = CommentCursor::new("/* x */");
            cursor.next(); // /
            cursor.next(); // *
            cursor.next(); // ' '
            cursor.next(); // x
            cursor.next(); // ' '
            assert!(cursor.is_comment_close());
        }
    }

    mod position_to_offset {
        use super::*;

        /// The offset of a character in the middle of a multi-line document
        /// is the sum of the lengths of all preceding lines, their newlines,
        /// and the character's column offset.
        #[test]
        fn test_offset_middle_of_document() {
            let text = "hello\nworld\nfoo";
            assert_eq!(position_to_offset(text, Position::new(1, 3)), Some(9));
        }

        /// The offset of the start of the file is 0.
        #[test]
        fn test_offset_start_of_file() {
            let text = "hello world";
            assert_eq!(position_to_offset(text, Position::new(0, 0)), Some(0));
        }

        /// The offset of a character in the middle of the first line is its column
        /// index.
        #[test]
        fn test_offset_middle_of_line() {
            let text = "hello world";
            assert_eq!(position_to_offset(text, Position::new(0, 6)), Some(6));
        }

        /// The offset of the start of the second line is the length of the first
        /// line plus the newline.
        #[test]
        fn test_offset_second_line() {
            let text = "hello\nworld";
            assert_eq!(position_to_offset(text, Position::new(1, 0)), Some(6));
        }

        /// A character offset beyond the end of a line clamps to the end of that
        /// line.
        #[test]
        fn test_offset_clamps_to_line_length() {
            let text = "hello\nworld";
            assert_eq!(position_to_offset(text, Position::new(0, 100)), Some(5));
        }

        /// A line number beyond the end of the document returns None.
        #[test]
        fn test_offset_line_out_of_bounds() {
            let text = "hello\nworld";
            assert_eq!(position_to_offset(text, Position::new(5, 0)), None);
        }

        /// The offset of the last character of the document is text.len().
        #[test]
        fn test_offset_end_of_document() {
            let text = "hello\nworld";
            assert_eq!(position_to_offset(text, Position::new(1, 5)), Some(11));
        }

        /// The offset of a character in a line following a multi-byte Unicode character
        /// is correctly calculated using byte offsets.
        #[test]
        fn test_offset_unicode() {
            let text = "héllo\nworld";
            assert_eq!(position_to_offset(text, Position::new(0, 2)), Some(3));
        }

        /// The offset of position (0, 0) in an empty document is 0.
        #[test]
        fn test_offset_empty_document() {
            let text = "";
            assert_eq!(position_to_offset(text, Position::new(0, 0)), Some(0));
        }

        /// A line number beyond the end of an empty document returns None.
        #[test]
        fn test_offset_empty_document_out_of_bounds() {
            let text = "";
            assert_eq!(position_to_offset(text, Position::new(1, 0)), None);
        }

        /// A position on an empty line returns the offset of that line's start.
        #[test]
        fn test_offset_empty_line() {
            let text = "hello\n\nworld";
            assert_eq!(position_to_offset(text, Position::new(1, 0)), Some(6));
        }

        /// A document with a trailing newline treats the content after the newline
        /// as an additional empty line.
        #[test]
        fn test_offset_trailing_newline() {
            let text = "hello\n";
            assert_eq!(position_to_offset(text, Position::new(1, 0)), Some(6));
        }
    }
}
