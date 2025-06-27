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

    /// Toggle colored output
    Colors(Option<bool>),

    /// Toggle type information display
    Types(Option<bool>),

    /// Toggle compact stack display
    Compact(Option<bool>),

    /// Toggle syntax highlighting
    Syntax(Option<bool>),

    /// Toggle Unicode symbols
    Unicode(Option<bool>),

    /// Cellular Automata Commands
    /// Show CA rule
    CARule(u8),

    /// Run CA with simple output
    CASimple(u8, usize, Option<String>),

    /// Launch interactive CA environment
    CAInteractive(u8, Option<String>),

    /// List famous CA rules
    CARules,

    /// Codd's Cellular Automata Commands
    /// Run Codd CA with simple output
    CoddSimple(String, usize, usize, usize),

    /// Launch interactive Codd CA environment
    CoddInteractive(String, usize, usize),

    /// List Codd CA patterns
    CoddPatterns,

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

        "colors" | "color" => {
            if parts.len() > 1 {
                match parts[1] {
                    "on" | "true" | "1" => ReplCommand::Colors(Some(true)),
                    "off" | "false" | "0" => ReplCommand::Colors(Some(false)),
                    _ => ReplCommand::Colors(None),
                }
            } else {
                ReplCommand::Colors(None)
            }
        }

        "types" | "type" => {
            if parts.len() > 1 {
                match parts[1] {
                    "on" | "true" | "1" => ReplCommand::Types(Some(true)),
                    "off" | "false" | "0" => ReplCommand::Types(Some(false)),
                    _ => ReplCommand::Types(None),
                }
            } else {
                ReplCommand::Types(None)
            }
        }

        "compact" => {
            if parts.len() > 1 {
                match parts[1] {
                    "on" | "true" | "1" => ReplCommand::Compact(Some(true)),
                    "off" | "false" | "0" => ReplCommand::Compact(Some(false)),
                    _ => ReplCommand::Compact(None),
                }
            } else {
                ReplCommand::Compact(None)
            }
        }

        "syntax" | "highlight" => {
            if parts.len() > 1 {
                match parts[1] {
                    "on" | "true" | "1" => ReplCommand::Syntax(Some(true)),
                    "off" | "false" | "0" => ReplCommand::Syntax(Some(false)),
                    _ => ReplCommand::Syntax(None),
                }
            } else {
                ReplCommand::Syntax(None)
            }
        }

        "unicode" | "symbols" => {
            if parts.len() > 1 {
                match parts[1] {
                    "on" | "true" | "1" => ReplCommand::Unicode(Some(true)),
                    "off" | "false" | "0" => ReplCommand::Unicode(Some(false)),
                    _ => ReplCommand::Unicode(None),
                }
            } else {
                ReplCommand::Unicode(None)
            }
        }

        "ca-rule" | "carule" => {
            if parts.len() > 1 {
                if let Ok(rule) = parts[1].parse::<u8>() {
                    ReplCommand::CARule(rule)
                } else {
                    ReplCommand::Unknown(input.to_string())
                }
            } else {
                ReplCommand::Unknown(input.to_string())
            }
        }

        "ca-simple" | "casimple" => {
            if parts.len() >= 3 {
                if let (Ok(rule), Ok(gens)) = (parts[1].parse::<u8>(), parts[2].parse::<usize>()) {
                    let pattern = if parts.len() > 3 {
                        Some(parts[3..].join(" "))
                    } else {
                        None
                    };
                    ReplCommand::CASimple(rule, gens, pattern)
                } else {
                    ReplCommand::Unknown(input.to_string())
                }
            } else {
                ReplCommand::Unknown(input.to_string())
            }
        }

        "ca" | "cellular" => {
            if parts.len() > 1 {
                if let Ok(rule) = parts[1].parse::<u8>() {
                    let pattern = if parts.len() > 2 {
                        Some(parts[2..].join(" "))
                    } else {
                        None
                    };
                    ReplCommand::CAInteractive(rule, pattern)
                } else {
                    ReplCommand::Unknown(input.to_string())
                }
            } else {
                ReplCommand::Unknown(input.to_string())
            }
        }

        "ca-rules" | "carules" => ReplCommand::CARules,

        "codd-simple" | "coddsimple" => {
            if parts.len() >= 5 {
                if let (Ok(width), Ok(height), Ok(gens)) = (
                    parts[2].parse::<usize>(),
                    parts[3].parse::<usize>(),
                    parts[4].parse::<usize>(),
                ) {
                    ReplCommand::CoddSimple(parts[1].to_string(), width, height, gens)
                } else {
                    ReplCommand::Unknown(input.to_string())
                }
            } else {
                ReplCommand::Unknown(input.to_string())
            }
        }

        "codd" => {
            if parts.len() >= 4 {
                if let (Ok(width), Ok(height)) =
                    (parts[2].parse::<usize>(), parts[3].parse::<usize>())
                {
                    ReplCommand::CoddInteractive(parts[1].to_string(), width, height)
                } else {
                    ReplCommand::Unknown(input.to_string())
                }
            } else {
                ReplCommand::Unknown(input.to_string())
            }
        }

        "codd-patterns" | "coddpatterns" => ReplCommand::CoddPatterns,

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

        ReplCommand::Colors(_) => {
            // This will be handled by the REPL implementation
            Ok("Color toggle command received".to_string())
        }

        ReplCommand::Types(_) => {
            // This will be handled by the REPL implementation
            Ok("Type display toggle command received".to_string())
        }

        ReplCommand::Compact(_) => {
            // This will be handled by the REPL implementation
            Ok("Compact display toggle command received".to_string())
        }

        ReplCommand::Syntax(_) => {
            // This will be handled by the REPL implementation
            Ok("Syntax highlighting toggle command received".to_string())
        }

        ReplCommand::Unicode(_) => {
            // This will be handled by the REPL implementation
            Ok("Unicode symbols toggle command received".to_string())
        }

        ReplCommand::CARule(rule) => {
            use crate::cellular_automata::ElementaryRule;
            let ca_rule = ElementaryRule::new(rule);
            let mut result = format!("Elementary Cellular Automaton Rule {}\n", rule);
            if let Some(name) = ca_rule.name() {
                result.push_str(&format!("{}\n\n", name));
            }
            result.push_str(&ca_rule.rule_table());
            Ok(result)
        }

        ReplCommand::CASimple(rule, generations, pattern) => {
            use crate::cellular_automata::{run_simple_ca, ElementaryRule};
            let ca_rule = ElementaryRule::new(rule);
            let result = run_simple_ca(ca_rule, generations, 79, pattern.as_deref())?;
            Ok(result)
        }

        ReplCommand::CAInteractive(rule, pattern) => {
            use crate::cellular_automata::{CAConfig, CAEnvironment, ElementaryRule};
            let ca_rule = ElementaryRule::new(rule);
            let config = CAConfig::default();

            let mut env = if let Some(p) = pattern {
                CAEnvironment::new_with_pattern(ca_rule, config, &p)?
            } else {
                CAEnvironment::new(ca_rule, config)
            };

            env.run()?;
            Ok("Cellular automaton session ended".to_string())
        }

        ReplCommand::CARules => {
            use crate::cellular_automata::famous_rules;
            let rules = famous_rules();
            let mut result = String::from("Famous Cellular Automaton Rules:\n\n");
            for (rule_num, description) in rules {
                result.push_str(&format!("Rule {}: {}\n", rule_num, description));
            }
            result.push_str("\nUse '.ca <rule>' to run interactively or '.ca-simple <rule> <generations>' for text output");
            Ok(result)
        }

        ReplCommand::CoddSimple(pattern, width, height, generations) => {
            use crate::codd_ca::{run_simple_codd_ca, CoddPatternType};

            let pattern_type = match pattern.as_str() {
                "empty" => CoddPatternType::Empty,
                "signal" => CoddPatternType::Signal,
                "replicator" => CoddPatternType::Replicator,
                _ => {
                    return Err(ReplError::command(
                        "Invalid pattern. Use: empty, signal, or replicator",
                    ))
                }
            };

            let result = run_simple_codd_ca(pattern_type, generations, width, height)?;
            Ok(result)
        }

        ReplCommand::CoddInteractive(pattern, width, height) => {
            use crate::codd_ca::{CoddEnvironment, CoddPatternType};

            let pattern_type = match pattern.as_str() {
                "empty" => CoddPatternType::Empty,
                "signal" => CoddPatternType::Signal,
                "replicator" => CoddPatternType::Replicator,
                _ => {
                    return Err(ReplError::command(
                        "Invalid pattern. Use: empty, signal, or replicator",
                    ))
                }
            };

            let mut env = CoddEnvironment::new(pattern_type, width, height);
            env.run()?;
            Ok("Codd's CA session ended".to_string())
        }

        ReplCommand::CoddPatterns => {
            use crate::codd_ca::codd_patterns;
            let patterns = codd_patterns();
            let mut result = String::from("Codd's Cellular Automaton Patterns:\n\n");
            for (pattern_type, description) in patterns {
                result.push_str(&format!("{:?}: {}\n", pattern_type, description));
            }
            result.push_str("\nUse '.codd <pattern> <width> <height>' for interactive mode");
            result.push_str(
                "\nUse '.codd-simple <pattern> <width> <height> <generations>' for text output",
            );
            Ok(result)
        }

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

Display Options:
  .colors [on/off]   - Toggle colored output
  .types [on/off]    - Toggle type information display
  .compact [on/off]  - Toggle compact stack display
  .syntax [on/off]   - Toggle syntax highlighting
  .unicode [on/off]  - Toggle Unicode symbols

Cellular Automata:
  .ca <rule> [pattern]      - Interactive CA with rule (0-255)
  .ca-simple <rule> <gens>  - Text output CA evolution
  .ca-rule <rule>           - Show rule table for rule number
  .ca-rules                 - List famous CA rules

Codd's Cellular Automata:
  .codd <pattern> <width> <height>           - Interactive Codd CA
  .codd-simple <pattern> <w> <h> <gens>      - Text output Codd CA
  .codd-patterns                             - List Codd CA patterns

Configuration:
  .set <key> <value> - Set configuration option
  .about             - Show about information

Examples:
  .save my_session.json
  .load examples/demo.json
  .trace on
  .ca 30
  .ca-simple 90 20 111
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
