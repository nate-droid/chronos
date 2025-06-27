//! Enhanced display utilities for REPL output formatting
//!
//! This module provides rich visual formatting capabilities for both human and machine consumption,
//! including syntax highlighting, colorized output, and structured formatting.

use chronos_core::Value;
use console::{style, Color};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for display formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// Enable colorized output
    pub use_colors: bool,
    /// Show type information alongside values
    pub show_types: bool,
    /// Use compact stack display format
    pub compact_stack: bool,
    /// Enable syntax highlighting for input
    pub highlight_syntax: bool,
    /// Use Unicode symbols for enhanced display
    pub unicode_symbols: bool,
    /// Maximum number of stack items to display
    pub max_stack_items: usize,
    /// Maximum width for value display
    pub max_value_width: usize,
    /// Show execution timing information
    pub show_timing: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            use_colors: true,
            show_types: false,
            compact_stack: false,
            highlight_syntax: true,
            unicode_symbols: true,
            max_stack_items: 20,
            max_value_width: 80,
            show_timing: false,
        }
    }
}

/// Color scheme for different output elements
#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub number: Color,
    pub string: Color,
    pub keyword: Color,
    pub operator: Color,
    pub comment: Color,
    pub error: Color,
    pub success: Color,
    pub warning: Color,
    pub info: Color,
    pub stack_index: Color,
    pub type_info: Color,
    pub prompt: Color,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            number: Color::Cyan,
            string: Color::Green,
            keyword: Color::Blue,
            operator: Color::Magenta,
            comment: Color::Color256(244), // Gray
            error: Color::Red,
            success: Color::Green,
            warning: Color::Yellow,
            info: Color::Blue,
            stack_index: Color::Color256(244), // Gray
            type_info: Color::Color256(180),   // Light blue
            prompt: Color::Color256(75),       // Light green
        }
    }
}

/// Enhanced stack formatting with rich visual elements
pub fn format_stack_rich(stack: &[Value], config: &DisplayConfig) -> String {
    if stack.is_empty() {
        return format_empty_stack(config);
    }

    let items_to_show = config.max_stack_items.min(stack.len());
    let scheme = ColorScheme::default();

    let mut lines = Vec::new();

    // Header
    let header = if config.use_colors {
        style("Stack:").fg(scheme.info).bold().to_string()
    } else {
        "Stack:".to_string()
    };
    lines.push(header);

    // Show truncation indicator if needed
    if stack.len() > config.max_stack_items {
        let truncated = stack.len() - config.max_stack_items;
        let indicator = if config.use_colors {
            style(format!("  ... ({} more items)", truncated))
                .fg(scheme.stack_index)
                .italic()
                .to_string()
        } else {
            format!("  ... ({} more items)", truncated)
        };
        lines.push(indicator);
    }

    // Stack items (from top to bottom)
    for (display_idx, value) in stack.iter().rev().take(items_to_show).enumerate() {
        let actual_idx = stack.len() - 1 - display_idx;
        let formatted_item = format_stack_item(actual_idx, value, config, &scheme);
        lines.push(formatted_item);
    }

    lines.join("\n")
}

/// Format a single stack item with index and value
fn format_stack_item(
    index: usize,
    value: &Value,
    config: &DisplayConfig,
    scheme: &ColorScheme,
) -> String {
    let index_str = if config.use_colors {
        style(format!("[{}]", index))
            .fg(scheme.stack_index)
            .to_string()
    } else {
        format!("[{}]", index)
    };

    let value_str = format_value_with_type(value, config, scheme);

    if config.compact_stack {
        format!("{} {}", index_str, value_str)
    } else {
        format!("  {} {}", index_str, value_str)
    }
}

/// Format a value with optional type information and colors
pub fn format_value_with_type(
    value: &Value,
    config: &DisplayConfig,
    scheme: &ColorScheme,
) -> String {
    let value_str = format_value_colored(value, config, scheme);

    if config.show_types {
        let type_str = get_value_type_string(value);
        let type_display = if config.use_colors {
            style(format!(" : {}", type_str))
                .fg(scheme.type_info)
                .italic()
                .to_string()
        } else {
            format!(" : {}", type_str)
        };
        format!("{}{}", value_str, type_display)
    } else {
        value_str
    }
}

