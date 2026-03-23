//! A parsed document giving structure to the tokens.
//!
//! The parsed structure assists in supporting:
//! - Autocompletion
//! - Hover Documentation
//! - Constant and Label Renaming

use crate::parser::tokenizer::Token;

/// Represents a parsed document split into tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RmsDocument {
    /// The raw text of the document.
    text: String,

    /// The tokens parsed from the document.
    tokens: Vec<Token>,

    /// Tokens grouped by line number. The stored value is an index
    /// into the `tokens` vector.
    /// `line_index[i]` contains the indices of tokens on line `i`.
    /// `line_index[i][j]` is the index of the `j`th token on line `i`.
    ///
    /// The language server reports positions using 0-based line and column
    /// indices. This vector provides lookup by allowing for an O(1) index into
    /// the line and a binary search to find the token within the line.
    line_index: Vec<Vec<usize>>,

    /// The ranges of text bytes that are part of comments, sorted by their
    /// locations in the document.
    /// No two comment ranges overlap.
    comment_ranges: Vec<CommentRange>,
}

/// Represents a range of text bytes that are part of a comment.
/// Note that while the line numbers are inclusive, the column numbers are
/// exclusive.
/// A comment that ends on the final line has the line number equal to the
/// final line number and the column number equal to the length of the final
/// line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CommentRange {
    /// The line number of the start of the comment range, inclusive.
    start_line: usize,
    /// The column number of the start of the comment range, inclusive.
    start_col: usize,
    /// The line number of the end of the comment range, inclusive.
    end_line: usize,
    /// The column number of the end of the comment range, exclusive.
    end_col: usize,
}

impl RmsDocument {
    /// Creates a new `RmsDocument` from the given text.
    /// Requires that the tokens are sorted by their position in the text.
    pub fn new(text: String, tokens: Vec<Token>) -> Self {
        // Keeps track of comment ranges while iterating.
        let mut comment_ranges = Vec::new();
        let mut comment_depth = 0u32;
        let mut current_comment_start = None;

        // Use the line index of the final token to determine the number of lines.
        let num_lines = tokens.last().map(|t| t.lineno() + 1).unwrap_or(0);
        let mut line_index = Vec::with_capacity(num_lines);
        for (i, token) in tokens.iter().enumerate() {
            // Ensure all lines up to and including the token's line are
            // initialized.
            line_index.resize(token.lineno() + 1, vec![]);
            line_index[token.lineno()].push(i);

            match token.text(&text) {
                "/*" => {
                    if comment_depth == 0 {
                        debug_assert!(current_comment_start.is_none());
                        current_comment_start = Some((token.lineno(), token.line_start()));
                    }
                    comment_depth = comment_depth.saturating_add(1)
                }
                "*/" => {
                    if comment_depth > 0 {
                        debug_assert!(current_comment_start.is_some());
                        comment_depth -= 1;
                        if comment_depth == 0 {
                            let (start_line, start_col) = current_comment_start.unwrap();
                            let (end_line, end_col) = (token.lineno(), token.line_end());
                            comment_ranges.push(CommentRange {
                                start_line,
                                start_col,
                                end_line,
                                end_col,
                            });
                            current_comment_start = None;
                        }
                    }
                }
                _ => {}
            }
        }

        // Push a final comment range for an unclosed comment.
        if let Some((start_line, start_col)) = current_comment_start {
            debug_assert!(comment_depth > 0);
            let last_token = tokens.last();
            let (end_line, end_col) = last_token
                .map(|t| (t.lineno(), t.line_end()))
                .unwrap_or((line_index.len() - 1, 0));
            comment_ranges.push(CommentRange {
                start_line,
                start_col,
                end_line,
                end_col,
            });
        }

        Self {
            text,
            tokens,
            line_index,
            comment_ranges,
        }
    }

    /// Returns a reference to the raw text of the document.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns `true` if the given position is inside a comment,
    /// `false` otherwise.
    /// Positions are 0-based line and column indices.
    /// `false` is returned for positions outside the document.
    pub fn is_comment(&self, lineno: usize, col: usize) -> bool {
        todo!()
    }

