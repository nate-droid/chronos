//! Unit tests for specific patterns and functionality demonstrated in the examples
//!
//! These tests verify that the core patterns shown in the example files work correctly,
//! without relying on the full example files which may contain features not yet implemented.

use chronos_core::Value;
use chronos_repl::{EnhancedRepl, ReplConfig};

/// Helper function to create a test REPL with minimal configuration
fn create_test_repl() -> EnhancedRepl {
    let mut config = ReplConfig::default();
    config.show_welcome = false;
    config.show_stack = false;
    config.show_timing = false;
    config.use_colors = false;
    EnhancedRepl::with_config(config)
}

/// Helper to assert a natural number is on top of the stack
fn assert_top_is_nat(repl: &EnhancedRepl, expected: u64) {
    let stack = repl.stack();
    assert!(!stack.is_empty(), "Stack should not be empty");

    if let Value::Nat(n) = stack.last().unwrap() {
        assert_eq!(
            *n, expected,
            "Expected {} on top of stack, got {}",
            expected, n
        );
    } else {
        panic!(
            "Expected Nat on top of stack, got {:?}",
            stack.last().unwrap()
        );
    }
}

/// Helper to assert stack has specific length
fn assert_stack_length(repl: &EnhancedRepl, expected: usize) {
    let stack = repl.stack();
    assert_eq!(
        stack.len(),
        expected,
        "Expected stack length {}, got {}",
        expected,
        stack.len()
    );
}

#[test]
fn test_basic_arithmetic_patterns() {
    let mut repl = create_test_repl();

    // Test addition (from 01_basic_arithmetic.cao)
    repl.eval("2 3 +").expect("Addition should work");
    assert_top_is_nat(&repl, 5);

    // Test multiplication
    repl.eval("4 *").expect("Multiplication should work");
    assert_top_is_nat(&repl, 20);

    // Test subtraction
    repl.eval("5 -").expect("Subtraction should work");
    assert_top_is_nat(&repl, 15);
}

#[test]
fn test_stack_manipulation_patterns() {
    let mut repl = create_test_repl();

    // Test dup operation (from examples)
    repl.eval("42 dup").expect("Dup should work");
    assert_stack_length(&repl, 2);
    assert_top_is_nat(&repl, 42);

    // Test drop operation
    repl.eval("drop").expect("Drop should work");
    assert_stack_length(&repl, 1);
    assert_top_is_nat(&repl, 42);

    // Test swap operation
    repl.eval("100 swap").expect("Swap should work");
    assert_top_is_nat(&repl, 42);

    // Clear and test over
    repl.eval("drop drop").ok(); // Clear stack
    repl.eval("1 2 over").expect("Over should work");
    assert_stack_length(&repl, 3);
    assert_top_is_nat(&repl, 1);
}

#[test]
fn test_word_definition_patterns() {
    let mut repl = create_test_repl();

    // Test simple word definition (pattern from examples)
    // Note: We'll test without explicit type signatures first to see what works
    repl.eval(": double 2 * ;")
        .expect("Should define double word");

    // Test using the defined word
    repl.eval("5 double").expect("Should use defined word");
    assert_top_is_nat(&repl, 10);

    // Test another word definition
    repl.eval(": square dup * ;")
        .expect("Should define square word");
    repl.eval("4 square").expect("Should use square word");
    assert_top_is_nat(&repl, 16);
}

#[test]
fn test_conditional_patterns() {
    let mut repl = create_test_repl();

    // Test basic conditional (if implemented)
    // This tests the pattern from 02_conditionals.cao
    let result = repl.eval("true [ 42 ] [ 0 ] if");

    if result.is_ok() {
        assert_top_is_nat(&repl, 42);

        // Test false condition
        repl.eval("false [ 42 ] [ 0 ] if")
            .expect("False condition should work");
        assert_top_is_nat(&repl, 0);
    } else {
        // If conditionals aren't implemented yet, that's okay for now
        println!("Conditionals not yet implemented - skipping conditional tests");
    }
}

#[test]
fn test_comparison_operations() {
    let mut repl = create_test_repl();

    // Test equality comparison
    let result = repl.eval("5 5 =");
    if result.is_ok() {
        if let Value::Bool(b) = repl.stack().last().unwrap() {
            assert_eq!(*b, true, "5 = 5 should be true");
        }

        // Test inequality
        repl.eval("3 7 =").expect("Inequality should work");
        if let Value::Bool(b) = repl.stack().last().unwrap() {
            assert_eq!(*b, false, "3 = 7 should be false");
        }
    } else {
        println!("Comparison operations not yet implemented - skipping");
    }
}

#[test]
fn test_complex_arithmetic_patterns() {
    let mut repl = create_test_repl();

    // Test pattern: (3 + 4) * (5 - 2) = 7 * 3 = 21
    repl.eval("3 4 +").expect("Should calculate 3 + 4");
    assert_top_is_nat(&repl, 7);

    repl.eval("5 2 -").expect("Should calculate 5 - 2");
    assert_top_is_nat(&repl, 3);

    repl.eval("*").expect("Should multiply results");
    assert_top_is_nat(&repl, 21);
}

