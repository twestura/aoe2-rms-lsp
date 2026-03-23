//! Tokenizer for the RMS language.
//! Turns a sequence of lexemes into a sequence of tokens.
//! The tokens are annotated with information needed for language server
//! features.

use crate::parser::lexer::Lexeme;

/// Returns a sequence of tokens that annotate the lexemes with
/// semantic information about their context in the RMS file.
pub fn tokenize(rms_text: &str, lexemes: Vec<Lexeme>) -> Vec<Token> {
    let mut tokens = vec![];
    let mut comment_depth = 0u32;
    for lexeme in lexemes {
        let token_text = lexeme.text(rms_text);
        match token_text {
            "/*" => comment_depth = comment_depth.saturating_add(1),
            "*/" if comment_depth > 0 => {
                let is_comment = true;
                tokens.push(Token { lexeme, is_comment });
                comment_depth -= 1;
                continue;
            }
            _ => {}
        }
        let is_comment = comment_depth > 0;
        tokens.push(Token { lexeme, is_comment });
    }
    tokens
}

/// Represents a token in the RMS language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token {
    /// The underlying text location of the token in the RMS file.
    lexeme: Lexeme,
    /// `true` if the token is within a comment, `false` otherwise.
    is_comment: bool,
}

impl Token {
    /// Returns the start byte index of the token in the RMS file (inclusive).
    pub fn text_start(&self) -> usize {
        self.lexeme.text_start
    }

    /// Returns the end byte index of the token in the RMS file (exclusive).
    pub fn text_end(&self) -> usize {
        self.lexeme.text_end
    }

    /// Returns the line number of the token in the RMS file.
    pub fn lineno(&self) -> usize {
        self.lexeme.lineno
    }

    /// Returns the start byte index of the token in the current line (inclusive).
    pub fn line_start(&self) -> usize {
        self.lexeme.line_start
    }

    /// Returns the end byte index of the token in the current line (exclusive).
    pub fn line_end(&self) -> usize {
        self.lexeme.line_end
    }

    /// Returns the text of the token as a `&str` slice from the RMS file.
    /// Requires that the `text_start` and `text_end` indices are valid for the
    /// source string.
    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        self.lexeme.text(source)
    }

    /// Returns `true` if the token is within a comment, `false` otherwise.
    pub fn is_comment(&self) -> bool {
        self.is_comment
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::lexer::lex;

    /// Helper function to tokenize a string using the lexer and tokenizer.
    fn tokenize_text(text: &str) -> Vec<Token> {
        tokenize(text, lex(text))
    }

    mod tokenize {
        use super::*;

        /// An empty string produces no tokens.
        #[test]
        fn test_empty() {
            assert!(tokenize_text("").is_empty());
        }

        /// A plain token outside a comment is not marked as a comment.
        #[test]
        fn test_plain_token_not_comment() {
            let text = "create_land";
            let tokens = tokenize_text(text);
            assert_eq!(tokens.len(), 1);
            assert!(!tokens[0].is_comment());
        }

        /// The opening comment delimiter is marked as a comment.
        #[test]
        fn test_comment_open_is_comment() {
            let text = "/* comment */";
            let tokens = tokenize_text(text);
            assert!(tokens[0].is_comment());
        }

        /// The closing comment delimiter is marked as a comment.
        #[test]
        fn test_comment_close_is_comment() {
            let text = "/* comment */";
            let tokens = tokenize_text(text);
            assert!(tokens[2].is_comment());
        }

        /// Tokens inside a comment are marked as comments.
        #[test]
        fn test_token_inside_comment_is_comment() {
            let text = "/* comment */";
            let tokens = tokenize_text(text);
            assert!(tokens[1].is_comment());
        }

        /// Tokens before a comment are not marked as comments.
        #[test]
        fn test_token_before_comment_not_comment() {
            let text = "before /* comment */";
            let tokens = tokenize_text(text);
            assert!(!tokens[0].is_comment());
        }

        /// Tokens after a comment are not marked as comments.
        #[test]
        fn test_token_after_comment_not_comment() {
            let text = "/* comment */ after";
            let tokens = tokenize_text(text);
            assert!(!tokens[3].is_comment());
        }

        /// Nested comments are handled correctly.
        #[test]
        fn test_nested_comment() {
            let text = "/* outer /* inner */ still outer */ after";
            let tokens = tokenize_text(text);
            let after = tokens.last().unwrap();
            assert_eq!(after.text(text), "after");
            assert!(!after.is_comment());
            // All tokens except "after" should be comments.
            for token in &tokens[..tokens.len() - 1] {
                assert!(token.is_comment());
            }
        }

        /// An unterminated comment marks all remaining tokens as comments.
        #[test]
        fn test_unterminated_comment() {
            let text = "before /* unterminated token";
            let tokens = tokenize_text(text);
            assert!(!tokens[0].is_comment());
            assert!(tokens[1].is_comment());
            assert!(tokens[2].is_comment());
        }

        /// A multiline comment marks tokens on all spanned lines as comments.
        #[test]
        fn test_multiline_comment() {
            let text = "/* comment\nstill comment */ after";
            let tokens = tokenize_text(text);
            let after = tokens.last().unwrap();
            assert_eq!(after.text(text), "after");
            assert!(!after.is_comment());
        }

        /// Token positions are correctly forwarded from the lexeme.
        #[test]
        fn test_token_position() {
            let text = "create_land";
            let tokens = tokenize_text(text);
            assert_eq!(tokens[0].text_start(), 0);
            assert_eq!(tokens[0].text_end(), 11);
            assert_eq!(tokens[0].lineno(), 0);
            assert_eq!(tokens[0].line_start(), 0);
            assert_eq!(tokens[0].line_end(), 11);
        }

        /// Token text is correctly retrieved from the source.
        #[test]
        fn test_token_text() {
            let text = "terrain_type GRASS";
            let tokens = tokenize_text(text);
            assert_eq!(tokens[0].text(text), "terrain_type");
            assert_eq!(tokens[1].text(text), "GRASS");
        }

        /// `/*` not surrounded by whitespace is not treated as a comment delimiter.
        #[test]
        fn test_comment_open_no_whitespace_not_comment() {
            let text = "/*comment*/";
            let tokens = tokenize_text(text);
            assert_eq!(tokens.len(), 1);
            assert!(!tokens[0].is_comment());
        }
    }
}
