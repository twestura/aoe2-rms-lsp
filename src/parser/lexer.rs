//! Turns a raw string into a sequence of [`Lexeme`]s.

/// Lexes a raw string into a sequence of [`Lexeme`]s.
pub fn lex(text: &str) -> Vec<Lexeme> {
    Lexer::new(text).lex()
}

/// Index information for raw text parsed from a RMS file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lexeme {
    /// The start byte index of the lexeme in the RMS file (inclusive).
    pub text_start: usize,
    /// The end byte index of the lexeme in the RMS file (exclusive).
    pub text_end: usize,
    /// The line number of the lexeme in the RMS file.
    pub lineno: usize,
    /// The start byte index of the lexeme in the current line (inclusive).
    pub line_start: usize,
    /// The end byte index of the lexeme in the current line (exclusive).
    pub line_end: usize,
}

impl Lexeme {
    /// Returns the text of the lexeme within the given source string.
    /// Requires that the `text_start` and `text_end` indices are valid for the source string.
    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        debug_assert!(self.text_start <= self.text_end);
        debug_assert!(self.text_end <= source.len());
        &source[self.text_start..self.text_end]
    }
}

/// Internal lexer state for [`lex`].
#[derive(Debug, Clone)]
struct Lexer<'a> {
    /// The raw text to lex.
    text: &'a str,
    /// The underlying bytes of the text being lexed.
    bytes: &'a [u8],
}

/// Represents a position of the text being lexed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct LexState {
    /// The current byte offset into the text being lexed.
    /// - `self.text[..offset]` has been lexed.
    /// - `self.text[offset]` is the next character to lex.
    /// - `self.text[offset..]` remains to be lexed.
    /// Requires `0 <= offset <= text.len()`.
    offset: usize,

    /// The current line number being lexed, 0-indexed.
    lineno: usize,

    /// The overall byte index of the start of the current line.
    line_offset: usize,
}

impl LexState {
    /// Returns the index of the current column in the current line.
    /// 0-indexed.
    fn column(&self) -> usize {
        self.offset - self.line_offset
    }
}

impl<'a> Lexer<'a> {
    /// Constructs a new lexer for the given text.
    fn new(text: &'a str) -> Self {
        Self {
            text,
            bytes: text.as_bytes(),
        }
    }

    /// Lexes the raw text into a sequence of [`Lexeme`]s.
    fn lex(&self) -> Vec<Lexeme> {
        let mut state = LexState::default();
        let mut lexemes = Vec::new();
        while state.offset < self.text.len() {
            let b = self.byte_at(state.offset);
            if b.is_ascii_whitespace() {
                state = self.consume_ascii_whitespace(state)
            } else {
                let (lexeme, new_state) = self.lex_nonwhitespace(state);
                lexemes.push(lexeme);
                state = new_state;
            }
        }
        lexemes
    }

    /// Returns the byte at the given offset in the text.
    /// Requires `offset` to be within the bounds of the text.
    fn byte_at(&self, offset: usize) -> u8 {
        debug_assert!(offset < self.text.len());
        self.bytes[offset]
    }

    /// Returns a `LexState` representing the position resulting from beginning
    /// at `init_state` and consuming any ASCII whitespace characters that
    /// follow.
    fn consume_ascii_whitespace(&self, init_state: LexState) -> LexState {
        let mut state = init_state;
        while state.offset < self.text.len() {
            let c = self.byte_at(state.offset);
            if !c.is_ascii_whitespace() {
                break;
            }
            state.offset += 1;
            if c == b'\n' {
                state.lineno += 1;
                state.line_offset = state.offset; // The line starts at the char after the \n.
            }
        }
        state
    }

