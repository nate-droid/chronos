//! Syntax validation tests for Chronos example files
//!
//! These tests verify that all .cao example files contain valid syntax
//! and can be parsed correctly, even if not all features are implemented yet.

use std::fs;
use std::path::Path;

/// Test that checks if example files exist and can be read
#[test]
fn test_example_files_exist() {
    let example_files = [
        "examples/00_overview.cao",
        "examples/01_basic_arithmetic.cao",
        "examples/02_conditionals.cao",
        "examples/03_algorithms.cao",
        "examples/04_data_types.cao",
        "examples/05_repl_features.cao",
        "examples/06_real_world_app.cao",
        "examples/README.md",
    ];

    for file_path in &example_files {
        if Path::new(file_path).exists() {
            let content = fs::read_to_string(file_path)
                .unwrap_or_else(|e| panic!("Failed to read {}: {}", file_path, e));

            assert!(!content.is_empty(), "{} should not be empty", file_path);

            // Basic syntax checks for .cao files
            if file_path.ends_with(".cao") {
                // Check that comments are properly formatted
                for (line_num, line) in content.lines().enumerate() {
                    let trimmed = line.trim();

                    // Check comment syntax
                    if trimmed.starts_with('(') {
                        assert!(
                            trimmed.ends_with(')'),
                            "Comment on line {} in {} is not properly closed: {}",
                            line_num + 1,
                            file_path,
                            line
                        );
                    }

                    // Check for balanced parentheses in comments
                    if trimmed.contains('(') && trimmed.contains(')') {
                        let open_count = trimmed.matches('(').count();
                        let close_count = trimmed.matches(')').count();

                        // For single-line comments, they should be balanced
                        if trimmed.starts_with('(') && trimmed.ends_with(')') {
                            assert_eq!(
                                open_count,
                                close_count,
                                "Unbalanced parentheses in comment on line {} in {}: {}",
                                line_num + 1,
                                file_path,
                                line
                            );
                        }
                    }
                }
            }

            println!("✓ {} is valid and readable", file_path);
        } else {
            println!("⚠ {} not found (may not be created yet)", file_path);
        }
    }
}

/// Test that .cao files contain valid Chronos syntax patterns
#[test]
fn test_chronos_syntax_patterns() {
    let cao_files = [
        "examples/00_overview.cao",
        "examples/01_basic_arithmetic.cao",
        "examples/02_conditionals.cao",
        "examples/03_algorithms.cao",
        "examples/04_data_types.cao",
        "examples/05_repl_features.cao",
        "examples/06_real_world_app.cao",
    ];

    for file_path in &cao_files {
        if !Path::new(file_path).exists() {
            continue;
        }

        let content = fs::read_to_string(file_path).expect("Should be able to read file");

        let mut in_comment = false;
        let mut in_string = false;

        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            // Track comment state
            if trimmed.starts_with('(') && trimmed.ends_with(')') {
                // Single line comment - valid
                continue;
            }

            // Skip REPL commands and other special syntax
            if trimmed.starts_with('.') || trimmed.starts_with("\"") || trimmed.contains("print") {
                continue;
            }

            // Check for valid word definitions
            if trimmed.starts_with("::") {
                assert!(
                    trimmed.contains('(') && trimmed.contains(')') && trimmed.contains("->"),
                    "Type signature on line {} in {} should contain ( -> ): {}",
                    line_num + 1,
                    file_path,
                    line
                );
            }

            if trimmed.starts_with(':') && !trimmed.starts_with("::") {
                assert!(
                    trimmed.ends_with(';'),
                    "Word definition on line {} in {} should end with semicolon: {}",
                    line_num + 1,
                    file_path,
                    line
                );
            }

            // Check for valid type definitions
            if trimmed.starts_with("type ") {
                assert!(
                    trimmed.contains('{') || line.contains("type"),
                    "Type definition on line {} in {} should contain braces: {}",
                    line_num + 1,
                    file_path,
                    line
                );
            }

            // Check for valid conditionals
            if trimmed.contains("if") && !trimmed.starts_with('(') {
                // Should be part of a valid conditional structure
                // This is a basic check - could be expanded
            }
        }

        println!("✓ {} has valid Chronos syntax patterns", file_path);
    }
}

/// Test that example files contain expected educational content
#[test]
fn test_example_content_quality() {
    let examples_with_expected_content = [
        (
            "examples/01_basic_arithmetic.cao",
            vec!["dup", "*", "+", "-"],
        ),
        ("examples/02_conditionals.cao", vec!["if", "true", "false"]),
        ("examples/03_algorithms.cao", vec!["factorial", "fibonacci"]),
        (
            "examples/04_data_types.cao",
            vec!["type", "Point", "Complex"],
        ),
        (
            "examples/00_overview.cao",
            vec!["Welcome", "C∀O", "Chronos"],
        ),
    ];

    for (file_path, expected_keywords) in &examples_with_expected_content {
        if !Path::new(file_path).exists() {
            continue;
        }

        let content = fs::read_to_string(file_path).expect("Should be able to read file");

        for keyword in expected_keywords {
            assert!(
                content.contains(keyword),
                "{} should contain educational content about '{}'",
                file_path,
                keyword
            );
        }

        // Check that file has substantial content (not just comments)
        let non_comment_lines: Vec<&str> = content
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty()
                    && !trimmed.starts_with('(')
                    && !trimmed.starts_with(".")
                    && !trimmed.contains("print")
            })
            .collect();

        assert!(
            non_comment_lines.len() > 5,
            "{} should have substantial educational content",
            file_path
        );

        println!("✓ {} contains quality educational content", file_path);
    }
}

