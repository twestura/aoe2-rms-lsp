//! Byte range for text in an Aoe2 RMS file.

use std::fmt::{Display, Formatter};

/// A byte range in an Aoe2 RMS file.
/// The range is a simple pair of byte offsets unassociated with text.
/// The client must check that the range consists of valid byte indices
/// when using it with a text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ByteRange {
    /// The start byte offset of the range.
    /// Requires `start <= end`.
    start: usize,
    /// The end byte offset of the range.
    /// Requires `end >= start`.
    end: usize,
}

impl ByteRange {
    /// Returns a new `ByteRange` with the given start and end offsets.
    /// Requires `start <= end`.
    /// `start` is inclusive and `end` is exclusive.
    pub fn new(start: usize, end: usize) -> Self {
        debug_assert!(start <= end);
        Self { start, end }
    }

    /// Returns the start byte offset of the range, inclusive.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the end byte offset of the range, exclusive.
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns the number of bytes in the range.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns `true` if the range is empty (start == end),
    /// `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

impl Display for ByteRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}..{}", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that `new` stores the start and end offsets correctly.
    #[test]
    fn new_stores_start_and_end() {
        let r = ByteRange::new(3, 7);
        assert_eq!(r.start(), 3);
        assert_eq!(r.end(), 7);
    }

    /// Tests that `len` returns end minus start.
    #[test]
    fn len_is_end_minus_start() {
        let r = ByteRange::new(3, 7);
        assert_eq!(r.len(), 4);
    }

    /// Tests that a range where start equals end has zero length.
    #[test]
    fn empty_range_has_zero_len() {
        let r = ByteRange::new(5, 5);
        assert_eq!(r.len(), 0);
    }

    /// Tests that `ByteRange` is `Copy` — the original remains usable after assignment.
    #[test]
    fn copy_is_independent() {
        let r = ByteRange::new(1, 4);
        let r2 = r;
        assert_eq!(r.start(), 1);
        assert_eq!(r2.start(), 1);
    }

    /// Tests that a non-empty range returns `false` from `is_empty`.
    #[test]
    fn is_empty_false_for_nonempty_range() {
        let r = ByteRange::new(3, 7);
        assert!(!r.is_empty());
    }

    /// Tests that a range where start equals end returns `true` from `is_empty`.
    #[test]
    fn is_empty_true_when_start_equals_end() {
        let r = ByteRange::new(5, 5);
        assert!(r.is_empty());
    }

    /// Tests that `is_empty` and `len` are consistent — `is_empty` iff `len == 0`.
    #[test]
    fn is_empty_consistent_with_len() {
        assert!(ByteRange::new(4, 4).is_empty());
        assert_eq!(ByteRange::new(4, 4).len(), 0);
        assert!(!ByteRange::new(4, 5).is_empty());
        assert_ne!(ByteRange::new(4, 5).len(), 0);
    }
}
