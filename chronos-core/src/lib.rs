//! # Chronos Core
//!
//! The core language runtime for C∀O (Chronos), a concatenative programming language
//! built on categorical foundations with ordinal verification.
//!
//! This library provides the essential components for parsing, type checking, and
//! executing C∀O programs without any interactive or distributed system dependencies.
//!
//! ## Quick Start
//!
//! ```rust
//! use chronos_core::{ChronosCore, Value};
//!
//! let mut core = ChronosCore::new();
//! core.eval("3 4 +").unwrap();
//! assert_eq!(core.pop().unwrap(), Value::Nat(7));
//! ```
//!
//! ## Architecture
//!
//! The core consists of several key components:
//!
//! - **Lexer**: Tokenizes C∀O source code
//! - **Parser**: Generates AST from token streams
//! - **Type System**: Categorical type checking and inference
//! - **Virtual Machine**: Stack-based execution engine
//! - **Core Library**: Built-in operations and standard library
//!
//! ## Features
//!
//! - **Concatenative Programming**: Postfix notation with stack-based computation
//! - **Categorical Types**: Types as objects, functions as morphisms
//! - **Proof-Carrying Code**: Ordinal analysis for termination guarantees
//! - **Interactive Development**: REPL-friendly design
//! - **Extensible**: User-defined types and operations

// Re-export core types for public API
pub use types::{
    MatchArm, OrdinalValue, Pattern, Token, Type, TypeDefinition, TypeSignature, Value,
    WordDefinition,
};

pub use core_lib::CoreLibrary;
pub use error::{ChronosError, ErrorContext, Result};
pub use lexer::Lexer;
pub use vm::VirtualMachine;

// Internal modules
mod core_lib;
mod error;
mod lexer;
mod types;
mod vm;
// mod parser; // Temporarily disabled until type_inference is available

/// Main interface for the Chronos Core language runtime
///
/// This struct provides a high-level API for evaluating C∀O code,
/// managing state, and accessing language features.
pub struct ChronosCore {
    vm: VirtualMachine,
    core_lib: CoreLibrary,
}

impl ChronosCore {
    /// Create a new Chronos Core runtime
    ///
    /// This initializes the virtual machine with the core library
    /// and sets up the basic execution environment.
    pub fn new() -> Self {
        let mut vm = VirtualMachine::new();
        let core_lib = CoreLibrary::new();

        // Load core library into VM - VM already loads its own core library

        Self { vm, core_lib }
    }

    /// Evaluate a string of C∀O code and return the top stack value
    ///
    /// This is the main evaluation function that handles the complete
    /// pipeline from source code to result.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronos_core::{ChronosCore, Value};
    ///
    /// let mut core = ChronosCore::new();
    /// core.eval("3 4 +").unwrap();
    /// assert_eq!(core.pop().unwrap(), Value::Nat(7));
    /// ```
    pub fn eval(&mut self, source: &str) -> Result<Value> {
        let tokens = self.tokenize(source)?;
        self.execute_tokens(&tokens)?;

        // TODO: No longer popping a value from the stack on eval
        // that should only be explicitly happening with a pop or "." command
        
        // if self.vm.stack().is_empty() {
        //     return Ok(Value::Unit);
        // }
        Ok(Value::Unit)
        
    }

    /// Tokenize source code into a token stream
    pub fn tokenize(&self, source: &str) -> Result<Vec<Token>> {
        let mut lexer = Lexer::new(source);
        lexer
            .tokenize()
            .map_err(|e| ChronosError::lex_error(e.to_string(), None))
    }

    /// Parse source code into tokens with quote processing
    pub fn parse(&self, source: &str) -> Result<Vec<Token>> {
        let tokens = self.tokenize(source)?;
        self.process_quotes(tokens)
    }

