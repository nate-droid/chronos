//! Parser for C∀O (Kao) source code
//!
//! This module handles parsing of tokenized C∀O code into executable structures.
//! It supports the concatenative syntax with postfix notation.

use chronos_core::lexer::LexError;

use crate::type_inference::TypeInferer;
use chronos_core::{
    OrdinalValue, Token, Type, TypeDefinition, TypeSignature, Value, WordDefinition, Lexer
};
use std::collections::HashMap;
use std::fmt;

/// Errors that can occur during parsing
#[derive(Debug, Clone)]
pub enum ParseError {
    /// Lexical error during tokenization
    LexError(LexError),
    /// Unexpected token
    UnexpectedToken(Token, String),
    /// Unexpected end of input
    UnexpectedEof(String),
    /// Missing token
    MissingToken(String),
    /// Invalid type signature
    InvalidTypeSignature(String),
    /// Invalid word definition
    InvalidWordDefinition(String),
    /// Invalid type definition
    InvalidTypeDefinition(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::LexError(e) => write!(f, "Lexical error: {}", e),
            ParseError::UnexpectedToken(token, expected) => {
                write!(f, "Unexpected token '{}', expected {}", token, expected)
            }
            ParseError::UnexpectedEof(expected) => {
                write!(f, "Unexpected end of input, expected {}", expected)
            }
            ParseError::MissingToken(expected) => {
                write!(f, "Missing {}", expected)
            }
            ParseError::InvalidTypeSignature(msg) => {
                write!(f, "Invalid type signature: {}", msg)
            }
            ParseError::InvalidWordDefinition(msg) => {
                write!(f, "Invalid word definition: {}", msg)
            }
            ParseError::InvalidTypeDefinition(msg) => {
                write!(f, "Invalid type definition: {}", msg)
            }
        }
    }
}

impl std::error::Error for ParseError {}

impl From<LexError> for ParseError {
    fn from(error: LexError) -> Self {
        ParseError::LexError(error)
    }
}

/// Parsed C∀O statements
#[derive(Debug, Clone)]
pub enum Statement {
    /// A sequence of tokens to execute
    Expression(Vec<Token>),
    /// A type signature declaration
    TypeSignatureDecl {
        name: String,
        signature: TypeSignature,
    },
    /// A word definition
    WordDefinition(WordDefinition),
    /// A type definition
    TypeDefinition(TypeDefinition),
    /// An axiom declaration
    AxiomDeclaration {
        name: String,
        signature: TypeSignature,
    },
}

/// The parser for C∀O source code
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    type_inferer: TypeInferer,
}

impl Parser {
    /// Create a new parser from source code
    pub fn new(input: &str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        Ok(Parser {
            tokens,
            position: 0,
            type_inferer: TypeInferer::new(),
        })
    }

    /// Get the current token without advancing
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Advance to the next token
    fn advance(&mut self) -> Option<&Token> {
        self.position += 1;
        self.current_token()
    }

    /// Parse a type from tokens
    fn parse_type(&mut self) -> Result<Type, ParseError> {
        match self.current_token() {
            Some(Token::Word(name)) => {
                let ty = match name.as_str() {
                    "Unit" => Type::Unit,
                    "Bool" => Type::Bool,
                    "Nat" => Type::Nat,
                    "Ordinal" => Type::Ordinal,
                    "Quote" => Type::Quote,
                    _ => Type::Variable(name.clone()),
                };
                self.advance();
                Ok(ty)
            }
            Some(token) => Err(ParseError::UnexpectedToken(
                token.clone(),
                "type name".to_string(),
            )),
            None => Err(ParseError::UnexpectedEof("type name".to_string())),
        }
    }

    /// Parse a type signature like "( Nat Nat -> Point )"
    fn parse_type_signature(&mut self) -> Result<TypeSignature, ParseError> {
        // Expect opening parenthesis
        match self.current_token() {
            Some(Token::Word(w)) if w == "(" => {
                self.advance();
            }
            _ => {
                return Err(ParseError::MissingToken(
                    "'(' for type signature".to_string(),
                ));
            }
        }

        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        let mut found_arrow = false;

        while let Some(token) = self.current_token() {
            match token {
                Token::Word(w) if w == ")" => {
                    self.advance();
                    break;
                }
                Token::Word(w) if w == "->" => {
                    self.advance();
                    found_arrow = true;
                }
                _ => {
                    let ty = self.parse_type()?;
                    if found_arrow {
                        outputs.push(ty);
                    } else {
                        inputs.push(ty);
                    }
                }
            }
        }

        Ok(TypeSignature { inputs, outputs })
    }

    /// Parse a quotation (code block)
    fn parse_quotation(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut quote_tokens = Vec::new();
        let mut depth = 1;

        // Skip the opening '['
        self.advance();

        while let Some(token) = self.current_token() {
            match token {
                Token::QuoteStart => {
                    depth += 1;
                    quote_tokens.push(token.clone());
                    self.advance();
                }
                Token::QuoteEnd => {
                    depth -= 1;
                    if depth == 0 {
                        self.advance();
                        break;
                    } else {
                        quote_tokens.push(token.clone());
                        self.advance();
                    }
                }
                _ => {
                    quote_tokens.push(token.clone());
                    self.advance();
                }
            }
        }

        if depth > 0 {
            return Err(ParseError::UnexpectedEof(
                "']' to close quotation".to_string(),
            ));
        }

        Ok(quote_tokens)
    }

