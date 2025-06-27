# Testing Guide for Chronos Project

This document explains how to run tests across the entire Chronos project workspace.

## Quick Start

### Option 1: Using Make (Recommended)
```bash
# Run all tests across the entire project
make test         # Runs unit tests + demo verification

# Quick testing (minimal output)
make test-quick   # Fast unit tests only

# Other useful commands
make test-core    # Test only chronos-core
make test-repl    # Test only chronos-repl  
make demo         # Run the enhanced display demo
make dev-check    # Format, lint, and quick test
```

### Option 2: Using Cargo Directly
```bash
# Test individual components (recommended)
cd chronos-core && cargo test --lib
cd chronos-repl && cargo test --lib

# Run the enhanced display demo
cd chronos-repl && cargo run --example demo_display

# Quick workspace test (may have warnings)
cargo test --workspace --lib
```

### Option 3: Manual Testing
```bash
# Test each component individually
cargo test --package chronos-core
cargo test --package chronos-repl
cargo test --package chronos
```

## What Gets Tested

The comprehensive test suite (`make test` or `cargo run --bin test_all`) runs:

### For Each Workspace Member:
1. **Compilation Check** - Ensures code compiles without errors
2. **Unit Tests** - Tests individual functions and modules
3. **Integration Tests** - Tests component interactions
4. **Documentation Tests** - Tests code examples in docs
5. **Examples** - Runs example programs to ensure they work

### Components Tested:
- **chronos-core** - Core language implementation (18 unit tests)
- **chronos-repl** - Enhanced REPL with visual interface (41 unit tests)
- **demo_display** - Enhanced visual interface demonstration

### Sample Output:
```
🧪 Running Chronos test suite...
Testing chronos-core...

running 18 tests
test error::tests::test_error_category ... ok
test error::tests::test_error_context_manager ... ok
test error::tests::test_error_context ... ok
test error::tests::test_error_display ... ok
test core_lib::tests::test_help_generation ... ok
test core_lib::tests::test_core_library_creation ... ok
test lexer::tests::test_basic_tokens ... ok
test core_lib::tests::test_arithmetic_signatures ... ok
test lexer::tests::test_booleans ... ok
test lexer::tests::test_quotes ... ok
test lexer::tests::test_nested_comments ... ok
test lexer::tests::test_comments ... ok
test tests::test_tokenization ... ok
test tests::test_type_conversions ... ok
test tests::test_basic_arithmetic ... ok
test tests::test_error_handling ... ok
test tests::test_stack_operations ... ok
test tests::test_word_definition ... ok

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Testing chronos-repl...

running 41 tests
test commands::tests::test_invalid_command ... ok
test commands::tests::test_set_command ... ok
test commands::tests::test_unknown_command ... ok
test commands::tests::test_command_parsing ... ok
test display::tests::test_keyword_detection ... ok
test display::tests::test_duration_formatting ... ok
test display::tests::test_empty_stack ... ok
test display::tests::test_legacy_format_stack ... ok
test display::tests::test_stack_formatting ... ok
test display::tests::test_value_formatting ... ok
test display::tests::test_syntax_highlighting ... ok
test error::tests::test_error_conversions ... ok
test error::tests::test_error_creation_helpers ... ok
test error::tests::test_error_display ... ok
test error::tests::test_session_error_display ... ok
test repl::tests::test_configuration ... ok
test repl::tests::test_basic_evaluation ... ok
test repl::tests::test_command_handling ... ok
test session::tests::test_history_management ... ok
test session::tests::test_auto_save_logic ... ok
test session::tests::test_session_creation ... ok
test session::tests::test_session_reset ... ok
test repl::tests::test_repl_creation ... ok
test repl::tests::test_performance_metrics ... ok
test repl::tests::test_tracing ... ok
test session::tests::test_session_with_name ... ok
test session::tests::test_stack_update ... ok
test session::tests::test_tags_management ... ok
test session::tests::test_word_definition ... ok
test tests::test_eval_string_convenience ... ok
test tracing::tests::test_performance_classification ... ok
test tests::test_performance_metrics ... ok
test tests::test_tracing ... ok
test tracing::tests::test_trace_creation ... ok
test tracing::tests::test_token_performance ... ok
test tracing::tests::test_trace_entry ... ok
test tracing::tests::test_trace_filtering ... ok
test tests::test_basic_evaluation ... ok
test session::tests::test_session_save_load ... ok
test repl::tests::test_session_persistence ... ok
test tests::test_session_persistence ... ok

test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Testing enhanced display demo...
✅ Demo works

🎉 Basic tests complete!
```

