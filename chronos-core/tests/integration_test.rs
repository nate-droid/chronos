//! Integration tests for Chronos Core
//!
//! These tests verify that the public API works correctly for common use cases
//! and that the core language functionality operates as expected.

use chronos_core::{ChronosCore, Value};

#[test]
fn test_basic_language_operations() {
    let mut core = ChronosCore::new();

    // Test arithmetic
    assert_eq!(core.eval("5 3 +").unwrap(), Value::Nat(8));
    assert_eq!(core.eval("10 4 -").unwrap(), Value::Nat(6));
    assert_eq!(core.eval("6 7 *").unwrap(), Value::Nat(42));
    assert_eq!(core.eval("20 4 /").unwrap(), Value::Nat(5));
}

#[test]
fn test_stack_manipulation() {
    let mut core = ChronosCore::new();

    // Push values and test stack operations
    core.push(Value::Nat(1));
    core.push(Value::Nat(2));
    core.push(Value::Nat(3));

    assert_eq!(core.stack_depth(), 3);

    // Test dup
    core.eval("dup").unwrap();
    assert_eq!(core.stack_depth(), 4);
    assert_eq!(core.pop().unwrap(), Value::Nat(3));
    assert_eq!(core.pop().unwrap(), Value::Nat(3));

    // Test swap
    core.push(Value::Nat(10));
    core.push(Value::Nat(20));
    core.eval("swap").unwrap();
    assert_eq!(core.pop().unwrap(), Value::Nat(10));
    assert_eq!(core.pop().unwrap(), Value::Nat(20));
}

#[test]
fn test_boolean_operations() {
    let mut core = ChronosCore::new();

    // Test boolean literals
    assert_eq!(core.eval("true").unwrap(), Value::Bool(true));
    assert_eq!(core.eval("false").unwrap(), Value::Bool(false));

    // Test comparisons
    assert_eq!(core.eval("5 3 >").unwrap(), Value::Bool(true));
    assert_eq!(core.eval("2 8 <").unwrap(), Value::Bool(true));
    assert_eq!(core.eval("4 4 =").unwrap(), Value::Bool(true));
    assert_eq!(core.eval("7 3 =").unwrap(), Value::Bool(false));
}

#[test]
fn test_complex_expressions() {
    let mut core = ChronosCore::new();

    // Test compound arithmetic: (5 + 3) * (6 - 2) = 8 * 4 = 32
    let result = core.eval("5 3 + 6 2 - *").unwrap();
    assert_eq!(result, Value::Nat(32));

    // Test nested stack operations
    core.clear_stack();
    core.eval("1 2 3 dup + swap").unwrap(); // [1, 2, 6, 3]
    
    assert_eq!(core.stack_depth(), 3);
    assert_eq!(core.pop().unwrap(), Value::Nat(2));
    assert_eq!(core.pop().unwrap(), Value::Nat(6));
}

#[test]
fn test_error_handling() {
    let mut core = ChronosCore::new();

    // Test stack underflow
    let result = core.eval("dup");
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(
            e.to_string().contains("Stack underflow") || e.to_string().contains("Unknown word")
        );
    }

    // Test division by zero
    let result = core.eval("5 0 /");
    assert!(result.is_err());

    // Test undefined word
    let result = core.eval("undefined-operation");
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Unknown word") || e.to_string().contains("undefined"));
    }
}

#[test]
fn test_tokenization() {
    let core = ChronosCore::new();

    // Test basic tokenization
    let tokens = core.tokenize("1 2 +").unwrap();
    assert_eq!(tokens.len(), 3);

    // Test with comments
    let tokens = core.tokenize("1 ( this is a comment ) 2 +").unwrap();
    // Should have 1, comment, 2, +
    assert!(tokens.len() >= 3);

    // Test with booleans
    let tokens = core.tokenize("true false").unwrap();
    assert_eq!(tokens.len(), 2);
}

#[test]
fn test_quote_handling() {
    let mut core = ChronosCore::new();

    // Test quote creation
    let result = core.eval("[ 2 * ]").unwrap();
    if let Value::Quote(tokens) = result {
        assert_eq!(tokens.len(), 2); // Should contain "2" and "*"
    } else {
        panic!("Expected Quote value");
    }
}

