//! Test Codd CA Commands
//!
//! Simple test to verify that Codd CA commands work correctly
//! by testing parsing and core functionality.

use chronos_repl::codd_ca::{codd_patterns, run_simple_codd_ca, CoddPatternType};
use chronos_repl::commands::parse_command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Codd CA Commands");
    println!("========================\n");

    // Test command parsing
    println!("1. Testing command parsing:");

    let test_commands = vec![
        ".codd-patterns",
        ".codd signal 20 15",
        ".codd-simple signal 15 10 3",
        ".codd replicator 30 25",
        ".codd empty 20 20",
    ];

    for cmd_str in &test_commands {
        let cmd = parse_command(cmd_str);
        println!("  '{}' -> {:?}", cmd_str, cmd);
    }
    println!();

    // Test patterns function
    println!("2. Testing codd_patterns function:");
    let patterns = codd_patterns();
    for (pattern_type, description) in patterns {
        println!("  {:?}: {}", pattern_type, description);
    }
    println!();

    // Test simple CA execution
    println!("3. Testing simple Codd CA execution:");
    println!("Running signal pattern for 3 generations...");
    let result = run_simple_codd_ca(CoddPatternType::Signal, 3, 12, 8)?;
    println!("{}", result);

    println!("4. Testing empty pattern:");
    let empty_result = run_simple_codd_ca(CoddPatternType::Empty, 2, 10, 5)?;
    println!("{}", empty_result);

    println!("Command tests completed successfully!");
    Ok(())
}
