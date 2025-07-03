//! Theorem Proving Demo for Chronos C∀O
//! Showcases the new theorem proving features in an interactive way

use chronos_repl::EnhancedRepl;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧮 Chronos C∀O Theorem Proving Demo");
    println!("=====================================\n");

    println!("Welcome to the enhanced C∀O REPL with theorem proving capabilities!");
    println!("This demo showcases the new features for mathematical reasoning.\n");

    let mut repl = EnhancedRepl::new();

    // Demo 1: Basic theorem proving workflow
    demo_basic_workflow(&mut repl)?;

    // Demo 2: Axiom and theorem management
    demo_axiom_management(&mut repl)?;

    // Demo 3: Interactive proof construction
    demo_interactive_proving(&mut repl)?;

    // Demo 4: Advanced features
    demo_advanced_features(&mut repl)?;

    // Interactive session
    interactive_session(&mut repl)?;

    Ok(())
}

fn demo_basic_workflow(repl: &mut EnhancedRepl) -> Result<(), Box<dyn std::error::Error>> {
    println!("📚 Demo 1: Basic Theorem Proving Workflow");
    println!("==========================================\n");

    println!("Let's start with some basic arithmetic and word definitions...\n");

    // Basic arithmetic
    execute_and_explain(repl, "3 4 +", "Basic arithmetic: 3 + 4")?;
    execute_and_explain(repl, ".s", "Show the stack")?;
    println!();

    // Define a simple function
    execute_and_explain(
        repl,
        ":: square ( Nat -> Nat ) ;",
        "Declare type signature for square",
    )?;
    execute_and_explain(repl, ": square dup * ;", "Define square function")?;
    execute_and_explain(repl, "5 square", "Test: 5 squared")?;
    execute_and_explain(repl, ".s", "Check the result")?;
    println!();

    // Show new help system
    execute_and_explain(repl, "help", "Show core library help")?;
    println!();

    execute_and_explain(repl, "words", "List all available words")?;
    println!();

    wait_for_user();
    Ok(())
}

fn demo_axiom_management(repl: &mut EnhancedRepl) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Demo 2: Axiom and Theorem Management");
    println!("========================================\n");

    println!("Let's explore axioms and theorems in our system...\n");

    // Show current axioms
    execute_and_explain(repl, ".axioms", "List all available axioms")?;
    println!();

    // Show current theorems
    execute_and_explain(repl, ".theorems", "List proven theorems")?;
    println!();

    // Create an axiom
    execute_and_explain(
        repl,
        ":: identity ( a -> a ) ;",
        "Declare identity function type",
    )?;
    execute_and_explain(repl, "axiom identity", "Declare identity as an axiom")?;
    println!();

    // Create more theorems
    execute_and_explain(
        repl,
        ":: double ( Nat -> Nat ) ;",
        "Declare double function",
    )?;
    execute_and_explain(
        repl,
        ": double 2 * ;",
        "Define double (this becomes a theorem)",
    )?;
    println!();

    // Check the updated lists
    execute_and_explain(repl, ".axioms", "Updated axiom list")?;
    println!();
    execute_and_explain(repl, ".theorems", "Updated theorem list")?;
    println!();

    wait_for_user();
    Ok(())
}

fn demo_interactive_proving(repl: &mut EnhancedRepl) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Demo 3: Interactive Proof Construction");
    println!("==========================================\n");

    println!("Now let's see the new proof workflow commands...\n");

    // Start a proof
    execute_and_explain(
        repl,
        ".assume \"P implies Q\"",
        "Add an assumption to our proof context",
    )?;
    execute_and_explain(repl, ".assume \"P is true\"", "Add another assumption")?;
    execute_and_explain(repl, ".goal \"Q is true\"", "Set our proof goal")?;
    println!();

    // Show proof state
    execute_and_explain(repl, ".prove", "Show current proof state")?;
    println!();

    // Complete the proof (conceptually)
    execute_and_explain(repl, ".qed", "Complete the proof (conceptual)")?;
    println!();

    // Mathematical example
    println!("Let's do a concrete mathematical proof...\n");
    execute_and_explain(repl, ".goal \"Prove: 2 * 3 = 6\"", "Set a concrete goal")?;
    execute_and_explain(repl, "2 3 *", "Compute 2 * 3")?;
    execute_and_explain(repl, "6 =", "Check equality with 6")?;
    execute_and_explain(repl, ".prove", "Show proof state")?;
    execute_and_explain(repl, ".qed", "Complete the proof")?;
    println!();

    wait_for_user();
    Ok(())
}

fn demo_advanced_features(repl: &mut EnhancedRepl) -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Demo 4: Advanced Features");
    println!("============================\n");

    println!("Let's explore some advanced theorem proving features...\n");

    // Type inference
    execute_and_explain(
        repl,
        ".infer square",
        "Infer the type of our square function",
    )?;
    println!();

    // Trace execution
    execute_and_explain(repl, ".trace", "Enable execution tracing")?;
    execute_and_explain(repl, "4 square", "Execute with tracing on")?;
    execute_and_explain(repl, ".trace", "Disable tracing")?;
    println!();

    // Performance analysis
    execute_and_explain(
        repl,
        ".benchmark \"3 square\" 10",
        "Benchmark the square function",
    )?;
    println!();

    // Session management
    execute_and_explain(repl, ".save \"demo_session\"", "Save our work")?;
    println!();

    // Complex function
    execute_and_explain(repl, ":: factorial ( Nat -> Nat ) ;", "Declare factorial")?;
    execute_and_explain(
        repl,
        ": factorial dup 1 <= [ drop 1 ] [ dup 1 - factorial * ] if ;",
        "Define factorial recursively",
    )?;
    execute_and_explain(repl, "5 factorial", "Compute 5!")?;
    execute_and_explain(repl, ".s", "Show result")?;
    println!();

    wait_for_user();
    Ok(())
}

fn interactive_session(repl: &mut EnhancedRepl) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Interactive Session");
    println!("======================\n");

    println!("Now you can try the enhanced REPL yourself!");
    println!("Available theorem proving commands:");
    println!("  .axioms          - List all axioms");
    println!("  .theorems        - List proven theorems");
    println!("  .assume <prop>   - Add assumption");
    println!("  .goal <prop>     - Set proof goal");
    println!("  .prove           - Show proof state");
    println!("  .qed             - Complete proof");
    println!("  .help            - Show all commands");
    println!("  help             - Show core library help");
    println!("  words            - List all words");
    println!("  quit             - Exit\n");

    println!("Try some examples:");
    println!("  3 4 + 7 = .qed");
    println!("  :: triple ( Nat -> Nat ) ; : triple 3 * ; 4 triple");
    println!("  .assume \"All cats are mammals\" .goal \"Fluffy is a mammal\"");
    println!();

    loop {
        print!("C∀O Theorem Prover> ");
        io::stdout().flush()?;

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                if input == "quit" || input == "exit" {
                    println!("Thank you for exploring C∀O theorem proving!");
                    println!(
                        "Remember: Every program is a proof, every execution is verification."
                    );
                    break;
                }

                match repl.eval(input) {
                    Ok(_) => {
                        // Success - continue
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }

    Ok(())
}

fn execute_and_explain(
    repl: &mut EnhancedRepl,
    command: &str,
    explanation: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 {}", explanation);
    println!("   Command: {}", command);
    println!("   Result:");

    match repl.eval(command) {
        Ok(_) => {
            println!("   ✓ Success\n");
        }
        Err(e) => {
            println!("   ⚠ {}\n", e);
        }
    }

    Ok(())
}

fn wait_for_user() {
    print!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!();
}
