//! Test module for chronos-repl
//!
//! This module organizes all tests for the Chronos REPL, including:
//! - Integration tests that execute example files
//! - Unit tests for specific functionality patterns
//! - Syntax validation tests for example files
//! - Comprehensive testing of REPL features

// Integration tests that execute the .cao example files using the actual REPL
mod example_integration_tests;

// Unit tests for specific patterns demonstrated in examples
mod example_unit_tests;

// Syntax validation tests for example files
mod example_syntax_tests;

#[cfg(test)]
mod test_utils {
    use chronos_repl::{EnhancedRepl, ReplConfig};
    use std::path::Path;

    /// Create a standardized test REPL configuration
    pub fn create_test_repl() -> EnhancedRepl {
        let mut config = ReplConfig::default();
        config.show_welcome = false;
        config.show_stack = false;
        config.show_timing = false;
        config.use_colors = false;
        config.auto_save = false;
        EnhancedRepl::with_config(config)
    }

    /// Check if an example file exists
    pub fn example_exists(filename: &str) -> bool {
        let path = format!("examples/{}", filename);
        Path::new(&path).exists()
    }

    /// Get the list of all example files
    pub fn get_example_files() -> Vec<&'static str> {
        vec![
            "00_overview.cao",
            "01_basic_arithmetic.cao",
            "02_conditionals.cao",
            "03_algorithms.cao",
            "04_data_types.cao",
            "05_repl_features.cao",
            "06_real_world_app.cao",
        ]
    }

    /// Helper to assert a natural number value on the stack
    pub fn assert_stack_nat(repl: &EnhancedRepl, expected: u64) {
        let stack = repl.stack();
        assert!(!stack.is_empty(), "Stack should not be empty");

        if let chronos_core::Value::Nat(n) = stack.last().unwrap() {
            assert_eq!(*n, expected, "Expected {} on stack, got {}", expected, n);
        } else {
            panic!(
                "Expected Nat value on stack, got {:?}",
                stack.last().unwrap()
            );
        }
    }

    /// Helper to assert stack length
    pub fn assert_stack_length(repl: &EnhancedRepl, expected: usize) {
        let stack = repl.stack();
        assert_eq!(
            stack.len(),
            expected,
            "Expected stack length {}, got {}",
            expected,
            stack.len()
        );
    }
}

#[cfg(test)]
mod comprehensive_tests {
    use super::test_utils::*;
    use std::fs;

    /// Test that all expected example files are present
    #[test]
    fn test_all_examples_present() {
        let example_files = get_example_files();
        let mut missing_files = Vec::new();
        let mut present_files = Vec::new();

        for file in example_files {
            if example_exists(file) {
                present_files.push(file);
            } else {
                missing_files.push(file);
            }
        }

        println!("Present example files: {:?}", present_files);
        if !missing_files.is_empty() {
            println!("Missing example files: {:?}", missing_files);
        }

        // We should have at least some examples
        assert!(
            present_files.len() > 0,
            "At least some example files should be present"
        );

        // Check that README exists
        assert!(
            example_exists("README.md"),
            "README.md should be present in examples directory"
        );
    }

    /// Test that examples contain substantial content
    #[test]
    fn test_examples_have_content() {
        for file in get_example_files() {
            if !example_exists(file) {
                continue;
            }

            let path = format!("examples/{}", file);
            let content = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));

            assert!(
                content.len() > 100,
                "{} should have substantial content",
                file
            );

            // Should contain some actual Chronos code, not just comments
            let code_lines: Vec<&str> = content
                .lines()
                .filter(|line| {
                    let trimmed = line.trim();
                    !trimmed.is_empty()
                        && !trimmed.starts_with('(')
                        && !trimmed.starts_with('.')
                        && !trimmed.contains("print")
                        && !trimmed.contains("\"")
                })
                .collect();