/// Format a value with appropriate coloring
fn format_value_colored(value: &Value, config: &DisplayConfig, scheme: &ColorScheme) -> String {
    let display_str = truncate_if_needed(&value.to_string(), config.max_value_width);

    if !config.use_colors {
        return display_str;
    }

    match value {
        Value::Unit => display_str,
        Value::Nat(_) => style(display_str).fg(scheme.number).to_string(),
        Value::Bool(_) => style(display_str).fg(scheme.keyword).to_string(),
        Value::String(_) => style(display_str).fg(scheme.string).to_string(),
        Value::Quote(_) => style(display_str).fg(scheme.string).to_string(),
        Value::Ordinal(_) => style(display_str).fg(scheme.number).to_string(),
        Value::Composite { .. } => style(display_str).fg(scheme.type_info).to_string(),
        Value::Option(_) => style(display_str).fg(scheme.type_info).to_string(),
        Value::Result(_) => style(display_str).fg(scheme.type_info).to_string(),
        Value::List(_) => style(display_str).fg(scheme.string).to_string(),
    }
}

/// Get the type string for a value
fn get_value_type_string(value: &Value) -> String {
    match value {
        Value::Unit => "Unit".to_string(),
        Value::Bool(_) => "Bool".to_string(),
        Value::Nat(_) => "Nat".to_string(),
        Value::String(_) => "String".to_string(),
        Value::Quote(_) => "Quote".to_string(),
        Value::Ordinal(_) => "Ordinal".to_string(),
        Value::Composite { type_name, .. } => type_name.clone(),
        Value::Option(_) => "Option".to_string(),
        Value::Result(_) => "Result".to_string(),
        Value::List(_) => "List".to_string(),
    }
}

/// Format empty stack display
fn format_empty_stack(config: &DisplayConfig) -> String {
    let scheme = ColorScheme::default();

    if config.unicode_symbols && config.use_colors {
        style("Stack: ∅ (empty)")
            .fg(scheme.stack_index)
            .italic()
            .to_string()
    } else if config.use_colors {
        style("Stack: <empty>")
            .fg(scheme.stack_index)
            .italic()
            .to_string()
    } else {
        "Stack: <empty>".to_string()
    }
}

/// Highlight syntax in input code
pub fn highlight_syntax(input: &str, config: &DisplayConfig) -> String {
    if !config.highlight_syntax || !config.use_colors {
        return input.to_string();
    }

    let scheme = ColorScheme::default();
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            // Numbers
            '0'..='9' => {
                let mut number = String::from(ch);
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() {
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                result.push_str(&style(number).fg(scheme.number).to_string());
            }
            // Operators
            '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' => {
                result.push_str(&style(ch).fg(scheme.operator).to_string());
            }
            // Comments (assuming parentheses-style comments)
            '(' => {
                let mut comment = String::from(ch);
                let mut depth = 1;
                while let Some(next_ch) = chars.next() {
                    comment.push(next_ch);
                    match next_ch {
                        '(' => depth += 1,
                        ')' => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                result.push_str(&style(comment).fg(scheme.comment).italic().to_string());
            }
            // Keywords and identifiers
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut word = String::from(ch);
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric()
                        || next_ch == '_'
                        || next_ch == '-'
                        || next_ch == '?'
                    {
                        word.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                if is_keyword(&word) {
                    result.push_str(&style(word).fg(scheme.keyword).bold().to_string());
                } else {
                    result.push_str(&word);
                }
            }
            // Quotations
            '[' => {
                result.push_str(&style(ch).fg(scheme.string).to_string());
            }
            ']' => {
                result.push_str(&style(ch).fg(scheme.string).to_string());
            }
            // Default
            _ => result.push(ch),
        }
    }

    result
}

/// Check if a word is a keyword
fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "if" | "unless"
            | "when"
            | "while"
            | "dup"
            | "drop"
            | "swap"
            | "over"
            | "rot"
            | "clear"
            | "depth"
            | "times"
            | "call"
            | "true"
            | "false"
            | "and"
            | "or"
            | "not"
            | "type"
            | "axiom"
            | "cast"
    )
}

/// Format execution time for display
pub fn format_duration(duration: Duration, config: &DisplayConfig) -> String {
    let scheme = ColorScheme::default();
    let nanos = duration.as_nanos();

    let time_str = if nanos < 1_000 {
        format!("{}ns", nanos)
    } else if nanos < 1_000_000 {
        format!("{:.1}μs", nanos as f64 / 1_000.0)
    } else if nanos < 1_000_000_000 {
        format!("{:.1}ms", nanos as f64 / 1_000_000.0)
    } else {
        format!("{:.1}s", nanos as f64 / 1_000_000_000.0)
    };

    if config.use_colors {
        style(time_str).fg(scheme.info).to_string()
    } else {
        time_str
    }
}

