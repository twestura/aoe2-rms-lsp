//! Tokenizer for the RMS language.
//! Turns a sequence of text chunks into a sequence of tokens.
//! The tokens are annotated with information needed for language server
//! features.

use crate::parser::{
    chunks::{Chunk, ChunkKind},
    line_offsets::LineOffsets,
    range::ByteRange,
};

/// A textual token in an Aoe2 RMS document.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    /// The range of bytes in the document that this token occupies.
    range: ByteRange,
}

/// Tokenizes the given text into a vector of tokens, grouped by line.
/// - `text` is the original text of the document.
/// - `chunks` is the list of ascii whitespace and non-whitespace substrings.
/// - `line_offsets` is the start byte offset for each line..
pub fn tokenize(text: &str, chunks: &[Chunk], line_offsets: &LineOffsets) -> Vec<Vec<Token>> {
    use ChunkKind::*;
    let num_lines = line_offsets.len();
    let mut tokens = vec![vec![]; num_lines];
    let mut lineno = 0;
    let mut comment_depth = 0u32;
    // Invariant: `lineno` is the line number of `chunk.start()`.
    for chunk in chunks {
        match chunk.kind() {
            Whitespace => {
                // After this loop, lineno is the line containing chunk.end().
                // Uses <= because chunk.end() is the exclusive end of the whitespace chunk,
                // so a line that starts exactly there is the line of the next text chunk.
                while lineno + 1 < num_lines && line_offsets[lineno + 1] <= chunk.end() {
                    lineno += 1;
                }
            }
            Text => match &text[chunk.start()..chunk.end()] {
                "/*" => comment_depth = comment_depth.saturating_add(1),
                "*/" if comment_depth > 0 => comment_depth -= 1,
                _lexeme => {
                    if comment_depth == 0 {
                        tokens[lineno].push(Token {
                            range: ByteRange::new(chunk.start(), chunk.end()),
                        })
                    }
                }
            },
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::chunks::chunk_text;

    /// Runs the pipeline of chunking then tokenizing `text`.
    fn tokenize_text(text: &str) -> Vec<Vec<Token>> {
        let (chunks, line_offsets) = chunk_text(text);
        tokenize(text, &chunks, &line_offsets)
    }

    /// Returns the text of the given `token` from `text`.
    /// Requires the start and end indices of the token to be within the text.
    fn token_text<'a>(token: &Token, text: &'a str) -> &'a str {
        &text[token.range.start()..token.range.end()]
    }

    mod tokenize {
        use super::*;

        /// An empty string produces one row with no tokens.
        #[test]
        fn empty_text_one_empty_row() {
            let rows = tokenize_text("");
            assert_eq!(rows.len(), 1);
            assert!(rows[0].is_empty());
        }

        /// A single token on the first line is placed in row 0.
        #[test]
        fn single_token_in_row_zero() {
            let text = "create_land";
            let rows = tokenize_text(text);
            assert_eq!(rows[0].len(), 1);
            assert_eq!(token_text(&rows[0][0], text), "create_land");
        }

        /// Two space-separated tokens on the same line are both placed in row 0.
        #[test]
        fn two_tokens_same_line_same_row() {
            let text = "terrain_type GRASS";
            let rows = tokenize_text(text);
            assert_eq!(rows[0].len(), 2);
            assert_eq!(token_text(&rows[0][0], text), "terrain_type");
            assert_eq!(token_text(&rows[0][1], text), "GRASS");
        }

        /// Tokens on separate lines are placed in separate rows.
        #[test]
        fn tokens_on_separate_lines() {
            let text = "create_land\nterrain_type";
            let rows = tokenize_text(text);
            assert_eq!(rows[0].len(), 1);
            assert_eq!(token_text(&rows[0][0], text), "create_land");
            assert_eq!(rows[1].len(), 1);
            assert_eq!(token_text(&rows[1][0], text), "terrain_type");
        }

        /// A token on a line followed by a newline and an indented token
        /// on the next line produces one token per row.
        #[test]
        fn indented_token_on_second_line() {
            let text = "create_land\n  terrain_type";
            let rows = tokenize_text(text);
            assert_eq!(rows[0].len(), 1);
            assert_eq!(rows[1].len(), 1);
            assert_eq!(token_text(&rows[1][0], text), "terrain_type");
        }

        /// A comment and its delimiters produce no tokens.
        #[test]
        fn comment_produces_no_tokens() {
            let text = "/* comment */";
            let rows = tokenize_text(text);
            assert!(rows[0].is_empty());
        }

        /// A token before a comment is kept; the comment is stripped.
        #[test]
        fn token_before_comment_kept() {
            let text = "before /* comment */";
            let rows = tokenize_text(text);
            assert_eq!(rows[0].len(), 1);
            assert_eq!(token_text(&rows[0][0], text), "before");
        }

        /// A token after a comment is kept; the comment is stripped.
        #[test]
        fn token_after_comment_kept() {
            let text = "/* comment */ after";
            let rows = tokenize_text(text);
            assert_eq!(rows[0].len(), 1);
            assert_eq!(token_text(&rows[0][0], text), "after");
        }

        /// Nested comments are fully stripped; only the token after is kept.
        #[test]
        fn nested_comment_stripped() {
            let text = "/* outer /* inner */ still outer */ after";
            let rows = tokenize_text(text);
            assert_eq!(rows[0].len(), 1);
            assert_eq!(token_text(&rows[0][0], text), "after");
        }

        /// An unterminated comment strips all remaining tokens.
        #[test]
        fn unterminated_comment_strips_rest() {
            let text = "before /* unterminated token";
            let rows = tokenize_text(text);
            assert_eq!(rows[0].len(), 1);
            assert_eq!(token_text(&rows[0][0], text), "before");
        }

        /// A multiline comment strips tokens on all spanned lines; the token
        /// after the closing delimiter is placed on its own line.
        #[test]
        fn multiline_comment_stripped() {
            let text = "/* comment\nstill comment */ after";
            let rows = tokenize_text(text);
            assert!(rows[0].is_empty());
            assert_eq!(rows[1].len(), 1);
            assert_eq!(token_text(&rows[1][0], text), "after");
        }

        /// `/*comment*/` with no surrounding whitespace is a single text chunk
        /// and is not treated as a comment delimiter.
        #[test]
        fn comment_open_no_whitespace_not_stripped() {
            let text = "/*comment*/";
            let rows = tokenize_text(text);
            assert_eq!(rows[0].len(), 1);
            assert_eq!(token_text(&rows[0][0], text), "/*comment*/");
        }

        /// Token byte range is correct for a token at the start of the file.
        #[test]
        fn token_range_at_file_start() {
            let text = "create_land";
            let rows = tokenize_text(text);
            assert_eq!(rows[0][0].range.start(), 0);
            assert_eq!(rows[0][0].range.end(), 11);
        }

        /// Token byte range is correct for the second token on a line.
        #[test]
        fn token_range_second_token_on_line() {
            let text = "terrain_type GRASS";
            let rows = tokenize_text(text);
            assert_eq!(rows[0][1].range.start(), 13);
            assert_eq!(rows[0][1].range.end(), 18);
        }

        /// Token byte range is correct for a token on the second line.
        #[test]
        fn token_range_second_line() {
            let text = "abc\nterrain_type";
            let rows = tokenize_text(text);
            assert_eq!(rows[1][0].range.start(), 4);
            assert_eq!(rows[1][0].range.end(), 16);
        }

        /// A trailing newline produces an extra empty row.
        #[test]
        fn trailing_newline_produces_empty_row() {
            let text = "abc\n";
            let rows = tokenize_text(text);
            assert_eq!(rows.len(), 2);
            assert_eq!(rows[0].len(), 1);
            assert!(rows[1].is_empty());
        }

        /// Multiple blank lines produce multiple empty rows.
        #[test]
        fn blank_lines_produce_empty_rows() {
            let text = "a\n\nb";
            let rows = tokenize_text(text);
            assert_eq!(rows.len(), 3);
            assert_eq!(rows[0].len(), 1);
            assert!(rows[1].is_empty());
            assert_eq!(rows[2].len(), 1);
        }
    }

    mod lineno_tracking {
        use super::*;

        /// Whitespace with no newline does not advance the line: both tokens
        /// land on row 0.
        #[test]
        fn spaces_only_no_line_advance() {
            let text = "a  b";
            let rows = tokenize_text(text);
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].len(), 2);
        }

        /// A newline at the very start of a whitespace chunk (the chunk is
        /// exactly `\n`) advances the line. This is the `<=` edge case: the
        /// next line's start offset equals chunk.end().
        #[test]
        fn newline_exactly_at_chunk_end_advances_line() {
            let text = "a\nb";
            let rows = tokenize_text(text);
            assert_eq!(token_text(&rows[0][0], text), "a");
            assert_eq!(token_text(&rows[1][0], text), "b");
        }

        /// A newline followed by spaces (newline mid-whitespace) still
        /// advances the line: the token after the spaces lands on row 1.
        #[test]
        fn newline_mid_whitespace_advances_line() {
            let text = "a\n  b";
            let rows = tokenize_text(text);
            assert_eq!(token_text(&rows[0][0], text), "a");
            assert_eq!(token_text(&rows[1][0], text), "b");
        }

        /// Two newlines in one whitespace chunk advance the line twice: the
        /// token after them lands on row 2.
        #[test]
        fn two_newlines_advance_line_twice() {
            let text = "a\n\nb";
            let rows = tokenize_text(text);
            assert_eq!(token_text(&rows[0][0], text), "a");
            assert_eq!(token_text(&rows[2][0], text), "b");
        }
    }
}
