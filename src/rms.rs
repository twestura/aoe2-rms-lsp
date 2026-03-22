//! Utility functions for .rms file parsing.

use tower_lsp::lsp_types::{Position, Range};

/// The context of a position within an RMS document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DocumentContext {
    /// Whether the position falls within a block comment.
    pub in_comment: bool,
}

/// Returns the `DocumentContext` at the given position in `text`.
///
/// Scans the document from the start to `position` in a single pass,
/// tracking comment depth.
///
/// - `in_comment` is `true` if the position falls within a block comment
///   or on a comment delimiter.
/// - Returns a context with `in_comment` set to `false` if `position` is out
///   of bounds.
///
/// See the
/// [LSP specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#position)
/// for position encoding details.
pub fn document_context_at(text: &str, position: Position) -> DocumentContext {
    let Some(offset) = position_to_offset(text, position) else {
        return DocumentContext { in_comment: false };
    };
    let mut cursor = RmsCursor::new(text);
    let mut depth = 0;
    for _ in 0..offset {
        if cursor.is_comment_open() {
            depth += 1;
        } else if cursor.is_comment_close() && depth > 0 {
            depth -= 1;
        }
        cursor.next();
    }
    let in_comment = depth > 0 || cursor.is_comment_open() || cursor.is_comment_close();
    DocumentContext { in_comment }
}

/// A cursor for scanning RMS text that tracks whitespace context
/// and can detect comment delimiters.
///
/// Treats the start of the file as preceded by whitespace.
struct RmsCursor<'a> {
    text: &'a str,
    offset: usize,
    prev_ws: bool,
}

impl<'a> RmsCursor<'a> {
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

/// Extracts the token at the given position from the text.
/// Returns `Some(token)` if the position is within text.
/// Otherwise returns `None` if the position is within whitespace.
///
/// Only ascii whitespace is considered whitespace, consistent with RMS.
/// Returns `None` if the position is out of bounds.
pub fn extract_token(text: &str, position: Position) -> Option<&str> {
    let line = text.split("\n").nth(position.line as usize)?;
    let col = position.character as usize;
    if col > line.len() {
        return None;
    }

    let right = line[col..]
        .find(|c: char| c.is_ascii_whitespace())
        .map(|i| i + col)
        .unwrap_or(line.len());
    // Return `None` if `col` is the index of a whitespace character.
    if right == col {
        return None;
    }
    let left = line[..col]
        .rfind(|c: char| c.is_ascii_whitespace())
        .map(|i| i + 1)
        .unwrap_or(0);
    debug_assert_ne!(
        left, right,
        "right > left because the first character is not whitespace"
    );
    Some(&line[left..right])
}

/// The result of extracting a completion text at a cursor position.
/// Note the returned token may be empty if the cursor's position is
/// within two consecutive whitespace characters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionText {
    /// The lowercase text to use for filtering completions.
    /// `token == &line[left..right]`, where `line` is the line of text
    /// containing the token.
    /// Empty when `left == right`.
    pub text: String,
    /// The prefix of the token up to the cursor position.
    /// All lowercase.
    /// Must be a prefix of `text`.
    pub prefix: String,
    /// The range of the token in the document.
    pub range: Range,
}

impl CompletionText {
    /// Creates a new `CompletionText` from a line of text and cursor position,
    /// where `line[left..right]` is the full token at the cursor.
    ///
    /// Requires `left <= right` and `left <= position.character`.
    fn new(line: &str, left: usize, right: usize, position: Position) -> Self {
        debug_assert!(left <= right);
        debug_assert!(left as u32 <= position.character);

        let text = line[left..right].to_lowercase();
        let prefix = line[left..right.min(position.character as usize)].to_lowercase();
        let range = Range {
            start: Position {
                line: position.line,
                character: left as u32,
            },
            end: Position {
                line: position.line,
                character: right as u32,
            },
        };
        Self {
            text,
            prefix,
            range,
        }
    }
}

