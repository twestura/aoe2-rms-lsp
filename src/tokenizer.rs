//! Lexer for RMS (Random Map Script) files.
//!
//! Tokenizes RMS source text by splitting on ASCII whitespace, with special
//! handling for comment delimiters and math expression parentheses.
//!
//! Comment delimiters `/*` and `*/` are only recognized as such when they
//! appear as a complete whitespace-separated token. A `(` at the start of a
//! whitespace-separated chunk is split off as its own token, as is a `)` at
//! the end of one. Tokens inside comments are skipped.

/// A single token extracted from an RMS document.
///
/// Line and column positions use zero-based indices. `start` and `end` are
/// byte offsets within the line, consistent with LSP UTF-8 position encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    /// The text of the token, borrowed from the source document.
    pub text: &'a str,
    /// The zero-based line number of the token.
    pub line: u32,
    /// The byte offset of the start of the token within its line, inclusive.
    pub start: u32,
    /// The byte offset of the end of the token within its line, exclusive.
    pub end: u32,
}

/// An iterator over the tokens in an RMS document.
///
/// Construct with [`Lexer::new`] and iterate to consume tokens. Comment
/// delimiters and comment contents are consumed internally and not yielded.
pub struct Lexer<'a> {
    /// The full source text.
    text: &'a str,
    /// The current byte offset into `text`.
    offset: usize,
    /// The zero-based current line number.
    line: u32,
    /// The byte offset of the first character of the current line.
    line_start: usize,
    /// The current comment nesting depth.
    comment_depth: u32,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer over the given source text.
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            offset: 0,
            line: 0,
            line_start: 0,
            comment_depth: 0,
        }
    }

    /// Advances past ASCII whitespace, updating line and column tracking.
    fn skip_whitespace(&mut self) {
        while self.offset < self.text.len() {
            // Safe to unwrap, as offset < len.
            let c = self.text[self.offset..].chars().next().unwrap();
            if !c.is_ascii_whitespace() {
                break;
            }
            if c == '\n' {
                self.line += 1;
                self.line_start = self.offset + 1;
            }
            self.offset += c.len_utf8();
        }
    }

    /// Returns the byte column of the given absolute byte offset within the
    /// current line.
    fn column(&self, offset: usize) -> u32 {
        (offset - self.line_start) as u32
    }

    /// Returns the next non-comment whitespace-separated [`Chunk`],
    /// advancing past any comment delimiters and comment contents.
    /// Returns `None` at end of text.
    fn next_chunk(&mut self) -> Option<Chunk> {
        loop {
            self.skip_whitespace();
            debug_assert!(self.offset <= self.text.len());
            if self.offset == self.text.len() {
                return None;
            }
            let start = self.offset;
            self.offset = self.text[start..]
                .find(|c: char| c.is_ascii_whitespace())
                .map(|i| start + i)
                .unwrap_or(self.text.len());
            match &self.text[start..self.offset] {
                "/*" => self.comment_depth += 1,
                "*/" => self.comment_depth = self.comment_depth.saturating_sub(1),
                _ if self.comment_depth > 0 => {}
                _ => {
                    return Some(Chunk {
                        start,
                        end: self.offset,
                        line: self.line,
                        column: self.column(start),
                    });
                }
            }
        }
    }
}

