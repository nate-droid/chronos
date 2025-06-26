//! REPL command handling for the Chronos interactive environment
//!
//! This module provides command parsing and execution for special REPL commands
//! that begin with a dot (.), such as .help, .stack, .save, etc.

use crate::error::{ReplError, Result};
use crate::session::Session;
use crate::tracing::ExecutionTrace;
use chronos_core::Value;
use std::path::Path;

/// A REPL command that can be executed
#[derive(Debug, Clone, PartialEq)]
pub enum ReplCommand {
    /// Show help information
    Help,

    /// Show current stack contents
    Stack,

    /// List all defined words
    Words,

    /// Clear the stack
    Clear,

    /// Reset the REPL state
    Reset,

    /// Toggle execution tracing
    Trace(Option<bool>),

    /// Save session to file
    Save(Option<String>),

    /// Load session from file
    Load(String),

    /// Show performance metrics
    Metrics,

    /// Show command history
    History,

    /// Show execution trace
    ShowTrace(Option<usize>),

    /// Benchmark code execution
    Benchmark(String),

    /// Set configuration option
    Set(String, String),

    /// Show about information
    About,

    /// Exit the REPL
    Quit,

    /// Unknown command
    Unknown(String),
}

/// Parse a command string into a ReplCommand
pub fn parse_command(input: &str) -> ReplCommand {
    let input = input.trim();

    if !input.starts_with('.') {
        return ReplCommand::Unknown(input.to_string());
    }

    let parts: Vec<&str> = input[1..].split_whitespace().collect();
    if parts.is_empty() {
        return ReplCommand::Unknown(input.to_string());
    }

    match parts[0] {
        "help" | "h" => ReplCommand::Help,
        "stack" | "s" => ReplCommand::Stack,
        "words" | "w" => ReplCommand::Words,
        "clear" | "c" => ReplCommand::Clear,
        "reset" | "r" => ReplCommand::Reset,
        "quit" | "q" | "exit" => ReplCommand::Quit,
        "about" => ReplCommand::About,
        "metrics" | "m" => ReplCommand::Metrics,
        "history" => ReplCommand::History,

        "trace" | "t" => {
            if parts.len() > 1 {
                match parts[1] {
                    "on" | "true" | "1" => ReplCommand::Trace(Some(true)),
                    "off" | "false" | "0" => ReplCommand::Trace(Some(false)),
                    _ => ReplCommand::Trace(None),
                }
            } else {
                ReplCommand::Trace(None)
            }
        }

        "save" => {
            if parts.len() > 1 {
                ReplCommand::Save(Some(parts[1..].join(" ")))
            } else {
                ReplCommand::Save(None)
            }
        }

        "load" => {
            if parts.len() > 1 {
                ReplCommand::Load(parts[1..].join(" "))
            } else {
                ReplCommand::Unknown(input.to_string())
            }
        }

        "trace-show" | "showtrace" => {
            if parts.len() > 1 {
                if let Ok(count) = parts[1].parse::<usize>() {
                    ReplCommand::ShowTrace(Some(count))
                } else {
                    ReplCommand::ShowTrace(None)
                }
            } else {
                ReplCommand::ShowTrace(None)
            }
        }

        "benchmark" | "bench" => {
            if parts.len() > 1 {
                ReplCommand::Benchmark(parts[1..].join(" "))
            } else {
                ReplCommand::Unknown(input.to_string())
            }
        }

        "set" => {
            if parts.len() >= 3 {
                ReplCommand::Set(parts[1].to_string(), parts[2..].join(" "))
            } else {
                ReplCommand::Unknown(input.to_string())
            }
        }

        _ => ReplCommand::Unknown(input.to_string()),
    }
}

