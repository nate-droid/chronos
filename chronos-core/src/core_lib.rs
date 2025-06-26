//! Core Library (Genesis Axioms) for C∀O (Kao)
//!
//! This module defines the fundamental axioms and core words that form the foundation
//! of the C∀O language. These are the minimal, provably consistent set of operations
//! that all C∀O environments start with.

use crate::types::{OrdinalValue, Type, TypeSignature, WordDefinition};
use std::collections::HashMap;

/// The Genesis Axioms - core definitions that bootstrap the C∀O system
pub struct CoreLibrary {
    /// Core word definitions
    core_words: HashMap<String, WordDefinition>,
    /// Core type signatures
    core_signatures: HashMap<String, TypeSignature>,
}

impl CoreLibrary {
    /// Create the core library with Genesis Axioms
    pub fn new() -> Self {
        let mut lib = CoreLibrary {
            core_words: HashMap::new(),
            core_signatures: HashMap::new(),
        };

        lib.define_genesis_axioms();
        lib
    }

    /// Define the Genesis Axioms that bootstrap the system
    fn define_genesis_axioms(&mut self) {
        // Stack manipulation primitives
        self.define_stack_words();

        // Arithmetic primitives
        self.define_arithmetic_words();

        // Logical primitives
        self.define_logical_words();

        // Control flow primitives
        self.define_control_words();

        // Type system primitives
        self.define_type_words();

        // System primitives
        self.define_system_words();
    }

    /// Define stack manipulation words
    fn define_stack_words(&mut self) {
        // dup: ( a -> a a ) - Duplicate top stack element
        self.define_builtin(
            "dup",
            vec![Type::Variable("a".to_string())],
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("a".to_string()),
            ],
            "Duplicate the top stack element",
        );

        // drop: ( a -> ) - Remove top stack element
        self.define_builtin(
            "drop",
            vec![Type::Variable("a".to_string())],
            vec![],
            "Remove the top stack element",
        );

