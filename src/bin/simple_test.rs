//! Simple test runner for the Chronos project
//!
//! This provides a straightforward way to run tests across all workspace components

use std::process::{Command, Stdio};
use std::time::Instant;

fn main() {
    println!("🧪 Chronos Project Test Runner");
    println!("==============================\n");

    let start_time = Instant::now();
    let mut all_passed = true;

    // Test chronos-core
    println!("🔍 Testing chronos-core...");
    if run_tests_for_crate("chronos-core") {
        println!("✅ chronos-core tests passed");
    } else {
        println!("❌ chronos-core tests failed");
        all_passed = false;
    }
    println!();

    // Test chronos-repl
    println!("🔍 Testing chronos-repl...");
    if run_tests_for_crate("chronos-repl") {
        println!("✅ chronos-repl tests passed");
    } else {
        println!("❌ chronos-repl tests failed");
        all_passed = false;
    }
    println!();

    // Run the demo to make sure examples work
    println!("🎯 Testing examples...");
    if run_demo() {
        println!("✅ Examples work correctly");
    } else {
        println!("❌ Examples failed");
        all_passed = false;
    }
    println!();

    // Summary
    let duration = start_time.elapsed();
    println!("📊 Test Summary");
    println!("===============");
    println!("Duration: {:?}", duration);

    if all_passed {
        println!("🎉 All tests passed!");
        std::process::exit(0);
    } else {
        println!("💥 Some tests failed. Check output above.");
        std::process::exit(1);
    }
}

fn run_tests_for_crate(crate_name: &str) -> bool {
    let output = Command::new("cargo")
        .args(&[
            "test",
            "--manifest-path",
            &format!("{}/Cargo.toml", crate_name),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(output) => {
            let success = output.status.success();
            if !success {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("  Error output: {}", stderr.trim());
            } else {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Some(test_count) = extract_test_count(&stdout) {
                    println!("  {} tests passed", test_count);
                }
            }
            success
        }
        Err(e) => {
            println!("  Failed to run tests: {}", e);
            false
        }
    }
}

fn run_demo() -> bool {
    let output = Command::new("cargo")
        .args(&["run", "--example", "demo_display"])
        .current_dir("chronos-repl")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn extract_test_count(output: &str) -> Option<usize> {
    for line in output.lines() {
        if line.contains("test result:") && line.contains("passed") {
            if let Some(passed_part) = line.split("passed").next() {
                if let Some(number_str) = passed_part.split_whitespace().last() {
                    if let Ok(count) = number_str.parse::<usize>() {
                        return Some(count);
                    }
                }
            }
        }
    }
    None
}
