//! REPL (Read-Eval-Print Loop) for C∀O (Kao)
//!
//! This module provides an interactive environment for C∀O development,
//! allowing users to define words, execute code, and explore the language.

use crate::core_lib::CoreLibrary;
use crate::ordinal::OrdinalVerifier;
use crate::parser::{ParseError, Parser, Statement};
use crate::types::{OrdinalValue, TypeDefinition, TypeSignature, WordDefinition};
use crate::vm::{VirtualMachine, VmError};
use std::collections::HashMap;
use std::fmt;

/// Errors that can occur in the REPL
#[derive(Debug, Clone)]
pub enum ReplError {
    /// Parse error
    ParseError(ParseError),
    /// VM execution error
    VmError(VmError),
    /// Type checking error
    TypeError(String),
    /// Definition error
    DefinitionError(String),
    /// I/O error
    IoError(String),
}

impl fmt::Display for ReplError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReplError::ParseError(e) => write!(f, "Parse error: {}", e),
            ReplError::VmError(e) => write!(f, "Runtime error: {}", e),
            ReplError::TypeError(e) => write!(f, "Type error: {}", e),
            ReplError::DefinitionError(e) => write!(f, "Definition error: {}", e),
            ReplError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for ReplError {}

impl From<ParseError> for ReplError {
    fn from(error: ParseError) -> Self {
        ReplError::ParseError(error)
    }
}

impl From<VmError> for ReplError {
    fn from(error: VmError) -> Self {
        ReplError::VmError(error)
    }
}

/// The REPL state and environment
pub struct Repl {
    /// The virtual machine
    vm: VirtualMachine,
    /// Core library
    core_lib: CoreLibrary,
    /// Ordinal verifier
    verifier: OrdinalVerifier,
    /// User-defined type definitions
    user_types: HashMap<String, TypeDefinition>,
    /// Pending type signatures (declared but not yet defined)
    pending_signatures: HashMap<String, TypeSignature>,
    /// Whether to show stack after each operation
    show_stack: bool,
    /// Whether to show ordinal costs
    show_ordinals: bool,
}

impl Repl {
    /// Create a new REPL environment
    pub fn new() -> Self {
        let mut repl = Repl {
            vm: VirtualMachine::new(),
            core_lib: CoreLibrary::new(),
            verifier: OrdinalVerifier::new(),
            user_types: HashMap::new(),
            pending_signatures: HashMap::new(),
            show_stack: false,
            show_ordinals: false,
        };

        // Load core library into VM
        for (name, word_def) in repl.core_lib.get_core_words() {
            repl.vm.define_word(word_def.clone());
        }

        repl
    }

    /// Evaluate a line of C∀O code
    pub fn eval(&mut self, input: &str) -> Result<(), ReplError> {
        let input = input.trim();

        // Handle special REPL commands
        if input.starts_with('.') {
            return self.handle_repl_command(input);
        }

        // Parse the input
        let mut parser = Parser::new(input)?;
        let statements = parser.parse_all()?;

        // Process each statement
        for statement in statements {
            self.eval_statement(statement)?;
        }

        // Show stack if enabled
        if self.show_stack && !self.vm.stack().is_empty() {
            self.show_stack_contents();
        }

        Ok(())
    }

    /// Evaluate a single statement
    fn eval_statement(&mut self, statement: Statement) -> Result<(), ReplError> {
        match statement {
            Statement::Expression(tokens) => {
                self.vm.execute_tokens(&tokens)?;
            }
            Statement::TypeSignatureDecl { name, signature } => {
                self.handle_type_signature_decl(name, signature)?;
            }
            Statement::WordDefinition(word_def) => {
                self.handle_word_definition(word_def)?;
            }
            Statement::TypeDefinition(type_def) => {
                self.handle_type_definition(type_def)?;
            }
            Statement::AxiomDeclaration { name, signature } => {
                self.handle_axiom_declaration(name, signature)?;
            }
        }
        Ok(())
    }

    /// Handle type signature declaration
    fn handle_type_signature_decl(
        &mut self,
        name: String,
        signature: TypeSignature,
    ) -> Result<(), ReplError> {
        // Store the signature for later use when the word is defined
        self.pending_signatures.insert(name.clone(), signature);
        println!("Type signature declared for '{}'", name);
        Ok(())
    }

