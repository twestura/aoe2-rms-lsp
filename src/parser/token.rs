//! Token type for the RMS parser.

use crate::parser::range::ByteRange;

/// A textual token in an Aoe2 RMS document.
/// Tokens are annotated with information needed for language server features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct Token {
    /// The range of bytes in the document that this token occupies.
    range: ByteRange,
}

impl Token {
    /// Creates a token spanning `[start, end)`.
    pub(super) fn new(start: usize, end: usize) -> Self {
        Self { range: ByteRange::new(start, end) }
    }

    /// Returns the start byte offset of this token, inclusive.
    pub(super) fn start(&self) -> usize {
        self.range.start()
    }

    /// Returns the end byte offset of this token, exclusive.
    pub(super) fn end(&self) -> usize {
        self.range.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `new` stores start and end, retrievable via `start()` and `end()`.
    #[test]
    fn new_roundtrip() {
        let t = Token::new(3, 7);
        assert_eq!(t.start(), 3);
        assert_eq!(t.end(), 7);
    }
}
