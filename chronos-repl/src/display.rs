//! Display utilities for REPL output formatting

use chronos_core::Value;

/// Format a stack for display
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

/// Format execution time for display
pub fn format_duration(duration: std::time::Duration) -> String {
    let nanos = duration.as_nanos();
    if nanos < 1_000 {
        format!("{}ns", nanos)
    } else if nanos < 1_000_000 {
        format!("{:.1}μs", nanos as f64 / 1_000.0)
    } else if nanos < 1_000_000_000 {
        format!("{:.1}ms", nanos as f64 / 1_000_000.0)
    } else {
        format!("{:.1}s", nanos as f64 / 1_000_000_000.0)
    }
}

/// Format a list of words for display
pub fn format_word_list(words: &[String]) -> String {
    if words.is_empty() {
        return "No words defined".to_string();
    }

    let mut result = String::new();
    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            result.push_str(", ");
        }
        result.push_str(word);
    }
    result
}

/// Format help text
pub fn format_help() -> String {
    r#"
Chronos REPL Commands:
  .help      - Show this help message
  .stack     - Show current stack contents
  .words     - List all defined words
  .clear     - Clear the stack
  .reset     - Reset the REPL state
  .trace     - Toggle execution tracing
  .save      - Save current session
  .load      - Load a session
  .quit      - Exit the REPL

For language help, see the documentation or examples.
"#
    .to_string()
}