    /// Handle word definition
    fn handle_word_definition(&mut self, mut word_def: WordDefinition) -> Result<(), ReplError> {
        // Check if we have a pending type signature
        if let Some(signature) = self.pending_signatures.remove(&word_def.name) {
            word_def.signature = signature;
        } else if word_def.signature.inputs.is_empty() && word_def.signature.outputs.is_empty() {
            return Err(ReplError::DefinitionError(format!(
                "Word '{}' needs a type signature. Use :: to declare it first.",
                word_def.name
            )));
        }

        // Verify termination using ordinal analysis
        match self.verifier.verify_termination(&word_def) {
            Ok(ordinal_cost) => {
                word_def.ordinal_cost = ordinal_cost.clone();
                if self.show_ordinals {
                    println!("Ordinal cost for '{}': {}", word_def.name, ordinal_cost);
                }
            }
            Err(e) => {
                return Err(ReplError::DefinitionError(format!(
                    "Ordinal verification failed for '{}': {}",
                    word_def.name, e
                )));
            }
        }

        // Define the word in the VM
        let name = word_def.name.clone();
        self.vm.define_word(word_def);
        println!("Defined word '{}'", name);
        Ok(())
    }

    /// Handle type definition
    fn handle_type_definition(&mut self, type_def: TypeDefinition) -> Result<(), ReplError> {
        let name = type_def.name.clone();
        self.user_types.insert(name.clone(), type_def);
        println!("Defined type '{}'", name);
        Ok(())
    }

    /// Handle axiom declaration
    fn handle_axiom_declaration(
        &mut self,
        name: String,
        signature: TypeSignature,
    ) -> Result<(), ReplError> {
        // Check if we have a pending signature that matches
        if let Some(pending_sig) = self.pending_signatures.remove(&name) {
            let axiom_def = WordDefinition {
                name: name.clone(),
                signature: pending_sig,
                body: vec![],
                is_axiom: true,
                ordinal_cost: OrdinalValue::Zero, // Axioms have no computational cost
            };
            self.vm.define_word(axiom_def);
            println!("Declared axiom '{}'", name);
        } else {
            return Err(ReplError::DefinitionError(format!(
                "Axiom '{}' needs a type signature declared with :: first",
                name
            )));
        }
        Ok(())
    }

    /// Handle REPL commands (starting with .)
    fn handle_repl_command(&mut self, command: &str) -> Result<(), ReplError> {
        let parts: Vec<&str> = command[1..].split_whitespace().collect();

        match parts.get(0) {
            Some(&"s") => {
                self.show_stack_contents();
            }
            Some(&"stack") => {
                self.show_stack = !self.show_stack;
                println!(
                    "Stack display: {}",
                    if self.show_stack { "ON" } else { "OFF" }
                );
            }
            Some(&"ordinals") => {
                self.show_ordinals = !self.show_ordinals;
                println!(
                    "Ordinal display: {}",
                    if self.show_ordinals { "ON" } else { "OFF" }
                );
            }
            Some(&"clear") => {
                self.vm.clear_stack();
                println!("Stack cleared");
            }
            Some(&"words") => {
                self.show_words();
            }
            Some(&"types") => {
                self.show_types();
            }
            Some(&"help") => {
                self.show_help();
            }
            Some(&"about") => {
                self.show_about();
            }
            Some(&"reset") => {
                *self = Repl::new();
                println!("REPL reset to initial state");
            }
            Some(cmd) => {
                println!("Unknown command: .{}", cmd);
                println!("Type '.help' for available commands");
            }
            None => {
                println!("Empty command. Type '.help' for available commands");
            }
        }
        Ok(())
    }

    /// Show current stack contents
    fn show_stack_contents(&self) {
        let stack = self.vm.stack();
        print!("<{}> ", stack.len());
        for value in stack {
            print!("{} ", value);
        }
        println!();
    }

    /// Show all defined words
    fn show_words(&self) {
        println!("Core words:");
        let mut core_words: Vec<_> = self.core_lib.get_core_words().keys().collect();
        core_words.sort();

        for chunk in core_words.chunks(8) {
            for word in chunk {
                print!("{:12} ", word);
            }
            println!();
        }

        // Note: User-defined words would be shown here if we tracked them
        println!("\nUse 'help' for detailed documentation");
    }

    /// Show all defined types
    fn show_types(&self) {
        println!("Core types:");
        println!("  Unit     Bool     Nat      Ordinal  Quote");

        if !self.user_types.is_empty() {
            println!("\nUser types:");
            for type_name in self.user_types.keys() {
                println!("  {}", type_name);
            }
        }
    }

