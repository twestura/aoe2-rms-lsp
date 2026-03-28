//! A sorted collection of non-overlapping byte ranges with O(log n) lookup.

use crate::parser::range::ByteRange;

/// A sorted, non-overlapping collection of non-empty byte ranges.
/// Supports O(log n) lookup to test whether a byte offset falls within any range.
///
/// # Invariants
/// - Every range satisfies `start < end` (non-empty).
/// - Ranges are stored in ascending order and do not overlap:
///   for consecutive ranges `a` and `b`, `a.end() <= b.start()`.
#[derive(Debug)]
pub(super) struct SortedRanges(Vec<ByteRange>);

impl SortedRanges {
    /// Creates an empty `SortedRanges`.
    /// The invariants hold vacuously for an empty collection.
    pub(super) fn new() -> Self {
        Self(vec![])
    }

    /// Appends a non-empty range spanning `[start, end)`.
    ///
    /// # Panics (debug builds only)
    /// - If `start >= end` (empty range).
    /// - If `start` is less than the end of the previous range (out of order or overlapping).
    pub(super) fn push(&mut self, start: usize, end: usize) {
        debug_assert!(start < end, "range must be non-empty: start={start} end={end}");
        debug_assert!(
            self.0.last().map_or(true, |last| last.end() <= start),
            "ranges must be pushed in order without overlap: \
             last.end()={} start={start}",
            self.0.last().map_or(0, |last| last.end()),
        );
        self.0.push(ByteRange::new(start, end));
    }

    /// Returns `true` if `offset` falls within any range.
    pub(super) fn contains(&self, offset: usize) -> bool {
        use std::cmp::Ordering::*;
        self.0
            .binary_search_by(|r| {
                if offset < r.start() {
                    Greater
                } else if offset >= r.end() {
                    Less
                } else {
                    Equal
                }
            })
            .is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to build a `SortedRanges` from a slice of `(start, end)` pairs.
    fn ranges(pairs: &[(usize, usize)]) -> SortedRanges {
        let mut sr = SortedRanges::new();
        for &(start, end) in pairs {
            sr.push(start, end);
        }
        sr
    }

    mod contains {
        use super::*;

        /// An offset before any range returns false.
        #[test]
        fn before_range() {
            assert!(!ranges(&[(5, 10)]).contains(3));
        }

        /// An offset at the start of a range returns true.
        #[test]
        fn at_range_start() {
            assert!(ranges(&[(5, 10)]).contains(5));
        }

        /// An offset in the interior of a range returns true.
        #[test]
        fn inside_range() {
            assert!(ranges(&[(5, 10)]).contains(7));
        }

        /// An offset at the last byte of a range (end - 1) returns true.
        #[test]
        fn at_range_last_byte() {
            assert!(ranges(&[(5, 10)]).contains(9));
        }

        /// An offset at range.end() (exclusive) returns false.
        #[test]
        fn at_range_end_exclusive() {
            assert!(!ranges(&[(5, 10)]).contains(10));
        }

        /// An offset in the gap between two ranges returns false.
        #[test]
        fn between_ranges() {
            assert!(!ranges(&[(0, 5), (10, 15)]).contains(7));
        }

        /// An offset inside the second of two ranges returns true.
        #[test]
        fn inside_second_range() {
            assert!(ranges(&[(0, 5), (10, 15)]).contains(12));
        }

        /// An empty `SortedRanges` always returns false.
        #[test]
        fn empty_ranges() {
            assert!(!ranges(&[]).contains(0));
        }
    }
}