    /// Parse a word definition: ": name body ;"
    fn parse_word_definition(&mut self) -> Result<WordDefinition, ParseError> {
        // Skip the ':'
        self.advance();

        // Get the word name
        let name = match self.current_token() {
            Some(Token::Word(name)) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => return Err(ParseError::MissingToken("word name".to_string())),
        };

        // Parse the body until ';'
        let mut body = Vec::new();
        while let Some(token) = self.current_token() {
            match token {
                Token::Word(w) if w == ";" => {
                    self.advance();
                    break;
                }
                Token::QuoteStart => {
                    let quote_tokens = self.parse_quotation()?;
                    body.push(Token::Literal(Value::Quote(quote_tokens)));
                }
                _ => {
                    body.push(token.clone());
                    self.advance();
                }
            }
        }

        // Try to infer the type signature from the body
        let signature = match self.type_inferer.infer_word_type(&body) {
            Ok(inferred_sig) => inferred_sig,
            Err(_) => {
                // If inference fails, use empty signature (will need explicit type later)
                TypeSignature {
                    inputs: vec![],
                    outputs: vec![],
                }
            }
        };

        Ok(WordDefinition {
            name,
            signature,
            body,
            is_axiom: false,
            ordinal_cost: OrdinalValue::Finite(1), // Default cost
        })
    }

    /// Parse a type signature declaration: ":: name ( types )"
    fn parse_type_signature_decl(&mut self) -> Result<(String, TypeSignature), ParseError> {
        // Skip the '::'
        self.advance();

        // Get the word name
        let name = match self.current_token() {
            Some(Token::Word(name)) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => return Err(ParseError::MissingToken("word name after '::'".to_string())),
        };

        // Check if there's a type signature following
        let signature = if let Some(Token::Word(w)) = self.current_token() {
            if w == "(" {
                self.parse_type_signature()?
            } else {
                // No type signature provided, use empty signature
                TypeSignature {
                    inputs: vec![],
                    outputs: vec![],
                }
            }
        } else {
            // No more tokens, use empty signature
            TypeSignature {
                inputs: vec![],
                outputs: vec![],
            }
        };

        // Expect and consume the trailing semicolon
        match self.current_token() {
            Some(Token::Word(w)) if w == ";" => {
                self.advance();
            }
            _ => {
                return Err(ParseError::MissingToken(
                    "';' after type signature".to_string(),
                ));
            }
        }

        Ok((name, signature))
    }

    /// Parse a type definition: "type Name { field1::Type1, field2::Type2 }"
    fn parse_type_definition(&mut self) -> Result<TypeDefinition, ParseError> {
        // Skip 'type'
        self.advance();

        // Get the type name
        let name = match self.current_token() {
            Some(Token::Word(name)) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => return Err(ParseError::MissingToken("type name".to_string())),
        };

        // Expect '{'
        match self.current_token() {
            Some(Token::Word(w)) if w == "{" => {
                self.advance();
            }
            _ => return Err(ParseError::MissingToken("'{' after type name".to_string())),
        }

        let mut fields = HashMap::new();
        let mut field_types = Vec::new();

        // Parse fields
        while let Some(token) = self.current_token() {
            match token {
                Token::Word(w) if w == "}" => {
                    self.advance();
                    break;
                }
                Token::Word(field_name) => {
                    let field_name = field_name.clone();
                    self.advance();

                    // Expect '::'
                    match self.current_token() {
                        Some(Token::Word(w)) if w == "::" => {
                            self.advance();
                        }
                        _ => {
                            return Err(ParseError::MissingToken(
                                "'::' after field name".to_string(),
                            ));
                        }
                    }

                    let field_type = self.parse_type()?;
                    field_types.push(field_type.clone());
                    fields.insert(field_name, field_type);

                    // Skip optional comma
                    if let Some(Token::Word(w)) = self.current_token() {
                        if w == "," {
                            self.advance();
                        }
                    }
                }
                _ => {
                    return Err(ParseError::UnexpectedToken(
                        token.clone(),
                        "field name or '}'".to_string(),
                    ));
                }
            }
        }

        // Constructor signature: field types -> composite type
        let constructor_signature = TypeSignature {
            inputs: field_types,
            outputs: vec![Type::Composite {
                name: name.clone(),
                fields: fields.clone(),
            }],
        };

        Ok(TypeDefinition {
            name,
            fields,
            constructor_signature,
        })
    }