/// Extracts the autocomplete text for the given position in the text.
/// - If the position is within text, returns the entire text.
/// - If the position is on a whitespace character preceded immediately by
///   text, returns the text.
/// - Returns `None` if the `position` is out of bounds.
pub fn extract_autocomplete_prefix(text: &str, position: Position) -> Option<CompletionText> {
    let line = text.split("\n").nth(position.line as usize)?;
    let col = position.character as usize;
    // The column index is out of bounds.
    if col > line.len() {
        return None;
    }

    let bytes = line.as_bytes();
    let mut left = col;
    while left > 0 && !bytes[left - 1].is_ascii_whitespace() {
        left -= 1;
    }
    let mut right = col;
    while right < line.len() && !bytes[right].is_ascii_whitespace() {
        right += 1;
    }

    Some(CompletionText::new(line, left, right, position))
}

/// Returns `true` if the cursor is on whitespace at `position` in `text`,
/// `false` otherwise. A cursor beyong the end of the line is not considered
/// whitespace.
// fn is_on_whitespace(text: &str, position: Position) -> bool {
//     text.split('\n')
//         .nth(position.line as usize)
//         .map_or(false, |line| {
//             let col = position.character as usize;
//             col < line.len() && line.as_bytes()[col].is_ascii_whitespace()
//         })
// }

/// Returns all tokens from `text` that precede `position`, in document order.
///
/// Tokens on earlier lines are always included. For tokens on the same line
/// as `position`:
/// - If the cursor is on whitespace, tokens whose end is at or before the
///   cursor character are included, so that completed tokens immediately
///   before the cursor are visible to the backwards scan.
/// - If the cursor is within a token, only tokens that end strictly before
///   the cursor character are included, excluding the partial token currently
///   being typed.
// fn tokens_before<'a>(text: &'a str, position: Position) -> Vec<Token<'a>> {
//     let mut tokens = vec![];
//     let lexer = Lexer::new(text);
//     for token in lexer {
//         if token.line < position.line {
//             tokens.push(token);
//         } else if token.line == position.line {
//             if is_on_whitespace(text, position) {
//                 if token.end <= position.character {
//                     tokens.push(token);
//                 }
//             } else {
//                 if token.end < position.character {
//                     tokens.push(token);
//                 }
//             }
//         } else {
//             break;
//         }
//     }
//     tokens
// }

/// Returns `true` if the cursor position is within an argument slot of a
/// preceding instruction, indicating that keyword completions should be
/// suppressed.
///
/// Scans backwards through the tokens that end at or before the cursor
/// position, counting arguments until a recognized instruction is found.
/// If the number of arguments seen is less than the instruction's total
/// argument count (required + optional), the cursor is in an argument
/// position.
///
/// A math expression delimited by `(` and `)` counts as a single argument.
/// The scan stops and returns `false` at block boundaries (`{`, `}`),
/// section headers, or the start of the document.
///
/// Returns `false` if the position is out of bounds.
// pub fn is_in_argument_position(text: &str, position: Position) -> bool {
//     let prev_tokens = tokens_before(text, position);
//     crate::_log(&format!(
//         "is_in_argument_position pos=({},{}) prev_tokens={:?}",
//         position.line,
//         position.character,
//         prev_tokens.iter().map(|t| t.text).collect::<Vec<_>>()
//     ));
//     let mut i = prev_tokens.len();
//     let mut args_seen = 0;
//     while i > 0 && args_seen < MAX_ARGS {
//         i -= 1;
//         let text = prev_tokens[i].text;
//         match text {
//             ")" => {
//                 // Consume the entire math expression.
//                 while i > 0 {
//                     i -= 1;
//                     if prev_tokens[i].text == "(" {
//                         break;
//                     }
//                 }
//                 args_seen += 1;
//             }
//             t if t == "{" || t == "}" || instructions::is_section_header(t) => return false,
//             t if args_seen < instructions::arguments(t).total() => return true,
//             _ => args_seen += 1,
//         }
//     }
//     false
// }

#[cfg(test)]
mod tests {
    use super::*;
    use tower_lsp::lsp_types::Position;

    mod document_context_at {
        use super::*;

        /// An out of bounds position returns a context with both fields false.
        #[test]
        fn test_out_of_bounds() {
            let text = "hello";
            assert_eq!(
                document_context_at(text, Position::new(5, 0)),
                DocumentContext { in_comment: false }
            );
        }

        /// A position inside a block returns the correct context.
        #[test]
        fn test_in_block() {
            let text = "<LAND_GENERATION>\ncreate_land {\nterrain_type GRASS\n}";
            assert_eq!(
                document_context_at(text, Position::new(2, 0)),
                DocumentContext { in_comment: false }
            );
        }