## Test Categories

### Unit Tests
Located in `src/` files with `#[cfg(test)]` modules:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_something() {
        // Test implementation
    }
}
```

### Integration Tests
Located in `tests/` directories:
- `chronos-core/tests/` - Core functionality integration tests
- `chronos-repl/tests/` - REPL integration tests

### Documentation Tests
Code examples in documentation comments are automatically tested:
```rust
/// This function adds two numbers
/// 
/// # Examples
/// 
/// ```
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Examples
Runnable examples in `examples/` directories:
- `chronos-repl/examples/demo_display.rs` - Visual interface demo

## Continuous Integration

For CI/CD pipelines, use:
```bash
make ci
```

This runs:
1. `cargo clean` - Clean build
2. `cargo check --workspace` - Compilation check
3. `make test-quick` - Core unit tests
4. `cargo clippy --workspace` - Linting

For local development:
```bash
make dev-check  # Format, lint, and quick test
```

## Troubleshooting

### Common Issues

**Tests fail to compile:**
```bash
# Check for compilation errors
make check
```

**Slow tests:**
- Tests taking >30 seconds will trigger a performance warning
- Consider optimizing slow tests or marking them as `#[ignore]`

**Missing dependencies:**
```bash
# Update dependencies
cargo update
```

### Test-Specific Commands

**Run only fast tests:**
```bash
make test-quick   # Fastest option
cargo test --lib  # Only unit tests, skip integration tests
```

**Run with verbose output:**
```bash
cargo test -- --nocapture
```

**Run specific test:**
```bash
cargo test test_name
```

**Run tests in release mode:**
```bash
cargo test --release
```

## Writing Tests

### Guidelines

1. **Test naming**: Use descriptive names that explain what is being tested
2. **Test organization**: Group related tests in modules
3. **Test data**: Use meaningful test data that represents real usage
4. **Error cases**: Test both success and failure scenarios
5. **Documentation**: Include doc tests for public APIs

### Example Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stack_formatting_with_values() {
        let stack = vec![Value::Nat(42), Value::Bool(true)];
        let config = DisplayConfig::default();
        
        let result = format_stack_rich(&stack, &config);
        
        assert!(result.contains("Stack:"));
        assert!(result.contains("42"));
        assert!(result.contains("true"));
    }
    
    #[test]
    fn test_empty_stack_formatting() {
        let stack = vec![];
        let config = DisplayConfig::default();
        
        let result = format_stack_rich(&stack, &config);
        
        assert!(result.contains("empty"));
    }
}
```

## Performance Testing

While not included in the basic test suite, you can run performance tests:

```bash
# Benchmark tests (if available)
cargo bench

# Profile test execution
cargo test --release -- --test-threads=1
```

## Coverage

To check test coverage (requires additional tools):

```bash
# Install coverage tools
cargo install cargo-tarpaulin

# Run coverage analysis
cargo tarpaulin --workspace
```

## Integration with IDEs

### VS Code
- Install the `rust-analyzer` extension
- Tests will show inline run/debug buttons
- Use Ctrl+Shift+P → "Rust Analyzer: Run" for quick test execution

### IntelliJ/CLion
- Tests appear in the project tree with run icons
- Right-click any test to run individually
- Integrated test results panel

## Help

## Current Test Status

### Working Tests:
- ✅ **chronos-core**: 18 unit tests passing
- ✅ **chronos-repl**: 41 unit tests passing  
- ✅ **Enhanced display demo**: Working correctly
- ✅ **Visual interface**: All formatting functions tested

### Test Summary:
- **Total Unit Tests**: 59 tests passing
- **Integration Examples**: Demo program runs successfully
- **Visual Features**: Rich formatting, syntax highlighting, color schemes all tested

### Known Issues:
- Some integration tests have warnings but don't affect core functionality
- Workspace-wide tests may show warnings due to development code

For questions about testing:
1. Check this documentation
2. Look at existing tests for examples
3. Run `cargo test --help` for cargo options
4. Run `make help` for available make targets

Happy testing! 🧪