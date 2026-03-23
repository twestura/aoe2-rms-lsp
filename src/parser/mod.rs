//! Parser for Aoe2 RMS files.

mod arguments;
mod document;
mod lexeme;
mod lexer;
mod predefined;
mod token;
mod tokenizer;

pub use document::RmsDocument;

/// TODO
pub fn parse(text: String) -> RmsDocument {
    // let lexemes = lexer::lex(text);
    // let tokens = tokenizer::parse(text, lexemes);
    // RmsDocument::new(text, tokens)
    RmsDocument::new(text)
}