        /// A position in plain text is not in a comment.
        #[test]
        fn test_not_in_comment() {
            let text = "create_land { }";
            assert!(!document_context_at(text, Position::new(0, 0)).in_comment);
        }

        /// A position inside a block comment is in a comment.
        #[test]
        fn test_in_comment() {
            let text = "/* this is a comment */";
            assert!(document_context_at(text, Position::new(0, 5)).in_comment);
        }

        /// A position on the opening delimiter is in a comment.
        #[test]
        fn test_on_opening_delimiter() {
            let text = "/* comment */";
            assert!(document_context_at(text, Position::new(0, 0)).in_comment);
        }

        /// A position on the closing delimiter is in a comment.
        #[test]
        fn test_on_closing_delimiter() {
            let text = "/* comment */";
            assert!(document_context_at(text, Position::new(0, 11)).in_comment);
        }

        /// A position inside a nested comment is in a comment.
        #[test]
        fn test_nested_comment() {
            let text = "/* outer /* inner */ still outer */";
            assert!(document_context_at(text, Position::new(0, 25)).in_comment);
        }

        /// A position after a closed comment is not in a comment.
        #[test]
        fn test_after_comment() {
            let text = "/* comment */ <PLAYER_SETUP>";
            assert!(!document_context_at(text, Position::new(0, 14)).in_comment);
        }

        /// An unterminated comment that reaches end of file is valid.
        #[test]
        fn test_unterminated_comment() {
            let text = "/* unterminated";
            assert!(document_context_at(text, Position::new(0, 5)).in_comment);
        }

        /// A position in a comment spanning multiple lines is in a comment.
        #[test]
        fn test_multiline_comment() {
            let text = "/* comment\nstill comment */";
            assert!(document_context_at(text, Position::new(1, 0)).in_comment);
        }

        /// A position after a multi-byte Unicode character correctly detects a comment.
        #[test]
        fn test_comment_unicode_preceding() {
            let text = "é /* comment */";
            assert!(document_context_at(text, Position::new(0, 4)).in_comment);
        }

        /// A brace inside a comment does not affect in_block.
        #[test]
        fn test_brace_inside_comment_ignored() {
            let text = "/* { */ base_terrain";
            assert_eq!(
                document_context_at(text, Position::new(0, 8)),
                DocumentContext { in_comment: false }
            );
        }
    }

    mod rms_cursor {
        use super::*;

        /// A cursor over an empty string has offset 0 and all peeks return None.
        #[test]
        fn test_empty_string() {
            let mut cursor = RmsCursor::new("");
            assert_eq!(cursor.peek(), None);
            assert_eq!(cursor.peek2(), None);
            assert_eq!(cursor.next(), None);
        }

        /// A cursor over a single character can peek and advance once.
        #[test]
        fn test_single_char() {
            let mut cursor = RmsCursor::new("a");
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
            let mut cursor = RmsCursor::new("ab");
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
            let mut cursor = RmsCursor::new("abc");
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
            let cursor = RmsCursor::new("hello world");
            assert_eq!(cursor.peek(), Some('h'));
            assert_eq!(cursor.peek2(), Some('e'));
        }

        /// A cursor over a longer string has correct state in the middle.
        #[test]
        fn test_longer_string_middle() {
            let mut cursor = RmsCursor::new("hello world");
            for _ in 0..5 {
                cursor.next();
            }
            assert_eq!(cursor.peek(), Some(' '));
            assert_eq!(cursor.peek2(), Some('w'));
        }

        /// A cursor over a longer string has correct state at the end.
        #[test]
        fn test_longer_string_end() {
            let mut cursor = RmsCursor::new("hello");
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
            let mut cursor = RmsCursor::new("é");
            assert_eq!(cursor.peek(), Some('é'));
            assert_eq!(cursor.next(), Some('é'));
            assert_eq!(cursor.next(), None);
        }

        /// A cursor correctly peeks past a multi-byte Unicode character.
        #[test]
        fn test_unicode_peek2() {
            let cursor = RmsCursor::new("éa");
            assert_eq!(cursor.peek(), Some('é'));
            assert_eq!(cursor.peek2(), Some('a'));
        }

