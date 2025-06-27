//! Demo program to showcase the enhanced visual interface
//!
//! This example demonstrates the rich formatting capabilities of the enhanced REPL display system.

use chronos_core::Value;
use chronos_repl::display::{
    DisplayConfig, format_banner, format_duration, format_error, format_help, format_info,
    format_stack_rich, format_success, format_warning, format_word_list,
};
use std::time::Duration;

fn main() {
    println!("🎨 Chronos REPL Enhanced Display Demo");
    println!("=====================================\n");

    // Create different display configurations
    let rich_config = DisplayConfig {
        use_colors: true,
        show_types: true,
        compact_stack: false,
        highlight_syntax: true,
        unicode_symbols: true,
        max_stack_items: 10,
        max_value_width: 80,
        show_timing: true,
    };

    let minimal_config = DisplayConfig {
        use_colors: false,
        show_types: false,
        compact_stack: true,
        highlight_syntax: false,
        unicode_symbols: false,
        max_stack_items: 5,
        max_value_width: 40,
        show_timing: false,
    };

    // Demo 1: Banner formatting
    println!("📋 Banner Formatting:");
    println!(
        "{}",
        format_banner("Welcome to C∀O Enhanced REPL", &rich_config)
    );
    println!();

    // Demo 2: Stack display with different configurations
    println!("📚 Stack Display Examples:");
    let sample_stack = vec![
        Value::Nat(42),
        Value::Bool(true),
        Value::Nat(1337),
        Value::Quote(vec![]),
        Value::Bool(false),
    ];

    println!("Rich Configuration (with colors and types):");
    println!("{}", format_stack_rich(&sample_stack, &rich_config));
    println!();

    println!("Minimal Configuration (compact, no colors):");
    println!("{}", format_stack_rich(&sample_stack, &minimal_config));
    println!();

    // Demo 3: Message formatting
    println!("💬 Message Formatting:");
    println!(
        "{}",
        format_success("Operation completed successfully!", &rich_config)
    );
    println!("{}", format_info("Stack depth is now 5", &rich_config));
    println!(
        "{}",
        format_warning("This operation may take some time", &rich_config)
    );
    println!(
        "{}",
        format_error("Division by zero attempted", &rich_config)
    );
    println!();

    // Demo 4: Duration formatting
    println!("⏱️  Timing Information:");
    let durations = vec![
        Duration::from_nanos(500),
        Duration::from_micros(150),
        Duration::from_millis(25),
        Duration::from_secs(2),
    ];

    for duration in durations {
        println!(
            "  Execution time: {}",
            format_duration(duration, &rich_config)
        );
    }
    println!();

    // Demo 5: Word list formatting
    println!("📖 Word List Formatting:");
    let words = vec![
        "dup".to_string(),
        "swap".to_string(),
        "over".to_string(),
        "rot".to_string(),
        "drop".to_string(),
        "if".to_string(),
        "unless".to_string(),
        "while".to_string(),
    ];

    println!("Available words:");
    println!("{}", format_word_list(&words, &rich_config));
    println!();

    // Demo 6: Help formatting
    println!("❓ Help System Preview:");
    let help_preview = format_help(&rich_config);
    // Show just the first few lines
    for (i, line) in help_preview.lines().enumerate() {
        if i < 8 {
            println!("{}", line);
        } else if i == 8 {
            println!("... (truncated for demo)");
            break;
        }
    }
    println!();

    println!("✨ Demo complete! The enhanced display system provides:");
    println!("  • Rich color formatting and syntax highlighting");
    println!("  • Configurable display options (colors, types, compact mode)");
    println!("  • Unicode symbols and visual enhancements");
    println!("  • Consistent formatting for errors, warnings, and info");
    println!("  • Flexible configuration for both humans and agents");
}

/// Simple helper to strip ANSI escape codes for width calculation
fn strip_ansi_codes(s: &str) -> String {
    // Simple regex-free approach to strip basic ANSI codes
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Skip ANSI escape sequence
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['
                while let Some(c) = chars.next() {
                    if c.is_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
}