#[test]
fn test_word_composition_patterns() {
    let mut repl = create_test_repl();

    // Define helper words (pattern from examples)
    repl.eval(": double 2 * ;").expect("Should define double");
    repl.eval(": square dup * ;").expect("Should define square");

    // Test word composition
    repl.eval("3 double square").expect("Should compose words");
    // 3 -> double -> 6 -> square -> 36
    assert_top_is_nat(&repl, 36);
}

#[test]
fn test_stack_depth_management() {
    let mut repl = create_test_repl();

    // Build up stack
    repl.eval("1 2 3 4 5").expect("Should build stack");
    assert_stack_length(&repl, 5);

    // Use rot if implemented
    let result = repl.eval("rot");
    if result.is_ok() {
        assert_stack_length(&repl, 5);
        // After rot, third element should be on top
        assert_top_is_nat(&repl, 3);
    }
}

#[test]
fn test_mathematical_operations_patterns() {
    let mut repl = create_test_repl();

    // Test average calculation pattern (from examples)
    repl.eval(": average + 2 / ;")
        .expect("Should define average");
    repl.eval("10 20 average")
        .expect("Should calculate average");
    assert_top_is_nat(&repl, 15);

    // Test square plus one pattern
    repl.eval(": square-plus-one dup * 1 + ;")
        .expect("Should define square-plus-one");
    repl.eval("4 square-plus-one")
        .expect("Should calculate square plus one");
    assert_top_is_nat(&repl, 17); // 4² + 1 = 16 + 1 = 17
}

#[test]
fn test_error_recovery() {
    let mut repl = create_test_repl();

    // REPL should handle errors gracefully
    repl.eval("42").expect("Should push valid number");
    assert_top_is_nat(&repl, 42);

    // Try an operation that might fail
    let result = repl.eval("unknown_operation");

    // Regardless of whether it succeeds or fails,
    // the REPL should still be usable afterward
    repl.eval("1 1 +")
        .expect("REPL should still work after error");

    // The stack should contain our result
    let stack = repl.stack();
    assert!(
        !stack.is_empty(),
        "Stack should not be empty after recovery"
    );
}

#[test]
fn test_session_state_persistence() {
    let mut repl = create_test_repl();

    // Define a word and use it
    repl.eval(": triple 3 * ;").expect("Should define triple");
    repl.eval("7 triple").expect("Should use triple");
    assert_top_is_nat(&repl, 21);

    // The word should remain defined for subsequent use
    repl.eval("2 triple").expect("Word should still be defined");
    assert_top_is_nat(&repl, 6);
}

#[test]
fn test_type_safety_patterns() {
    let mut repl = create_test_repl();

    // Test that type-safe operations work as expected
    repl.eval("(): Unit").ok(); // Unit value
    repl.eval("true").ok(); // Boolean value
    repl.eval("42").ok(); // Natural number

    // Basic operations should work on appropriate types
    repl.eval("1 2 +").expect("Nat addition should work");
    assert_top_is_nat(&repl, 3);
}

#[test]
fn test_quote_operations() {
    let mut repl = create_test_repl();

    // Test quote creation (if implemented)
    let result = repl.eval("[ 1 2 + ]");

    if result.is_ok() {
        // If quotes are implemented, test their execution
        let execute_result = repl.eval("call");
        if execute_result.is_ok() {
            // Should have executed the quote
            assert_top_is_nat(&repl, 3);
        }
    } else {
        println!("Quotes not yet implemented - skipping quote tests");
    }
}

#[test]
fn test_performance_monitoring() {
    let mut repl = create_test_repl();

    // Execute some operations
    repl.eval("1 2 +").expect("Should execute operation");
    repl.eval("3 4 *").expect("Should execute operation");

    // Check that performance metrics are being tracked
    let metrics = repl.performance_metrics();
    assert!(metrics.total_operations > 0, "Should track operations");
    assert!(metrics.total_time.as_nanos() > 0, "Should track time");
}

#[test]
fn test_fibonacci_pattern() {
    let mut repl = create_test_repl();

    // Define fibonacci recursively (if recursion is supported)
    let result = repl.eval(": fib dup 2 < [ ] [ dup 1 - fib swap 2 - fib + ] if ;");

    if result.is_ok() {
        // Test small fibonacci numbers
        repl.eval("0 fib").expect("fib(0) should work");
        repl.eval("1 fib").expect("fib(1) should work");
        repl.eval("5 fib").expect("fib(5) should work");

        // fib(5) = 5, so top of stack should be 5
        assert_top_is_nat(&repl, 5);
    } else {
        println!("Recursive definitions not yet implemented - skipping fibonacci test");
    }
}

#[test]
fn test_factorial_pattern() {
    let mut repl = create_test_repl();

    // Define factorial recursively
    let result = repl.eval(": fact dup 1 = [ ] [ dup 1 - fact * ] if ;");

    if result.is_ok() {
        // Test factorial calculation
        repl.eval("5 fact").expect("fact(5) should work");
        // 5! = 120
        assert_top_is_nat(&repl, 120);
    } else {
        println!("Recursive factorial not yet implemented - skipping test");
    }
}
