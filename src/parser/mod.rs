//! Parser for Aoe2 RMS files.

use crate::parser::{line_offsets::LineOffsets, range::ByteRange};

mod arguments;
mod chunks;
mod document;
mod lexer;
mod line_offsets;
mod predefined;
mod range;
mod tokenizer;

#[derive(Debug)]
pub struct RmsDocument {
    text: String,
    line_offsets: LineOffsets,
    tokens: Vec<Vec<Token>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Token {
    range: ByteRange,
}

impl RmsDocument {
    pub fn parse(text: String) -> Self {
        let (chunks, line_offsets) = chunks::chunk_text(&text);
        Self {
            text,
            line_offsets: line_offsets,
            tokens: vec![],
        }
    }
}
