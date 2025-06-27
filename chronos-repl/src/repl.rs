//! Enhanced REPL implementation for the Chronos programming language
//!
//! This module provides the main REPL interface that builds on top of chronos-core
//! to provide an interactive development environment with session management,
//! tracing, performance monitoring, and enhanced user experience features.

use crate::commands::{execute_command, parse_command, ReplCommand};
use crate::display::{
    format_duration, format_error, format_help, format_info, format_stack_rich, DisplayConfig,
};
use crate::error::{ReplError, Result};
use crate::session::Session;
use crate::tracing::{ExecutionTrace, OperationCategory, TraceContext, TraceEntry};
use chronos_core::{ChronosCore, Token, Value};
use serde::{Deserialize, Serialize};

use std::io::{self, Write};
use std::path::Path;
use std::time::{Duration, Instant};

/// Configuration for the Enhanced REPL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplConfig {
    /// Whether to show stack after each operation
    pub show_stack: bool,

    /// Whether to show execution timing
    pub show_timing: bool,

    /// Whether to use colored output
    pub use_colors: bool,

    /// Maximum stack items to display
    pub max_stack_display: usize,

    /// Whether to auto-save sessions
    pub auto_save: bool,

    /// Auto-save interval in seconds
    pub auto_save_interval: u64,

    /// Default session file name
    pub default_session_file: String,

    /// Prompt string
    pub prompt: String,

    /// Whether to show welcome message
    pub show_welcome: bool,

    /// Display configuration for rich formatting
    pub display: DisplayConfig,
}

impl Default for ReplConfig {
    fn default() -> Self {
        let display = DisplayConfig::default();

        Self {
            show_stack: false,
            show_timing: false,
            use_colors: display.use_colors,
            max_stack_display: display.max_stack_items,
            auto_save: false,
            auto_save_interval: 300, // 5 minutes
            default_session_file: "chronos_session.json".to_string(),
            prompt: "C∀O> ".to_string(),
            show_welcome: true,
            display,
        }
    }
}

/// Performance metrics for the current session
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Total operations executed
    pub total_operations: usize,

    /// Total execution time
    pub total_time: Duration,

    /// Maximum stack depth reached
    pub max_stack_depth: usize,

    /// Number of errors encountered
    pub error_count: usize,

    /// Average execution time per operation
    pub average_time: Duration,

    /// Operations per second
    pub ops_per_second: f64,
}

/// Result of evaluating code with tracing
#[derive(Debug, Clone)]
pub struct EvalResult {
    /// Final value on top of stack (if any)
    pub value: Option<Value>,

    /// Execution time
    pub duration: Duration,

    /// Trace entries (if tracing was enabled)
    pub trace_entries: Vec<TraceEntry>,

    /// Whether any errors occurred
    pub had_errors: bool,
}

/// The Enhanced REPL for Chronos
pub struct EnhancedRepl {
    /// Core language engine
    core: ChronosCore,

    /// Session management
    session: Session,

    /// Execution trace
    trace: ExecutionTrace,

    /// REPL configuration
    config: ReplConfig,

    /// Whether tracing is enabled
    tracing_enabled: bool,

    /// Performance metrics
    metrics: PerformanceMetrics,

    /// Current nesting level (for quotes, etc.)
    nesting_level: usize,

    /// Whether the REPL should exit
    should_exit: bool,

    /// Last auto-save time
    last_auto_save: Instant,
}

impl EnhancedRepl {
    /// Create a new Enhanced REPL
    pub fn new() -> Self {
        Self::with_config(ReplConfig::default())
    }

    /// Create a new Enhanced REPL with custom configuration
    pub fn with_config(config: ReplConfig) -> Self {
        Self {
            core: ChronosCore::new(),
            session: Session::new(),
            trace: ExecutionTrace::new(),
            config,
            tracing_enabled: false,
            metrics: PerformanceMetrics::default(),
            nesting_level: 0,
            should_exit: false,
            last_auto_save: Instant::now(),
        }
    }

