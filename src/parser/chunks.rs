//! Splits a sequence of bytes into chunks of whitespace and text.
//! Keeps track of the byte indices of newlines for the overall text
//! during the chunking pass.
//!
//! Only ASCII whitespace is considered as whitespace. General Unicode
//! whitespace, such as a nonbreaking space, is treated as nonwhitespace text.

use crate::parser::{line_offsets::LineOffsets, range::ByteRange};

/// Whether a chunk consists of whitespace or nonwhitespace characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChunkKind {
    /// ASCII whitespace.
    Whitespace,
    /// Non-whitespace characters.
    Text,
}

/// The indices of a maximal substring consisting of either ASCII whitespace or
/// nonwhitespace characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Chunk {
    /// The start and end byte indices of the chunk in the overall text buffer.
    range: ByteRange,
    /// Whether the chunk consists of ASCII whitespace or nonwhitespace.
    kind: ChunkKind,
}

/// Chunks the given text into whitespace and nonwhitespace substrings.
/// Returns the chunks and the byte offsets of the start of each line.
/// The returned chunks are non-overlapping and sorted by their start byte
/// index.
pub fn chunk_text(text: &str) -> (Vec<Chunk>, LineOffsets) {
    let mut chunker = Chunker::new(text);
    chunker.chunk();
    (chunker.chunks, chunker.line_offsets)
}

/// Returns a `Chunk` representing a non-whitespace text substring.
fn make_text_chunk(start: usize, end: usize) -> Chunk {
    Chunk {
        range: ByteRange::new(start, end),
        kind: ChunkKind::Text,
    }
}

/// Returns a `Chunk` representing an ASCII whitespace substring.
fn make_ws_chunk(start: usize, end: usize) -> Chunk {
    Chunk {
        range: ByteRange::new(start, end),
        kind: ChunkKind::Whitespace,
    }
}

/// A helper struct for chunking text.
/// Maintains the state of the iteration over the input bytes.
#[derive(Debug)]
struct Chunker<'a> {
    /// The input bytes of the text to be chunked.
    bytes: &'a [u8],
    /// The byte offsets of the start of each line in the input text.
    line_offsets: LineOffsets,
    /// The chunks of the text, sorted by their start byte index.
    chunks: Vec<Chunk>,
    /// The current index into the input bytes.
    index: usize,
}

impl<'a> Chunker<'a> {
    /// Creates a new `Chunker` for the given text.
    /// The initial line indices are a vector with a single `0` element,
    /// representing the start of the first line.
    fn new(text: &'a str) -> Self {
        Self {
            bytes: text.as_bytes(),
            line_offsets: LineOffsets::new(),
            chunks: vec![],
            index: 0,
        }
    }

    /// Parses the input bytes into chunks.
    /// Called exactly once after initialization to perform the chunking.
    fn chunk(&mut self) {
        debug_assert!(self.index == 0);
        while self.index < self.bytes.len() {
            if self.bytes[self.index].is_ascii_whitespace() {
                self.chunk_whitespace();
            } else {
                self.chunk_text();
            }
        }
    }

    /// Chunks a run of whitespace characters.
    /// Mutates `self.chunks` and `self.line_offsets`.
    /// Advances `self.index` to the end of the whitespace run.
    /// Requires `self.index` to be at the start of a whitespace run.
    fn chunk_whitespace(&mut self) {
        debug_assert!(self.index < self.bytes.len());
        debug_assert!(self.bytes[self.index].is_ascii_whitespace());
        let start = self.index;
        while self.index < self.bytes.len() {
            let byte = self.bytes[self.index];
            if !byte.is_ascii_whitespace() {
                break;
            }
            self.index += 1;
            if byte == b'\n' {
                // The next line starts after the newline character,
                // so the line offset is pushed after incrementing.
                self.line_offsets.push(self.index);
            }
        }
        self.chunks.push(make_ws_chunk(start, self.index));
    }