        /// A cursor over a string with a multi-byte Unicode character followed by
        /// ASCII has correct offsets throughout.
        #[test]
        fn test_unicode_then_ascii() {
            let mut cursor = RmsCursor::new("héllo");
            assert_eq!(cursor.next(), Some('h'));
            assert_eq!(cursor.next(), Some('é'));
            assert_eq!(cursor.next(), Some('l'));
        }

        /// A cursor at the start of `/* comment */` detects a comment opener.
        #[test]
        fn test_comment_open_basic() {
            let cursor = RmsCursor::new("/* comment */");
            assert!(cursor.is_comment_open());
        }

        /// A comment opener not preceded by whitespace is not detected.
        #[test]
        fn test_comment_open_no_preceding_whitespace() {
            let mut cursor = RmsCursor::new("a/* comment */");
            cursor.next();
            assert!(!cursor.is_comment_open());
        }

        /// A comment opener not followed by whitespace is not detected.
        #[test]
        fn test_comment_open_no_following_whitespace() {
            let cursor = RmsCursor::new("/*comment */");
            assert!(!cursor.is_comment_open());
        }

        /// A comment opener at end of file is detected.
        #[test]
        fn test_comment_open_at_eof() {
            let cursor = RmsCursor::new("/*");
            assert!(cursor.is_comment_open());
        }

        /// A comment opener preceded by whitespace mid-text is detected.
        #[test]
        fn test_comment_open_after_whitespace() {
            let mut cursor = RmsCursor::new(" /* comment */");
            cursor.next();
            assert!(cursor.is_comment_open());
        }

        /// A plain `/` not followed by `*` is not a comment opener.
        #[test]
        fn test_comment_open_plain_slash() {
            let cursor = RmsCursor::new("/ something");
            assert!(!cursor.is_comment_open());
        }

        /// A cursor at `*/` preceded by whitespace detects a comment closer.
        #[test]
        fn test_comment_close_basic() {
            let mut cursor = RmsCursor::new("/* x */ rest");
            assert_eq!(cursor.next(), Some('/'));
            assert_eq!(cursor.next(), Some('*'));
            assert_eq!(cursor.next(), Some(' '));
            assert_eq!(cursor.next(), Some('x'));
            assert_eq!(cursor.next(), Some(' '));
            assert!(cursor.is_comment_close());
        }

        /// A comment closer not preceded by whitespace is not detected.
        #[test]
        fn test_comment_close_no_preceding_whitespace() {
            let mut cursor = RmsCursor::new("/* x*/ rest");
            assert_eq!(cursor.next(), Some('/'));
            assert_eq!(cursor.next(), Some('*'));
            assert_eq!(cursor.next(), Some(' '));
            assert_eq!(cursor.next(), Some('x'));
            assert!(!cursor.is_comment_close());
        }

        /// A comment closer not followed by whitespace is not detected.
        #[test]
        fn test_comment_close_no_following_whitespace() {
            let mut cursor = RmsCursor::new("/* x */rest");
            assert_eq!(cursor.next(), Some('/'));
            assert_eq!(cursor.next(), Some('*'));
            assert_eq!(cursor.next(), Some(' '));
            assert_eq!(cursor.next(), Some('x'));
            assert_eq!(cursor.next(), Some(' '));
            assert!(!cursor.is_comment_close());
        }