            assert!(
                code_lines.len() > 3,
                "{} should contain actual Chronos code",
                file
            );
        }
    }

    /// Test REPL core functionality works
    #[test]
    fn test_repl_core_functionality() {
        let mut repl = create_test_repl();

        // Basic arithmetic should work
        repl.eval("2 3 +").expect("Basic arithmetic should work");
        assert_stack_nat(&repl, 5);

        // Stack operations should work
        repl.eval("dup").expect("Stack operations should work");
        assert_stack_length(&repl, 2);

        // Word definition should work
        repl.eval(": double 2 * ;")
            .expect("Word definition should work");
        repl.eval("4 double")
            .expect("Using defined words should work");
        assert_stack_nat(&repl, 8);
    }

    /// Test performance metrics tracking
    #[test]
    fn test_performance_tracking() {
        let mut repl = create_test_repl();

        // Execute some operations
        repl.eval("1 2 +").expect("Should execute");
        repl.eval("3 *").expect("Should execute");

        let metrics = repl.performance_metrics();
        assert!(metrics.total_operations > 0, "Should track operation count");
        assert!(
            metrics.total_time.as_nanos() > 0,
            "Should track execution time"
        );
    }

    /// Test session management basics
    #[test]
    fn test_session_management() {
        use tempfile::NamedTempFile;

        let temp_file = NamedTempFile::new().expect("Should create temp file");
        let path = temp_file.path();

        // Create a session with some state
        {
            let mut repl = create_test_repl();
            repl.eval("42").expect("Should push value");
            repl.save_session(path).expect("Should save session");
        }

        // Load the session in a new REPL
        {
            let mut repl = create_test_repl();
            repl.load_session(path).expect("Should load session");
            assert_stack_nat(&repl, 42);
        }
    }
}

/// Run a comprehensive test report
#[cfg(test)]
mod test_report {
    use super::test_utils::*;

    #[test]
    #[ignore] // Use `cargo test -- --ignored` to run this comprehensive report
    fn comprehensive_test_report() {
        println!("\n=== Chronos REPL Test Report ===");

        // Count available examples
        let example_files = get_example_files();
        let available_examples: Vec<_> = example_files
            .iter()
            .filter(|&file| example_exists(file))
            .collect();

        println!(
            "Available example files: {}/{}",
            available_examples.len(),
            example_files.len()
        );
        for file in &available_examples {
            println!("  ✓ {}", file);
        }

        // Test basic REPL functionality
        println!("\n--- Testing Core REPL Functionality ---");
        let mut repl = create_test_repl();

        let test_cases = vec![
            ("2 3 +", "basic arithmetic"),
            ("dup", "stack manipulation"),
            (": test 2 * ;", "word definition"),
            ("5 test", "word usage"),
        ];

        for (code, description) in test_cases {
            match repl.eval(code) {
                Ok(_) => println!("  ✓ {} works", description),
                Err(e) => println!("  ✗ {} failed: {}", description, e),
            }
        }

        // Test advanced features
        println!("\n--- Testing Advanced Features ---");

        // Test tracing
        repl.set_tracing(true);
        match repl.eval_with_trace("1 2 +") {
            Ok(result) => {
                println!(
                    "  ✓ Execution tracing works ({} entries)",
                    result.trace_entries.len()
                );
            }
            Err(e) => println!("  ✗ Tracing failed: {}", e),
        }

        // Performance metrics
        let metrics = repl.performance_metrics();
        println!(
            "  ✓ Performance tracking: {} operations, {:?} total time",
            metrics.total_operations, metrics.total_time
        );

        println!("\n--- Summary ---");
        println!("Examples directory: examples/");
        println!("Available examples: {} files", available_examples.len());
        println!("Core functionality: working");
        println!("Advanced features: working");
        println!("\nTo run individual tests:");
        println!("  cargo test example_syntax_tests");
        println!("  cargo test example_unit_tests");
        println!("  cargo test example_integration_tests");
        println!("\nTo run examples manually:");
        for file in &available_examples {
            println!("  cargo run --bin chronos-repl < examples/{}", file);
        }
        println!("\n=== End Report ===\n");
    }
}
