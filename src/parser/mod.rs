//! Parser for Aoe2 RMS files.

use crate::parser::{line_offsets::LineOffsets, tokenizer::Token};

mod arguments;
mod chunks;
mod line_offsets;
mod predefined;
mod range;
mod tokenizer;

/// Represents a parsed Aoe2 RMS document.
#[derive(Debug)]
pub struct RmsDocument {
    /// The original text of the document.
    text: String,

    /// The byte offsets of each line in the document.
    line_offsets: LineOffsets,

    /// The tokens in the document, grouped by line.
    /// - `tokens[i]` contains the tokens for line `i`, using 0-indexed
    ///    line numbers.
    /// - Each row of tokens is sorted.
    /// - Tokens do not contain comments or whitespace.
    /// - Tokens do not overlap.
    tokens: Vec<Vec<Token>>,
}

impl RmsDocument {
    pub fn parse(text: String) -> Self {
        let (chunks, line_offsets) = chunks::chunk_text(&text);
        let tokens = tokenizer::tokenize(&text, &chunks, &line_offsets);
        Self {
            text,
            line_offsets,
            tokens,
        }
    }

    // TODO
    // Returns the token at the given line and column, if one exists.
    // - `lineno`: The 0-based line index.
    // - `col`: The 0-based column index.
    // pub fn token_at(&self, lineno: usize, col: usize) -> Option<Token> {
    //     use std::cmp::Ordering::*;
    //     let line = self.line_index.get(lineno)?;
    //     line.binary_search_by(|i| {
    //         let token = &self.tokens[*i];
    //         if col < token.line_start() {
    //             Greater
    //         } else if col >= token.line_end() {
    //             Less
    //         } else {
    //             Equal
    //         }
    //     })
    //     .ok() // Discard the error result if a token is not found.
    //     .map(|i| self.tokens[line[i]])
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::parser::{lexer::lex, tokenizer::tokenize};

//     fn parse(text: &str) -> RmsDocument {
//         let text = text.to_string();
//         let tokens = tokenize(&text, lex(&text));
//         RmsDocument::new(text, tokens)
//     }

//     mod token_at {
//         use super::*;

//         /// A position within a token returns that token.
//         #[test]
//         fn test_within_token() {
//             let doc = parse("create_land");
//             let token = doc.token_at(0, 5).unwrap();
//             assert_eq!(token.text(&doc.text), "create_land");
//         }

//         /// A position at the start of a token returns that token.
//         #[test]
//         fn test_at_start_of_token() {
//             let doc = parse("create_land");
//             let token = doc.token_at(0, 0).unwrap();
//             assert_eq!(token.text(&doc.text), "create_land");
//         }

//         /// A position at the last character of a token returns that token.
//         #[test]
//         fn test_at_last_character_of_token() {
//             let doc = parse("create_land");
//             let token = doc.token_at(0, 10).unwrap();
//             assert_eq!(token.text(&doc.text), "create_land");
//         }

//         /// A position on whitespace between tokens returns None.
//         #[test]
//         fn test_whitespace_returns_none() {
//             let doc = parse("terrain_type GRASS");
//             assert!(doc.token_at(0, 12).is_none());
//         }

//         /// A position beyond the end of the line returns None.
//         #[test]
//         fn test_beyond_end_of_line_returns_none() {
//             let doc = parse("create_land");
//             assert!(doc.token_at(0, 100).is_none());
//         }

//         /// A line number beyond the end of the document returns None.
//         #[test]
//         fn test_line_out_of_bounds_returns_none() {
//             let doc = parse("create_land");
//             assert!(doc.token_at(99, 0).is_none());
//         }

//         /// A position on the second line returns the correct token.
//         #[test]
//         fn test_second_line_token() {
//             let doc = parse("create_land {\nterrain_type GRASS\n}");
//             let token = doc.token_at(1, 5).unwrap();
//             assert_eq!(token.text(&doc.text), "terrain_type");
//         }

//         /// A position on the second token on a line returns the correct token.
//         #[test]
//         fn test_second_token_on_line() {
//             let doc = parse("terrain_type GRASS");
//             let token = doc.token_at(0, 13).unwrap();
//             assert_eq!(token.text(&doc.text), "GRASS");
//         }

//         /// A token inside a comment is correctly returned.
//         #[test]
//         fn test_token_inside_comment() {
//             let doc = parse("/* comment */");
//             let token = doc.token_at(0, 5).unwrap();
//             assert_eq!(token.text(&doc.text), "comment");
//             assert!(token.is_comment());
//         }

//         /// A position on an empty line returns None.
//         #[test]
//         fn test_empty_line_returns_none() {
//             let doc = parse("abc\n\ndef");
//             assert!(doc.token_at(1, 0).is_none());
//         }

//         /// A document with many lines returns the correct token on each line.
//         #[test]
//         fn test_many_lines() {
//             let doc = parse(
//                 "<PLAYER_SETUP>\nrandom_placement\n\n<LAND_GENERATION>\nbase_terrain GRASS\ncreate_land {\nterrain_type DIRT\n}",
//             );
//             assert_eq!(
//                 doc.token_at(0, 0).unwrap().text(&doc.text),
//                 "<PLAYER_SETUP>"
//             );
//             assert_eq!(
//                 doc.token_at(1, 0).unwrap().text(&doc.text),
//                 "random_placement"
//             );
//             assert!(doc.token_at(2, 0).is_none());
//             assert_eq!(
//                 doc.token_at(3, 0).unwrap().text(&doc.text),
//                 "<LAND_GENERATION>"
//             );
//             assert_eq!(doc.token_at(4, 0).unwrap().text(&doc.text), "base_terrain");
//             assert_eq!(doc.token_at(4, 13).unwrap().text(&doc.text), "GRASS");
//             assert_eq!(doc.token_at(5, 0).unwrap().text(&doc.text), "create_land");
//             assert_eq!(doc.token_at(6, 0).unwrap().text(&doc.text), "terrain_type");
//             assert_eq!(doc.token_at(6, 13).unwrap().text(&doc.text), "DIRT");
//             assert_eq!(doc.token_at(7, 0).unwrap().text(&doc.text), "}");
//         }
//     }
// }