/// A raw whitespace-separated chunk of text, located within the source document.
/// Used internally by [`Lexer`] before paren-splitting and token construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Chunk {
    /// The byte offset of the start of the chunk within the full source text.
    start: usize,
    /// The byte offset of the end of the chunk within the full source text, exclusive.
    end: usize,
    /// The zero-based line number of the chunk.
    line: u32,
    /// The byte offset of the start of the chunk within its line.
    column: u32,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        let chunk = self.next_chunk()?;
        let text = &self.text[chunk.start..chunk.end];

        // Math expression start, return only the starting parenthesis.
        if text.starts_with('(') && text.len() > 1 {
            self.offset = chunk.start + 1;
            return Some(Token {
                text: &self.text[chunk.start..chunk.start + 1],
                line: chunk.line,
                start: chunk.column,
                end: chunk.column + 1,
            });
        }

        // Math expression end, avoiding cases such as `rnd(MIN,MAX)`.
        // Return the text up to but not including the closing parenthesis.
        if text.ends_with(')') && text.len() > 1 && !text.contains('(') {
            let split = chunk.end - 1;
            self.offset = split;
            return Some(Token {
                text: &self.text[chunk.start..split],
                line: chunk.line,
                start: chunk.column,
                end: self.column(split),
            });
        }

        // Generic case, return the full chunk text.
        Some(Token {
            text,
            line: chunk.line,
            start: chunk.column,
            end: self.column(chunk.end),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Collects all tokens from the given text into a vector, including
    /// position information. Used in tests that verify token positions.
    fn tokens<'a>(text: &'a str) -> Vec<Token<'a>> {
        Lexer::new(text).collect()
    }

    /// Collects the text of all tokens from the given text into a vector,
    /// discarding position information. Used in tests that only verify token
    /// text.
    fn token_texts(text: &str) -> Vec<&str> {
        Lexer::new(text).map(|t| t.text).collect()
    }

    mod basic {
        use super::*;

        /// An empty document yields no tokens.
        #[test]
        fn test_empty() {
            assert!(tokens("").is_empty());
        }

        /// A document with only whitespace yields no tokens.
        #[test]
        fn test_whitespace_only() {
            assert!(tokens("   \t\n  ").is_empty());
        }

        /// A single token is yielded with the correct text and position.
        #[test]
        fn test_single_token() {
            assert_eq!(
                tokens("<PLAYER_SETUP>"),
                vec![Token {
                    text: "<PLAYER_SETUP>",
                    line: 0,
                    start: 0,
                    end: 14
                }]
            );
        }

        /// Multiple tokens on a single line are yielded in order.
        #[test]
        fn test_multiple_tokens_single_line() {
            assert_eq!(
                token_texts("effect_amount MOD_RESOURCE AMOUNT_STARTING_FOOD ATTR_ADD 10000"),
                vec![
                    "effect_amount",
                    "MOD_RESOURCE",
                    "AMOUNT_STARTING_FOOD",
                    "ATTR_ADD",
                    "10000"
                ]
            );
        }

        /// Tokens on separate lines have correct line numbers.
        #[test]
        fn test_tokens_on_separate_lines() {
            let ts = tokens("<PLAYER_SETUP>\n<LAND_GENERATION>");
            assert_eq!(
                ts[0],
                Token {
                    text: "<PLAYER_SETUP>",
                    line: 0,
                    start: 0,
                    end: 14
                }
            );
            assert_eq!(
                ts[1],
                Token {
                    text: "<LAND_GENERATION>",
                    line: 1,
                    start: 0,
                    end: 17
                }
            );
        }

        /// A token indented with whitespace has the correct column.
        #[test]
        fn test_indented_token() {
            let ts = tokens("    terrain_type");
            assert_eq!(ts[0].start, 4);
            assert_eq!(ts[0].end, 16);
        }

        /// Tokens on the second line have correct column positions.
        #[test]
        fn test_second_line_columns() {
            let ts = tokens("abc\n    def");
            assert_eq!(
                ts[1],
                Token {
                    text: "def",
                    line: 1,
                    start: 4,
                    end: 7
                }
            );
        }

        /// Multiple tokens on the second line have correct positions.
        #[test]
        fn test_second_line_multiple_tokens() {
            let ts = tokens("foo\nbar baz");
            assert_eq!(ts[1].text, "bar");
            assert_eq!(ts[1].line, 1);
            assert_eq!(ts[1].start, 0);
            assert_eq!(ts[2].text, "baz");
            assert_eq!(ts[2].line, 1);
            assert_eq!(ts[2].start, 4);
        }

        /// Tabs are treated as whitespace separators.
        #[test]
        fn test_tab_separator() {
            assert_eq!(token_texts("a\tb"), vec!["a", "b"]);
        }
    }

    mod comments {
        use super::*;

        /// Tokens inside a comment are skipped.
        #[test]
        fn test_comment_skipped() {
            assert_eq!(token_texts("/* comment */"), Vec::<&str>::new());
        }

        /// Tokens before and after a comment are yielded.
        #[test]
        fn test_tokens_around_comment() {
            assert_eq!(
                token_texts("before /* comment */ after"),
                vec!["before", "after"]
            );
        }

        /// A multi-line comment is fully skipped.
        #[test]
        fn test_multiline_comment() {
            assert_eq!(
                token_texts("before /* comment\nstill comment */ after"),
                vec!["before", "after"]
            );
        }

        /// Nested comments are handled correctly.
        #[test]
        fn test_nested_comment() {
            assert_eq!(
                token_texts("/* outer /* inner */ still outer */ after"),
                vec!["after"]
            );
        }

        /// An unterminated comment skips all remaining tokens.
        #[test]
        fn test_unterminated_comment() {
            assert!(token_texts("before /* unterminated token").is_empty() == false);
            assert_eq!(token_texts("before /* unterminated token"), vec!["before"]);
        }

        /// `/*` not surrounded by whitespace is not a comment delimiter.
        #[test]
        fn test_comment_open_no_whitespace() {
            assert_eq!(token_texts("/*comment*/"), vec!["/*comment*/"]);
        }

        /// `*/` not preceded by whitespace is not a comment closer.
        #[test]
        fn test_comment_close_no_preceding_whitespace() {
            assert_eq!(token_texts("/* x*/ after"), Vec::<&str>::new());
        }
    }

    mod parentheses {
        use super::*;

        /// A `(` at the start of a chunk is split as its own token.
        #[test]
        fn test_leading_paren_split() {
            assert_eq!(token_texts("(A + B)"), vec!["(", "A", "+", "B", ")"]);
        }

        /// A `)` at the end of a chunk is split as its own token.
        #[test]
        fn test_trailing_paren_split() {
            assert_eq!(token_texts("A + B)"), vec!["A", "+", "B", ")"]);
        }

        /// A standalone `(` is returned as a normal token.
        #[test]
        fn test_standalone_open_paren() {
            assert_eq!(token_texts("("), vec!["("]);
        }

        /// A standalone `)` is returned as a normal token.
        #[test]
        fn test_standalone_close_paren() {
            assert_eq!(token_texts(")"), vec![")"]);
        }

        /// A full math expression is split into individual tokens.
        #[test]
        fn test_math_expression() {
            assert_eq!(
                token_texts("number_of_objects (GOLD + 2)"),
                vec!["number_of_objects", "(", "GOLD", "+", "2", ")"]
            );
        }

        /// `rnd(min,max)` does not have its trailing `)` split off.
        #[test]
        fn test_rnd_not_split() {
            assert_eq!(token_texts("rnd(1,6)"), vec!["rnd(1,6)"]);
        }

        /// A `(` at the start of a chunk with correct line and column.
        #[test]
        fn test_leading_paren_position() {
            let ts = tokens("(A");
            assert_eq!(
                ts[0],
                Token {
                    text: "(",
                    line: 0,
                    start: 0,
                    end: 1
                }
            );
            assert_eq!(
                ts[1],
                Token {
                    text: "A",
                    line: 0,
                    start: 1,
                    end: 2
                }
            );
        }

        /// A `)` split from the end of a chunk has correct positions.
        #[test]
        fn test_trailing_paren_position() {
            let ts = tokens("B)");
            assert_eq!(
                ts[0],
                Token {
                    text: "B",
                    line: 0,
                    start: 0,
                    end: 1
                }
            );
            assert_eq!(
                ts[1],
                Token {
                    text: ")",
                    line: 0,
                    start: 1,
                    end: 2
                }
            );
        }
    }

    mod positions {
        use super::*;

        /// Token positions are correct after a multi-byte Unicode character
        /// on the same line.
        #[test]
        fn test_unicode_preceding_token() {
            let ts = tokens("é abc");
            assert_eq!(ts[0].text, "é");
            assert_eq!(ts[0].start, 0);
            assert_eq!(ts[0].end, 2); // é is 2 bytes
            assert_eq!(ts[1].text, "abc");
            assert_eq!(ts[1].start, 3);
        }

        /// Column resets to 0 at the start of each new line.
        #[test]
        fn test_column_resets_on_newline() {
            let ts = tokens("abc\ndef");
            assert_eq!(ts[1].start, 0);
        }

        /// Token end is the exclusive byte offset past the last character.
        #[test]
        fn test_token_end_exclusive() {
            let ts = tokens("abc");
            assert_eq!(ts[0].end, 3);
        }
    }
}
