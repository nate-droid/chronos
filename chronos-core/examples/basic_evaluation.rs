//! Basic evaluation example for Chronos Core
//!
//! This example demonstrates the fundamental operations of the C∀O language
//! including arithmetic, stack manipulation, and basic control flow.

use chronos_core::{ChronosCore, ChronosError, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Chronos Core Basic Evaluation Example ===\n");

    let mut core = ChronosCore::new();

    // Basic arithmetic
    println!("1. Basic Arithmetic");
    println!("-------------------");

    let result = core.eval("3 4 +")?;
    println!("3 4 + = {}", result);
    assert_eq!(result, Value::Nat(7));

    let result = core.eval("10 3 -")?;
    println!("10 3 - = {}", result);
    assert_eq!(result, Value::Nat(7));

    let result = core.eval("6 7 *")?;
    println!("6 7 * = {}", result);
    assert_eq!(result, Value::Nat(42));

    let result = core.eval("15 3 /")?;
    println!("15 3 / = {}", result);
    assert_eq!(result, Value::Nat(5));

    let result = core.eval("17 5 mod")?;
    println!("17 5 mod = {}", result);
    assert_eq!(result, Value::Nat(2));

    core.clear_stack();
    println!();

    // Stack manipulation
    println!("2. Stack Manipulation");
    println!("---------------------");

    // Build up a stack
    core.eval("1 2 3")?;
    println!("After '1 2 3': stack = {:?}", core.get_stack());
    assert_eq!(core.stack_depth(), 3);

    core.eval("dup")?;
    println!("After 'dup': stack = {:?}", core.get_stack());
    assert_eq!(core.stack_depth(), 4);

    core.eval("swap")?;
    println!("After 'swap': stack = {:?}", core.get_stack());

    core.eval("over")?;
    println!("After 'over': stack = {:?}", core.get_stack());

    core.eval("rot")?;
    println!("After 'rot': stack = {:?}", core.get_stack());

    core.clear_stack();
    println!();

    // Boolean operations
    println!("3. Boolean Operations");
    println!("---------------------");

    let result = core.eval("true")?;
    println!("true = {}", result);
    assert_eq!(result, Value::Bool(true));

    let result = core.eval("false")?;
    println!("false = {}", result);
    assert_eq!(result, Value::Bool(false));

    let result = core.eval("true false and")?;
    println!("true false and = {}", result);
    assert_eq!(result, Value::Bool(false));

    let result = core.eval("true false or")?;
    println!("true false or = {}", result);
    assert_eq!(result, Value::Bool(true));

    let result = core.eval("true not")?;
    println!("true not = {}", result);
    assert_eq!(result, Value::Bool(false));

    core.clear_stack();
    println!();

    // Comparison operations
    println!("4. Comparison Operations");
    println!("------------------------");

    let result = core.eval("5 3 >")?;
    println!("5 3 > = {}", result);
    assert_eq!(result, Value::Bool(true));

    let result = core.eval("2 7 <")?;
    println!("2 7 < = {}", result);
    assert_eq!(result, Value::Bool(true));

    let result = core.eval("4 4 =")?;
    println!("4 4 = = {}", result);
    assert_eq!(result, Value::Bool(true));

    let result = core.eval("3 5 >=")?;
    println!("3 5 >= = {}", result);
    assert_eq!(result, Value::Bool(false));

    let result = core.eval("8 2 <=")?;
    println!("8 2 <= = {}", result);
    assert_eq!(result, Value::Bool(false));

    core.clear_stack();
    println!();

    // Working with quotes (code blocks)
    println!("5. Working with Quotes");
    println!("----------------------");

    // Push a quote onto the stack
    core.eval("[ 2 * ]")?;
    println!("After '[ 2 * ]': stack depth = {}", core.stack_depth());

    // Note: Actual quote execution would require additional VM features
    // For now, we can demonstrate quote creation
    if let Some(Value::Quote(tokens)) = core.pop() {
        println!("Quote contains {} tokens", tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            println!("  Token {}: {}", i, token);
        }
    }

    core.clear_stack();
    println!();

    // Complex expressions
    println!("6. Complex Expressions");
    println!("----------------------");

    // Compute (5 + 3) * (7 - 2)
    let result = core.eval("5 3 + 7 2 - *")?;
    println!("(5 + 3) * (7 - 2) = 5 3 + 7 2 - * = {}", result);
    assert_eq!(result, Value::Nat(40));

    // Compute the average of three numbers: (10 + 20 + 30) / 3
    let result = core.eval("10 20 + 30 + 3 /")?;
    println!("Average of 10, 20, 30 = (10 + 20 + 30) / 3 = {}", result);
    assert_eq!(result, Value::Nat(20));

    // Check if a number is even: n mod 2 = 0
    let result = core.eval("42 2 mod 0 =")?;
    println!("42 is even? 42 2 mod 0 = = {}", result);
    assert_eq!(result, Value::Bool(true));

    core.clear_stack();
    println!();

    // Error handling demonstration
    println!("7. Error Handling");
    println!("-----------------");

    // Demonstrate stack underflow
    match core.eval("dup") {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => println!("Expected error: {}", e),
    }

    // Demonstrate division by zero
    match core.eval("5 0 /") {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => println!("Expected error: {}", e),
    }

    // Demonstrate undefined word
    match core.eval("undefined-operation") {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => println!("Expected error: {}", e),
    }

    println!();

    // Direct stack manipulation
    println!("8. Direct Stack Access");
    println!("----------------------");

    // Push values directly
    core.push(Value::Nat(100));
    core.push(Value::Bool(true));
    core.push(Value::Nat(42));

    println!("After direct pushes: stack = {:?}", core.get_stack());

    // Pop values
    if let Some(value) = core.pop() {
        println!("Popped: {}", value);
    }

    println!("After pop: stack = {:?}", core.get_stack());

    // Check available words
    println!();
    println!("9. Available Operations");
    println!("-----------------------");

    let words = core.get_words();
    println!("Available words ({} total):", words.len());
    for (i, word) in words.iter().enumerate() {
        if i > 0 && i % 8 == 0 {
            println!();
        }
        print!("{:>8} ", word);
    }
    println!("\n");

    // Performance demonstration
    println!("10. Performance Test");
    println!("--------------------");

    use std::time::Instant;

    let start = Instant::now();
    for _ in 0..1000 {
        core.eval("1 1 + drop")?;
    }
    let duration = start.elapsed();

    println!("1000 simple operations took: {:?}", duration);
    println!("Average per operation: {:?}", duration / 1000);

    println!("\n=== Example completed successfully! ===");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let mut core = ChronosCore::new();

        assert_eq!(core.eval("2 3 +").unwrap(), Value::Nat(5));
        assert_eq!(core.eval("10 4 -").unwrap(), Value::Nat(6));
        assert_eq!(core.eval("3 7 *").unwrap(), Value::Nat(21));
        assert_eq!(core.eval("15 3 /").unwrap(), Value::Nat(5));
    }

    #[test]
    fn test_stack_operations() {
        let mut core = ChronosCore::new();

        core.eval("1 2 3").unwrap();
        assert_eq!(core.stack_depth(), 3);

        core.eval("dup").unwrap();
        assert_eq!(core.stack_depth(), 4);

        let top = core.pop().unwrap();
        assert_eq!(top, Value::Nat(3));
    }

    #[test]
    fn test_boolean_operations() {
        let mut core = ChronosCore::new();

        assert_eq!(core.eval("true false and").unwrap(), Value::Bool(false));
        assert_eq!(core.eval("true false or").unwrap(), Value::Bool(true));
        assert_eq!(core.eval("true not").unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_error_cases() {
        let mut core = ChronosCore::new();

        // Stack underflow
        assert!(core.eval("dup").is_err());

        // Undefined word
        assert!(core.eval("undefined").is_err());
    }

    #[test]
    fn test_complex_expressions() {
        let mut core = ChronosCore::new();

        // (5 + 3) * (7 - 2) = 8 * 5 = 40
        assert_eq!(core.eval("5 3 + 7 2 - *").unwrap(), Value::Nat(40));

        // Check even number: 42 mod 2 = 0
        assert_eq!(core.eval("42 2 mod 0 =").unwrap(), Value::Bool(true));
    }
}