    /// Process quote tokens and convert them to Quote values
    fn process_quotes(&self, tokens: Vec<Token>) -> Result<Vec<Token>> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            match &tokens[i] {
                Token::QuoteStart => {
                    // Find matching QuoteEnd
                    let quote_tokens = self.extract_quote(&tokens, i)?;
                    let quote_end_index = self.find_quote_end(&tokens, i)?;

                    // Create a Quote value from the tokens inside the quotes
                    result.push(Token::Literal(Value::Quote(quote_tokens)));

                    // Skip to after the QuoteEnd
                    i = quote_end_index + 1;
                }
                _ => {
                    result.push(tokens[i].clone());
                    i += 1;
                }
            }
        }

        Ok(result)
    }

    /// Extract tokens between QuoteStart and QuoteEnd
    fn extract_quote(&self, tokens: &[Token], start: usize) -> Result<Vec<Token>> {
        let mut quote_tokens = Vec::new();
        let mut depth = 0;
        let mut i = start;

        while i < tokens.len() {
            match &tokens[i] {
                Token::QuoteStart => {
                    depth += 1;
                    if depth > 1 {
                        // Nested quote - include the QuoteStart token
                        quote_tokens.push(tokens[i].clone());
                    }
                }
                Token::QuoteEnd => {
                    depth -= 1;
                    if depth == 0 {
                        // Found matching end quote
                        break;
                    } else {
                        // Nested quote - include the QuoteEnd token
                        quote_tokens.push(tokens[i].clone());
                    }
                }
                _ => {
                    if depth > 0 {
                        quote_tokens.push(tokens[i].clone());
                    }
                }
            }
            i += 1;
        }

        if depth > 0 {
            return Err(ChronosError::parse_error("Unclosed quote", None, None));
        }

        // Recursively process nested quotes
        self.process_quotes(quote_tokens)
    }

    /// Find the index of the matching QuoteEnd for a QuoteStart at the given index
    fn find_quote_end(&self, tokens: &[Token], start: usize) -> Result<usize> {
        let mut depth = 0;
        let mut i = start;

        while i < tokens.len() {
            match &tokens[i] {
                Token::QuoteStart => depth += 1,
                Token::QuoteEnd => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(i);
                    }
                }
                _ => {}
            }
            i += 1;
        }

        Err(ChronosError::parse_error("Unclosed quote", None, None))
    }

    /// Execute a sequence of tokens
    pub fn execute_tokens(&mut self, tokens: &[Token]) -> Result<()> {
        self.vm
            .execute_tokens(tokens)
            .map_err(|e| ChronosError::runtime_error(e.to_string(), None))
    }

    /// Execute source code with quote parsing
    pub fn eval_source(&mut self, source: &str) -> Result<()> {
        let tokens = self.parse(source)?;
        self.execute_tokens(&tokens)
    }

    /// Execute a single token
    pub fn execute_token(&mut self, token: &Token) -> Result<()> {
        self.vm
            .execute_token(token)
            .map_err(|e| ChronosError::runtime_error(e.to_string(), None))
    }

    /// Define a new word in the virtual machine
    pub fn define_word(&mut self, definition: WordDefinition) -> Result<()> {
        self.vm.define_word(definition);
        Ok(())
    }

    /// Get the current stack as a vector of values
    pub fn get_stack(&self) -> Vec<Value> {
        self.vm.stack().to_vec()
    }

    /// Get the stack depth
    pub fn stack_depth(&self) -> usize {
        self.vm.stack().len()
    }

    /// Push a value onto the stack
    pub fn push(&mut self, value: Value) {
        self.vm.push(value);
    }

    /// Pop a value from the stack
    pub fn pop(&mut self) -> Option<Value> {
        self.vm.pop().ok()
    }

    /// Clear the stack
    pub fn clear_stack(&mut self) {
        self.vm.clear_stack();
    }

    /// Get all defined words including user-defined words
    pub fn get_words(&self) -> Vec<String> {
        let mut words = vec![
            "dup".to_string(),
            "drop".to_string(),
            "swap".to_string(),
            "over".to_string(),
            "rot".to_string(),
            "+".to_string(),
            "-".to_string(),
            "*".to_string(),
            "/".to_string(),
            "=".to_string(),
            "<".to_string(),
            ">".to_string(),
            "if".to_string(),
            ".".to_string(),
            ".s".to_string(),
            "print".to_string(),
        ];

        // Add user-defined words
        words.extend(self.vm.get_user_words());
        words
    }

    /// Check if a word is defined
    pub fn is_word_defined(&self, name: &str) -> bool {
        // Check built-in words
        matches!(
            name,
            "dup"
                | "drop"
                | "swap"
                | "over"
                | "rot"
                | "+"
                | "-"
                | "*"
                | "/"
                | "="
                | "<"
                | ">"
                | "if"
                | "."
                | ".s"
                | "print"
                | "::"
                | ":"
        ) || self.vm.get_word_definition(name).is_some()
    }

    /// Reset the virtual machine to initial state
    pub fn reset(&mut self) {
        self.vm = VirtualMachine::new();
        // VM already loads its own core library
    }

    /// Get user-defined words from the VM
    pub fn get_user_words(&self) -> Vec<String> {
        self.vm.get_user_words()
    }

    /// Get documentation for a core word (simplified)
    pub fn get_word_doc(&self, _name: &str) -> Option<&str> {
        None // Simplified for now
    }
}