    /// Start an interactive REPL session
    pub fn start_interactive(&mut self) -> Result<()> {
        if self.config.show_welcome {
            self.show_welcome();
        }

        loop {
            // Handle auto-save
            if self.should_auto_save() {
                if let Err(e) = self.try_auto_save() {
                    let error_msg = format!("Auto-save failed: {}", e);
                    eprintln!("{}", format_error(&error_msg, &self.config.display));
                }
            }

            // Show prompt
            print!("{}", self.config.prompt);
            io::stdout()
                .flush()
                .map_err(|e| ReplError::io(e.to_string()))?;

            // Read input
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    // EOF reached - exit gracefully
                    println!("\nFarewell! May your axioms remain consistent.");
                    break;
                }
                Ok(_) => {
                    let input = input.trim();

                    if input.is_empty() {
                        continue;
                    }

                    // Handle the input
                    if let Err(e) = self.handle_input(input) {
                        eprintln!("{}", format_error(&e.to_string(), &self.config.display));
                        self.metrics.error_count += 1;
                    }

                    // Check if we should exit
                    if self.should_exit {
                        break;
                    }
                }
                Err(e) => {
                    let error_msg = format!("Error reading input: {}", e);
                    eprintln!("{}", format_error(&error_msg, &self.config.display));
                    break;
                }
            }
        }

        // Final save if auto-save is enabled
        if self.config.auto_save {
            if let Err(e) = self.try_auto_save() {
                let error_msg = format!("Final auto-save failed: {}", e);
                eprintln!("{}", format_error(&error_msg, &self.config.display));
            }
        }

        Ok(())
    }

    /// Handle a line of input
    fn handle_input(&mut self, input: &str) -> Result<()> {
        // Add to command history
        self.session.add_to_history(input.to_string());

        // Check if it's a command
        if input.starts_with('.') {
            return self.handle_command(input);
        }

        // Otherwise, evaluate as Chronos code
        self.eval(input)?;

        // Show stack if configured
        if self.config.show_stack {
            self.show_stack();
        }

        Ok(())
    }

    /// Handle a REPL command
    fn handle_command(&mut self, input: &str) -> Result<()> {
        let command = parse_command(input);

        // Special handling for quit command
        if matches!(command, ReplCommand::Quit) {
            self.should_exit = true;
            println!("Farewell! May your axioms remain consistent.");
            return Ok(());
        }

        // Special handling for display commands
        match &command {
            ReplCommand::Colors(setting) => {
                let enabled = setting.unwrap_or(!self.config.display.use_colors);
                self.config.display.use_colors = enabled;
                self.config.use_colors = enabled;
                let status = if enabled { "enabled" } else { "disabled" };
                let message = format!("Colored output {}", status);
                println!("{}", format_info(&message, &self.config.display));
                return Ok(());
            }
            ReplCommand::Types(setting) => {
                let enabled = setting.unwrap_or(!self.config.display.show_types);
                self.config.display.show_types = enabled;
                let status = if enabled { "enabled" } else { "disabled" };
                let message = format!("Type information display {}", status);
                println!("{}", format_info(&message, &self.config.display));
                return Ok(());
            }
            ReplCommand::Compact(setting) => {
                let enabled = setting.unwrap_or(!self.config.display.compact_stack);
                self.config.display.compact_stack = enabled;
                let status = if enabled { "enabled" } else { "disabled" };
                let message = format!("Compact stack display {}", status);
                println!("{}", format_info(&message, &self.config.display));
                return Ok(());
            }
            ReplCommand::Syntax(setting) => {
                let enabled = setting.unwrap_or(!self.config.display.highlight_syntax);
                self.config.display.highlight_syntax = enabled;
                let status = if enabled { "enabled" } else { "disabled" };
                let message = format!("Syntax highlighting {}", status);
                println!("{}", format_info(&message, &self.config.display));
                return Ok(());
            }
            ReplCommand::Unicode(setting) => {
                let enabled = setting.unwrap_or(!self.config.display.unicode_symbols);
                self.config.display.unicode_symbols = enabled;
                let status = if enabled { "enabled" } else { "disabled" };
                let message = format!("Unicode symbols {}", status);
                println!("{}", format_info(&message, &self.config.display));
                return Ok(());
            }
            _ => {}
        }

        // Special handling for help command to use rich formatting
        if matches!(command, ReplCommand::Help) {
            let help_text = format_help(&self.config.display);
            println!("{}", help_text);
            return Ok(());
        }

        let mut stack = self.core.get_stack();
        let result = execute_command(
            command,
            &mut stack,
            &mut self.session,
            &mut self.trace,
            &mut self.tracing_enabled,
        )?;

        // Update the core's stack
        self.core.clear_stack();
        for value in stack {
            self.core.push(value);
        }

        // Update session stack
        self.session.update_stack(self.core.get_stack());

        if !result.is_empty() {
            println!("{}", result);
        }

        Ok(())
    }

    /// Evaluate Chronos code
    pub fn eval(&mut self, input: &str) -> Result<()> {
        let start_time = Instant::now();

        if self.tracing_enabled {
            self.eval_with_tracing(input)?;
        } else {
            match self.core.eval_source(input) {
                Ok(()) => {}
                Err(e) => {
                    let error_msg = e.to_string();
                    if error_msg.contains("QUIT_REQUESTED") {
                        self.should_exit = true;
                        println!("Farewell! May your axioms remain consistent.");
                        return Ok(());
                    } else {
                        return Err(ReplError::from(e));
                    }
                }
            }
        }

        let duration = start_time.elapsed();

        // Update metrics
        self.metrics.total_operations += 1;
        self.metrics.total_time += duration;
        self.metrics.max_stack_depth = self.metrics.max_stack_depth.max(self.core.stack_depth());
        self.metrics.average_time = self.metrics.total_time / self.metrics.total_operations as u32;

        if self.metrics.total_time.as_secs_f64() > 0.0 {
            self.metrics.ops_per_second =
                self.metrics.total_operations as f64 / self.metrics.total_time.as_secs_f64();
        }

        // Update session statistics
        self.session.update_stats(1, duration.as_millis() as u64, 0);
        self.session.update_stack(self.core.get_stack());

        // Sync user-defined words from VM to session
        self.sync_user_words();

        // Show timing if configured
        if self.config.show_timing {
            println!(
                "Execution time: {}",
                format_duration(duration, &self.config.display)
            );
        }

        Ok(())
    }

    /// Evaluate code with detailed tracing
    pub fn eval_with_trace(&mut self, input: &str) -> Result<EvalResult> {
        let start_time = Instant::now();
        let _initial_stack = self.core.get_stack();

        let trace_entries = if self.tracing_enabled {
            self.eval_with_tracing(input)?;
            // Get the new trace entries
            let mut entries = Vec::new();
            for entry in self.trace.recent_entries(100) {
                if entry.timestamp >= start_time {
                    entries.push(entry.clone());
                }
            }
            entries
        } else {
            self.core.eval(input).map_err(ReplError::from)?;
            Vec::new()
        };

        let duration = start_time.elapsed();
        let final_stack = self.core.get_stack();

        // Get the top value if available
        let value = final_stack.last().cloned();

        Ok(EvalResult {
            value,
            duration,
            trace_entries,
            had_errors: false, // Would need to be set by error handling
        })
    }

    /// Evaluate with detailed tracing enabled
    fn eval_with_tracing(&mut self, input: &str) -> Result<()> {
        // Parse the input with quote processing
        let tokens = self.core.parse(input).map_err(ReplError::from)?;

        // Execute each token with tracing
        for token in tokens {
            let stack_before = self.core.get_stack();
            let start_time = Instant::now();

            // Execute the token
            let result = self.core.execute_tokens(&[token.clone()]);
            let duration = start_time.elapsed();
            let stack_after = self.core.get_stack();

            // Determine operation category
            let category = self.classify_operation(&token);

            // Create trace context
            let context = TraceContext::new(category);

            // Record the trace
            let error = result.as_ref().err().map(|e| e.to_string());
            self.trace
                .trace_execution(token, stack_before, stack_after, duration, error, context);

            // Propagate any error
            result.map_err(ReplError::from)?;
        }

        Ok(())
    }

    /// Classify an operation for tracing purposes
    fn classify_operation(&self, token: &Token) -> OperationCategory {
        match token {
            Token::Word(word) => match word.as_str() {
                "dup" | "drop" | "swap" | "over" | "rot" => OperationCategory::StackOp,
                "+" | "-" | "*" | "/" | "mod" => OperationCategory::Arithmetic,
                "=" | "<" | ">" | "<=" | ">=" | "!=" => OperationCategory::Comparison,
                "." | ".s" => OperationCategory::System,
                _ => OperationCategory::Word,
            },
            Token::Literal(_) => OperationCategory::StackOp,
            Token::QuoteStart | Token::QuoteEnd => OperationCategory::Quote,
            _ => OperationCategory::Other,
        }
    }

    /// Get current stack contents
    pub fn stack(&self) -> Vec<Value> {
        self.core.get_stack()
    }

    /// Get performance metrics
    pub fn performance_metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }

    /// Save session to file
    pub fn save_session<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.session.save(path).map_err(ReplError::from)
    }

    /// Load session from file
    pub fn load_session<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.session = Session::load(path)?;

        // Restore the stack
        let stack = self.session.data().stack.clone();
        self.core.clear_stack();
        for value in stack {
            self.core.push(value);
        }

        // Restore user-defined words
        for (name, _tokens) in self.session.user_words() {
            // For now, we'll need to reconstruct word definitions
            // This is a simplified approach - in practice we'd want to store
            // full word definitions in the session
            println!("Restored word: {}", name);
        }

        Ok(())
    }

    /// Set tracing enabled/disabled
    pub fn set_tracing(&mut self, enabled: bool) {
        self.tracing_enabled = enabled;
    }

    /// Clear the execution trace
    pub fn clear_trace(&mut self) {
        self.trace.clear();
    }

    /// Get the execution trace
    pub fn trace(&self) -> &ExecutionTrace {
        &self.trace
    }

    /// Show the current stack
    fn show_stack(&self) {
        let stack_display = format_stack_rich(&self.core.get_stack(), &self.config.display);
        println!("{}", stack_display);
    }

    /// Show welcome message
    fn show_welcome(&self) {
        use crate::display::{format_banner, format_info};

        let title = format!("C∀O (Kao) v{}", env!("CARGO_PKG_VERSION"));
        println!("{}", format_banner(&title, &self.config.display));
        println!();

        let subtitle = "Categorical ∀xiomatic Ordinal Programming Language";
        println!("{}", format_info(subtitle, &self.config.display));

        let repl_info = "Enhanced Interactive REPL";
        println!("{}", format_info(repl_info, &self.config.display));
        println!();

        let help_hint = "Type '.help' for available commands, '.quit' to exit";
        println!("{}", format_info(help_hint, &self.config.display));
        println!();
    }

    /// Check if auto-save should be performed
    fn should_auto_save(&self) -> bool {
        if !self.config.auto_save || !self.session.is_dirty() {
            return false;
        }

        let elapsed = self.last_auto_save.elapsed().as_secs();
        elapsed >= self.config.auto_save_interval
    }

    /// Try to perform auto-save
    fn try_auto_save(&mut self) -> Result<()> {
        if let Some(path) = self.session.file_path().map(|p| p.to_path_buf()) {
            self.session.save(&path)?;
            self.last_auto_save = Instant::now();
        } else {
            // Save to default location
            let path = &self.config.default_session_file;
            self.session.save(Path::new(path))?;
            self.last_auto_save = Instant::now();
        }
        Ok(())
    }

    /// Get REPL configuration
    pub fn config(&self) -> &ReplConfig {
        &self.config
    }

    /// Get mutable REPL configuration
    pub fn config_mut(&mut self) -> &mut ReplConfig {
        &mut self.config
    }

    /// Reset the REPL to initial state
    pub fn reset(&mut self) {
        self.core = ChronosCore::new();
        self.session.reset();
        self.trace.clear();
        self.metrics = PerformanceMetrics::default();
        self.nesting_level = 0;
        self.should_exit = false;
    }

    /// Toggle colored output
    pub fn toggle_colors(&mut self) -> bool {
        self.config.display.use_colors = !self.config.display.use_colors;
        self.config.use_colors = self.config.display.use_colors;
        self.config.display.use_colors
    }

    /// Toggle type information display
    pub fn toggle_types(&mut self) -> bool {
        self.config.display.show_types = !self.config.display.show_types;
        self.config.display.show_types
    }

    /// Toggle compact stack display
    pub fn toggle_compact(&mut self) -> bool {
        self.config.display.compact_stack = !self.config.display.compact_stack;
        self.config.display.compact_stack
    }

    /// Toggle syntax highlighting
    pub fn toggle_syntax_highlighting(&mut self) -> bool {
        self.config.display.highlight_syntax = !self.config.display.highlight_syntax;
        self.config.display.highlight_syntax
    }

    /// Toggle Unicode symbols
    pub fn toggle_unicode(&mut self) -> bool {
        self.config.display.unicode_symbols = !self.config.display.unicode_symbols;
        self.config.display.unicode_symbols
    }

    /// Set maximum stack items to display
    pub fn set_max_stack_display(&mut self, max_items: usize) {
        self.config.display.max_stack_items = max_items;
        self.config.max_stack_display = max_items;
    }

    /// Get display configuration
    pub fn display_config(&self) -> &DisplayConfig {
        &self.config.display
    }

    /// Get mutable display configuration
    pub fn display_config_mut(&mut self) -> &mut DisplayConfig {
        &mut self.config.display
    }

    /// Synchronize user-defined words from the VM to the session
    fn sync_user_words(&mut self) {
        let user_words = self.core.get_user_words();
        for word_name in user_words {
            // For now, we'll store empty token sequences since we don't have access
            // to the actual token sequence from the VM dictionary
            // This is a simplified implementation - ideally we'd store the actual tokens
            if !self.session.user_words().contains_key(&word_name) {
                self.session.define_word(word_name, vec![]);
            }
        }
    }
}