    /// Show help information
    fn show_help(&self) {
        println!("C∀O REPL Commands:");
        println!("==================");
        println!();
        println!("REPL Commands (start with .):");
        println!("  .s          Show stack contents");
        println!("  .stack      Toggle automatic stack display");
        println!("  .ordinals   Toggle ordinal cost display");
        println!("  .clear      Clear the stack");
        println!("  .words      List all defined words");
        println!("  .types      List all types");
        println!("  .help       Show this help");
        println!("  .about      About C∀O");
        println!("  .reset      Reset REPL to initial state");
        println!();
        println!("Language Syntax:");
        println!("  :: name ( inputs -> outputs ) ;   Declare type signature");
        println!("  : name body ;                      Define word");
        println!("  type Name {{ field::Type, ... }}     Define type");
        println!("  axiom name                         Declare axiom");
        println!("  [ code ]                           Quotation");
        println!("  ( comment )                        Comment");
        println!();
        println!("Examples:");
        println!("  3 4 +                             Simple arithmetic");
        println!("  :: square ( Nat -> Nat ) ;        Type signature");
        println!("  : square dup * ;                  Word definition");
        println!("  5 square .                        Use and print");
        println!();
        println!("Type 'help' (without .) for core library documentation");
    }

    /// Show about information
    fn show_about(&self) {
        println!("C∀O (Kao) - Categorical ∀xiomatic Ordinal Programming Language");
        println!("==============================================================");
        println!();
        println!("Version: 0.1.0 (Phase 1 - Standalone Core)");
        println!();
        println!("C∀O is an evolving axiomatic programming language that combines:");
        println!("• Category Theory foundations (types as objects, functions as morphisms)");
        println!("• Ordinal Analysis for termination proofs and consistency");
        println!("• Concatenative programming with postfix notation");
        println!("• Collaborative verification and language evolution");
        println!();
        println!("This is the standalone core implementation. Future phases will add:");
        println!("• Network architecture (Hypervisor and Cells)");
        println!("• True ordinal verification");
        println!("• Self-evolution and metaprogramming");
        println!();
        println!("For more information, see the README.md and Implementation-plans.md");
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repl_creation() {
        let repl = Repl::new();
        assert!(!repl.show_stack);
        assert!(!repl.show_ordinals);
    }

    #[test]
    fn test_simple_arithmetic() {
        let mut repl = Repl::new();

        // Test basic arithmetic
        let result = repl.eval("3 4 +");
        assert!(result.is_ok());

        // Check that 7 is on the stack
        assert_eq!(repl.vm.stack().len(), 1);
        if let crate::types::Value::Nat(n) = &repl.vm.stack()[0] {
            assert_eq!(*n, 7);
        } else {
            panic!("Expected Nat(7) on stack");
        }
    }

    #[test]
    fn test_word_definition() {
        let mut repl = Repl::new();

        // Define a simple word
        let result = repl.eval(":: double ( Nat -> Nat ) ;");
        assert!(result.is_ok());

        let result = repl.eval(": double 2 * ;");
        assert!(result.is_ok());

        // Use the word
        let result = repl.eval("5 double");
        assert!(result.is_ok());

        // Check result
        assert_eq!(repl.vm.stack().len(), 1);
        if let crate::types::Value::Nat(n) = &repl.vm.stack()[0] {
            assert_eq!(*n, 10);
        } else {
            panic!("Expected Nat(10) on stack");
        }
    }

    #[test]
    fn test_repl_commands() {
        let mut repl = Repl::new();

        // Test stack toggle
        assert!(!repl.show_stack);
        let result = repl.eval(".stack");
        assert!(result.is_ok());
        assert!(repl.show_stack);

        // Test ordinals toggle
        assert!(!repl.show_ordinals);
        let result = repl.eval(".ordinals");
        assert!(result.is_ok());
        assert!(repl.show_ordinals);
    }

    #[test]
    fn test_quotations() {
        let mut repl = Repl::new();

        // Test quotation creation
        let result = repl.eval("[ 2 * ]");
        assert!(result.is_ok());

        // Check that a quote is on the stack
        assert_eq!(repl.vm.stack().len(), 1);
        if let crate::types::Value::Quote(_) = &repl.vm.stack()[0] {
            // Success
        } else {
            panic!("Expected Quote on stack");
        }
    }
}
