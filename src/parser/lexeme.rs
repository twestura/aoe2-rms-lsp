//! Raw text parsed from a RMS file.

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lexeme<'a> {
    /// The text of the lexeme. Does not contain ascii whitespace.
    text: &'a str,
    /// The line number of the lexeme in the RMS file.
    lineno: usize,
    /// The start byte index of the lexeme in the current line (inclusive).
    start: usize,
    /// The end byte index of the lexeme in the current line (exclusive).
    end: usize,
}

impl<'a> Lexeme<'a> {
    /// Creates a new lexeme from the given text and position range.
    pub fn new(text: &'a str, lineno: usize, start: usize, end: usize) -> Self {
        Self {
            text,
            lineno,
            start,
            end,
        }
    }

    /// Returns the text of the lexeme.
    pub fn text(&self) -> &'a str {
        self.text
    }

    /// Returns the line number of the lexeme in the RMS file.
    pub fn lineno(&self) -> usize {
        self.lineno
    }

    /// Returns the start byte index of the lexeme in the RMS file (inclusive).
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the end byte index of the lexeme in the RMS file (exclusive).
    pub fn end(&self) -> usize {
        self.end
    }
}

impl Display for Lexeme<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}