/// Format a list of words for display
pub fn format_word_list(words: &[String], config: &DisplayConfig) -> String {
    if words.is_empty() {
        let scheme = ColorScheme::default();
        return if config.use_colors {
            style("No words defined")
                .fg(scheme.stack_index)
                .italic()
                .to_string()
        } else {
            "No words defined".to_string()
        };
    }

    let scheme = ColorScheme::default();
    let mut result = String::new();
    let max_line_length = 80;
    let mut current_line_length = 0;

    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            if current_line_length + word.len() + 2 > max_line_length {
                result.push('\n');
                current_line_length = 0;
            } else {
                result.push_str("  ");
                current_line_length += 2;
            }
        }

        let formatted_word = if config.use_colors {
            style(word).fg(scheme.keyword).to_string()
        } else {
            word.clone()
        };

        result.push_str(&formatted_word);
        current_line_length += word.len();
    }

    result
}

/// Format error messages with enhanced styling
pub fn format_error(message: &str, config: &DisplayConfig) -> String {
    let scheme = ColorScheme::default();

    if config.use_colors {
        let prefix = style("Error:").fg(scheme.error).bold();
        format!("{} {}", prefix, message)
    } else {
        format!("Error: {}", message)
    }
}

/// Format success messages
pub fn format_success(message: &str, config: &DisplayConfig) -> String {
    let scheme = ColorScheme::default();

    if config.use_colors {
        if config.unicode_symbols {
            let prefix = style("✓").fg(scheme.success).bold();
            format!("{} {}", prefix, message)
        } else {
            let prefix = style("OK:").fg(scheme.success).bold();
            format!("{} {}", prefix, message)
        }
    } else {
        message.to_string()
    }
}

/// Format warning messages
pub fn format_warning(message: &str, config: &DisplayConfig) -> String {
    let scheme = ColorScheme::default();

    if config.use_colors {
        if config.unicode_symbols {
            let prefix = style("⚠").fg(scheme.warning).bold();
            format!("{} {}", prefix, message)
        } else {
            let prefix = style("Warning:").fg(scheme.warning).bold();
            format!("{} {}", prefix, message)
        }
    } else {
        format!("Warning: {}", message)
    }
}

/// Format info messages
pub fn format_info(message: &str, config: &DisplayConfig) -> String {
    let scheme = ColorScheme::default();

    if config.use_colors {
        if config.unicode_symbols {
            let prefix = style("ℹ").fg(scheme.info);
            format!("{} {}", prefix, message)
        } else {
            let prefix = style("Info:").fg(scheme.info);
            format!("{} {}", prefix, message)
        }
    } else {
        message.to_string()
    }
}

/// Format help text with enhanced styling
pub fn format_help(config: &DisplayConfig) -> String {
    let scheme = ColorScheme::default();

    let sections = vec![
        (
            "Basic Commands:",
            vec![
                (".help", "Show this help message"),
                (".stack", "Show current stack contents"),
                (".words", "List all defined words"),
                (".clear", "Clear the stack"),
                (".reset", "Reset the REPL state"),
                (".quit", "Exit the REPL"),
            ],
        ),
        (
            "Session Management:",
            vec![
                (".save [file]", "Save current session"),
                (".load <file>", "Load a session"),
                (".history", "Show command history"),
            ],
        ),
        (
            "Debugging & Analysis:",
            vec![
                (".trace", "Toggle execution tracing"),
                (".metrics", "Show performance metrics"),
                (".showtrace [n]", "Show last n trace entries"),
            ],
        ),
        (
            "Display Options:",
            vec![
                (".colors", "Toggle colored output"),
                (".types", "Toggle type display"),
                (".compact", "Toggle compact display"),
            ],
        ),
    ];

    let mut result = String::new();

    if config.use_colors {
        result.push_str(
            &style("Chronos REPL Commands")
                .fg(scheme.info)
                .bold()
                .underlined()
                .to_string(),
        );
    } else {
        result.push_str("Chronos REPL Commands");
        result.push_str("\n====================");
    }
    result.push_str("\n\n");

    for (section_title, commands) in sections {
        if config.use_colors {
            result.push_str(&style(section_title).fg(scheme.keyword).bold().to_string());
        } else {
            result.push_str(section_title);
        }
        result.push('\n');

        for (command, description) in commands {
            if config.use_colors {
                let cmd_style = style(format!("  {:<15}", command)).fg(scheme.operator);
                result.push_str(&format!("{} {}\n", cmd_style, description));
            } else {
                result.push_str(&format!("  {:<15} {}\n", command, description));
            }
        }
        result.push('\n');
    }

    result.push_str("For language help, see the documentation or examples.\n");
    result
}

