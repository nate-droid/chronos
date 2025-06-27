//! Test program for new REPL functionality
//! This tests the new help, words, and theorem proving commands

use crate::repl::Repl;
use crate::vm::VirtualMachine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_builtin_words() {
        let mut vm = VirtualMachine::new();

        // Test help command
        let result = vm.execute_word("help");
        assert!(result.is_ok(), "help command should execute successfully");

        // Test words command
        let result = vm.execute_word("words");
        assert!(result.is_ok(), "words command should execute successfully");
    }

    #[test]
    fn test_theorem_proving_commands() {
        let mut repl = Repl::new();

        // Test axioms command
        let result = repl.eval(".axioms");
        assert!(result.is_ok(), ".axioms command should work");

        // Test theorems command
        let result = repl.eval(".theorems");
        assert!(result.is_ok(), ".theorems command should work");

        // Test assume command
        let result = repl.eval(".assume \"P implies Q\"");
        assert!(result.is_ok(), ".assume command should work");

        // Test goal command
        let result = repl.eval(".goal \"Q\"");
        assert!(result.is_ok(), ".goal command should work");

        // Test prove command
        let result = repl.eval(".prove");
        assert!(result.is_ok(), ".prove command should work");

        // Test qed command
        let result = repl.eval(".qed");
        assert!(result.is_ok(), ".qed command should work");
    }

    #[test]
    fn test_basic_functionality_still_works() {
        let mut repl = Repl::new();

        // Test basic arithmetic
        let result = repl.eval("3 4 +");
        assert!(result.is_ok(), "Basic arithmetic should still work");

        // Test stack operations
        let result = repl.eval("5 dup *");
        assert!(result.is_ok(), "Stack operations should still work");

        // Test word definition
        let result = repl.eval(":: square ( Nat -> Nat ) ;");
        assert!(result.is_ok(), "Type signature should work");

        let result = repl.eval(": square dup * ;");
        assert!(result.is_ok(), "Word definition should work");

        let result = repl.eval("6 square");
        assert!(result.is_ok(), "Using defined word should work");
    }

    #[test]
    fn test_axiom_and_theorem_tracking() {
        let mut repl = Repl::new();

        // Define a type signature and declare as axiom
        let result = repl.eval(":: identity ( a -> a ) ;");
        assert!(result.is_ok(), "Type signature should work");

        let result = repl.eval("axiom identity");
        assert!(result.is_ok(), "Axiom declaration should work");

        // Check that axioms are tracked
        let result = repl.eval(".axioms");
        assert!(result.is_ok(), "Should be able to list axioms");

        // Define a theorem (non-axiom word)
        let result = repl.eval(":: double ( Nat -> Nat ) ;");
        assert!(result.is_ok(), "Type signature should work");

        let result = repl.eval(": double 2 * ;");
        assert!(result.is_ok(), "Word definition should work");

        // Check that theorems are tracked
        let result = repl.eval(".theorems");
        assert!(result.is_ok(), "Should be able to list theorems");
    }
}

// Standalone test functions that can be called from main
pub fn test_help_command() {
    println!("Testing help command...");
    let mut vm = VirtualMachine::new();
    match vm.execute_word("help") {
        Ok(_) => println!("✓ help command works"),
        Err(e) => println!("✗ help command failed: {}", e),
    }
}

pub fn test_words_command() {
    println!("Testing words command...");
    let mut vm = VirtualMachine::new();
    match vm.execute_word("words") {
        Ok(_) => println!("✓ words command works"),
        Err(e) => println!("✗ words command failed: {}", e),
    }
}

pub fn test_new_repl_commands() {
    println!("Testing new REPL commands...");
    let mut repl = Repl::new();

    let commands = [
        ".axioms",
        ".theorems",
        ".assume \"test assumption\"",
        ".goal \"test goal\"",
        ".prove",
        ".qed",
    ];

    for cmd in &commands {
        match repl.eval(cmd) {
            Ok(_) => println!("✓ {} works", cmd),
            Err(e) => println!("✗ {} failed: {}", cmd, e),
        }
    }
}

pub fn run_all_tests() {
    println!("Running all new feature tests...\n");

    test_help_command();
    println!();

    test_words_command();
    println!();

    test_new_repl_commands();
    println!();

    println!("All tests completed!");
}
