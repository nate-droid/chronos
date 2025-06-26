//! Lexer for C∀O (Kao) source code
//!
//! This module handles tokenization of C∀O source code, converting text into
//! a sequence of tokens that can be parsed and executed.

use crate::types::{Token, Value};
use std::fmt;

/// Errors that can occur during lexical analysis
#[derive(Debug, Clone)]
pub enum LexError {
    /// Invalid character encountered
    InvalidCharacter(char, usize),
    /// Unterminated comment
    UnterminatedComment(usize),
    /// Invalid number format
    InvalidNumber(String, usize),
    /// Unexpected end of input
    UnexpectedEof,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::InvalidCharacter(ch, pos) => {
                write!(f, "Invalid character '{}' at position {}", ch, pos)
            }
            LexError::UnterminatedComment(pos) => {
                write!(f, "Unterminated comment starting at position {}", pos)
            }
            LexError::InvalidNumber(s, pos) => {
                write!(f, "Invalid number '{}' at position {}", s, pos)
            }
            LexError::UnexpectedEof => {
                write!(f, "Unexpected end of input")
            }
        }
    }
}

impl std::error::Error for LexError {}

/// The lexer for C∀O source code
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given input string
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            current_char: None,
        };
        lexer.current_char = lexer.input.chars().next();
        lexer
    }

    /// Advance to the next character
    fn advance(&mut self) {
        self.position += 1;
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = self.input.chars().nth(self.position);
        }
    }

    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Parse a comment (enclosed in parentheses)
    fn parse_comment(&mut self) -> Result<Token, LexError> {
        let start_pos = self.position;
        self.advance(); // skip opening '('

        let mut comment_text = String::new();
        let mut depth = 1;

        while let Some(ch) = self.current_char {
            match ch {
                '(' => {
                    depth += 1;
                    comment_text.push(ch);
                    self.advance();
                }
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        self.advance(); // skip closing ')'
                        return Ok(Token::Comment(comment_text.trim().to_string()));
                    } else {
                        comment_text.push(ch);
                        self.advance();
                    }
                }
                _ => {
                    comment_text.push(ch);
                    self.advance();
                }
            }
        }

        Err(LexError::UnterminatedComment(start_pos))
    }

    /// Parse a number (natural number)
    fn parse_number(&mut self) -> Result<Token, LexError> {
        let start_pos = self.position;
        let mut number_str = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                number_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        match number_str.parse::<u64>() {
            Ok(n) => Ok(Token::Literal(Value::Nat(n))),
            Err(_) => Err(LexError::InvalidNumber(number_str, start_pos)),
        }
    }

    /// Heuristic to determine if parentheses contain a comment
    fn looks_like_comment(&self) -> bool {
        let mut pos = self.position + 1; // Skip opening paren
        let mut depth = 1;
        let mut content = String::new();

        // Look ahead to see what's inside the parentheses
        while pos < self.input.len() && depth > 0 {
            if let Some(ch) = self.input.chars().nth(pos) {
                match ch {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth > 0 {
                            content.push(ch);
                        }
                    }
                    _ => content.push(ch),
                }
                pos += 1;
            } else {
                break;
            }
        }

        let content = content.trim();

        // Type signatures contain arrows (->) and type names (usually capitalized)
        // Comments contain prose (multiple words, articles, etc.)
        if content.contains("->") {
            false // Likely a type signature
        } else if content.split_whitespace().count() >= 3 {
            // Multiple words suggest prose comment
            true
        } else {
            // Short content could be either - err on side of treating as non-comment
            false
        }
    }

    /// Parse a word (identifier) or symbol
    fn parse_word(&mut self) -> Token {
        let mut word = String::new();

        // Check if this is a multi-character operator first
        if let Some(ch) = self.current_char {
            if let Some(next_ch) = self.input.chars().nth(self.position + 1) {
                match (ch, next_ch) {
                    (':', ':') | ('<', '=') | ('>', '=') | ('<', '>') | ('-', '>') => {
                        word.push(ch);
                        word.push(next_ch);
                        self.advance();
                        self.advance();
                        return Token::Word(word);
                    }
                    _ => {}
                }
            }
        }

        // Parse single character or alphanumeric words
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' || ch == '?' || ch == '!' {
                word.push(ch);
                self.advance();
            } else if word.is_empty()
                && ch.is_ascii_punctuation()
                && ch != '['
                && ch != ']'
                && ch != '"'
            {
                // Single character symbols (including parentheses)
                word.push(ch);
                self.advance();
                break;
            } else {
                break;
            }
        }

        // Check for boolean literals
        match word.as_str() {
            "true" => Token::Literal(Value::Bool(true)),
            "false" => Token::Literal(Value::Bool(false)),
            _ => Token::Word(word),
        }
    }

    /// Parse a string literal (for system messages, etc.)
    fn parse_string(&mut self) -> Result<Token, LexError> {
        self.advance(); // skip opening quote
        let mut string_content = String::new();

        while let Some(ch) = self.current_char {
            match ch {
                '"' => {
                    self.advance(); // skip closing quote
                    // For now, treat strings as words for simplicity
                    // In a full implementation, we'd need a String type
                    return Ok(Token::Word(string_content));
                }
                '\\' => {
                    self.advance();
                    if let Some(escaped) = self.current_char {
                        match escaped {
                            'n' => string_content.push('\n'),
                            't' => string_content.push('\t'),
                            'r' => string_content.push('\r'),
                            '\\' => string_content.push('\\'),
                            '"' => string_content.push('"'),
                            _ => {
                                string_content.push('\\');
                                string_content.push(escaped);
                            }
                        }
                        self.advance();
                    }
                }
                _ => {
                    string_content.push(ch);
                    self.advance();
                }
            }
        }

        Err(LexError::UnexpectedEof)
    }

    /// Get the next token from the input
    pub fn next_token(&mut self) -> Result<Option<Token>, LexError> {
        self.skip_whitespace();

        match self.current_char {
            None => Ok(None),
            Some('(') => {
                // Heuristic to detect comments vs type signatures:
                // Comments typically contain prose and spaces
                // Type signatures contain structured content like: ( Nat -> Nat )
                if self.looks_like_comment() {
                    let token = self.parse_comment()?;
                    Ok(Some(token))
                } else {
                    // This is just a parenthesis token
                    let token = self.parse_word();
                    Ok(Some(token))
                }
            }
            Some('[') => {
                self.advance();
                Ok(Some(Token::QuoteStart))
            }
            Some(']') => {
                self.advance();
                Ok(Some(Token::QuoteEnd))
            }
            Some('"') => {
                let token = self.parse_string()?;
                Ok(Some(token))
            }
            Some(ch) if ch.is_ascii_digit() => {
                let token = self.parse_number()?;
                Ok(Some(token))
            }
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let token = self.parse_word();
                Ok(Some(token))
            }
            Some(ch) if ch.is_ascii_punctuation() && ch != '[' && ch != ']' && ch != '"' => {
                let token = self.parse_word();
                Ok(Some(token))
            }
            Some(ch) => Err(LexError::InvalidCharacter(ch, self.position)),
        }
    }

    /// Tokenize the entire input string
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token()? {
            // Skip comments for now (they could be preserved for documentation)
            if !matches!(token, Token::Comment(_)) {
                tokens.push(token);
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("3 4 + dup");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Literal(Value::Nat(3)));
        assert_eq!(tokens[1], Token::Literal(Value::Nat(4)));
        assert_eq!(tokens[2], Token::Word("+".to_string()));
        assert_eq!(tokens[3], Token::Word("dup".to_string()));
    }

    #[test]
    fn test_quotes() {
        let mut lexer = Lexer::new("[ dup * ]");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::QuoteStart);
        assert_eq!(tokens[1], Token::Word("dup".to_string()));
        assert_eq!(tokens[2], Token::Word("*".to_string()));
        assert_eq!(tokens[3], Token::QuoteEnd);
    }

    #[test]
    fn test_booleans() {
        let mut lexer = Lexer::new("true false");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::Literal(Value::Bool(true)));
        assert_eq!(tokens[1], Token::Literal(Value::Bool(false)));
    }

    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("3 ( this is a comment ) 4");
        let tokens = lexer.tokenize().unwrap();

        // Comments are filtered out in tokenize()
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::Literal(Value::Nat(3)));
        assert_eq!(tokens[1], Token::Literal(Value::Nat(4)));
    }

    #[test]
    fn test_nested_comments() {
        let mut lexer = Lexer::new("( outer ( inner ) comment )");
        let token = lexer.next_token().unwrap().unwrap();

        if let Token::Comment(text) = token {
            assert_eq!(text, "outer ( inner ) comment");
        } else {
            panic!("Expected comment token");
        }
    }
}
