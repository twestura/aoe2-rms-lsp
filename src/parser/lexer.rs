//! Turns a raw string into a sequence of [`Lexeme`]s.

use crate::parser::lexeme::Lexeme;

/// Lexes a raw string into a sequence of [`Lexeme`]s.
pub fn lex<'a>(text: &'a str) -> Vec<Lexeme<'a>> {
    Lexer::new(text).lex()
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
    line_start: usize,
}

impl LexState {
    /// Returns the index of the current column in the current line.
    /// 0-indexed.
    fn column(&self) -> usize {
        self.offset - self.line_start
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
    fn lex(&self) -> Vec<Lexeme<'a>> {
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
                state.line_start = state.offset;
            }
        }
        state
    }

    /// Lexes a single non-whitespace lexeme starting at the given `start`
    /// position.
    /// Requires:
    /// - The starting offset is a valid byte index.
    /// - The starting character is not whitespace.
    fn lex_nonwhitespace(&self, init_state: LexState) -> (Lexeme<'a>, LexState) {
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
        let text = &self.text[init_state.offset..state.offset];
        let lexeme = Lexeme::new(text, init_state.lineno, init_state.column(), state.column());
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
            let lexemes = lex("create_land");
            assert_eq!(lexemes.len(), 1);
            assert_eq!(lexemes[0].to_string(), "create_land");
        }

        /// Multiple nonwhitespace sequences on a single line produce multiple
        /// lexemes.
        #[test]
        fn test_multiple_tokens() {
            let lexemes = lex("terrain_type GRASS");
            assert_eq!(lexemes.len(), 2);
            assert_eq!(lexemes[0].to_string(), "terrain_type");
            assert_eq!(lexemes[1].to_string(), "GRASS");
        }

        /// Nonwhitespace sequences on separate lines are all produced.
        #[test]
        fn test_tokens_on_separate_lines() {
            let lexemes = lex("create_land {\nterrain_type GRASS\n}");
            assert_eq!(
                lexemes.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
                vec!["create_land", "{", "terrain_type", "GRASS", "}"]
            );
        }

        /// A parenthesis is treated as a regular nonwhitespace character.
        #[test]
        fn test_parenthesis_is_lexeme() {
            let lexemes = lex("(A + B)");
            assert_eq!(
                lexemes.iter().map(|l| l.to_string()).collect::<Vec<_>>(),
                vec!["(A", "+", "B)"]
            );
        }

        /// `rnd(min,max)` is a single lexeme.
        #[test]
        fn test_rnd_is_single_lexeme() {
            let lexemes = lex("rnd(1,6)");
            assert_eq!(lexemes.len(), 1);
            assert_eq!(lexemes[0].to_string(), "rnd(1,6)");
        }

        /// Comment delimiters are treated as regular lexemes.
        #[test]
        fn test_comment_delimiters_are_lexemes() {
            let lexemes = lex("/* comment */");
            assert_eq!(
                lexemes.iter().map(|l| l.to_string()).collect::<Vec<_>>(),
                vec!["/*", "comment", "*/"]
            );
        }

        /// Lexeme positions are correct for a token on the first line.
        #[test]
        fn test_position_first_line() {
            let lexemes = lex("create_land");
            assert_eq!(lexemes[0].lineno(), 0);
            assert_eq!(lexemes[0].start(), 0);
            assert_eq!(lexemes[0].end(), 11);
        }

        /// Lexeme positions are correct for a token on the second line.
        #[test]
        fn test_position_second_line() {
            let lexemes = lex("abc\nterrain_type");
            assert_eq!(lexemes[1].lineno(), 1);
            assert_eq!(lexemes[1].start(), 0);
            assert_eq!(lexemes[1].end(), 12);
        }

        /// Lexeme positions are correct for an indented token.
        #[test]
        fn test_position_indented() {
            let lexemes = lex("\tterrain_type");
            assert_eq!(lexemes[0].lineno(), 0);
            assert_eq!(lexemes[0].start(), 1);
            assert_eq!(lexemes[0].end(), 13);
        }

        /// Lexeme positions are correct for the second token on a line.
        #[test]
        fn test_position_second_token_on_line() {
            let lexemes = lex("terrain_type GRASS");
            assert_eq!(lexemes[1].lineno(), 0);
            assert_eq!(lexemes[1].start(), 13);
            assert_eq!(lexemes[1].end(), 18);
        }

        /// A section header is a single lexeme.
        #[test]
        fn test_section_header() {
            let lexemes = lex("<PLAYER_SETUP>");
            assert_eq!(lexemes.len(), 1);
            assert_eq!(lexemes[0].to_string(), "<PLAYER_SETUP>");
        }

        /// Tabs are treated as whitespace separators.
        #[test]
        fn test_tab_separator() {
            let lexemes = lex("a\tb");
            assert_eq!(
                lexemes.iter().map(|l| l.to_string()).collect::<Vec<_>>(),
                vec!["a", "b"]
            );
        }
    }
}