#[test]
fn test_value_conversions() {
    use chronos_core::{FromValue, IntoValue};

    // Test into_value conversions
    let bool_val: Value = true.into_value();
    assert_eq!(bool_val, Value::Bool(true));

    let nat_val: Value = 42u64.into_value();
    assert_eq!(nat_val, Value::Nat(42));

    // Test from_value conversions
    let extracted_bool: bool = bool::from_value(Value::Bool(false)).unwrap();
    assert_eq!(extracted_bool, false);

    let extracted_nat: u64 = u64::from_value(Value::Nat(100)).unwrap();
    assert_eq!(extracted_nat, 100);

    // Test error cases
    let result = bool::from_value(Value::Nat(42));
    assert!(result.is_err());
}

#[test]
fn test_stack_state_management() {
    let mut core = ChronosCore::new();

    // Test empty stack
    assert_eq!(core.stack_depth(), 0);
    assert!(core.get_stack().is_empty());

    // Build up stack
    core.push(Value::Nat(1));
    core.push(Value::Bool(true));
    core.push(Value::Nat(42));

    let stack = core.get_stack();
    assert_eq!(stack.len(), 3);
    assert_eq!(stack[0], Value::Nat(1));
    assert_eq!(stack[1], Value::Bool(true));
    assert_eq!(stack[2], Value::Nat(42));

    // Test clear
    core.clear_stack();
    assert_eq!(core.stack_depth(), 0);
}

#[test]
fn test_reset_functionality() {
    let mut core = ChronosCore::new();

    // Modify state
    core.push(Value::Nat(100));
    core.push(Value::Bool(false));
    assert_eq!(core.stack_depth(), 2);

    // Reset should clear everything
    core.reset();
    assert_eq!(core.stack_depth(), 0);

    // Should still be able to evaluate
    let result = core.eval("3 4 +").unwrap();
    assert_eq!(result, Value::Nat(7));
}

#[test]
fn test_word_introspection() {
    let core = ChronosCore::new();

    // Should have some core words available
    let words = core.get_words();
    assert!(!words.is_empty());
    assert!(words.contains(&"dup".to_string()));
    assert!(words.contains(&"+".to_string()));

    // Test word existence check
    assert!(core.is_word_defined("dup"));
    assert!(core.is_word_defined("+"));
}

#[test]
fn test_performance_characteristics() {
    let mut core = ChronosCore::new();

    // Test that basic operations are reasonably fast
    use std::time::Instant;

    let start = Instant::now();
    for _ in 0..100 {
        core.eval("1 1 + drop").unwrap();
    }
    let duration = start.elapsed();

    // Should complete 100 operations in reasonable time (< 10ms)
    assert!(
        duration.as_millis() < 10,
        "Operations too slow: {:?}",
        duration
    );
}

#[test]
fn test_concurrent_cores() {
    // Test that multiple ChronosCore instances can work independently
    let mut core1 = ChronosCore::new();
    let mut core2 = ChronosCore::new();

    core1.push(Value::Nat(10));
    core2.push(Value::Nat(20));

    assert_eq!(core1.stack_depth(), 1);
    assert_eq!(core2.stack_depth(), 1);

    assert_eq!(core1.pop().unwrap(), Value::Nat(10));
    assert_eq!(core2.pop().unwrap(), Value::Nat(20));
}

#[test]
fn test_error_recovery() {
    let mut core = ChronosCore::new();

    // Cause an error
    let _ = core.eval("undefined-word");

    // Should still be able to evaluate correctly after error
    let result = core.eval("5 5 +");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Nat(10));
}

#[test]
fn test_value_display() {
    // Test that values display correctly
    assert_eq!(Value::Nat(42).to_string(), "42");
    assert_eq!(Value::Bool(true).to_string(), "true");
    assert_eq!(Value::Bool(false).to_string(), "false");
    assert_eq!(Value::Unit.to_string(), "()");
}

#[test]
fn test_comprehensive_workflow() {
    let mut core = ChronosCore::new();

    // Simulate a realistic workflow
    // 1. Push some initial values
    core.push(Value::Nat(10));
    core.push(Value::Nat(5));

    // 2. Perform arithmetic
    let result = core.eval("+").unwrap();
    assert_eq!(result, Value::Nat(15));

    // 3. Duplicate and manipulate
    core.push(Value::Nat(3));
    core.eval("dup *").unwrap(); // 3 * 3 = 9
    assert_eq!(core.stack_depth(), 0);

    // 4. Compare with previous result
    core.push(Value::Nat(15));
    let comparison = core.eval("<").unwrap(); // 9 < 15 = true
    assert_eq!(comparison, Value::Bool(true));

    // 5. Final state should be clean
    assert_eq!(core.stack_depth(), 0);
}