/// Execute a REPL command
pub fn execute_command(
    command: ReplCommand,
    stack: &mut Vec<Value>,
    session: &mut Session,
    trace: &mut ExecutionTrace,
    tracing_enabled: &mut bool,
) -> Result<String> {
    match command {
        ReplCommand::Help => Ok(get_help_text()),

        ReplCommand::Stack => {
            if stack.is_empty() {
                Ok("Stack is empty".to_string())
            } else {
                let mut result = String::new();
                for (i, value) in stack.iter().enumerate() {
                    result.push_str(&format!("[{}] {}\n", i, value));
                }
                Ok(result.trim_end().to_string())
            }
        }

        ReplCommand::Words => {
            let words: Vec<String> = session.user_words().keys().cloned().collect();
            if words.is_empty() {
                Ok("No user-defined words".to_string())
            } else {
                Ok(format!("Defined words: {}", words.join(", ")))
            }
        }

        ReplCommand::Clear => {
            stack.clear();
            session.update_stack(stack.clone());
            Ok("Stack cleared".to_string())
        }

        ReplCommand::Reset => {
            stack.clear();
            session.reset();
            trace.clear();
            Ok("REPL state reset".to_string())
        }

        ReplCommand::Trace(setting) => match setting {
            Some(enabled) => {
                *tracing_enabled = enabled;
                if enabled {
                    Ok("Execution tracing enabled".to_string())
                } else {
                    Ok("Execution tracing disabled".to_string())
                }
            }
            None => {
                *tracing_enabled = !*tracing_enabled;
                if *tracing_enabled {
                    Ok("Execution tracing enabled".to_string())
                } else {
                    Ok("Execution tracing disabled".to_string())
                }
            }
        },

        ReplCommand::Save(path) => {
            let save_path = path.unwrap_or_else(|| "session.json".to_string());
            session.save(Path::new(&save_path))?;
            Ok(format!("Session saved to {}", save_path))
        }

        ReplCommand::Load(path) => {
            let loaded_session = Session::load(Path::new(&path))?;
            *session = loaded_session;
            // Update the stack from the loaded session
            *stack = session.data().stack.clone();
            Ok(format!("Session loaded from {}", path))
        }

        ReplCommand::Metrics => {
            let stats = &session.data().stats;
            Ok(format!(
                "Performance Metrics:\n\
                Operations: {}\n\
                Total time: {}ms\n\
                Max stack depth: {}\n\
                Words defined: {}\n\
                Errors: {}",
                stats.operations_count,
                stats.total_time_ms,
                stats.max_stack_depth,
                stats.words_defined,
                stats.errors_count
            ))
        }

        ReplCommand::History => {
            let history = session.history();
            if history.is_empty() {
                Ok("No command history".to_string())
            } else {
                let mut result = String::new();
                for (i, cmd) in history.iter().enumerate() {
                    result.push_str(&format!("{}: {}\n", i + 1, cmd));
                }
                Ok(result.trim_end().to_string())
            }
        }

        ReplCommand::ShowTrace(count) => {
            let entries = if let Some(c) = count {
                trace.recent_entries(c)
            } else {
                trace.recent_entries(10)
            };

            if entries.is_empty() {
                Ok("No trace entries".to_string())
            } else {
                let mut result = String::new();
                for entry in entries {
                    result.push_str(&format!(
                        "{}: {} ({:?})\n",
                        entry.id, entry.token, entry.duration
                    ));
                }
                Ok(result.trim_end().to_string())
            }
        }

        ReplCommand::Benchmark(code) => {
            // This would need to be implemented by the calling REPL
            // For now, return a placeholder
            Ok(format!("Benchmark for '{}' not yet implemented", code))
        }

        ReplCommand::Set(key, value) => match key.as_str() {
            "trace" => match value.as_str() {
                "true" | "on" | "1" => {
                    *tracing_enabled = true;
                    Ok("Tracing enabled".to_string())
                }
                "false" | "off" | "0" => {
                    *tracing_enabled = false;
                    Ok("Tracing disabled".to_string())
                }
                _ => Err(ReplError::command("Invalid trace setting")),
            },
            _ => Err(ReplError::command(format!("Unknown setting: {}", key))),
        },

        ReplCommand::About => Ok(get_about_text()),

        ReplCommand::Quit => Ok("Goodbye!".to_string()),

        ReplCommand::Unknown(cmd) => Err(ReplError::command(format!("Unknown command: {}", cmd))),
    }
}

/// Get help text for REPL commands
fn get_help_text() -> String {
    r#"Chronos REPL Commands:

Basic Commands:
  .help, .h          - Show this help message
  .stack, .s         - Show current stack contents
  .words, .w         - List all defined words
  .clear, .c         - Clear the stack
  .reset, .r         - Reset the REPL state
  .quit, .q          - Exit the REPL

Session Management:
  .save [file]       - Save current session (default: session.json)
  .load <file>       - Load a session from file

Debugging & Analysis:
  .trace, .t         - Toggle execution tracing
  .trace on/off      - Enable/disable tracing
  .metrics, .m       - Show performance metrics
  .history           - Show command history
  .showtrace [n]     - Show last n trace entries (default: 10)

Configuration:
  .set <key> <value> - Set configuration option
  .about             - Show about information

Examples:
  .save my_session.json
  .load examples/demo.json
  .trace on
  .set trace true"#
        .to_string()
}

/// Get about text
fn get_about_text() -> String {
    format!(
        r#"Chronos REPL v{}

C∀O (Kao) - Categorical ∀xiomatic Ordinal Programming Language

An evolving axiomatic programming language that combines:
- Category Theory foundations
- Ordinal Analysis for termination proofs
- Concatenative (stack-based) programming
- Collaborative verification and evolution

This is the interactive development environment for exploring
and developing programs in the Chronos language."#,
        env!("CARGO_PKG_VERSION")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parsing() {
        assert_eq!(parse_command(".help"), ReplCommand::Help);
        assert_eq!(parse_command(".h"), ReplCommand::Help);
        assert_eq!(parse_command(".stack"), ReplCommand::Stack);
        assert_eq!(parse_command(".quit"), ReplCommand::Quit);
        assert_eq!(parse_command(".trace"), ReplCommand::Trace(None));
        assert_eq!(parse_command(".trace on"), ReplCommand::Trace(Some(true)));
        assert_eq!(parse_command(".trace off"), ReplCommand::Trace(Some(false)));
        assert_eq!(parse_command(".save"), ReplCommand::Save(None));
        assert_eq!(
            parse_command(".save test.json"),
            ReplCommand::Save(Some("test.json".to_string()))
        );
        assert_eq!(
            parse_command(".load test.json"),
            ReplCommand::Load("test.json".to_string())
        );
    }

    #[test]
    fn test_unknown_command() {
        match parse_command(".unknown") {
            ReplCommand::Unknown(_) => {}
            _ => panic!("Expected Unknown command"),
        }
    }

    #[test]
    fn test_invalid_command() {
        match parse_command("not a command") {
            ReplCommand::Unknown(_) => {}
            _ => panic!("Expected Unknown command"),
        }
    }

    #[test]
    fn test_set_command() {
        assert_eq!(
            parse_command(".set trace true"),
            ReplCommand::Set("trace".to_string(), "true".to_string())
        );
    }
}