    /// Returns the token at the given line and column, if one exists.
    /// - `lineno`: The 0-based line index.
    /// - `col`: The 0-based column index.
    pub fn token_at(&self, lineno: usize, col: usize) -> Option<Token> {
        use std::cmp::Ordering::*;
        let line = self.line_index.get(lineno)?;
        line.binary_search_by(|i| {
            let token = &self.tokens[*i];
            if col < token.line_start() {
                Greater
            } else if col >= token.line_end() {
                Less
            } else {
                Equal
            }
        })
        .ok() // Discard the error result if a token is not found.
        .map(|i| self.tokens[line[i]])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{lexer::lex, tokenizer::tokenize};

    fn parse(text: &str) -> RmsDocument {
        let text = text.to_string();
        let tokens = tokenize(&text, lex(&text));
        RmsDocument::new(text, tokens)
    }

    mod token_at {
        use super::*;

        /// A position within a token returns that token.
        #[test]
        fn test_within_token() {
            let doc = parse("create_land");
            let token = doc.token_at(0, 5).unwrap();
            assert_eq!(token.text(&doc.text), "create_land");
        }

        /// A position at the start of a token returns that token.
        #[test]
        fn test_at_start_of_token() {
            let doc = parse("create_land");
            let token = doc.token_at(0, 0).unwrap();
            assert_eq!(token.text(&doc.text), "create_land");
        }

        /// A position at the last character of a token returns that token.
        #[test]
        fn test_at_last_character_of_token() {
            let doc = parse("create_land");
            let token = doc.token_at(0, 10).unwrap();
            assert_eq!(token.text(&doc.text), "create_land");
        }

        /// A position on whitespace between tokens returns None.
        #[test]
        fn test_whitespace_returns_none() {
            let doc = parse("terrain_type GRASS");
            assert!(doc.token_at(0, 12).is_none());
        }

        /// A position beyond the end of the line returns None.
        #[test]
        fn test_beyond_end_of_line_returns_none() {
            let doc = parse("create_land");
            assert!(doc.token_at(0, 100).is_none());
        }

        /// A line number beyond the end of the document returns None.
        #[test]
        fn test_line_out_of_bounds_returns_none() {
            let doc = parse("create_land");
            assert!(doc.token_at(99, 0).is_none());
        }

        /// A position on the second line returns the correct token.
        #[test]
        fn test_second_line_token() {
            let doc = parse("create_land {\nterrain_type GRASS\n}");
            let token = doc.token_at(1, 5).unwrap();
            assert_eq!(token.text(&doc.text), "terrain_type");
        }

        /// A position on the second token on a line returns the correct token.
        #[test]
        fn test_second_token_on_line() {
            let doc = parse("terrain_type GRASS");
            let token = doc.token_at(0, 13).unwrap();
            assert_eq!(token.text(&doc.text), "GRASS");
        }

        /// A token inside a comment is correctly returned.
        #[test]
        fn test_token_inside_comment() {
            let doc = parse("/* comment */");
            let token = doc.token_at(0, 5).unwrap();
            assert_eq!(token.text(&doc.text), "comment");
            assert!(token.is_comment());
        }

        /// A position on an empty line returns None.
        #[test]
        fn test_empty_line_returns_none() {
            let doc = parse("abc\n\ndef");
            assert!(doc.token_at(1, 0).is_none());
        }

        /// A document with many lines returns the correct token on each line.
        #[test]
        fn test_many_lines() {
            let doc = parse(
                "<PLAYER_SETUP>\nrandom_placement\n\n<LAND_GENERATION>\nbase_terrain GRASS\ncreate_land {\nterrain_type DIRT\n}",
            );
            assert_eq!(
                doc.token_at(0, 0).unwrap().text(&doc.text),
                "<PLAYER_SETUP>"
            );
            assert_eq!(
                doc.token_at(1, 0).unwrap().text(&doc.text),
                "random_placement"
            );
            assert!(doc.token_at(2, 0).is_none());
            assert_eq!(
                doc.token_at(3, 0).unwrap().text(&doc.text),
                "<LAND_GENERATION>"
            );
            assert_eq!(doc.token_at(4, 0).unwrap().text(&doc.text), "base_terrain");
            assert_eq!(doc.token_at(4, 13).unwrap().text(&doc.text), "GRASS");
            assert_eq!(doc.token_at(5, 0).unwrap().text(&doc.text), "create_land");
            assert_eq!(doc.token_at(6, 0).unwrap().text(&doc.text), "terrain_type");
            assert_eq!(doc.token_at(6, 13).unwrap().text(&doc.text), "DIRT");
            assert_eq!(doc.token_at(7, 0).unwrap().text(&doc.text), "}");
        }
    }
}
