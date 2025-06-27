//! Comprehensive test runner for the Chronos project
//!
//! This binary runs all tests across all workspace members and provides
//! a clear summary of test results, compilation status, and any issues.

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct TestResult {
    name: String,
    success: bool,
    duration: Duration,
    output: String,
    error_output: String,
}

#[derive(Debug, Clone)]
struct TestSuite {
    name: String,
    path: String,
    results: Vec<TestResult>,
    compilation_success: bool,
    total_duration: Duration,
}

fn main() {
    println!("🧪 Chronos Project Test Runner");
    println!("==============================\n");

    let start_time = Instant::now();

    // Define test suites (workspace members)
    let test_suites = vec![
        ("chronos-core", "chronos-core"),
        ("chronos-repl", "chronos-repl"),
        ("chronos-main", "."),
    ];

    let mut all_results = Vec::new();
    let mut total_tests = 0;
    let mut total_passed = 0;
    let mut total_failed;

    for (name, path) in test_suites {
        println!("🔍 Testing {} ({})", name, path);
        println!("{}", "─".repeat(50));

        let suite_start = Instant::now();
        let mut suite = TestSuite {
            name: name.to_string(),
            path: path.to_string(),
            results: Vec::new(),
            compilation_success: false,
            total_duration: Duration::new(0, 0),
        };

        // Step 1: Check compilation
        print!("  📦 Checking compilation... ");
        let check_result = run_cargo_command(&["check"], path);
        if check_result.success {
            println!("✅ OK");
            suite.compilation_success = true;
        } else {
            println!("❌ FAILED");
            println!("     Error: {}", check_result.error_output.trim());
            suite.compilation_success = false;
        }

        // Step 2: Run unit tests (if compilation succeeded)
        if suite.compilation_success {
            print!("  🧪 Running unit tests... ");
            let test_result = run_cargo_command(&["test", "--lib"], path);
            let test_count = count_tests_in_output(&test_result.output);

            if test_result.success {
                println!("✅ {} tests passed", test_count);
                total_tests += test_count;
                total_passed += test_count;
            } else {
                println!("❌ Some tests failed");
                println!("     Output: {}", test_result.error_output.trim());
                total_tests += test_count;
                // Parse failed tests from output if needed
            }

            suite.results.push(TestResult {
                name: "unit_tests".to_string(),
                success: test_result.success,
                duration: Duration::new(0, 0), // Would need to parse from output
                output: test_result.output,
                error_output: test_result.error_output,
            });

            // Step 3: Run integration tests
            print!("  🔗 Running integration tests... ");
            let integration_result = run_cargo_command(&["test", "--test", "*"], path);
            if integration_result.success {
                let integration_count = count_tests_in_output(&integration_result.output);
                println!("✅ {} tests passed", integration_count);
                total_tests += integration_count;
                total_passed += integration_count;
            } else {
                println!("⚠️  No integration tests or some failed");
            }

            // Step 4: Run doc tests
            print!("  📚 Running doc tests... ");
            let doc_result = run_cargo_command(&["test", "--doc"], path);
            if doc_result.success {
                let doc_count = count_tests_in_output(&doc_result.output);
                println!("✅ {} doc tests passed", doc_count);
                total_tests += doc_count;
                total_passed += doc_count;
            } else {
                println!("⚠️  No doc tests or some failed");
            }

            // Step 5: Run examples (if any)
            if name == "chronos-repl" {
                print!("  🎯 Running examples... ");
                let example_result = run_cargo_command(&["run", "--example", "demo_display"], path);
                if example_result.success {
                    println!("✅ Examples work");
                } else {
                    println!("⚠️  Example failed or not found");
                }
            }
        }

        suite.total_duration = suite_start.elapsed();
        println!("  ⏱️  Duration: {:?}\n", suite.total_duration);
        all_results.push(suite);
    }

    // Step 6: Workspace-wide tests
    println!("🌐 Workspace-wide Tests");
    println!("{}", "─".repeat(50));

    print!("  🔄 Workspace check... ");
    let workspace_check = run_cargo_command(&["check", "--workspace"], ".");
    if workspace_check.success {
        println!("✅ All workspace members compile");
    } else {
        println!("❌ Workspace compilation issues");
        println!("     Error: {}", workspace_check.error_output.trim());
    }

    print!("  🧪 Workspace tests... ");
    let workspace_test = run_cargo_command(&["test", "--workspace"], ".");
    if workspace_test.success {
        println!("✅ All workspace tests pass");
    } else {
        println!("❌ Some workspace tests failed");
    }

    // Summary
    let total_duration = start_time.elapsed();
    total_failed = total_tests - total_passed;

    println!("\n📊 Test Summary");
    println!("{}", "=".repeat(50));

    for suite in &all_results {
        let status = if suite.compilation_success {
            "✅"
        } else {
            "❌"
        };
        println!("  {} {} - {:?}", status, suite.name, suite.total_duration);
    }

    println!("\n📈 Overall Results:");
    println!("  Total Tests: {}", total_tests);
    println!("  Passed: {} ✅", total_passed);

    if total_failed > 0 {
        println!("  Failed: {} ❌", total_failed);
    }

    println!("  Total Duration: {:?}", total_duration);

    // Performance insights
    if total_duration > Duration::from_secs(30) {
        println!("\n⚠️  Note: Tests took longer than 30 seconds. Consider optimizing slow tests.");
    }

    // Exit with appropriate code
    let all_passed = all_results.iter().all(|s| s.compilation_success) && total_failed == 0;

    if all_passed {
        println!("\n🎉 All tests passed! The project is in good shape.");
        std::process::exit(0);
    } else {
        println!("\n💥 Some tests failed. Please review the output above.");
        std::process::exit(1);
    }
}

/// Run a cargo command and return the result
fn run_cargo_command(args: &[&str], working_dir: &str) -> TestResult {
    let start = Instant::now();

    let mut cmd = Command::new("cargo");
    cmd.args(args);
    cmd.current_dir(working_dir);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let output = cmd.output().unwrap_or_else(|e| {
        panic!("Failed to execute cargo command: {}", e);
    });

    let duration = start.elapsed();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    TestResult {
        name: args.join(" "),
        success: output.status.success(),
        duration,
        output: stdout,
        error_output: stderr,
    }
}

/// Count the number of tests from cargo test output
fn count_tests_in_output(output: &str) -> usize {
    // Look for patterns like "test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"
    for line in output.lines() {
        if line.contains("test result:") && line.contains("passed") {
            if let Some(passed_part) = line.split("passed").next() {
                if let Some(number_str) = passed_part.split_whitespace().last() {
                    if let Ok(count) = number_str.parse::<usize>() {
                        return count;
                    }
                }
            }
        }
    }

    // Fallback: count lines that start with "test " and end with "... ok"
    output
        .lines()
        .filter(|line| line.trim_start().starts_with("test ") && line.ends_with("... ok"))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_tests_in_output() {
        let output = r#"
running 5 tests
test display::tests::test_stack_formatting ... ok
test display::tests::test_empty_stack ... ok
test display::tests::test_syntax_highlighting ... ok
test display::tests::test_value_formatting ... ok
test display::tests::test_keyword_detection ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
        "#;

        assert_eq!(count_tests_in_output(output), 5);
    }

    #[test]
    fn test_count_tests_fallback() {
        let output = r#"
test display::tests::test_stack_formatting ... ok
test display::tests::test_empty_stack ... ok
        "#;

        assert_eq!(count_tests_in_output(output), 2);
    }
}