    /// Parse an axiom declaration: "axiom name"
    fn parse_axiom_declaration(&mut self) -> Result<(String, TypeSignature), ParseError> {
        // Skip 'axiom'
        self.advance();

        // Get the axiom name
        let name = match self.current_token() {
            Some(Token::Word(name)) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => return Err(ParseError::MissingToken("axiom name".to_string())),
        };

        // For now, assume axioms have empty signatures
        // In practice, they would need to have been declared with :: first
        let signature = TypeSignature {
            inputs: vec![],
            outputs: vec![],
        };

        Ok((name, signature))
    }

    /// Parse a single statement
    pub fn parse_statement(&mut self) -> Result<Option<Statement>, ParseError> {
        match self.current_token() {
            None => Ok(None),
            Some(Token::Word(w)) => {
                match w.as_str() {
                    ":" => {
                        let word_def = self.parse_word_definition()?;
                        Ok(Some(Statement::WordDefinition(word_def)))
                    }
                    "::" => {
                        let (name, signature) = self.parse_type_signature_decl()?;
                        Ok(Some(Statement::TypeSignatureDecl { name, signature }))
                    }
                    "type" => {
                        let type_def = self.parse_type_definition()?;
                        Ok(Some(Statement::TypeDefinition(type_def)))
                    }
                    "axiom" => {
                        let (name, signature) = self.parse_axiom_declaration()?;
                        Ok(Some(Statement::AxiomDeclaration { name, signature }))
                    }
                    _ => {
                        // Parse as expression
                        let mut expr_tokens = Vec::new();
                        while let Some(token) = self.current_token() {
                            // Stop at statement keywords
                            if let Token::Word(w) = token {
                                if matches!(w.as_str(), ":" | "::" | "type" | "axiom") {
                                    break;
                                }
                            }

                            match token {
                                Token::QuoteStart => {
                                    let quote_tokens = self.parse_quotation()?;
                                    expr_tokens.push(Token::Literal(Value::Quote(quote_tokens)));
                                }
                                _ => {
                                    expr_tokens.push(token.clone());
                                    self.advance();
                                }
                            }
                        }

                        if !expr_tokens.is_empty() {
                            Ok(Some(Statement::Expression(expr_tokens)))
                        } else {
                            Ok(None)
                        }
                    }
                }
            }
            Some(_token) => {
                // Parse as expression
                let mut expr_tokens = Vec::new();
                while let Some(token) = self.current_token() {
                    match token {
                        Token::QuoteStart => {
                            let quote_tokens = self.parse_quotation()?;
                            expr_tokens.push(Token::Literal(Value::Quote(quote_tokens)));
                        }
                        _ => {
                            expr_tokens.push(token.clone());
                            self.advance();
                        }
                    }
                }

                if !expr_tokens.is_empty() {
                    Ok(Some(Statement::Expression(expr_tokens)))
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// Parse all statements in the input
    pub fn parse_all(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements = Vec::new();
        while let Some(statement) = self.parse_statement()? {
            statements.push(statement);
        }
        Ok(statements)
    }

    /// Add a known word signature to the type inferer
    pub fn add_word_signature(&mut self, name: String, signature: TypeSignature) {
        self.type_inferer.add_word_signature(name, signature);
    }

    /// Enable or disable type inference debugging
    pub fn set_type_debug(&mut self, debug: bool) {
        self.type_inferer.set_debug(debug);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression() {
        let mut parser = Parser::new("3 4 +").unwrap();
        let statements = parser.parse_all().unwrap();

        assert_eq!(statements.len(), 1);
        if let Statement::Expression(tokens) = &statements[0] {
            assert_eq!(tokens.len(), 3);
        } else {
            panic!("Expected expression statement");
        }
    }

    #[test]
    fn test_parse_word_definition() {
        let mut parser = Parser::new(": square dup * ;").unwrap();
        let statements = parser.parse_all().unwrap();

        assert_eq!(statements.len(), 1);
        if let Statement::WordDefinition(word_def) = &statements[0] {
            assert_eq!(word_def.name, "square");
            assert_eq!(word_def.body.len(), 2);
        } else {
            panic!("Expected word definition");
        }
    }

    #[test]
    fn test_parse_type_signature() {
        let mut parser = Parser::new(":: square ( Nat -> Nat ) ;").unwrap();
        let statements = parser.parse_all().unwrap();

        assert_eq!(statements.len(), 1);
        if let Statement::TypeSignatureDecl { name, signature } = &statements[0] {
            assert_eq!(name, "square");
            assert_eq!(signature.inputs.len(), 1);
            assert_eq!(signature.outputs.len(), 1);
        } else {
            panic!("Expected type signature declaration");
        }
    }

    #[test]
    fn test_parse_quote() {
        let mut parser = Parser::new("[ dup * ]").unwrap();
        let statements = parser.parse_all().unwrap();

        assert_eq!(statements.len(), 1);
        if let Statement::Expression(tokens) = &statements[0] {
            assert_eq!(tokens.len(), 1);
            if let Token::Literal(Value::Quote(quote_tokens)) = &tokens[0] {
                assert_eq!(quote_tokens.len(), 2);
            } else {
                panic!("Expected quote literal");
            }
        } else {
            panic!("Expected expression statement");
        }
    }
}