impl Default for ChronosCore {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a new virtual machine instance
///
/// This is a convenience function for creating a VM without the full
/// ChronosCore wrapper when you need more direct control.
pub fn create_vm() -> VirtualMachine {
    VirtualMachine::new()
}

/// Create a new core library instance
///
/// This provides access to the built-in operations and their documentation.
pub fn create_core_lib() -> CoreLibrary {
    CoreLibrary::new()
}

/// Parse a string into tokens
///
/// Convenience function for tokenizing without creating a full ChronosCore instance.
pub fn parse_tokens(source: &str) -> Result<Vec<Token>> {
    let mut lexer = Lexer::new(source);
    lexer
        .tokenize()
        .map_err(|e| ChronosError::lex_error(e.to_string(), None))
}

/// Traits for extending the core functionality

/// Trait for objects that can be evaluated in the Chronos runtime
pub trait Evaluable {
    /// Evaluate this object in the given virtual machine
    fn eval(&self, vm: &mut VirtualMachine) -> Result<()>;
}

impl Evaluable for Token {
    fn eval(&self, vm: &mut VirtualMachine) -> Result<()> {
        vm.execute_token(self)
            .map_err(|e| ChronosError::runtime_error(e.to_string(), None))
    }
}

impl Evaluable for Vec<Token> {
    fn eval(&self, vm: &mut VirtualMachine) -> Result<()> {
        vm.execute_tokens(self)
            .map_err(|e| ChronosError::runtime_error(e.to_string(), None))
    }
}

/// Trait for types that can be converted to C∀O values
pub trait IntoValue {
    /// Convert this object into a C∀O value
    fn into_value(self) -> Value;
}

impl IntoValue for bool {
    fn into_value(self) -> Value {
        Value::Bool(self)
    }
}

impl IntoValue for u64 {
    fn into_value(self) -> Value {
        Value::Nat(self)
    }
}

impl IntoValue for Vec<Token> {
    fn into_value(self) -> Value {
        Value::Quote(self)
    }
}

/// Trait for types that can be converted from C∀O values
pub trait FromValue: Sized {
    /// Try to convert a C∀O value into this type
    fn from_value(value: Value) -> Result<Self>;
}

impl FromValue for bool {
    fn from_value(value: Value) -> Result<Self> {
        match value {
            Value::Bool(b) => Ok(b),
            _ => Err(ChronosError::type_error(
                "Expected Bool",
                Some("Bool".to_string()),
                None,
            )),
        }
    }
}

impl FromValue for u64 {
    fn from_value(value: Value) -> Result<Self> {
        match value {
            Value::Nat(n) => Ok(n),
            _ => Err(ChronosError::type_error(
                "Expected Nat",
                Some("Nat".to_string()),
                None,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let mut core = ChronosCore::new();
        core.eval("3 4 +").unwrap();
        assert_eq!(core.pop().unwrap(), Value::Nat(7));
    }

    #[test]
    fn test_stack_operations() {
        let mut core = ChronosCore::new();

        // eval("1 2 3") pushes 1, 2, 3
        let _ = core.eval("1 2 3").unwrap();
        
        assert_eq!(core.stack_depth(), 3);

        let top = core.pop().unwrap();
        assert_eq!(top, Value::Nat(3));
        assert_eq!(core.stack_depth(), 2);
        assert_eq!(core.pop(), Some(Value::Nat(2)));
    }

    #[test]
    fn test_word_definition() {
        let mut core = ChronosCore::new();

        // This would require implementing word definition parsing
        // For now, test the basic infrastructure
        assert!(core.get_words().len() > 0); // Should have core words
    }

    #[test]
    fn test_error_handling() {
        let mut core = ChronosCore::new();

        // Test undefined word
        let result = core.eval("undefined-word");
        assert!(result.is_err());
    }

    #[test]
    fn test_tokenization() {
        let tokens = parse_tokens("3 4 +").unwrap();
        assert_eq!(tokens.len(), 3);

        if let Token::Literal(Value::Nat(n)) = &tokens[0] {
            assert_eq!(*n, 3);
        } else {
            panic!("Expected Nat literal");
        }
    }

    #[test]
    fn test_type_conversions() {
        let value: Value = true.into_value();
        assert_eq!(value, Value::Bool(true));

        let extracted: bool = bool::from_value(Value::Bool(false)).unwrap();
        assert_eq!(extracted, false);
    }
}
