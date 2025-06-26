//! Basic usage example for chronos-repl
//!
//! This example demonstrates the core functionality of the Enhanced REPL,
//! including evaluation, session management, and tracing capabilities.

use chronos_repl::{EnhancedRepl, ReplConfig};
use std::path::Path;
use tempfile::NamedTempFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Chronos REPL Basic Usage Example");
    println!("=================================\n");

    // Create a new REPL instance
    let mut repl = EnhancedRepl::new();
    println!("✓ Created new REPL instance");

    // Example 1: Basic arithmetic
    println!("\n1. Basic Arithmetic:");
    repl.eval("2 3 +")?;
    println!("   Evaluated: 2 3 +");
    println!("   Stack: {:?}", repl.stack());

    repl.eval("dup *")?;
    println!("   Evaluated: dup *");
    println!("   Stack: {:?}", repl.stack());

    // Example 2: Word definition
    println!("\n2. Word Definition:");
    repl.eval("square : dup *")?;
    println!("   Defined word: square : dup *");

    repl.eval("5 square")?;
    println!("   Evaluated: 5 square");
    println!("   Stack: {:?}", repl.stack());

    // Example 3: Performance metrics
    println!("\n3. Performance Metrics:");
    let metrics = repl.performance_metrics();
    println!("   Total operations: {}", metrics.total_operations);
    println!("   Total time: {:?}", metrics.total_time);
    println!("   Max stack depth: {}", metrics.max_stack_depth);

    // Example 4: Execution tracing
    println!("\n4. Execution Tracing:");
    repl.set_tracing(true);
    let result = repl.eval_with_trace("3 4 + 2 *")?;
    println!("   Evaluated with tracing: 3 4 + 2 *");
    println!("   Result: {:?}", result.value);
    println!("   Execution time: {:?}", result.duration);
    println!("   Trace entries: {}", result.trace_entries.len());

    for (i, entry) in result.trace_entries.iter().enumerate() {
        println!("     {}: {} ({:?})", i + 1, entry.token, entry.duration);
    }

    // Example 5: Session management
    println!("\n5. Session Management:");
    let temp_file = NamedTempFile::new()?;
    let session_path = temp_file.path();

    // Save the current session
    repl.save_session(session_path)?;
    println!("   ✓ Session saved to temporary file");

    // Create a new REPL and load the session
    let mut new_repl = EnhancedRepl::new();
    new_repl.load_session(session_path)?;
    println!("   ✓ Session loaded in new REPL instance");
    println!("   Restored stack: {:?}", new_repl.stack());

    // Example 6: Custom configuration
    println!("\n6. Custom Configuration:");
    let config = ReplConfig {
        show_stack: true,
        show_timing: true,
        prompt: "DEMO> ".to_string(),
        use_colors: false,
        ..Default::default()
    };

    let mut configured_repl = EnhancedRepl::with_config(config);
    println!("   ✓ Created REPL with custom configuration");
    println!("   Prompt: {}", configured_repl.config().prompt);
    println!("   Show stack: {}", configured_repl.config().show_stack);
    println!("   Show timing: {}", configured_repl.config().show_timing);

    // Example 7: Error handling
    println!("\n7. Error Handling:");
    match repl.eval("invalid operation") {
        Ok(_) => println!("   Unexpected success"),
        Err(e) => println!("   ✓ Caught error: {}", e),
    }

    // Example 8: Stack manipulation
    println!("\n8. Stack Manipulation:");
    let mut repl = EnhancedRepl::new();

    // Build up a stack
    repl.eval("1 2 3 4 5")?;
    println!("   After '1 2 3 4 5': {:?}", repl.stack());

    // Demonstrate stack operations
    repl.eval("swap")?;
    println!("   After 'swap': {:?}", repl.stack());

    repl.eval("dup")?;
    println!("   After 'dup': {:?}", repl.stack());

    repl.eval("drop")?;
    println!("   After 'drop': {:?}", repl.stack());

    // Example 9: Complex expressions
    println!("\n9. Complex Expressions:");
    repl.eval("10 dup + 5 - 3 *")?;
    println!("   Evaluated: 10 dup + 5 - 3 *");
    println!("   Final result: {:?}", repl.stack());

    // Example 10: Reset functionality
    println!("\n10. Reset Functionality:");
    println!("    Before reset - Stack size: {}", repl.stack().len());
    repl.reset();
    println!("    After reset - Stack size: {}", repl.stack().len());
    println!("    ✓ REPL reset to initial state");

    println!("\n=================================");
    println!("✓ All examples completed successfully!");
    println!("Try running the interactive REPL with: cargo run --bin chronos-repl");

    Ok(())
}
