//! Parser for Aoe2 RMS files.

use tower_lsp::lsp_types::{
    CompletionResponse, Hover, HoverContents, MarkupContent, MarkupKind, Position, Range,
};

use crate::parser::{
    completion::CompletionText, line_offsets::LineOffsets, sorted_ranges::SortedRanges,
    token::Token, token_lines::TokenLines,
};

mod arguments;
mod chunks;
mod completion;
mod hover;
mod line_offsets;
mod predefined;
mod range;
mod sorted_ranges;
mod token;
mod token_lines;
mod tokenizer;

/// Represents a parsed Aoe2 RMS document.
#[derive(Debug)]
pub struct RmsDocument {
    /// The original text of the document.
    text: String,

    /// The byte offsets of each line in the document.
    line_offsets: LineOffsets,

    /// The tokens in the document, grouped by line.
    tokens: TokenLines,

    /// Byte ranges of all block comments, sorted by start offset.
    comment_ranges: SortedRanges,
}

impl RmsDocument {
    /// Parses the given text into an `RmsDocument`.
    pub fn parse(text: String) -> Self {
        let (chunks, line_offsets) = chunks::chunk_text(&text);
        let (tokens, comment_ranges) = tokenizer::tokenize(&text, &chunks, &line_offsets);
        Self {
            text,
            line_offsets,
            tokens,
            comment_ranges,
        }
    }

