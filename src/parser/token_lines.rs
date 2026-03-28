//! Line-indexed token storage for an RMS document.

use std::ops::Index;

use crate::parser::token::Token;

/// Tokens in an RMS document, grouped by line.
/// - `lines[i]` contains the tokens for line `i` (0-indexed).
/// - Each line's tokens are sorted by byte offset.
/// - Tokens do not include comments or whitespace, and do not overlap.
#[derive(Debug)]
pub(super) struct TokenLines(Vec<Vec<Token>>);

impl TokenLines {
    /// Creates a `TokenLines` with `num_lines` empty lines.
    pub(super) fn new(num_lines: usize) -> Self {
        Self(vec![vec![]; num_lines])
    }

    /// Returns the number of lines.
    pub(super) fn len(&self) -> usize {
        self.0.len()
    }

    /// Appends `token` to the given line.
    pub(super) fn push(&mut self, lineno: usize, token: Token) {
        self.0[lineno].push(token);
    }

    /// Returns the token on `lineno` that covers `offset`, if one exists.
    /// `offset` is an absolute byte offset into the document.
    pub(super) fn token_at(&self, lineno: usize, offset: usize) -> Option<Token> {
        use std::cmp::Ordering::*;
        let line = self.0.get(lineno)?;
        line.binary_search_by(|t| {
            if offset < t.start() {
                Greater
            } else if offset >= t.end() {
                Less
            } else {
                Equal
            }
        })
        .ok()
        .map(|i| line[i])
    }
}

impl Index<usize> for TokenLines {
    type Output = [Token];
    fn index(&self, lineno: usize) -> &[Token] {
        &self.0[lineno]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::token::Token;

    /// Builds a single-line `TokenLines` containing the given tokens.
    fn single_line(tokens: &[Token]) -> TokenLines {
        let mut lines = TokenLines::new(1);
        for &t in tokens {
            lines.push(0, t);
        }
        lines
    }

    mod token_at {
        use super::*;

        /// An offset at the first byte of a token returns that token.
        #[test]
        fn offset_at_token_start() {
            let t = Token::new(5, 10);
            let lines = single_line(&[t]);
            assert_eq!(lines.token_at(0, 5), Some(t));
        }

        /// An offset at the last byte of a token (end - 1) returns that token.
        #[test]
        fn offset_at_token_last_byte() {
            let t = Token::new(5, 10);
            let lines = single_line(&[t]);
            assert_eq!(lines.token_at(0, 9), Some(t));
        }

        /// An offset exactly at token.end() is past the token and returns None.
        #[test]
        fn offset_at_token_end_exclusive() {
            let t = Token::new(5, 10);
            let lines = single_line(&[t]);
            assert_eq!(lines.token_at(0, 10), None);
        }

        /// An offset in the gap between two tokens returns None.
        #[test]
        fn offset_between_tokens() {
            let lines = single_line(&[Token::new(0, 3), Token::new(5, 8)]);
            assert_eq!(lines.token_at(0, 4), None);
        }

        /// An out-of-bounds line number returns None.
        #[test]
        fn lineno_out_of_bounds() {
            let lines = single_line(&[Token::new(0, 5)]);
            assert_eq!(lines.token_at(99, 0), None);
        }

        /// An empty line returns None for any offset.
        #[test]
        fn empty_line() {
            let lines = TokenLines::new(1);
            assert_eq!(lines.token_at(0, 0), None);
        }
    }
}