        // swap: ( a b -> b a ) - Exchange top two elements
        self.define_builtin(
            "swap",
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("b".to_string()),
            ],
            vec![
                Type::Variable("b".to_string()),
                Type::Variable("a".to_string()),
            ],
            "Exchange the top two stack elements",
        );

        // over: ( a b -> a b a ) - Copy second element to top
        self.define_builtin(
            "over",
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("b".to_string()),
            ],
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("b".to_string()),
                Type::Variable("a".to_string()),
            ],
            "Copy the second stack element to the top",
        );

        // rot: ( a b c -> b c a ) - Rotate three elements
        self.define_builtin(
            "rot",
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("b".to_string()),
                Type::Variable("c".to_string()),
            ],
            vec![
                Type::Variable("b".to_string()),
                Type::Variable("c".to_string()),
                Type::Variable("a".to_string()),
            ],
            "Rotate the top three stack elements",
        );

        // nip: ( a b -> b ) - Remove second element
        self.define_builtin(
            "nip",
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("b".to_string()),
            ],
            vec![Type::Variable("b".to_string())],
            "Remove the second stack element",
        );

        // tuck: ( a b -> b a b ) - Copy top below second
        self.define_builtin(
            "tuck",
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("b".to_string()),
            ],
            vec![
                Type::Variable("b".to_string()),
                Type::Variable("a".to_string()),
                Type::Variable("b".to_string()),
            ],
            "Copy the top element below the second",
        );
    }

    /// Define arithmetic words for natural numbers
    fn define_arithmetic_words(&mut self) {
        // +: ( Nat Nat -> Nat ) - Addition
        self.define_builtin(
            "+",
            vec![Type::Nat, Type::Nat],
            vec![Type::Nat],
            "Add two natural numbers",
        );

        // -: ( Nat Nat -> Nat ) - Subtraction (with underflow protection)
        self.define_builtin(
            "-",
            vec![Type::Nat, Type::Nat],
            vec![Type::Nat],
            "Subtract two natural numbers (result ≥ 0)",
        );

        // *: ( Nat Nat -> Nat ) - Multiplication
        self.define_builtin(
            "*",
            vec![Type::Nat, Type::Nat],
            vec![Type::Nat],
            "Multiply two natural numbers",
        );

        // /: ( Nat Nat -> Nat ) - Division
        self.define_builtin(
            "/",
            vec![Type::Nat, Type::Nat],
            vec![Type::Nat],
            "Divide two natural numbers (integer division)",
        );

        // mod: ( Nat Nat -> Nat ) - Modulo
        self.define_builtin(
            "mod",
            vec![Type::Nat, Type::Nat],
            vec![Type::Nat],
            "Modulo operation",
        );

        // 1+: ( Nat -> Nat ) - Increment
        self.define_builtin(
            "1+",
            vec![Type::Nat],
            vec![Type::Nat],
            "Increment a natural number",
        );

        // 1-: ( Nat -> Nat ) - Decrement
        self.define_builtin(
            "1-",
            vec![Type::Nat],
            vec![Type::Nat],
            "Decrement a natural number (minimum 0)",
        );
    }

    /// Define logical and comparison words
    fn define_logical_words(&mut self) {
        // =: ( a a -> Bool ) - Equality test
        self.define_builtin(
            "=",
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("a".to_string()),
            ],
            vec![Type::Bool],
            "Test equality of two values",
        );

        // <>: ( a a -> Bool ) - Inequality test
        self.define_builtin(
            "<>",
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("a".to_string()),
            ],
            vec![Type::Bool],
            "Test inequality of two values",
        );

        // <: ( Nat Nat -> Bool ) - Less than
        self.define_builtin(
            "<",
            vec![Type::Nat, Type::Nat],
            vec![Type::Bool],
            "Test if first number is less than second",
        );

        // >: ( Nat Nat -> Bool ) - Greater than
        self.define_builtin(
            ">",
            vec![Type::Nat, Type::Nat],
            vec![Type::Bool],
            "Test if first number is greater than second",
        );

        // <=: ( Nat Nat -> Bool ) - Less than or equal
        self.define_builtin(
            "<=",
            vec![Type::Nat, Type::Nat],
            vec![Type::Bool],
            "Test if first number is less than or equal to second",
        );

        // >=: ( Nat Nat -> Bool ) - Greater than or equal
        self.define_builtin(
            ">=",
            vec![Type::Nat, Type::Nat],
            vec![Type::Bool],
            "Test if first number is greater than or equal to second",
        );

        // not: ( Bool -> Bool ) - Logical negation
        self.define_builtin(
            "not",
            vec![Type::Bool],
            vec![Type::Bool],
            "Logical negation",
        );

        // and: ( Bool Bool -> Bool ) - Logical AND
        self.define_builtin(
            "and",
            vec![Type::Bool, Type::Bool],
            vec![Type::Bool],
            "Logical AND operation",
        );

        // or: ( Bool Bool -> Bool ) - Logical OR
        self.define_builtin(
            "or",
            vec![Type::Bool, Type::Bool],
            vec![Type::Bool],
            "Logical OR operation",
        );
    }

    /// Define control flow words
    fn define_control_words(&mut self) {
        // if: ( Bool Quote Quote -> ) - Conditional execution
        self.define_builtin(
            "if",
            vec![Type::Bool, Type::Quote, Type::Quote],
            vec![],
            "Execute first quote if true, second if false",
        );

        // when: ( Bool Quote -> ) - Execute quote if true
        self.define_builtin(
            "when",
            vec![Type::Bool, Type::Quote],
            vec![],
            "Execute quote if condition is true",
        );

        // unless: ( Bool Quote -> ) - Execute quote if false
        self.define_builtin(
            "unless",
            vec![Type::Bool, Type::Quote],
            vec![],
            "Execute quote if condition is false",
        );

        // times: ( Nat Quote -> ) - Execute quote n times
        self.define_builtin(
            "times",
            vec![Type::Nat, Type::Quote],
            vec![],
            "Execute quote the specified number of times",
        );

        // call: ( Quote -> ) - Execute a quote
        self.define_builtin(
            "call",
            vec![Type::Quote],
            vec![],
            "Execute the quote on top of the stack",
        );
    }

    /// Define type system words
    fn define_type_words(&mut self) {
        // type-of: ( a -> Type ) - Get type of value
        self.define_builtin(
            "type-of",
            vec![Type::Variable("a".to_string())],
            vec![Type::Variable("Type".to_string())],
            "Get the type of a value",
        );

        // is-type?: ( a Type -> Bool ) - Type check
        self.define_builtin(
            "is-type?",
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("Type".to_string()),
            ],
            vec![Type::Bool],
            "Check if value is of specified type",
        );

        // cast: ( a Type -> b ) - Type cast (unsafe)
        self.define_builtin(
            "cast",
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("Type".to_string()),
            ],
            vec![Type::Variable("b".to_string())],
            "Cast value to specified type (unsafe operation)",
        );
    }

    /// Define system and meta-programming words
    fn define_system_words(&mut self) {
        // .: ( a -> ) - Print value
        self.define_builtin(
            ".",
            vec![Type::Variable("a".to_string())],
            vec![],
            "Print the top stack value",
        );

        // .s: ( -> ) - Print stack contents
        self.define_builtin(".s", vec![], vec![], "Print the entire stack contents");

        // words: ( -> ) - List all defined words
        self.define_builtin(
            "words",
            vec![],
            vec![],
            "List all defined words in the dictionary",
        );

        // see: ( Word -> ) - Show definition of word
        self.define_builtin(
            "see",
            vec![Type::Variable("Word".to_string())],
            vec![],
            "Show the definition of a word",
        );

        // --ordinal: ( Quote -> Ordinal ) - Calculate ordinal cost
        self.define_builtin(
            "--ordinal",
            vec![Type::Quote],
            vec![Type::Ordinal],
            "Calculate the ordinal cost of a quote",
        );

        // submit-to-hypervisor: ( Quote -> ) - Submit definition globally
        self.define_builtin(
            "submit-to-hypervisor",
            vec![Type::Quote],
            vec![],
            "Submit a definition to the global hypervisor",
        );

        // clear: ( ... -> ) - Clear the stack
        self.define_builtin("clear", vec![], vec![], "Clear all values from the stack");

        // depth: ( -> Nat ) - Get stack depth
        self.define_builtin(
            "depth",
            vec![],
            vec![Type::Nat],
            "Get the current stack depth",
        );

        // help: ( -> ) - Show help information
        self.define_builtin("help", vec![], vec![], "Show help information");
    }

    /// Helper to define a builtin word with signature
    fn define_builtin(&mut self, name: &str, inputs: Vec<Type>, outputs: Vec<Type>, _doc: &str) {
        let signature = TypeSignature { inputs, outputs };

        let word_def = WordDefinition {
            name: name.to_string(),
            signature: signature.clone(),
            body: vec![],   // Builtins have no body - implemented in VM
            is_axiom: true, // Core library words are axiomatic
            ordinal_cost: OrdinalValue::Finite(1), // Base cost for primitives
        };

        self.core_signatures.insert(name.to_string(), signature);
        self.core_words.insert(name.to_string(), word_def);
    }

    /// Get all core word definitions
    pub fn get_core_words(&self) -> &HashMap<String, WordDefinition> {
        &self.core_words
    }

    /// Get all core type signatures
    pub fn get_core_signatures(&self) -> &HashMap<String, TypeSignature> {
        &self.core_signatures
    }

    /// Get signature for a core word
    pub fn get_signature(&self, word: &str) -> Option<&TypeSignature> {
        self.core_signatures.get(word)
    }

    /// Check if a word is a core builtin
    pub fn is_builtin(&self, word: &str) -> bool {
        self.core_words.contains_key(word)
    }

    /// Get documentation for core words
    pub fn get_word_docs(&self) -> HashMap<String, String> {
        let mut docs = HashMap::new();

        // Stack manipulation
        docs.insert(
            "dup".to_string(),
            "( a -> a a ) Duplicate the top stack element".to_string(),
        );
        docs.insert(
            "drop".to_string(),
            "( a -> ) Remove the top stack element".to_string(),
        );
        docs.insert(
            "swap".to_string(),
            "( a b -> b a ) Exchange the top two stack elements".to_string(),
        );
        docs.insert(
            "over".to_string(),
            "( a b -> a b a ) Copy the second element to the top".to_string(),
        );
        docs.insert(
            "rot".to_string(),
            "( a b c -> b c a ) Rotate the top three elements".to_string(),
        );

        // Arithmetic
        docs.insert(
            "+".to_string(),
            "( Nat Nat -> Nat ) Add two natural numbers".to_string(),
        );
        docs.insert(
            "-".to_string(),
            "( Nat Nat -> Nat ) Subtract two natural numbers".to_string(),
        );
        docs.insert(
            "*".to_string(),
            "( Nat Nat -> Nat ) Multiply two natural numbers".to_string(),
        );
        docs.insert(
            "/".to_string(),
            "( Nat Nat -> Nat ) Divide two natural numbers".to_string(),
        );

        // Logic
        docs.insert(
            "=".to_string(),
            "( a a -> Bool ) Test equality of two values".to_string(),
        );
        docs.insert(
            "<".to_string(),
            "( Nat Nat -> Bool ) Test if first < second".to_string(),
        );
        docs.insert(
            ">".to_string(),
            "( Nat Nat -> Bool ) Test if first > second".to_string(),
        );
        docs.insert(
            "not".to_string(),
            "( Bool -> Bool ) Logical negation".to_string(),
        );

        // Control flow
        docs.insert(
            "if".to_string(),
            "( Bool Quote Quote -> ) Conditional execution".to_string(),
        );
        docs.insert(
            "when".to_string(),
            "( Bool Quote -> ) Execute quote if true".to_string(),
        );
        docs.insert(
            "times".to_string(),
            "( Nat Quote -> ) Execute quote n times".to_string(),
        );

        // System
        docs.insert(
            ".".to_string(),
            "( a -> ) Print the top stack value".to_string(),
        );
        docs.insert(
            ".s".to_string(),
            "( -> ) Print the entire stack contents".to_string(),
        );
        docs.insert(
            "words".to_string(),
            "( -> ) List all defined words".to_string(),
        );
        docs.insert(
            "--ordinal".to_string(),
            "( Quote -> Ordinal ) Calculate ordinal cost".to_string(),
        );

        docs
    }

    /// Display help for all core words
    pub fn show_help(&self) -> String {
        let mut help = String::new();
        help.push_str("C∀O Core Library (Genesis Axioms)\n");
        help.push_str("==================================\n\n");

        help.push_str("Stack Manipulation:\n");
        help.push_str("  dup     ( a -> a a )        Duplicate top element\n");
        help.push_str("  drop    ( a -> )            Remove top element\n");
        help.push_str("  swap    ( a b -> b a )      Exchange top two\n");
        help.push_str("  over    ( a b -> a b a )    Copy second to top\n");
        help.push_str("  rot     ( a b c -> b c a )  Rotate three elements\n\n");

        help.push_str("Arithmetic:\n");
        help.push_str("  +       ( Nat Nat -> Nat )  Addition\n");
        help.push_str("  -       ( Nat Nat -> Nat )  Subtraction\n");
        help.push_str("  *       ( Nat Nat -> Nat )  Multiplication\n");
        help.push_str("  /       ( Nat Nat -> Nat )  Division\n\n");

        help.push_str("Logic & Comparison:\n");
        help.push_str("  =       ( a a -> Bool )     Equality test\n");
        help.push_str("  <       ( Nat Nat -> Bool ) Less than\n");
        help.push_str("  >       ( Nat Nat -> Bool ) Greater than\n");
        help.push_str("  not     ( Bool -> Bool )    Logical negation\n\n");

        help.push_str("Control Flow:\n");
        help.push_str("  if      ( Bool Quote Quote -> ) Conditional\n");
        help.push_str("  when    ( Bool Quote -> )   Execute if true\n");
        help.push_str("  times   ( Nat Quote -> )    Execute n times\n\n");

        help.push_str("System:\n");
        help.push_str("  .       ( a -> )            Print value\n");
        help.push_str("  .s      ( -> )              Print stack\n");
        help.push_str("  words   ( -> )              List words\n");
        help.push_str("  help    ( -> )              Show this help\n");
        help.push_str("  --ordinal ( Quote -> Ordinal ) Ordinal cost\n");

        help
    }
}

