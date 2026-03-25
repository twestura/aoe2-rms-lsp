/// A half-open byte offset range `[start, end)` within a document's text buffer.
///
/// This is a type alias for [`std::ops::Range<usize>`] rather than a newtype so
/// that callers can construct ranges with the standard `start..end` syntax and
/// use all of the standard library's range utilities without wrapping or
/// unwrapping.
///
/// # Relationship to LSP ranges
///
/// [`lsp_types::Range`] uses `{ line: u32, character: u32 }` positions — a
/// line/character range for the LSP wire protocol. `ByteRange` operates on
/// raw byte offsets into the document text and is an entirely separate concept;
/// conversion between the two requires the document's `line_offsets` table.
pub type ByteRange = std::ops::Range<usize>;