    /// Chunks a run of nonwhitespace characters.
    /// Mutates `self.chunks`. Does not mutate `self.line_offsets`.
    /// Advances `self.index` to the end of the text run.
    /// Requires `self.index` to be at the start of a text run.
    fn chunk_text(&mut self) {
        debug_assert!(self.index < self.bytes.len());
        debug_assert!(!self.bytes[self.index].is_ascii_whitespace());
        let start = self.index;
        while self.index < self.bytes.len() {
            let byte = self.bytes[self.index];
            if byte.is_ascii_whitespace() {
                break;
            }
            self.index += 1;
        }
        self.chunks.push(make_text_chunk(start, self.index));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod empty {
        use super::*;

        /// Tests that chunking an empty string produces no chunks.
        #[test]
        fn empty_text_no_chunks() {
            let (chunks, _) = chunk_text("");
            assert!(chunks.is_empty());
        }

        /// Tests that chunking an empty string produces a single line offset of 0.
        #[test]
        fn empty_text_line_offsets() {
            let (_, lo) = chunk_text("");
            assert_eq!(lo, LineOffsets::new());
        }
    }

    mod single_chunk {
        use super::*;

        /// Tests that a single word produces one Text chunk covering the whole text.
        #[test]
        fn single_word() {
            let (chunks, _) = chunk_text("hello");
            assert_eq!(chunks, vec![make_text_chunk(0, 5)]);
        }

        /// Tests that a single run of whitespace produces one Whitespace chunk.
        #[test]
        fn single_whitespace_run() {
            let (chunks, _) = chunk_text("   ");
            assert_eq!(chunks, vec![make_ws_chunk(0, 3)]);
        }
    }

    mod multi_chunk {
        use super::*;

        /// Tests that a word, a space, and another word produce three alternating chunks.
        #[test]
        fn word_space_word() {
            let (chunks, _) = chunk_text("a b");
            assert_eq!(
                chunks,
                vec![
                    make_text_chunk(0, 1),
                    make_ws_chunk(1, 2),
                    make_text_chunk(2, 3)
                ]
            );
        }

        /// Tests that no two adjacent chunks share the same kind.
        #[test]
        fn chunk_kinds_alternate() {
            let (chunks, _) = chunk_text("a b c d");
            for window in chunks.windows(2) {
                assert_ne!(window[0].kind, window[1].kind);
            }
        }

        /// Tests that chunks are contiguous and together cover every byte of the text.
        #[test]
        fn chunks_cover_all_bytes() {
            let text_str = "hello world\nfoo";
            let (chunks, _) = chunk_text(text_str);
            let mut expected_start = 0;
            for chunk in &chunks {
                assert_eq!(chunk.range.start(), expected_start);
                expected_start = chunk.range.end();
            }
            assert_eq!(expected_start, text_str.len());
        }
    }

    mod line_offsets {
        use super::*;

        /// Tests that a single newline produces a line offset pointing just past it.
        #[test]
        fn newline_adds_line_offset() {
            let (_, lo) = chunk_text("a\nb");
            assert_eq!(lo[0], 0);
            assert_eq!(lo[1], 2);
            assert_eq!(lo.get(2), None);
        }

        /// Tests that multiple separated newlines each produce a line offset.
        #[test]
        fn multiple_newlines() {
            let (_, lo) = chunk_text("a\nb\nc");
            assert_eq!(lo[0], 0);
            assert_eq!(lo[1], 2);
            assert_eq!(lo[2], 4);
            assert_eq!(lo.get(3), None);
        }

        /// Tests that consecutive newlines each produce a separate line offset.
        #[test]
        fn consecutive_newlines() {
            let (_, lo) = chunk_text("a\n\nb");
            assert_eq!(lo[0], 0);
            assert_eq!(lo[1], 2);
            assert_eq!(lo[2], 3);
            assert_eq!(lo.get(3), None);
        }

        /// Tests that a trailing newline produces a line offset equal to text length.
        #[test]
        fn trailing_newline() {
            let text_str = "a\n";
            let (_, lo) = chunk_text(text_str);
            assert_eq!(lo[0], 0);
            assert_eq!(lo[1], text_str.len());
            assert_eq!(lo.get(2), None);
        }

        /// Tests that multiple trailing newlines each produce a separate line offset.
        #[test]
        fn multiple_trailing_newlines() {
            let text_str = "a\n\n";
            let (_, lo) = chunk_text(text_str);
            assert_eq!(lo[0], 0);
            assert_eq!(lo[1], 2);
            assert_eq!(lo[2], text_str.len());
            assert_eq!(lo.get(3), None);
        }

        /// Tests that `\r\n` adds only one line offset (triggered by `\n`, not `\r`).
        #[test]
        fn crlf_one_line_offset() {
            let (_, lo) = chunk_text("a\r\nb");
            assert_eq!(lo[0], 0);
            assert_eq!(lo[1], 3);
            assert_eq!(lo.get(2), None);
        }
    }

    mod unicode {
        use super::*;

        /// Tests that a non-breaking space (U+00A0) is classified as Text, not Whitespace.
        #[test]
        fn non_ascii_whitespace_is_text() {
            let (chunks, _) = chunk_text("\u{00A0}");
            assert_eq!(chunks.len(), 1);
            assert_eq!(chunks[0].kind, ChunkKind::Text);
        }

        /// Tests that a multi-byte Unicode character produces a Text chunk with correct byte range.
        #[test]
        fn multibyte_unicode_correct_range() {
            let (chunks, _) = chunk_text("é"); // U+00E9: 2 bytes (0xC3 0xA9)
            assert_eq!(chunks, vec![make_text_chunk(0, 2)]);
        }
    }

    mod text_chunk_contents {
        use super::*;

        /// Tests that no Text chunk contains an ASCII whitespace byte.
        #[test]
        fn no_ascii_whitespace_in_text_chunks() {
            let text_str = "hello world\nfoo\tbar";
            let (chunks, _) = chunk_text(text_str);
            for chunk in chunks.iter().filter(|c| c.kind == ChunkKind::Text) {
                let bytes = &text_str.as_bytes()[chunk.range.start()..chunk.range.end()];
                assert!(!bytes.iter().any(|b| b.is_ascii_whitespace()));
            }
        }
    }
}