    /// Lexes a single non-whitespace lexeme starting at the given `start`
    /// position.
    /// Requires:
    /// - The starting offset is a valid byte index.
    /// - The starting character is not whitespace.
    fn lex_nonwhitespace(&self, init_state: LexState) -> (Lexeme, LexState) {
        debug_assert!(init_state.offset < self.text.len());
        debug_assert!(!self.byte_at(init_state.offset).is_ascii_whitespace());
        let mut state = init_state;
        while state.offset < self.text.len() {
            let c = self.byte_at(state.offset);
            if c.is_ascii_whitespace() {
                break;
            }
            state.offset += 1;
        }
        let lexeme = Lexeme {
            text_start: init_state.offset,
            text_end: state.offset,
            lineno: init_state.lineno,
            line_start: init_state.column(),
            line_end: state.column(),
        };
        (lexeme, state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod lex {
        use super::*;

        /// An empty string produces no lexemes.
        #[test]
        fn test_empty() {
            assert!(lex("").is_empty());
        }

        /// A string of only whitespace produces no lexemes.
        #[test]
        fn test_whitespace_only() {
            assert!(lex("   \t\n  ").is_empty());
        }

        /// A single nonwhitespace sequence produces a single lexeme.
        #[test]
        fn test_single_token() {
            let text = "create_land";
            let lexemes = lex(text);
            assert_eq!(lexemes.len(), 1);
            assert_eq!(lexemes[0].text(text), "create_land");
        }

        /// Multiple nonwhitespace sequences on a single line produce multiple
        /// lexemes.
        #[test]
        fn test_multiple_tokens() {
            let text = "terrain_type GRASS";
            let lexemes = lex(text);
            assert_eq!(lexemes.len(), 2);
            assert_eq!(lexemes[0].text(text), "terrain_type");
            assert_eq!(lexemes[1].text(text), "GRASS");
        }

        /// Nonwhitespace sequences on separate lines are all produced.
        #[test]
        fn test_tokens_on_separate_lines() {
            let text = "create_land {\nterrain_type GRASS\n}";
            let lexemes = lex(text);
            let texts: Vec<&str> = lexemes.iter().map(|lexeme| lexeme.text(text)).collect();
            assert_eq!(
                texts,
                vec!["create_land", "{", "terrain_type", "GRASS", "}"]
            );
        }

        /// A parenthesis is treated as a regular nonwhitespace character.
        #[test]
        fn test_parenthesis_is_lexeme() {
            let text = "(A + B)";
            let lexemes = lex(text);
            let texts: Vec<&str> = lexemes.iter().map(|lexeme| lexeme.text(text)).collect();
            assert_eq!(texts, vec!["(A", "+", "B)"]);
        }

        /// `rnd(min,max)` is a single lexeme.
        #[test]
        fn test_rnd_is_single_lexeme() {
            let text = "rnd(1,6)";
            let lexemes = lex(text);
            assert_eq!(lexemes.len(), 1);
            assert_eq!(lexemes[0].text(text), "rnd(1,6)");
        }

        /// Comment delimiters are treated as regular lexemes.
        #[test]
        fn test_comment_delimiters_are_lexemes() {
            let text = "/* comment */";
            let lexemes = lex(text);
            let texts: Vec<&str> = lexemes.iter().map(|lexeme| lexeme.text(text)).collect();
            assert_eq!(texts, vec!["/*", "comment", "*/"]);
        }

        /// Lexeme absolute positions are correct for a token on the first line.
        #[test]
        fn test_abs_position_first_line() {
            let lexemes = lex("create_land");
            assert_eq!(lexemes[0].text_start, 0);
            assert_eq!(lexemes[0].text_end, 11);
        }

        /// Lexeme absolute positions are correct for a token on the second line.
        #[test]
        fn test_abs_position_second_line() {
            let lexemes = lex("abc\nterrain_type");
            assert_eq!(lexemes[1].text_start, 4);
            assert_eq!(lexemes[1].text_end, 16);
        }

        /// Lexeme line and column positions are correct for a token on the first line.
        #[test]
        fn test_line_position_first_line() {
            let lexemes = lex("create_land");
            assert_eq!(lexemes[0].lineno, 0);
            assert_eq!(lexemes[0].line_start, 0);
            assert_eq!(lexemes[0].line_end, 11);
        }

        /// Lexeme line and column positions are correct for a token on the second line.
        #[test]
        fn test_line_position_second_line() {
            let lexemes = lex("abc\nterrain_type");
            assert_eq!(lexemes[1].lineno, 1);
            assert_eq!(lexemes[1].line_start, 0);
            assert_eq!(lexemes[1].line_end, 12);
        }

        /// Lexeme positions are correct for an indented token.
        #[test]
        fn test_position_indented() {
            let lexemes = lex("\tterrain_type");
            assert_eq!(lexemes[0].lineno, 0);
            assert_eq!(lexemes[0].line_start, 1);
            assert_eq!(lexemes[0].line_end, 13);
        }

        /// Lexeme positions are correct for the second token on a line.
        #[test]
        fn test_position_second_token_on_line() {
            let lexemes = lex("terrain_type GRASS");
            assert_eq!(lexemes[1].lineno, 0);
            assert_eq!(lexemes[1].line_start, 13);
            assert_eq!(lexemes[1].line_end, 18);
        }

        /// A section header is a single lexeme.
        #[test]
        fn test_section_header() {
            let text = "<PLAYER_SETUP>";
            let lexemes = lex(text);
            assert_eq!(lexemes.len(), 1);
            assert_eq!(lexemes[0].text(text), "<PLAYER_SETUP>");
        }

        /// Tabs are treated as whitespace separators.
        #[test]
        fn test_tab_separator() {
            let text = "a\tb";
            let lexemes = lex(text);
            let texts: Vec<&str> = lexemes.iter().map(|lexeme| lexeme.text(text)).collect();
            assert_eq!(texts, vec!["a", "b"]);
        }

        /// A multi-byte Unicode character is treated as a single nonwhitespace
        /// sequence and byte indices are correct.
        #[test]
        fn test_unicode_character() {
            let text = "é abc";
            let lexemes = lex(text);
            assert_eq!(lexemes[0].text(text), "é");
            assert_eq!(lexemes[0].text_start, 0);
            assert_eq!(lexemes[0].text_end, 2);
            assert_eq!(lexemes[1].text(text), "abc");
            assert_eq!(lexemes[1].text_start, 3);
        }

        /// Multiple consecutive newlines increment the line number correctly.
        #[test]
        fn test_multiple_consecutive_newlines() {
            let text = "abc\n\n\ndef";
            let lexemes = lex(text);
            assert_eq!(lexemes[1].lineno, 3);
            assert_eq!(lexemes[1].line_start, 0);
        }

        /// A token at end of file with no trailing newline is correctly lexed.
        #[test]
        fn test_token_at_eof_no_newline() {
            let text = "abc";
            let lexemes = lex(text);
            assert_eq!(lexemes.len(), 1);
            assert_eq!(lexemes[0].text_end, 3);
        }

        /// `#const` is treated as a single lexeme.
        #[test]
        fn test_hash_const_is_single_lexeme() {
            let text = "#const GOLD 66";
            let lexemes = lex(text);
            assert_eq!(lexemes[0].text(text), "#const");
        }

        /// `#define` is treated as a single lexeme.
        #[test]
        fn test_hash_define_is_single_lexeme() {
            let text = "#define MY_LABEL";
            let lexemes = lex(text);
            assert_eq!(lexemes[0].text(text), "#define");
        }

        /// A math expression with a leading `(` attached to a token is a single
        /// lexeme.
        #[test]
        fn test_leading_paren_attached_to_token() {
            let text = "(A";
            let lexemes = lex(text);
            assert_eq!(lexemes.len(), 1);
            assert_eq!(lexemes[0].text(text), "(A");
        }

        /// A math expression with a trailing `)` attached to a token is a single
        /// lexeme.
        #[test]
        fn test_trailing_paren_attached_to_token() {
            let text = "B)";
            let lexemes = lex(text);
            assert_eq!(lexemes.len(), 1);
            assert_eq!(lexemes[0].text(text), "B)");
        }

        /// A whitespace-separated math expression produces multiple lexemes.
        #[test]
        fn test_whitespace_separated_math_expression() {
            let text = "( A + B )";
            let lexemes = lex(text);
            let texts: Vec<&str> = lexemes.iter().map(|lexeme| lexeme.text(text)).collect();
            assert_eq!(texts, vec!["(", "A", "+", "B", ")"]);
        }

        /// A math expression used as an argument produces the correct lexemes.
        #[test]
        fn test_math_expression_as_argument() {
            let text = "number_of_objects (A + B)";
            let lexemes = lex(text);
            let texts: Vec<&str> = lexemes.iter().map(|lexeme| lexeme.text(text)).collect();
            assert_eq!(texts, vec!["number_of_objects", "(A", "+", "B)"]);
        }

        /// Positions within a math expression are correct.
        #[test]
        fn test_math_expression_positions() {
            let text = "( A )";
            let lexemes = lex(text);
            assert_eq!(lexemes[1].text(text), "A");
            assert_eq!(lexemes[1].line_start, 2);
            assert_eq!(lexemes[1].line_end, 3);
        }
    }

    mod lexeme_text {
        use super::*;

        /// Returns the correct text for a lexeme on the first line.
        #[test]
        fn test_first_line() {
            let text = "create_land";
            let lexemes = lex(text);
            assert_eq!(lexemes[0].text(text), "create_land");
        }

        /// Returns the correct text for a lexeme on the second line.
        #[test]
        fn test_second_line() {
            let text = "abc\nterrain_type";
            let lexemes = lex(text);
            assert_eq!(lexemes[1].text(text), "terrain_type");
        }

        /// Returns the correct text for an indented lexeme.
        #[test]
        fn test_indented() {
            let text = "\tterrain_type";
            let lexemes = lex(text);
            assert_eq!(lexemes[0].text(text), "terrain_type");
        }
    }
}
