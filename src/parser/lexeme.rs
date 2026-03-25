//! Lexeme type produced by the first pass of the parser.
//!
//! A [`Lexeme`] represents a single contiguous run of either ASCII whitespace
//! or non-ASCII-whitespace characters within the document text. Only the ASCII
//! whitespace characters are treated as delimiters; Unicode whitespace such as
//! non-breaking space is classified as a word character. The full document can
//! be reconstructed by concatenating the text of all lexemes in order.

use super::range::ByteRange;

/// Whether a [`Lexeme`] is a run of ASCII whitespace or a run of other
/// characters.
///
/// Only the ASCII whitespace characters (space `0x20`, tab `0x09`, newline
/// `0x0A`, carriage return `0x0D`, vertical tab `0x0B`, form feed `0x0C`) are
/// treated as whitespace. Unicode whitespace characters such as non-breaking
/// space (`U+00A0`) are classified as [`Word`][LexemeKind::Word] characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexemeKind {
    /// A contiguous run of ASCII whitespace characters.
    Whitespace,
    /// A contiguous run of characters that are not ASCII whitespace.
    /// This includes all non-ASCII characters, even Unicode whitespace.
    Word,
}

/// A contiguous run of characters of uniform kind within the document text.
///
/// Lexemes are produced by the first tokenization pass and cover the entire
/// document without gaps or overlaps: every byte belongs to exactly one lexeme.
/// The text of a lexeme is `&document_text[lexeme.range.clone()]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexeme {
    /// Whether this run is whitespace or a word.
    pub kind: LexemeKind,
    /// The byte range of this run within the document text.
    pub range: ByteRange,
}