    /// Returns a hover tooltip for the given position, if one exists.
    pub fn hover(&self, pos: Position) -> Option<Hover> {
        let token = self.token_at(pos.line as usize, pos.character as usize)?;
        let token_text = self.token_text(token);
        let hover_text = hover::lookup_hover(token_text)?;
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: hover_text.to_string(),
            }),
            range: None,
        })
    }

    /// Returns completion suggestions for the given position, if any.
    ///
    /// Returns `None` if:
    /// - The position is out of bounds.
    /// - The position falls within a block comment.
    ///
    /// Otherwise, extracts the token (or partial token) ending at `pos` as the
    /// completion prefix and matches it case-insensitively against the list of
    /// completable tokens. Returns the full list when the prefix is empty.
    pub fn completion(&self, pos: Position) -> Option<CompletionResponse> {
        let (lineno, col) = (pos.line as usize, pos.character as usize);
        let offset = self.offset_at(lineno, col)?;
        if self.comment_ranges.contains(offset) {
            return None;
        }

        // Sets `i..j` to the maximal range of non-whitespace characters around `offset`,
        // where the range can end at `offset` and must contain the character at `offset`
        // if that character is not ascii whitespace.
        let bytes = self.text.as_bytes();
        let mut i = offset;
        while i > 0 && !bytes[i - 1].is_ascii_whitespace() {
            i -= 1;
        }
        let mut j = offset;
        while j < self.text.len() && !bytes[j].is_ascii_whitespace() {
            j += 1;
        }
        debug_assert!(i <= offset && offset <= j);
        debug_assert!((i..j).all(|k| !bytes[k].is_ascii_whitespace()));

        let line_start = self.line_offsets.get(lineno).unwrap();
        let range = Range {
            start: Position::new(lineno as u32, (i - line_start) as u32),
            end: Position::new(lineno as u32, (j - line_start) as u32),
        };
        let completion_text = CompletionText::new(&self.text, i, offset, j, range);
        let matches = completion::filter_tokens(&completion_text);
        let completion_items = matches
            .iter()
            .map(|token| token.to_completion_item(completion_text.range))
            .collect();
        Some(CompletionResponse::Array(completion_items))
    }

    /// Returns the byte offset of the given line and column,
    /// if it is within the document.
    fn offset_at(&self, lineno: usize, col: usize) -> Option<usize> {
        let line_start = self.line_offsets.get(lineno)?;
        let next_line_start = self.line_offsets.get(lineno + 1).unwrap_or(self.text.len());
        let offset = line_start + col;
        (offset < next_line_start).then_some(offset)
    }

    /// Returns the source text of the given token.
    fn token_text(&self, token: Token) -> &str {
        &self.text[token.start()..token.end()]
    }

    /// Returns the token at the given line and column, if one exists.
    /// - `lineno`: The 0-based line index.
    /// - `col`: The 0-based column index.
    fn token_at(&self, lineno: usize, col: usize) -> Option<Token> {
        let offset = self.offset_at(lineno, col)?;
        self.tokens.token_at(lineno, offset)
    }

    /// Returns `true` if the given position falls within or on a block comment.
    /// - `lineno`: The 0-based line index.
    /// - `col`: The 0-based column index.
    fn is_in_comment(&self, lineno: usize, col: usize) -> bool {
        self.offset_at(lineno, col)
            .is_some_and(|offset| self.comment_ranges.contains(offset))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parses `text` into an `RmsDocument`.
    fn parse(text: &str) -> RmsDocument {
        RmsDocument::parse(text.to_string())
    }

    mod offset_at {
        use super::*;

        /// A column in the middle of the only line returns the correct absolute offset.
        #[test]
        fn col_on_single_line() {
            let doc = parse("create_land");
            assert_eq!(doc.offset_at(0, 5), Some(5));
        }

        /// A column on the second line is added to that line's start offset.
        #[test]
        fn col_on_second_line() {
            let doc = parse("abc\ndefgh");
            // Line 1 starts at offset 4; col 2 → offset 6.
            assert_eq!(doc.offset_at(1, 2), Some(6));
        }

        /// Column 0 on any line returns that line's start offset.
        #[test]
        fn col_zero_returns_line_start() {
            let doc = parse("abc\ndefgh");
            assert_eq!(doc.offset_at(1, 0), Some(4));
        }

        /// A column pointing at the newline character is within the line's range
        /// and returns Some.
        #[test]
        fn col_at_newline_returns_some() {
            let doc = parse("abc\ndef");
            // '\n' is at offset 3, which is col 3 on line 0.
            assert_eq!(doc.offset_at(0, 3), Some(3));
        }

        /// A column one past the newline character is on the next line and
        /// returns None.
        #[test]
        fn col_past_newline_returns_none() {
            let doc = parse("abc\ndef");
            assert_eq!(doc.offset_at(0, 4), None);
        }

        /// A column beyond the end of the last line returns None.
        #[test]
        fn col_beyond_last_line_returns_none() {
            let doc = parse("abc");
            assert_eq!(doc.offset_at(0, 10), None);
        }

        /// An out-of-bounds line number returns None.
        #[test]
        fn line_out_of_bounds_returns_none() {
            let doc = parse("abc");
            assert_eq!(doc.offset_at(99, 0), None);
        }

        /// Column 0 on an empty line (between two newlines) returns that
        /// line's start offset.
        #[test]
        fn col_zero_on_empty_line() {
            let doc = parse("abc\n\ndef");
            // Line 1 is empty; its start offset is 4.
            assert_eq!(doc.offset_at(1, 0), Some(4));
        }
    }

    mod token_at {
        use super::*;

        /// A column in the interior of a token returns that token.
        #[test]
        fn col_within_token() {
            let doc = parse("create_land");
            assert_eq!(doc.token_text(doc.token_at(0, 5).unwrap()), "create_land");
        }

        /// A column at the first character of a token returns that token.
        #[test]
        fn col_at_token_start() {
            let doc = parse("create_land");
            assert_eq!(doc.token_text(doc.token_at(0, 0).unwrap()), "create_land");
        }

        /// A column at the last character of a token returns that token.
        #[test]
        fn col_at_token_end() {
            let doc = parse("create_land");
            // "create_land" is 11 chars; last char is at col 10.
            assert_eq!(doc.token_text(doc.token_at(0, 10).unwrap()), "create_land");
        }

        /// A column on whitespace between two tokens returns None.
        #[test]
        fn col_on_whitespace_returns_none() {
            let doc = parse("terrain_type GRASS");
            assert!(doc.token_at(0, 12).is_none());
        }

        /// A column beyond the end of the line returns None.
        #[test]
        fn col_beyond_line_returns_none() {
            let doc = parse("create_land");
            assert!(doc.token_at(0, 100).is_none());
        }

        /// A line number beyond the end of the document returns None.
        #[test]
        fn line_out_of_bounds_returns_none() {
            let doc = parse("create_land");
            assert!(doc.token_at(99, 0).is_none());
        }

        /// A column on the second token of a line returns that token.
        #[test]
        fn col_on_second_token() {
            let doc = parse("terrain_type GRASS");
            assert_eq!(doc.token_text(doc.token_at(0, 13).unwrap()), "GRASS");
        }

        /// A column on the second line returns the correct token on that line.
        #[test]
        fn col_on_second_line() {
            let doc = parse("create_land {\nterrain_type GRASS\n}");
            assert_eq!(doc.token_text(doc.token_at(1, 5).unwrap()), "terrain_type");
        }

        /// A column inside a comment block returns None because comments are stripped.
        #[test]
        fn col_inside_comment_returns_none() {
            let doc = parse("/* comment */");
            assert!(doc.token_at(0, 5).is_none());
        }

        /// A column on an empty line returns None.
        #[test]
        fn col_on_empty_line_returns_none() {
            let doc = parse("abc\n\ndef");
            assert!(doc.token_at(1, 0).is_none());
        }

        /// Tokens are correctly located across many lines with blank lines and
        /// multiple tokens per line.
        #[test]
        fn many_lines() {
            let doc = parse(
                "<PLAYER_SETUP>\nrandom_placement\n\n<LAND_GENERATION>\nbase_terrain GRASS\ncreate_land {\nterrain_type DIRT\n}",
            );
            assert_eq!(
                doc.token_text(doc.token_at(0, 0).unwrap()),
                "<PLAYER_SETUP>"
            );
            assert_eq!(
                doc.token_text(doc.token_at(1, 0).unwrap()),
                "random_placement"
            );
            assert!(doc.token_at(2, 0).is_none());
            assert_eq!(
                doc.token_text(doc.token_at(3, 0).unwrap()),
                "<LAND_GENERATION>"
            );
            assert_eq!(doc.token_text(doc.token_at(4, 0).unwrap()), "base_terrain");
            assert_eq!(doc.token_text(doc.token_at(4, 13).unwrap()), "GRASS");
            assert_eq!(doc.token_text(doc.token_at(5, 0).unwrap()), "create_land");
            assert_eq!(doc.token_text(doc.token_at(6, 0).unwrap()), "terrain_type");
            assert_eq!(doc.token_text(doc.token_at(6, 13).unwrap()), "DIRT");
            assert_eq!(doc.token_text(doc.token_at(7, 0).unwrap()), "}");
        }
    }
}
