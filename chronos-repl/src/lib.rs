//! Chronos REPL - Interactive Development Environment
//!
//! This crate provides an enhanced Read-Eval-Print Loop (REPL) and interactive
//! development environment for the Chronos programming language. It builds on
//! top of `chronos-core` to provide:
//!
//! - Interactive command-line interface
//! - Session management and persistence
//! - Execution tracing and debugging
//! - Performance monitoring and benchmarking
//! - Enhanced user experience features
//!
//! # Quick Start
//!
//! ```rust
//! use chronos_repl::EnhancedRepl;
//!
//! let mut repl = EnhancedRepl::new();
//!
//! // Evaluate some code
//! repl.eval("2 3 +").unwrap();
//!
//! // Check the result
//! let stack = repl.stack();
//! assert_eq!(stack.len(), 1);
//! ```
//!
//! # Features
//!
//! ## Session Management
//! Save and load REPL sessions including user-defined words, types, and settings:
//!
//! ```rust
//! use chronos_repl::EnhancedRepl;
//! use std::path::Path;
//!
//! let mut repl = EnhancedRepl::new();
//! repl.eval("hello : 'Hello, World!' .").unwrap();
//! repl.save_session(Path::new("my_session.json")).unwrap();
//!
//! // Later...
//! let mut new_repl = EnhancedRepl::new();
//! new_repl.load_session(Path::new("my_session.json")).unwrap();
//! new_repl.eval("hello").unwrap(); // Still works!
//! ```
//!
//! ## Execution Tracing
//! Monitor execution with detailed tracing:
//!
//! ```rust
//! use chronos_repl::EnhancedRepl;
//!
//! let mut repl = EnhancedRepl::new();
//! repl.set_tracing(true);
//! let result = repl.eval_with_trace("2 3 + dup *").unwrap();
//!
//! for entry in result.trace_entries {
//!     println!("Executed {} in {:?}", entry.token, entry.duration);
//! }
//! ```

pub mod cellular_automata;
pub mod codd_ca;
pub mod commands;
pub mod display;
pub mod error;
pub mod repl;
pub mod session;
pub mod tracing;

// Re-export chronos-core types for convenience
pub use chronos_core::{ChronosCore, ChronosError, Token, Value};

// Main exports
pub use cellular_automata::{CAEnvironment, ElementaryCA, ElementaryRule};
pub use codd_ca::{CoddCA, CoddEnvironment, CoddPatternType, CoddState};
pub use error::{ReplError, Result};
pub use repl::EnhancedRepl;
pub use session::{Session, SessionData};
pub use tracing::{ExecutionTrace, TraceEntry};

// Performance and debugging
pub use repl::{PerformanceMetrics, ReplConfig};

/// Start an interactive REPL session
///
/// This is a convenience function that creates a new `EnhancedRepl` instance
/// and starts an interactive command-line interface.
///
/// # Example
///
/// ```no_run
/// chronos_repl::start_interactive_repl().unwrap();
/// ```
///
/// # Errors
///
/// Returns an error if the REPL fails to start or encounters an I/O error.
pub fn start_interactive_repl() -> Result<()> {
    let mut repl = EnhancedRepl::new();
    repl.start_interactive()
}

/// Evaluate a string of Chronos code and return the result
///
/// This is a convenience function for one-off evaluations without
/// maintaining REPL state.
///
/// # Example
///
/// ```rust
/// use chronos_repl::eval_string;
///
/// let result = eval_string("2 3 + 4 *").unwrap();
/// // result contains the final stack state and any trace information
/// ```
pub fn eval_string(code: &str) -> Result<Value> {
    let mut repl = EnhancedRepl::new();
    repl.eval(code)?;

    // Return top of stack if available
    if let Some(value) = repl.stack().last() {
        Ok(value.clone())
    } else {
        Ok(Value::Unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_evaluation() {
        let mut repl = EnhancedRepl::new();
        assert!(repl.eval("42").is_ok());

        let stack = repl.stack();
        assert_eq!(stack.len(), 1);
        if let Value::Nat(n) = &stack[0] {
            assert_eq!(*n, 42);
        } else {
            panic!("Expected Nat value");
        }
    }

    #[test]
    fn test_eval_string_convenience() {
        let result = eval_string("10").unwrap();
        if let Value::Nat(n) = result {
            assert_eq!(n, 10);
        } else {
            panic!("Expected Nat value");
        }
    }

    #[test]
    fn test_session_persistence() {
        use tempfile::NamedTempFile;

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Create a session with some values
        {
            let mut repl = EnhancedRepl::new();
            repl.eval("42").unwrap();
            repl.save_session(path).unwrap();
        }

        // Load the session in a new REPL
        {
            let mut repl = EnhancedRepl::new();
            repl.load_session(path).unwrap();

            let stack = repl.stack();
            assert_eq!(stack.len(), 1);
            if let Value::Nat(n) = &stack[0] {
                assert_eq!(*n, 42);
            } else {
                panic!("Expected Nat value");
            }
        }
    }

    #[test]
    fn test_tracing() {
        let mut repl = EnhancedRepl::new();
        repl.set_tracing(true);

        let result = repl.eval_with_trace("42").unwrap();
        assert!(result.trace_entries.len() > 0);

        // Check that we traced the execution
        let tokens: Vec<String> = result
            .trace_entries
            .iter()
            .map(|entry| entry.token.to_string())
            .collect();

        assert!(tokens.contains(&"42".to_string()));
    }

    #[test]
    fn test_performance_metrics() {
        let mut repl = EnhancedRepl::new();

        // Execute some operations
        repl.eval("1").unwrap();
        repl.eval("2").unwrap();

        let metrics = repl.performance_metrics();
        assert!(metrics.total_operations > 0);
        assert!(metrics.total_time.as_nanos() > 0);
        assert!(metrics.max_stack_depth > 0);
    }
}
