//! Parser for Aoe2 RMS files.

mod arguments;
mod document;
mod lexer;
mod predefined;
mod tokenizer;

pub use document::RmsDocument;

/// TODO
pub fn parse(text: String) -> RmsDocument {
    let lexemes = lexer::lex(&text);
    let tokens = tokenizer::tokenize(&text, lexemes);
    RmsDocument::new(text, tokens)
}
