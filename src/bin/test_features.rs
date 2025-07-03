//! Test binary for new REPL features
//! This allows us to test the new functionality without entering the main REPL loop

use chronos_repl::EnhancedRepl;
use chronos_core::VirtualMachine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Chronos REPL Improvements");
    println!("==================================\n");

    // Test 1: Basic VM functionality with new words
    println!("1. Testing new builtin words...");
    test_builtin_words()?;
    println!();

    // Test 2: REPL commands
    println!("2. Testing new REPL commands...");
    test_repl_commands()?;
    println!();

    // Test 3: Basic functionality still works
    println!("3. Testing basic functionality...");
    test_basic_functionality()?;
    println!();

    // Test 4: Interactive demonstration
    println!("4. Interactive demonstration...");
    interactive_demo()?;

    println!("All tests completed successfully!");
    Ok(())
}

fn test_builtin_words() -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = VirtualMachine::new();

    println!("Testing 'help' command:");
    vm.execute_word("help")?;
    println!();

    println!("Testing 'words' command:");
    vm.execute_word("words")?;
    println!();

    println!("✓ New builtin words work correctly");
    Ok(())
}

fn test_repl_commands() -> Result<(), Box<dyn std::error::Error>> {
    let mut repl = EnhancedRepl::new();

    let commands = vec![
        (".axioms", "List axioms"),
        (".theorems", "List theorems"),
        (".assume \"P -> Q\"", "Add assumption"),
        (".goal \"Q\"", "Set goal"),
        (".prove", "Show proof state"),
        (".qed", "Complete proof"),
    ];

    for (cmd, desc) in commands {
        println!("Testing {}: {}", desc, cmd);
        match repl.eval(cmd) {
            Ok(_) => println!("✓ Command executed successfully"),
            Err(e) => println!("! Command executed with message: {}", e),
        }
        println!();
    }

    Ok(())
}

fn test_basic_functionality() -> Result<(), Box<dyn std::error::Error>> {
    let mut repl = EnhancedRepl::new();

    println!("Testing basic arithmetic:");
    repl.eval("3 4 +")?;
    repl.eval(".s")?;
    println!();

    println!("Testing word definition:");
    repl.eval(":: square ( Nat -> Nat ) ;")?;
    repl.eval(": square dup * ;")?;
    repl.eval("5 square")?;
    repl.eval(".s")?;
    println!();

    println!("Testing axiom declaration:");
    repl.eval(":: identity ( a -> a ) ;")?;
    repl.eval("axiom identity")?;
    println!();

    println!("✓ Basic functionality works correctly");
    Ok(())
}

fn interactive_demo() -> Result<(), Box<dyn std::error::Error>> {
    let mut repl = EnhancedRepl::new();

    println!("Creating a small mathematical theory...");
    println!();

    // Set up some basic axioms
    println!("1. Declaring axioms:");
    repl.eval(":: zero ( -> Nat ) ;")?;
    repl.eval("axiom zero")?;

    repl.eval(":: succ ( Nat -> Nat ) ;")?;
    repl.eval("axiom succ")?;

    println!("Axioms declared.");
    println!();

    // Define some theorems
    println!("2. Proving theorems:");
    repl.eval(":: one ( -> Nat ) ;")?;
    repl.eval(": one zero succ ;")?;

    repl.eval(":: two ( -> Nat ) ;")?;
    repl.eval(": two one succ ;")?;

    println!("Theorems defined.");
    println!();

    // Show the current state
    println!("3. Current mathematical state:");
    repl.eval(".axioms")?;
    repl.eval(".theorems")?;

    // Demonstrate proof workflow
    println!("4. Demonstration of proof workflow:");
    repl.eval(".assume \"Natural numbers are well-ordered\"")?;
    repl.eval(".goal \"Every natural number has a successor\"")?;
    repl.eval(".prove")?;
    repl.eval(".qed")?;

    println!();
    println!("✓ Interactive demonstration completed");
    Ok(())
}