impl Default for CoreLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_library_creation() {
        let core_lib = CoreLibrary::new();

        // Check that essential words are defined
        assert!(core_lib.is_builtin("dup"));
        assert!(core_lib.is_builtin("+"));
        assert!(core_lib.is_builtin("if"));
        assert!(core_lib.is_builtin("."));

        // Check type signatures exist
        assert!(core_lib.get_signature("dup").is_some());
        assert!(core_lib.get_signature("+").is_some());
    }

    #[test]
    fn test_arithmetic_signatures() {
        let core_lib = CoreLibrary::new();

        let add_sig = core_lib.get_signature("+").unwrap();
        assert_eq!(add_sig.inputs.len(), 2);
        assert_eq!(add_sig.outputs.len(), 1);
        assert_eq!(add_sig.inputs[0], Type::Nat);
        assert_eq!(add_sig.inputs[1], Type::Nat);
        assert_eq!(add_sig.outputs[0], Type::Nat);
    }

    #[test]
    fn test_help_generation() {
        let core_lib = CoreLibrary::new();
        let help = core_lib.show_help();

        assert!(help.contains("Genesis Axioms"));
        assert!(help.contains("Stack Manipulation"));
        assert!(help.contains("Arithmetic"));
        assert!(help.contains("dup"));
        assert!(help.contains("+"));
    }
}