        /// A comment closer at end of file is detected.
        #[test]
        fn test_comment_close_at_eof() {
            let mut cursor = RmsCursor::new("/* x */");
            assert_eq!(cursor.next(), Some('/'));
            assert_eq!(cursor.next(), Some('*'));
            assert_eq!(cursor.next(), Some(' '));
            assert_eq!(cursor.next(), Some('x'));
            assert_eq!(cursor.next(), Some(' '));
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

    mod extract_token {
        use super::*;

        /// A position within a token returns that token.
        #[test]
        fn test_middle_of_token() {
            let text = "create_land";
            assert_eq!(
                extract_token(text, Position::new(0, 5)),
                Some("create_land")
            );
        }

        /// A position on the first character of a token returns that token.
        #[test]
        fn test_first_character_of_token() {
            let text = "create_land";
            assert_eq!(
                extract_token(text, Position::new(0, 0)),
                Some("create_land")
            );
        }

        /// A position on the last character of a token returns that token.
        #[test]
        fn test_last_character_of_token() {
            let text = "create_land";
            assert_eq!(
                extract_token(text, Position::new(0, 10)),
                Some("create_land")
            );
        }

        /// A position within whitespace returns None.
        #[test]
        fn test_whitespace_returns_none() {
            let text = "create_land   create_player_lands";
            assert_eq!(extract_token(text, Position::new(0, 12)), None);
        }

        /// A position on the whitespace immediately after a token returns None.
        #[test]
        fn test_whitespace_immediately_after_token() {
            let text = "abc def";
            assert_eq!(extract_token(text, Position::new(0, 3)), None);
        }

        /// A position at the start of a line returns the first token.
        #[test]
        fn test_start_of_line() {
            let text = "terrain_type GRASS";
            assert_eq!(
                extract_token(text, Position::new(0, 0)),
                Some("terrain_type")
            );
        }

        /// A position at the last character of the last token on a line returns that token.
        #[test]
        fn test_end_of_line() {
            let text = "terrain_type GRASS";
            assert_eq!(extract_token(text, Position::new(0, 17)), Some("GRASS"));
        }

        /// A position on the first token of a multi-token line returns that token.
        #[test]
        fn test_multi_token_line_first() {
            let text = "terrain_type GRASS";
            assert_eq!(
                extract_token(text, Position::new(0, 5)),
                Some("terrain_type")
            );
        }

        /// A position on the second token of a multi-token line returns that token.
        #[test]
        fn test_multi_token_line_second() {
            let text = "terrain_type GRASS";
            assert_eq!(extract_token(text, Position::new(0, 13)), Some("GRASS"));
        }

        /// A position on a token on the second line returns that token.
        #[test]
        fn test_second_line_token() {
            let text = "create_land {\nterrain_type GRASS\n}";
            assert_eq!(
                extract_token(text, Position::new(1, 0)),
                Some("terrain_type")
            );
        }

        /// A position on the second token of the second line returns that token.
        #[test]
        fn test_second_line_second_token() {
            let text = "create_land {\nterrain_type GRASS\n}";
            assert_eq!(extract_token(text, Position::new(1, 13)), Some("GRASS"));
        }

        /// A position on whitespace on the second line returns None.
        #[test]
        fn test_second_line_whitespace() {
            let text = "create_land {\nterrain_type GRASS\n}";
            assert_eq!(extract_token(text, Position::new(1, 12)), None);
        }

        /// A position on a token on the third line returns that token.
        #[test]
        fn test_third_line_token() {
            let text = "create_land {\nterrain_type GRASS\nnumber_of_tiles 300\n}";
            assert_eq!(
                extract_token(text, Position::new(2, 0)),
                Some("number_of_tiles")
            );
        }

        /// A position on the first line is unaffected by content on later lines.
        #[test]
        fn test_first_line_unaffected_by_later_lines() {
            let text = "create_land {\nterrain_type GRASS\n}";
            assert_eq!(
                extract_token(text, Position::new(0, 0)),
                Some("create_land")
            );
        }

        /// A non-breaking space is not treated as a token separator,
        /// so it is included in the token.
        #[test]
        fn test_unicode_whitespace_not_a_separator() {
            let text = "abc\u{00A0}def";
            assert_eq!(
                extract_token(text, Position::new(0, 0)),
                Some("abc\u{00A0}def")
            );
        }

        /// An ASCII space is treated as a token separator.
        #[test]
        fn test_ascii_space_is_a_separator() {
            let text = "abc def";
            assert_eq!(extract_token(text, Position::new(0, 0)), Some("abc"));
        }

        /// A tab is treated as a token separator.
        #[test]
        fn test_ascii_tab_is_a_separator() {
            let text = "abc\tdef";
            assert_eq!(extract_token(text, Position::new(0, 0)), Some("abc"));
        }

        /// A position within a token containing a non-breaking space returns the full token.
        #[test]
        fn test_cursor_within_token_with_unicode_whitespace() {
            let text = "abc\u{00A0}def";
            assert_eq!(
                extract_token(text, Position::new(0, 5)),
                Some("abc\u{00A0}def")
            );
        }

        /// A character offset beyond the end of a line returns None.
        #[test]
        fn test_character_out_of_bounds() {
            let text = "A{";
            assert_eq!(extract_token(text, Position::new(0, 15)), None);
        }
    }

    mod extract_autocomplete_prefix {
        use super::*;

        /// A position within a token returns the full token.
        #[test]
        fn test_within_token() {
            let text = "create_land";
            assert_eq!(
                extract_autocomplete_prefix(text, Position::new(0, 5)),
                Some(CompletionText {
                    text: "create_land".to_string(),
                    prefix: "creat".to_string(),
                    range: Range {
                        start: Position::new(0, 0),
                        end: Position::new(0, 11),
                    },
                })
            );
        }

        /// A position on the first character of a token returns the full token.
        #[test]
        fn test_first_character_of_token() {
            let text = "create_land";
            assert_eq!(
                extract_autocomplete_prefix(text, Position::new(0, 0)),
                Some(CompletionText {
                    text: "create_land".to_string(),
                    prefix: "".to_string(),
                    range: Range {
                        start: Position::new(0, 0),
                        end: Position::new(0, 11),
                    },
                })
            );
        }

        /// A position on the last character of a token returns the full token.
        #[test]
        fn test_last_character_of_token() {
            let text = "create_land";
            assert_eq!(
                extract_autocomplete_prefix(text, Position::new(0, 10)),
                Some(CompletionText {
                    text: "create_land".to_string(),
                    prefix: "create_lan".to_string(),
                    range: Range {
                        start: Position::new(0, 0),
                        end: Position::new(0, 11),
                    },
                })
            );
        }

        /// A position on the whitespace immediately after a token returns the token.
        #[test]
        fn test_whitespace_immediately_after_token() {
            let text = "abc def";
            assert_eq!(
                extract_autocomplete_prefix(text, Position::new(0, 3)),
                Some(CompletionText {
                    text: "abc".to_string(),
                    prefix: "abc".to_string(),
                    range: Range {
                        start: Position::new(0, 0),
                        end: Position::new(0, 3),
                    },
                })
            );
        }

        /// A position on the second whitespace after a token returns an empty token.
        #[test]
        fn test_whitespace_not_immediately_after_token() {
            let text = "abc  def";
            assert_eq!(
                extract_autocomplete_prefix(text, Position::new(0, 4)),
                Some(CompletionText {
                    text: "".to_string(),
                    prefix: "".to_string(),
                    range: Range {
                        start: Position::new(0, 4),
                        end: Position::new(0, 4),
                    },
                })
            );
        }

        /// A position on leading whitespace returns an empty token.
        #[test]
        fn test_leading_whitespace() {
            let text = "  abc";
            assert_eq!(
                extract_autocomplete_prefix(text, Position::new(0, 0)),
                Some(CompletionText {
                    text: "".to_string(),
                    prefix: "".to_string(),
                    range: Range {
                        start: Position::new(0, 0),
                        end: Position::new(0, 0),
                    },
                })
            );
        }

        /// A position on a token on the second line returns the full token.
        #[test]
        fn test_second_line_token() {
            let text = "create_land {\nterrain_type GRASS\n}";
            assert_eq!(
                extract_autocomplete_prefix(text, Position::new(1, 5)),
                Some(CompletionText {
                    text: "terrain_type".to_string(),
                    prefix: "terra".to_string(),
                    range: Range {
                        start: Position::new(1, 0),
                        end: Position::new(1, 12),
                    },
                })
            );
        }

        /// A position on whitespace immediately after a token on the second line returns the token.
        #[test]
        fn test_second_line_whitespace_after_token() {
            let text = "create_land {\nterrain_type GRASS\n}";
            assert_eq!(
                extract_autocomplete_prefix(text, Position::new(1, 12)),
                Some(CompletionText {
                    text: "terrain_type".to_string(),
                    prefix: "terrain_type".to_string(),
                    range: Range {
                        start: Position::new(1, 0),
                        end: Position::new(1, 12),
                    },
                })
            );
        }

        /// A non-breaking space is not treated as a token separator.
        #[test]
        fn test_unicode_whitespace_not_a_separator() {
            let text = "abc\u{00A0}def";
            assert_eq!(
                extract_autocomplete_prefix(text, Position::new(0, 0)),
                Some(CompletionText {
                    text: "abc\u{00A0}def".to_string(),
                    prefix: "".to_string(),
                    range: Range {
                        start: Position::new(0, 0),
                        end: Position::new(0, 8),
                    },
                })
            );
        }

        /// A line number beyond the end of the document returns None.
        #[test]
        fn test_line_out_of_bounds() {
            let text = "create_land";
            assert_eq!(extract_autocomplete_prefix(text, Position::new(1, 0)), None);
        }

        /// A character offset beyond the end of a line returns None.
        #[test]
        fn test_character_out_of_bounds() {
            let text = "create_land";
            assert_eq!(
                extract_autocomplete_prefix(text, Position::new(0, 100)),
                None
            );
        }
    }

    // mod is_in_argument_position {
    //     use super::*;

    //     /// An empty document is not an argument position.
    //     #[test]
    //     fn test_empty_document() {
    //         assert!(!is_in_argument_position("", Position::new(0, 0)));
    //     }

    //     /// A zero-argument command is not followed by an argument position.
    //     #[test]
    //     fn test_zero_arg_command() {
    //         // "random_placement " — cursor at the space (col 16)
    //         assert!(!is_in_argument_position(
    //             "random_placement ",
    //             Position::new(0, 16)
    //         ));
    //     }

    //     /// Cursor immediately after a command, before its argument.
    //     #[test]
    //     fn test_before_first_arg() {
    //         // "create_object " — create_object ends at col 13, cursor at the space
    //         assert!(is_in_argument_position(
    //             "create_object ",
    //             Position::new(0, 13)
    //         ));
    //     }

    //     /// Cursor within the first argument.
    //     #[test]
    //     fn test_within_first_arg() {
    //         // "create_object SCOUT" — cursor mid-SCOUT at col 16
    //         assert!(is_in_argument_position(
    //             "create_object SCOUT",
    //             Position::new(0, 16)
    //         ));
    //     }

    //     /// Cursor just after the first argument is no longer an argument position.
    //     #[test]
    //     fn test_after_first_arg() {
    //         // "create_object SCOUT " — SCOUT ends at col 19, cursor at the space
    //         assert!(!is_in_argument_position(
    //             "create_object SCOUT ",
    //             Position::new(0, 19)
    //         ));
    //     }

    //     /// Cursor after a section header is not an argument position.
    //     #[test]
    //     fn test_after_section_header() {
    //         assert!(!is_in_argument_position(
    //             "<OBJECTS_GENERATION>\n",
    //             Position::new(1, 0)
    //         ));
    //     }

    //     /// Cursor immediately inside an open brace is not an argument position.
    //     #[test]
    //     fn test_after_open_brace() {
    //         // Line 1 is "\t", the { on line 0 is a boundary.
    //         assert!(!is_in_argument_position(
    //             "create_object SCOUT {\n\t",
    //             Position::new(1, 1)
    //         ));
    //     }

    //     /// Cursor after a close brace is not an argument position.
    //     #[test]
    //     fn test_after_close_brace() {
    //         assert!(!is_in_argument_position(
    //             "create_object SCOUT {\n}\n",
    //             Position::new(2, 0)
    //         ));
    //     }

    //     /// Cursor after an attribute name inside a block, before its argument.
    //     #[test]
    //     fn test_attribute_before_arg() {
    //         // Line 1: "\tnumber_of_objects " — token at col 1..18, cursor at space col 18
    //         let text = "create_object SCOUT {\n\tnumber_of_objects ";
    //         assert!(is_in_argument_position(text, Position::new(1, 18)));
    //     }

    //     /// Cursor on a new line after an attribute and its argument is not an argument position.
    //     #[test]
    //     fn test_attribute_after_arg() {
    //         let text = "create_object SCOUT {\n\tnumber_of_objects 5\n\t";
    //         assert!(!is_in_argument_position(text, Position::new(2, 1)));
    //     }

    //     /// A math expression counts as a single argument.
    //     #[test]
    //     fn test_math_expression_counts_as_one_arg() {
    //         // "number_of_objects (A + B) " — cursor at the trailing space (col 25)
    //         assert!(!is_in_argument_position(
    //             "number_of_objects (A + B) ",
    //             Position::new(0, 25)
    //         ));
    //     }

    //     /// Cursor before a math expression argument.
    //     #[test]
    //     fn test_before_math_expression_arg() {
    //         // "number_of_objects " — token ends at col 17, cursor at the space
    //         assert!(is_in_argument_position(
    //             "number_of_objects ",
    //             Position::new(0, 17)
    //         ));
    //     }

    //     /// The optional argument slot of circle_radius is an argument position.
    //     #[test]
    //     fn test_optional_arg_slot() {
    //         // "circle_radius 20 " — "20" ends at col 16, cursor at the space
    //         assert!(is_in_argument_position(
    //             "circle_radius 20 ",
    //             Position::new(0, 16)
    //         ));
    //     }

    //     /// After all arguments including optional, is not an argument position.
    //     #[test]
    //     fn test_after_all_args() {
    //         // "circle_radius 20 5 " — "5" ends at col 18, cursor at the space
    //         assert!(!is_in_argument_position(
    //             "circle_radius 20 5 ",
    //             Position::new(0, 18)
    //         ));
    //     }

    //     /// A two-argument command: cursor after the first argument is still an argument position.
    //     #[test]
    //     fn test_two_required_args_after_first() {
    //         // "land_position 50 " — "50" ends at col 16, cursor at the space
    //         assert!(is_in_argument_position(
    //             "land_position 50 ",
    //             Position::new(0, 16)
    //         ));
    //     }

    //     /// A two-argument command: cursor after both arguments is not an argument position.
    //     #[test]
    //     fn test_two_required_args_after_both() {
    //         // "land_position 50 50 " — second "50" ends at col 19, cursor at the space
    //         assert!(!is_in_argument_position(
    //             "land_position 50 50 ",
    //             Position::new(0, 19)
    //         ));
    //     }
    // }

    // mod tokens_before {
    //     use super::*;

    //     /// No tokens before the start of the document.
    //     #[test]
    //     fn test_empty_document() {
    //         assert!(tokens_before("", Position::new(0, 0)).is_empty());
    //     }

    //     /// The current token being typed is not included.
    //     #[test]
    //     fn test_current_token_excluded() {
    //         // Cursor is mid-token "SCOUT" at col 14, within "create_object SCOUT"
    //         let text = "create_object SCOUT";
    //         let ts = tokens_before(text, Position::new(0, 16));
    //         assert_eq!(
    //             ts.iter().map(|t| t.text).collect::<Vec<_>>(),
    //             vec!["create_object"]
    //         );
    //     }

    //     /// A partial token being typed is not included.
    //     #[test]
    //     fn test_partial_token_excluded() {
    //         // Cursor at col 14, inside "S" in "create_object S"
    //         let text = "create_object S";
    //         let ts = tokens_before(text, Position::new(0, 14));
    //         assert_eq!(
    //             ts.iter().map(|t| t.text).collect::<Vec<_>>(),
    //             vec!["create_object"]
    //         );
    //     }

    //     /// Cursor at end of line after a partial token is in argument position.
    //     #[test]
    //     fn test_partial_token_at_end_of_line() {
    //         assert!(is_in_argument_position(
    //             "create_object S",
    //             Position::new(0, 15)
    //         ));
    //     }

    //     /// A completed token whose end equals the cursor position is included
    //     /// when the cursor is on whitespace.
    //     #[test]
    //     fn test_completed_token_at_cursor_included() {
    //         // Cursor at col 19, the space after "SCOUT" in "create_object SCOUT "
    //         let text = "create_object SCOUT ";
    //         let ts = tokens_before(text, Position::new(0, 19));
    //         assert_eq!(
    //             ts.iter().map(|t| t.text).collect::<Vec<_>>(),
    //             vec!["create_object", "SCOUT"]
    //         );
    //     }

    //     /// Tokens on earlier lines are all included.
    //     #[test]
    //     fn test_tokens_on_earlier_lines_included() {
    //         let text = "create_object SCOUT {\n\tnumber_of_objects ";
    //         let ts = tokens_before(text, Position::new(1, 18));
    //         assert_eq!(
    //             ts.iter().map(|t| t.text).collect::<Vec<_>>(),
    //             vec!["create_object", "SCOUT", "{", "number_of_objects"]
    //         );
    //     }

    //     /// Tokens on later lines are not included.
    //     #[test]
    //     fn test_tokens_on_later_lines_excluded() {
    //         let text = "create_object SCOUT {\n\tnumber_of_objects 5\n}";
    //         let ts = tokens_before(text, Position::new(0, 0));
    //         assert!(ts.is_empty());
    //     }
    // }
}