/// Truncate string if it exceeds maximum width
fn truncate_if_needed(s: &str, max_width: usize) -> String {
    if s.len() <= max_width {
        s.to_string()
    } else {
        format!("{}...", &s[..max_width.saturating_sub(3)])
    }
}

/// Format a banner or section divider
pub fn format_banner(text: &str, config: &DisplayConfig) -> String {
    let scheme = ColorScheme::default();
    let width = 60;

    if config.use_colors {
        let border = "═".repeat(width);
        let styled_border = style(&border).fg(scheme.info);
        let styled_text = style(text).fg(scheme.info).bold();

        format!(
            "{}\n{:^width$}\n{}",
            styled_border,
            styled_text,
            styled_border,
            width = width
        )
    } else {
        let border = "=".repeat(width);
        format!("{}\n{:^width$}\n{}", border, text, border, width = width)
    }
}

/// Create a progress indicator
pub fn format_progress(current: usize, total: usize, config: &DisplayConfig) -> String {
    let scheme = ColorScheme::default();
    let percentage = (current as f64 / total as f64 * 100.0) as usize;

    if config.unicode_symbols && config.use_colors {
        let filled = "█".repeat(percentage / 5);
        let empty = "░".repeat(20 - percentage / 5);
        let bar = format!("{}{}", filled, empty);
        style(format!("[{}] {}%", bar, percentage))
            .fg(scheme.info)
            .to_string()
    } else {
        format!("[{}/{}] {}%", current, total, percentage)
    }
}

/// Basic stack formatting for compatibility (legacy function)
pub fn format_stack(stack: &[Value], max_items: usize) -> String {
    if stack.is_empty() {
        return "Stack: <empty>".to_string();
    }

    let items: Vec<String> = stack
        .iter()
        .rev()
        .take(max_items)
        .enumerate()
        .map(|(i, value)| format!("[{}] {}", stack.len() - 1 - i, value))
        .collect();

    format!("Stack:\n{}", items.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chronos_core::Value;

    #[test]
    fn test_stack_formatting() {
        let config = DisplayConfig::default();
        let stack = vec![Value::Nat(42), Value::Bool(true), Value::Nat(7)];

        let formatted = format_stack_rich(&stack, &config);
        assert!(formatted.contains("Stack:"));
        assert!(formatted.contains("[2]"));
        assert!(formatted.contains("7"));
    }

    #[test]
    fn test_empty_stack() {
        let config = DisplayConfig::default();
        let stack = vec![];

        let formatted = format_stack_rich(&stack, &config);
        assert!(formatted.contains("empty"));
    }

    #[test]
    fn test_syntax_highlighting() {
        let config = DisplayConfig::default();
        let input = "3 4 + dup";

        let highlighted = highlight_syntax(input, &config);
        // With colors disabled for testing, should return original
        let no_color_config = DisplayConfig {
            use_colors: false,
            ..config
        };
        let plain = highlight_syntax(input, &no_color_config);
        assert_eq!(plain, input);
    }

    #[test]
    fn test_value_formatting() {
        let config = DisplayConfig {
            show_types: true,
            ..DisplayConfig::default()
        };
        let scheme = ColorScheme::default();
        let value = Value::Nat(42);

        let formatted = format_value_with_type(&value, &config, &scheme);
        assert!(formatted.contains("42"));
        assert!(formatted.contains("Nat"));
    }

    #[test]
    fn test_keyword_detection() {
        assert!(is_keyword("if"));
        assert!(is_keyword("dup"));
        assert!(!is_keyword("myword"));
    }

    #[test]
    fn test_duration_formatting() {
        let config = DisplayConfig::default();
        let duration = Duration::from_nanos(1500);

        let formatted = format_duration(duration, &config);
        assert!(formatted.contains("μs") || formatted.contains("ns"));
    }

    #[test]
    fn test_legacy_format_stack() {
        let stack = vec![Value::Nat(42), Value::Bool(true), Value::Nat(7)];
        let formatted = format_stack(&stack, 10);
        assert!(formatted.contains("Stack:"));
        assert!(formatted.contains("[2]"));
        assert!(formatted.contains("7"));
    }
}