impl Default for EnhancedRepl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_repl_creation() {
        let repl = EnhancedRepl::new();
        assert_eq!(repl.stack().len(), 0);
        assert!(!repl.tracing_enabled);
    }

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
    fn test_performance_metrics() {
        let mut repl = EnhancedRepl::new();
        repl.eval("42").unwrap();

        let metrics = repl.performance_metrics();
        assert_eq!(metrics.total_operations, 1);
        assert!(metrics.total_time.as_nanos() > 0);
    }

    #[test]
    fn test_tracing() {
        let mut repl = EnhancedRepl::new();
        repl.set_tracing(true);

        let result = repl.eval_with_trace("42").unwrap();
        assert!(result.trace_entries.len() > 0);

        // Check that we have the expected tokens
        let tokens: Vec<String> = result
            .trace_entries
            .iter()
            .map(|entry| entry.token.to_string())
            .collect();

        // Should contain our input tokens
        assert!(tokens.iter().any(|t| t.contains("42")));
    }

    #[test]
    fn test_session_persistence() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Create and save a session
        {
            let mut repl = EnhancedRepl::new();
            repl.eval("42").unwrap();
            repl.save_session(path).unwrap();
        }

        // Load the session
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
    fn test_command_handling() {
        let mut repl = EnhancedRepl::new();

        // Test stack command
        repl.eval("42").unwrap();
        assert!(repl.handle_command(".stack").is_ok());

        // Test clear command
        assert!(repl.handle_command(".clear").is_ok());
        assert_eq!(repl.stack().len(), 0);
    }

    #[test]
    fn test_configuration() {
        let mut config = ReplConfig::default();
        config.show_stack = true;
        config.show_timing = true;

        let repl = EnhancedRepl::with_config(config);
        assert!(repl.config().show_stack);
        assert!(repl.config().show_timing);
    }
}
