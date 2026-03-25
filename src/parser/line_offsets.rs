//! A data structure for storing line offsets in a RMS file.

use std::ops::Index;

/// A vector mapping line numbers to byte offsets.
///
/// The element at index `i` represents the offset of the start of line `i`.
///
/// Line numbers are 0-indexed and are strictly increasing.
/// Only the newline character \n marks the end of a line.
/// A carriage return\r by itself is not treated as a line end,
/// and the \r\n sequence is treated as a single line end due to
/// the \n character.
///
/// The first element is always 0, representing the start of the file.
/// An empty file is considered to have a single line at offset 0.
/// A file that ends with a newline character has an offset at the
/// index past that character, which equals the file length but itself
/// is not a valid byte index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineOffsets {
    /// The byte offsets of each line:
    /// - Nonempty.
    /// - The first element is 0.
    /// - All elements are strictly increasing.
    offsets: Vec<usize>,
}

impl LineOffsets {
    /// Constructs a new `LineOffsets` with a single line at offset 0.
    pub fn new() -> Self {
        Self { offsets: vec![0] }
    }

    /// Pushes a new line offset onto the end of the offset list.
    /// Requires that `offset` is strictly greater than the last element.
    pub fn push(&mut self, offset: usize) {
        debug_assert!(*self.offsets.last().unwrap() < offset);
        self.offsets.push(offset);
    }

    /// Returns the byte offset of the given line number, if it exists.
    pub fn get(&self, line_number: usize) -> Option<usize> {
        if line_number < self.offsets.len() {
            Some(self.offsets[line_number])
        } else {
            None
        }
    }

    /// Returns the number of lines.
    pub fn len(&self) -> usize {
        self.offsets.len()
    }
}

impl Default for LineOffsets {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for LineOffsets {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.offsets[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;

        /// Tests that a new `LineOffsets` has exactly one entry at offset 0.
        #[test]
        fn has_single_zero_offset() {
            let lo = LineOffsets::new();
            assert_eq!(lo.get(0), Some(0));
            assert_eq!(lo.get(1), None);
        }
    }

    mod default {
        use super::*;

        /// Tests that `Default` produces the same value as `new`.
        #[test]
        fn default_equals_new() {
            assert_eq!(LineOffsets::default(), LineOffsets::new());
        }
    }

    mod push {
        use super::*;

        /// Tests that pushing a single offset makes it retrievable at index 1.
        #[test]
        fn push_single() {
            let mut lo = LineOffsets::new();
            lo.push(5);
            assert_eq!(lo.get(1), Some(5));
        }

        /// Tests that pushing multiple offsets stores them all in the correct order.
        #[test]
        fn push_multiple() {
            let mut lo = LineOffsets::new();
            lo.push(3);
            lo.push(7);
            lo.push(12);
            assert_eq!(lo.get(0), Some(0));
            assert_eq!(lo.get(1), Some(3));
            assert_eq!(lo.get(2), Some(7));
            assert_eq!(lo.get(3), Some(12));
        }
    }

    mod get {
        use super::*;

        /// Tests that `get` returns `None` for an out-of-bounds index on a
        /// multi-element `LineOffsets`.
        #[test]
        fn out_of_bounds_on_multi_element() {
            let mut lo = LineOffsets::new();
            lo.push(4);
            lo.push(9);
            assert_eq!(lo.get(3), None);
        }

        /// Tests that `get` returns the correct offset for each line in a
        /// multi-element `LineOffsets`.
        #[test]
        fn multi_element_returns_correct_offsets() {
            let mut lo = LineOffsets::new();
            lo.push(4);
            lo.push(9);
            assert_eq!(lo.get(0), Some(0));
            assert_eq!(lo.get(1), Some(4));
            assert_eq!(lo.get(2), Some(9));
        }
    }

    mod len {
        use super::*;

        /// Tests that a new `LineOffsets` has length 1.
        #[test]
        fn new_has_len_one() {
            assert_eq!(LineOffsets::new().len(), 1);
        }

        /// Tests that `len` increases by one after each `push`.
        #[test]
        fn len_increases_with_push() {
            let mut lo = LineOffsets::new();
            lo.push(5);
            assert_eq!(lo.len(), 2);
            lo.push(10);
            assert_eq!(lo.len(), 3);
        }
    }

    mod index {
        use super::*;

        /// Tests that the index operator returns 0 for a single-element `LineOffsets`.
        #[test]
        fn index_single_element() {
            let lo = LineOffsets::new();
            assert_eq!(lo[0], 0);
        }

        /// Tests that the index operator returns the correct offset for each
        /// element of a multi-element `LineOffsets`.
        #[test]
        fn index_multi_element() {
            let mut lo = LineOffsets::new();
            lo.push(5);
            lo.push(10);
            assert_eq!(lo[0], 0);
            assert_eq!(lo[1], 5);
            assert_eq!(lo[2], 10);
        }
    }
}