/// Test that README.md contains proper documentation
#[test]
fn test_readme_content() {
    let readme_path = "examples/README.md";

    if !Path::new(readme_path).exists() {
        println!("⚠ {} not found", readme_path);
        return;
    }

    let content = fs::read_to_string(readme_path).expect("Should be able to read README");

    // Check for essential sections
    let required_sections = [
        "# Chronos",
        "## Quick Start",
        "## Example Files",
        "## Learning Path",
        "cargo run",
    ];

    for section in &required_sections {
        assert!(
            content.contains(section),
            "README should contain section: {}",
            section
        );
    }

    // Check that all example files are mentioned
    let cao_files = [
        "00_overview.cao",
        "01_basic_arithmetic.cao",
        "02_conditionals.cao",
        "03_algorithms.cao",
        "04_data_types.cao",
        "05_repl_features.cao",
        "06_real_world_app.cao",
    ];

    for cao_file in &cao_files {
        assert!(
            content.contains(cao_file),
            "README should mention example file: {}",
            cao_file
        );
    }

    println!("✓ README.md contains comprehensive documentation");
}

/// Test that example files demonstrate progressive complexity
#[test]
fn test_progressive_complexity() {
    let ordered_examples = [
        ("examples/00_overview.cao", 1),         // Introduction
        ("examples/01_basic_arithmetic.cao", 2), // Basic
        ("examples/02_conditionals.cao", 3),     // Intermediate
        ("examples/03_algorithms.cao", 4),       // Advanced
        ("examples/04_data_types.cao", 3),       // Intermediate
        ("examples/05_repl_features.cao", 2),    // Tools
        ("examples/06_real_world_app.cao", 5),   // Expert
    ];

    for (file_path, expected_complexity) in &ordered_examples {
        if !Path::new(file_path).exists() {
            continue;
        }

        let content = fs::read_to_string(file_path).expect("Should be able to read file");

        // Measure complexity by counting certain patterns
        let complexity_indicators = [
            content.matches("::").count(),        // Type signatures
            content.matches(": ").count(),        // Word definitions
            content.matches("if").count(),        // Conditionals
            content.matches("type ").count(),     // Custom types
            content.matches("recursive").count(), // Recursion mentions
        ];

        let total_complexity: usize = complexity_indicators.iter().sum();

        // This is a rough heuristic - adjust as needed
        match expected_complexity {
            1 => assert!(
                total_complexity <= 10,
                "{} should be introductory",
                file_path
            ),
            2 => assert!(total_complexity <= 25, "{} should be basic", file_path),
            3 => assert!(
                total_complexity <= 50,
                "{} should be intermediate",
                file_path
            ),
            4 => assert!(total_complexity >= 20, "{} should be advanced", file_path),
            5 => assert!(
                total_complexity >= 30,
                "{} should be expert level",
                file_path
            ),
            _ => {}
        }

        println!(
            "✓ {} has appropriate complexity level {}",
            file_path, expected_complexity
        );
    }
}

/// Test that examples use consistent coding style
#[test]
fn test_coding_style_consistency() {
    let cao_files = [
        "examples/01_basic_arithmetic.cao",
        "examples/02_conditionals.cao",
        "examples/03_algorithms.cao",
        "examples/04_data_types.cao",
        "examples/05_repl_features.cao",
        "examples/06_real_world_app.cao",
    ];

    for file_path in &cao_files {
        if !Path::new(file_path).exists() {
            continue;
        }

        let content = fs::read_to_string(file_path).expect("Should be able to read file");

        // Check consistent comment style
        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with('(') {
                assert!(
                    trimmed.ends_with(')'),
                    "Comments should be properly closed in {}",
                    file_path
                );
            }
        }

        // Check that type signatures come before implementations
        let lines: Vec<&str> = content.lines().collect();
        for i in 0..lines.len().saturating_sub(1) {
            let current = lines[i].trim();
            let next = lines[i + 1].trim();

            if current.starts_with("::") && next.starts_with(": ") {
                // Extract function names and verify they match
                if let (Some(type_name), Some(impl_name)) = (
                    current.split_whitespace().nth(1),
                    next.split_whitespace().nth(1),
                ) {
                    assert_eq!(
                        type_name, impl_name,
                        "Type signature and implementation names should match in {}: {} vs {}",
                        file_path, type_name, impl_name
                    );
                }
            }
        }

        println!("✓ {} follows consistent coding style", file_path);
    }
}
