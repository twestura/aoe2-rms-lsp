//! A parsed document giving structure to the tokens.
//!
//! The parsed structure assists in supporting:
//! - Autocompletion
//! - Hover Documentation
//! - Constant and Label Renaming

/// Represents a parsed document split into tokens.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RmsDocument {
    /// The raw text of the document.
    text: String,
}

impl RmsDocument {
    /// Creates a new `RmsDocument` from the given text.
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
