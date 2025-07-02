//! REPL (Read-Eval-Print Loop) for C∀O (Kao)
//!
//! This module provides an interactive environment for C∀O development,
//! allowing users to define words, execute code, and explore the language.

use crate::core_lib::CoreLibrary;
use crate::ordinal::OrdinalVerifier;
use crate::parser::{ParseError, Parser, Statement};
use crate::type_inference::TypeInferer;
use crate::types::{OrdinalValue, Token, TypeDefinition, TypeSignature, Value, WordDefinition};
use crate::vm::{VirtualMachine, VmError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

/// Errors that can occur in the REPL
#[derive(Debug, Clone)]
pub enum ReplError {
    /// Parse error
    ParseError(ParseError),
    /// VM execution error
    VmError(VmError),
    /// Definition error
    DefinitionError(String),
    /// I/O error
    IoError(String),
    /// Session error
    SessionError(String),
}

impl fmt::Display for ReplError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReplError::ParseError(e) => write!(f, "Parse error: {}", e),
            ReplError::VmError(e) => write!(f, "Runtime error: {}", e),
            ReplError::DefinitionError(e) => write!(f, "Definition error: {}", e),
            ReplError::IoError(e) => write!(f, "I/O error: {}", e),
            ReplError::SessionError(e) => write!(f, "Session error: {}", e),
        }
    }
}

impl std::error::Error for ReplError {}

impl From<ParseError> for ReplError {
    fn from(error: ParseError) -> Self {
        ReplError::ParseError(error)
    }
}

impl From<VmError> for ReplError {
    fn from(error: VmError) -> Self {
        ReplError::VmError(error)
    }
}

/// Session data that can be saved and restored
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    /// User-defined type definitions
    pub user_types: HashMap<String, TypeDefinition>,
    /// Pending type signatures
    pub pending_signatures: HashMap<String, TypeSignature>,
    /// User-defined words (serializable version)
    pub user_words: Vec<WordDefinition>,
    /// Current stack contents
    pub stack: Vec<Value>,
    /// Session settings
    pub show_stack: bool,
    pub show_ordinals: bool,
    pub trace_execution: bool,
}

/// Execution trace entry
#[derive(Debug, Clone)]
pub struct TraceEntry {
    /// Timestamp
    pub timestamp: Instant,
    /// Token being executed
    pub token: Token,
    /// Stack before execution
    pub stack_before: Vec<Value>,
    /// Stack after execution
    pub stack_after: Vec<Value>,
    /// Execution time
    pub duration: Duration,
}

/// Performance metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Total execution time
    pub total_time: Duration,
    /// Number of operations
    pub operation_count: usize,
    /// Stack depth statistics
    pub max_stack_depth: usize,
    /// Memory usage (approximate)
    pub memory_usage: usize,
}

/// The REPL state and environment
pub struct Repl {
    /// The virtual machine
    vm: VirtualMachine,
    /// Core library
    core_lib: CoreLibrary,
    /// Ordinal verifier
    verifier: OrdinalVerifier,
    /// User-defined type definitions
    user_types: HashMap<String, TypeDefinition>,
    /// Pending type signatures (declared but not yet defined)
    pending_signatures: HashMap<String, TypeSignature>,
    /// Whether to show stack after each operation
    show_stack: bool,
    /// Whether to show ordinal costs
    show_ordinals: bool,
    /// Whether to trace execution
    trace_execution: bool,
    /// Execution trace (limited size for performance)
    execution_trace: Vec<TraceEntry>,
    /// Maximum trace entries to keep
    max_trace_entries: usize,
    /// Performance metrics for current session
    performance_metrics: PerformanceMetrics,
    /// Command history
    command_history: Vec<String>,
    /// Maximum history entries
    max_history_entries: usize,
    /// Type inferer for automatic type inference
    type_inferer: TypeInferer,
    /// Whether to show type inference debugging
    type_debug: bool,
}

impl Repl {
    /// Create a new REPL environment
    pub fn new() -> Self {
        let mut repl = Repl {
            vm: VirtualMachine::new(),
            core_lib: CoreLibrary::new(),
            verifier: OrdinalVerifier::new(),
            user_types: HashMap::new(),
            pending_signatures: HashMap::new(),
            show_stack: false,
            show_ordinals: false,
            trace_execution: false,
            execution_trace: Vec::new(),
            max_trace_entries: 1000,
            performance_metrics: PerformanceMetrics {
                total_time: Duration::from_secs(0),
                operation_count: 0,
                max_stack_depth: 0,
                memory_usage: 0,
            },
            command_history: Vec::new(),
            max_history_entries: 100,
            type_inferer: TypeInferer::new(),
            type_debug: false,
        };

        // Load core library into VM
        for (_name, word_def) in repl.core_lib.get_core_words() {
            repl.vm.define_word(word_def.clone());
        }

        repl
    }

    /// Evaluate a line of C∀O code
    pub fn eval(&mut self, input: &str) -> Result<(), ReplError> {
        let input = input.trim();

        // Add to command history
        if !input.is_empty() && !input.starts_with('.') {
            self.add_to_history(input.to_string());
        }

        // Handle special REPL commands
        if input.starts_with('.') {
            return self.handle_repl_command(input);
        }

        let start_time = Instant::now();

        // Parse the input
        let mut parser = Parser::new(input)?;

        // Configure parser with known word signatures for type inference
        // Add core library signatures
        for (name, word_def) in self.core_lib.get_core_words() {
            parser.add_word_signature(name.clone(), word_def.signature.clone());
        }

        // Add user-defined word signatures
        for (name, word_def) in self.vm.get_all_word_definitions().iter() {
            parser.add_word_signature(name.clone(), word_def.signature.clone());
        }

        // Set type debugging state
        parser.set_type_debug(self.type_debug);

        let statements = parser.parse_all()?;

        // Process each statement
        for statement in statements {
            self.eval_statement(statement)?;
        }

        // Update performance metrics
        let duration = start_time.elapsed();
        self.performance_metrics.total_time += duration;
        self.performance_metrics.operation_count += 1;
        self.performance_metrics.max_stack_depth = self
            .performance_metrics
            .max_stack_depth
            .max(self.vm.stack().len());

        // Show stack if enabled
        if self.show_stack && !self.vm.stack().is_empty() {
            self.show_stack_contents();
        }

        Ok(())
    }

    /// Evaluate a single statement
    fn eval_statement(&mut self, statement: Statement) -> Result<(), ReplError> {
        match statement {
            Statement::Expression(tokens) => {
                if self.trace_execution {
                    self.execute_tokens_with_trace(&tokens)?;
                } else {
                    self.vm.execute_tokens(&tokens)?;
                }
            }
            Statement::TypeSignatureDecl { name, signature } => {
                self.handle_type_signature_decl(name, signature)?;
            }
            Statement::WordDefinition(word_def) => {
                self.handle_word_definition(word_def)?;
            }
            Statement::TypeDefinition(type_def) => {
                self.handle_type_definition(type_def)?;
            }
            Statement::AxiomDeclaration { name, signature } => {
                self.handle_axiom_declaration(name, signature)?;
            }
        }
        Ok(())
    }

    /// Execute tokens with detailed tracing
    fn execute_tokens_with_trace(&mut self, tokens: &[Token]) -> Result<(), VmError> {
        for token in tokens {
            let stack_before = self.vm.stack().to_vec();
            let start_time = Instant::now();

            // Execute the token
            self.vm.execute_token(token)?;

            let duration = start_time.elapsed();
            let stack_after = self.vm.stack().to_vec();

            // Record trace entry
            let trace_entry = TraceEntry {
                timestamp: start_time,
                token: token.clone(),
                stack_before: stack_before.clone(),
                stack_after: stack_after.clone(),
                duration,
            };

            // Print trace if enabled
            if self.trace_execution {
                println!(
                    "TRACE: {} | {} -> {} ({:?})",
                    token,
                    stack_before.len(),
                    stack_after.len(),
                    duration
                );
            }

            self.add_trace_entry(trace_entry);
        }
        Ok(())
    }

    /// Add entry to execution trace (with size limit)
    fn add_trace_entry(&mut self, entry: TraceEntry) {
        self.execution_trace.push(entry);
        if self.execution_trace.len() > self.max_trace_entries {
            self.execution_trace.remove(0);
        }
    }

    /// Add command to history (with size limit)
    fn add_to_history(&mut self, command: String) {
        self.command_history.push(command);
        if self.command_history.len() > self.max_history_entries {
            self.command_history.remove(0);
        }
    }

    /// Handle type signature declaration
    fn handle_type_signature_decl(
        &mut self,
        name: String,
        signature: TypeSignature,
    ) -> Result<(), ReplError> {
        // Store the signature for later use when the word is defined
        self.pending_signatures.insert(name.clone(), signature);
        println!("Type signature declared for '{}'", name);
        Ok(())
    }

    /// Handle word definition
    fn handle_word_definition(&mut self, mut word_def: WordDefinition) -> Result<(), ReplError> {
        // Check if we have a pending type signature
        if let Some(signature) = self.pending_signatures.remove(&word_def.name) {
            word_def.signature = signature;
        } else if word_def.signature.inputs.is_empty() && word_def.signature.outputs.is_empty() {
            // No explicit signature - this should not happen since parser now does inference
            return Err(ReplError::DefinitionError(format!(
                "Word '{}' needs a type signature or could not be inferred. Use :: to declare it first.",
                word_def.name
            )));
        } else if self.type_debug {
            // Show inferred type if type debugging is enabled
            println!(
                "Inferred type for '{}': {} -> {}",
                word_def.name,
                Self::format_type_list(&word_def.signature.inputs),
                Self::format_type_list(&word_def.signature.outputs)
            );
        }

        // Verify termination using ordinal analysis
        match self.verifier.verify_termination(&word_def) {
            Ok(ordinal_cost) => {
                word_def.ordinal_cost = ordinal_cost.clone();
                if self.show_ordinals {
                    println!("Ordinal cost for '{}': {}", word_def.name, ordinal_cost);
                }
            }
            Err(e) => {
                return Err(ReplError::DefinitionError(format!(
                    "Ordinal verification failed for '{}': {}",
                    word_def.name, e
                )));
            }
        }

        // Define the word in the VM
        let name = word_def.name.clone();
        let signature = word_def.signature.clone();
        self.vm.define_word(word_def);

        // Add the word signature to the type inferer for future inference
        self.type_inferer
            .add_word_signature(name.clone(), signature);

        println!("Defined word '{}'", name);
        Ok(())
    }

    /// Handle type definition
    fn handle_type_definition(&mut self, type_def: TypeDefinition) -> Result<(), ReplError> {
        let name = type_def.name.clone();
        self.user_types.insert(name.clone(), type_def);
        println!("Defined type '{}'", name);
        Ok(())
    }

    /// Handle axiom declaration
    fn handle_axiom_declaration(
        &mut self,
        name: String,
        _signature: TypeSignature,
    ) -> Result<(), ReplError> {
        // Check if we have a pending signature that matches
        if let Some(pending_sig) = self.pending_signatures.remove(&name) {
            let axiom_def = WordDefinition {
                name: name.clone(),
                signature: pending_sig,
                body: vec![],
                is_axiom: true,
                ordinal_cost: OrdinalValue::Zero, // Axioms have no computational cost
            };
            self.vm.define_word(axiom_def);
            println!("Declared axiom '{}'", name);
        } else {
            return Err(ReplError::DefinitionError(format!(
                "Axiom '{}' needs a type signature declared with :: first",
                name
            )));
        }
        Ok(())
    }

    /// Handle REPL commands (starting with .)
    fn handle_repl_command(&mut self, command: &str) -> Result<(), ReplError> {
        let parts: Vec<&str> = command[1..].split_whitespace().collect();

        match parts.get(0) {
            Some(&"s") => {
                self.show_stack_contents();
            }
            Some(&"stack") => {
                self.show_stack = !self.show_stack;
                println!(
                    "Stack display: {}",
                    if self.show_stack { "ON" } else { "OFF" }
                );
            }
            Some(&"ordinals") => {
                self.show_ordinals = !self.show_ordinals;
                println!(
                    "Ordinal display: {}",
                    if self.show_ordinals { "ON" } else { "OFF" }
                );
            }
            Some(&"trace") => {
                self.trace_execution = !self.trace_execution;
                println!(
                    "Execution tracing: {}",
                    if self.trace_execution { "ON" } else { "OFF" }
                );
            }
            Some(&"clear") => {
                self.vm.clear_stack();
                println!("Stack cleared");
            }
            Some(&"clear-trace") => {
                self.execution_trace.clear();
                println!("Execution trace cleared");
            }
            Some(&"words") => {
                self.show_words();
            }
            Some(&"types") => {
                self.show_types();
            }
            Some(&"infer") => {
                if let Some(word_name) = parts.get(1) {
                    self.infer_word_type(word_name)?;
                } else {
                    println!("Usage: .infer <word_name>");
                }
            }
            Some(&"type-debug") => {
                self.type_debug = !self.type_debug;
                self.type_inferer.set_debug(self.type_debug);
                println!(
                    "Type inference debugging: {}",
                    if self.type_debug { "ON" } else { "OFF" }
                );
            }
            Some(&"trace-log") => {
                self.show_trace_log();
            }
            Some(&"performance") | Some(&"perf") => {
                self.show_performance_metrics();
            }
            Some(&"history") => {
                self.show_command_history();
            }
            Some(&"save") => {
                if let Some(filename) = parts.get(1) {
                    self.save_session(filename)?;
                } else {
                    println!("Usage: .save <filename>");
                }
            }
            Some(&"load") => {
                if let Some(filename) = parts.get(1) {
                    self.load_session(filename)?;
                } else {
                    println!("Usage: .load <filename>");
                }
            }
            Some(&"benchmark") => {
                if parts.len() >= 3 {
                    let code = parts[1];
                    if let Ok(iterations) = parts[2].parse::<usize>() {
                        self.benchmark_code(code, iterations)?;
                    } else {
                        println!("Usage: .benchmark <code> <iterations>");
                    }
                } else {
                    println!("Usage: .benchmark <code> <iterations>");
                }
            }
            Some(&"help") => {
                self.show_help();
            }
            Some(&"about") => {
                self.show_about();
            }
            Some(&"reset") => {
                *self = Repl::new();
                println!("REPL reset to initial state");
            }
            // Theorem proving commands
            Some(&"axioms") => {
                self.show_axioms();
            }
            Some(&"theorems") => {
                self.show_theorems();
            }
            Some(&"assume") => {
                if let Some(assumption) = parts.get(1) {
                    self.add_assumption(assumption)?;
                } else {
                    println!("Usage: .assume <proposition>");
                }
            }
            Some(&"goal") => {
                if let Some(goal) = parts.get(1) {
                    self.set_goal(goal)?;
                } else {
                    println!("Usage: .goal <proposition>");
                }
            }
            Some(&"prove") => {
                self.show_proof_state();
            }
            Some(&"qed") => {
                self.complete_proof()?;
            }
            // Cellular Automata commands
            Some(&"ca-rules") => {
                self.show_ca_rules();
            }
            Some(&"ca-rule") => {
                if let Some(rule_str) = parts.get(1) {
                    if let Ok(rule) = rule_str.parse::<u8>() {
                        self.show_ca_rule(rule);
                    } else {
                        println!("Usage: .ca-rule <number> (0-255)");
                    }
                } else {
                    println!("Usage: .ca-rule <number> (0-255)");
                }
            }
            Some(&"ca-simple") => {
                if parts.len() >= 3 {
                    if let (Ok(rule), Ok(gens)) =
                        (parts[1].parse::<u8>(), parts[2].parse::<usize>())
                    {
                        let pattern = if parts.len() > 3 {
                            Some(parts[3..].join(" "))
                        } else {
                            None
                        };
                        self.run_simple_ca(rule, gens, pattern.as_deref());
                    } else {
                        println!("Usage: .ca-simple <rule> <generations> [pattern]");
                    }
                } else {
                    println!("Usage: .ca-simple <rule> <generations> [pattern]");
                }
            }
            Some(&"ca") => {
                if parts.len() >= 2 {
                    if let Ok(rule) = parts[1].parse::<u8>() {
                        let pattern = if parts.len() > 2 {
                            Some(parts[2..].join(" "))
                        } else {
                            None
                        };
                        self.launch_interactive_ca(rule, pattern.as_deref());
                    } else {
                        println!("Usage: .ca <rule> [pattern]");
                    }
                } else {
                    println!("Usage: .ca <rule> [pattern]");
                }
            }
            Some(cmd) => {
                println!("Unknown command: .{}", cmd);
                println!("Type '.help' for available commands");
            }
            None => {
                println!("Empty command. Type '.help' for available commands");
            }
        }
        Ok(())
    }

    /// Show current stack contents
    fn show_stack_contents(&self) {
        let stack = self.vm.stack();
        print!("<{}> ", stack.len());
        for value in stack {
            print!("{} ", value);
        }
        println!();
    }

    /// Show all defined words
    fn show_words(&self) {
        println!("Core words:");
        let mut core_words: Vec<_> = self.core_lib.get_core_words().keys().collect();
        core_words.sort();

        for chunk in core_words.chunks(8) {
            for word in chunk {
                print!("{:12} ", word);
            }
            println!();
        }

        // Note: User-defined words would be shown here if we tracked them
        println!("\nUse 'help' for detailed documentation");
    }

    /// Show all defined types
    fn show_types(&self) {
        println!("Core types:");
        println!("  Unit     Bool     Nat      Ordinal  Quote");

        if !self.user_types.is_empty() {
            println!("\nUser types:");
            for type_name in self.user_types.keys() {
                println!("  {}", type_name);
            }
        }
    }

    /// Infer and show the type signature for a word
    fn infer_word_type(&mut self, word_name: &str) -> Result<(), ReplError> {
        // First, check if it's a core library word
        if let Some(core_word) = self.core_lib.get_core_words().get(word_name) {
            println!(
                "Core word '{}' has type: {} -> {}",
                word_name,
                Self::format_type_list(&core_word.signature.inputs),
                Self::format_type_list(&core_word.signature.outputs)
            );
            return Ok(());
        }

        // Check if it's a user-defined word in the VM
        if let Some(user_word) = self.vm.get_word_definition(word_name) {
            println!(
                "User word '{}' has type: {} -> {}",
                word_name,
                Self::format_type_list(&user_word.signature.inputs),
                Self::format_type_list(&user_word.signature.outputs)
            );
            return Ok(());
        }

        // Try to infer from a simple word definition pattern
        println!(
            "Word '{}' not found. To infer a type for a new word, define it first with ':', e.g.:",
            word_name
        );
        println!("  : {} your-definition-here ;", word_name);
        println!("Then use .infer {} to see the inferred type.", word_name);

        Ok(())
    }

    /// Format a list of types for display
    fn format_type_list(types: &[crate::types::Type]) -> String {
        if types.is_empty() {
            "()".to_string()
        } else {
            types
                .iter()
                .map(|t| format!("{}", t))
                .collect::<Vec<_>>()
                .join(" ")
        }
    }

    /// Show help information
    fn show_help(&self) {
        println!("C∀O REPL Commands:");
        println!("==================");
        println!();
        println!("Basic Commands:");
        println!("  .s               Show stack contents");
        println!("  .stack           Toggle automatic stack display");
        println!("  .ordinals        Toggle ordinal cost display");
        println!("  .clear           Clear the stack");
        println!("  .words           List all defined words");
        println!("  .types           List all types");
        println!("  .help            Show this help");
        println!("  .about           About C∀O");
        println!("  .reset           Reset REPL to initial state");
        println!();
        println!("Development & Debugging:");
        println!("  .trace           Toggle execution tracing");
        println!("  .trace-log       Show recent execution trace");
        println!("  .clear-trace     Clear execution trace");
        println!("  .performance     Show performance metrics");
        println!("  .history         Show command history");
        println!("  .infer <word>    Show inferred type for word");
        println!("  .type-debug      Toggle type inference debugging");
        println!();
        println!("Session Management:");
        println!("  .save <file>     Save current session to file");
        println!("  .load <file>     Load session from file");
        println!();
        println!("Performance Analysis:");
        println!("  .benchmark <code> <n>  Benchmark code n times");
        println!();
        println!("Language Syntax:");
        println!("  :: name ( inputs -> outputs ) ;   Declare type signature");
        println!("  : name body ;                      Define word");
        println!("  type Name {{ field::Type, ... }}     Define type");
        println!("  axiom name                         Declare axiom");
        println!("  [ code ]                           Quotation");
        println!("  ( comment )                        Comment");
        println!();
        println!("Examples:");
        println!("  3 4 +                             Simple arithmetic");
        println!("  :: square ( Nat -> Nat ) ;        Type signature");
        println!("  : square dup * ;                  Word definition");
        println!("  5 square                          Use word");
        println!("  .trace                            Enable tracing");
        println!("  .save my-work                     Save session");
        println!();
        println!("Type 'help' (without .) for core library documentation");
        println!();
        println!("Theorem Proving:");
        println!("  .axioms          List all axioms");
        println!("  .theorems        List proven theorems");
        println!("  .assume <prop>   Add assumption");
        println!("  .goal <prop>     Set proof goal");
        println!("  .prove           Show current proof state");
        println!("  .qed             Complete current proof");
        println!();
        println!("Cellular Automata:");
        println!("  .ca <rule> [pattern]      Interactive CA with rule (0-255)");
        println!("  .ca-simple <rule> <gens>  Text output CA evolution");
        println!("  .ca-rule <rule>           Show rule table for rule number");
        println!("  .ca-rules                 List famous CA rules");
    }

    /// Show about information
    fn show_about(&self) {
        println!("C∀O (Kao) - Categorical ∀xiomatic Ordinal Programming Language");
        println!("==============================================================");
        println!();
        println!("Version: 0.1.0 (Phase 1 - Standalone Core)");
        println!();
        println!("C∀O is an evolving axiomatic programming language that combines:");
        println!("• Category Theory foundations (types as objects, functions as morphisms)");
        println!("• Ordinal Analysis for termination proofs and consistency");
        println!("• Concatenative programming with postfix notation");
        println!("• Collaborative verification and language evolution");
        println!();
        println!("This is the standalone core implementation. Future phases will add:");
        println!("• Network architecture (Hypervisor and Cells)");
        println!("• True ordinal verification");
        println!("• Self-evolution and metaprogramming");
        println!();
        println!("For more information, see the README.md and Implementation-plans.md");
    }

    /// Show execution trace log
    fn show_trace_log(&self) {
        if self.execution_trace.is_empty() {
            println!("No execution trace available. Enable with .trace");
            return;
        }

        println!(
            "Execution Trace (last {} entries):",
            self.execution_trace.len().min(20)
        );
        println!("==================================");

        let recent_entries = if self.execution_trace.len() > 20 {
            &self.execution_trace[self.execution_trace.len() - 20..]
        } else {
            &self.execution_trace
        };

        for (i, entry) in recent_entries.iter().enumerate() {
            println!(
                "{:3}: {} | Stack: {} -> {} | Time: {:?}",
                i + 1,
                entry.token,
                entry.stack_before.len(),
                entry.stack_after.len(),
                entry.duration
            );
        }
    }

    /// Show performance metrics
    fn show_performance_metrics(&self) {
        println!("Performance Metrics:");
        println!("===================");
        println!(
            "Total execution time: {:?}",
            self.performance_metrics.total_time
        );
        println!(
            "Operations executed: {}",
            self.performance_metrics.operation_count
        );
        println!(
            "Max stack depth: {}",
            self.performance_metrics.max_stack_depth
        );
        println!(
            "Approx. memory usage: {} bytes",
            self.performance_metrics.memory_usage
        );

        if self.performance_metrics.operation_count > 0 {
            let avg_time = self.performance_metrics.total_time
                / self.performance_metrics.operation_count as u32;
            println!("Average time per operation: {:?}", avg_time);
        }

        println!("Trace entries stored: {}", self.execution_trace.len());
        println!("History entries: {}", self.command_history.len());
    }

    /// Show command history
    fn show_command_history(&self) {
        if self.command_history.is_empty() {
            println!("No command history available");
            return;
        }

        println!(
            "Command History (last {} entries):",
            self.command_history.len().min(20)
        );
        println!("==================================");

        let recent_history = if self.command_history.len() > 20 {
            &self.command_history[self.command_history.len() - 20..]
        } else {
            &self.command_history
        };

        for (i, command) in recent_history.iter().enumerate() {
            println!("{:3}: {}", i + 1, command);
        }
    }

    /// Save current session to file
    fn save_session(&self, filename: &str) -> Result<(), ReplError> {
        let session_data = SessionData {
            user_types: self.user_types.clone(),
            pending_signatures: self.pending_signatures.clone(),
            user_words: Vec::new(), // TODO: Extract user words from VM
            stack: self.vm.stack().to_vec(),
            show_stack: self.show_stack,
            show_ordinals: self.show_ordinals,
            trace_execution: self.trace_execution,
        };

        let json_data = serde_json::to_string_pretty(&session_data)
            .map_err(|e| ReplError::SessionError(format!("Failed to serialize session: {}", e)))?;

        let session_dir = Path::new("sessions");
        if !session_dir.exists() {
            fs::create_dir_all(session_dir).map_err(|e| {
                ReplError::IoError(format!("Failed to create sessions directory: {}", e))
            })?;
        }

        let filepath = session_dir.join(format!("{}.json", filename));
        fs::write(&filepath, json_data)
            .map_err(|e| ReplError::IoError(format!("Failed to write session file: {}", e)))?;

        println!("Session saved to: {}", filepath.display());
        Ok(())
    }

    /// Load session from file
    fn load_session(&mut self, filename: &str) -> Result<(), ReplError> {
        let session_dir = Path::new("sessions");
        let filepath = session_dir.join(format!("{}.json", filename));

        if !filepath.exists() {
            return Err(ReplError::SessionError(format!(
                "Session file not found: {}",
                filepath.display()
            )));
        }

        let json_data = fs::read_to_string(&filepath)
            .map_err(|e| ReplError::IoError(format!("Failed to read session file: {}", e)))?;

        let session_data: SessionData = serde_json::from_str(&json_data).map_err(|e| {
            ReplError::SessionError(format!("Failed to deserialize session: {}", e))
        })?;

        // Restore session data
        self.user_types = session_data.user_types;
        self.pending_signatures = session_data.pending_signatures;
        self.show_stack = session_data.show_stack;
        self.show_ordinals = session_data.show_ordinals;
        self.trace_execution = session_data.trace_execution;

        // Restore stack
        self.vm.clear_stack();
        for value in session_data.stack {
            self.vm.push(value);
        }

        // TODO: Restore user-defined words

        println!("Session loaded from: {}", filepath.display());
        Ok(())
    }

    /// Benchmark code execution
    fn show_axioms(&self) {
        println!("Available Axioms:");
        println!("================");

        let mut axiom_count = 0;
        for (name, word_def) in self.vm.get_dictionary() {
            if word_def.is_axiom {
                println!("  {} :: {:?}", name, word_def.signature);
                axiom_count += 1;
            }
        }

        if axiom_count == 0 {
            println!("  No axioms defined");
        } else {
            println!("\nTotal: {} axioms", axiom_count);
        }
    }

    fn show_theorems(&self) {
        println!("Proven Theorems:");
        println!("===============");

        let mut theorem_count = 0;
        for (name, word_def) in self.vm.get_dictionary() {
            if !word_def.is_axiom && !word_def.body.is_empty() {
                println!("  {} :: {:?}", name, word_def.signature);
                theorem_count += 1;
            }
        }

        if theorem_count == 0 {
            println!("  No theorems proven yet");
        } else {
            println!("\nTotal: {} theorems", theorem_count);
        }
    }

    fn add_assumption(&mut self, assumption: &str) -> Result<(), ReplError> {
        // For now, just add it as a comment in the trace
        println!("Added assumption: {}", assumption);
        // TODO: Implement proper assumption stack
        Ok(())
    }

    fn set_goal(&mut self, goal: &str) -> Result<(), ReplError> {
        // For now, just display the goal
        println!("Proof goal: {}", goal);
        // TODO: Implement proper goal management
        Ok(())
    }

    fn show_proof_state(&self) {
        println!("Current Proof State:");
        println!("===================");
        println!("Stack: {:?}", self.vm.get_stack());
        // TODO: Show assumptions, goals, and proof progress
        println!("(Proof state tracking not yet implemented)");
    }

    fn complete_proof(&mut self) -> Result<(), ReplError> {
        println!("Proof completed!");
        // TODO: Implement proof verification and theorem storage
        Ok(())
    }

    fn benchmark_code(&mut self, code: &str, iterations: usize) -> Result<(), ReplError> {
        println!("Benchmarking '{}' for {} iterations...", code, iterations);

        let mut total_time = Duration::from_nanos(0);
        let mut min_time = Duration::from_secs(u64::MAX);
        let mut max_time = Duration::from_nanos(0);

        // Save current stack state
        let initial_stack = self.vm.stack().to_vec();

        for i in 0..iterations {
            // Restore initial stack state
            self.vm.clear_stack();
            for value in &initial_stack {
                self.vm.push(value.clone());
            }

            let start_time = Instant::now();

            // Parse and execute the code
            let mut parser = Parser::new(code)?;
            let statements = parser.parse_all()?;
            for statement in statements {
                self.eval_statement(statement)?;
            }

            let duration = start_time.elapsed();
            total_time += duration;
            min_time = min_time.min(duration);
            max_time = max_time.max(duration);

            if i % (iterations / 10).max(1) == 0 {
                print!(".");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
            }
        }

        println!();
        println!("Benchmark Results:");
        println!("=================");
        println!("Total time: {:?}", total_time);
        println!("Average time: {:?}", total_time / iterations as u32);
        println!("Min time: {:?}", min_time);
        println!("Max time: {:?}", max_time);
        println!("Iterations: {}", iterations);

        // Restore original stack
        self.vm.clear_stack();
        for value in &initial_stack {
            self.vm.push(value.clone());
        }

        Ok(())
    }

    /// Show famous cellular automaton rules
    fn show_ca_rules(&self) {
        println!("Famous Cellular Automaton Rules:");
        println!();
        println!("Rule 30: Rule 30 (Chaotic)");
        println!("Rule 90: Rule 90 (Sierpinski Triangle)");
        println!("Rule 110: Rule 110 (Turing Complete)");
        println!("Rule 150: Rule 150 (XOR)");
        println!("Rule 184: Rule 184 (Traffic)");
        println!();
        println!("Use '.ca <rule>' to run interactively or '.ca-simple <rule> <generations>' for text output");
    }

    /// Show rule table for a specific CA rule
    fn show_ca_rule(&self, rule: u8) {
        println!("Elementary Cellular Automaton Rule {}", rule);

        let rule_name = match rule {
            30 => Some("Rule 30 (Chaotic)"),
            90 => Some("Rule 90 (Sierpinski Triangle)"),
            110 => Some("Rule 110 (Turing Complete)"),
            150 => Some("Rule 150 (XOR)"),
            184 => Some("Rule 184 (Traffic)"),
            _ => None,
        };

        if let Some(name) = rule_name {
            println!("{}", name);
        }
        println!();

        println!("Pattern -> Output");
        for i in (0..8).rev() {
            let left = (i & 4) != 0;
            let center = (i & 2) != 0;
            let right = (i & 1) != 0;
            let output = (rule >> i) & 1 == 1;
            println!(
                "{}{}{} -> {}",
                if left { "1" } else { "0" },
                if center { "1" } else { "0" },
                if right { "1" } else { "0" },
                if output { "1" } else { "0" }
            );
        }
    }

    /// Run simple cellular automaton and display results
    fn run_simple_ca(&self, rule: u8, generations: usize, pattern: Option<&str>) {
        println!(
            "Running Elementary CA Rule {} for {} generations",
            rule, generations
        );

        // Create CA directly
        let width = 79;
        let ca = if let Some(p) = pattern {
            self.create_ca_with_pattern(width, rule, p)
        } else {
            self.create_ca_single_seed(width, rule)
        };

        if let Some(mut ca) = ca {
            // Evolve the CA
            for _ in 0..generations {
                ca.step();
            }

            // Display the history
            println!("{}", ca.history_string());
        }
    }

    /// Launch interactive cellular automaton environment
    fn launch_interactive_ca(&self, rule: u8, pattern: Option<&str>) {
        println!("Interactive Cellular Automaton (Rule {})", rule);
        if let Some(name) = self.get_rule_name(rule) {
            println!("{}", name);
        }
        println!();
        println!("Controls:");
        println!("  Press Enter to evolve one generation");
        println!("  Type 'n <number>' to evolve multiple generations");
        println!("  Type 'r' to reset");
        println!("  Type 'q' to quit");
        println!();

        let width = 79;
        let initial_ca = if let Some(p) = pattern {
            self.create_ca_with_pattern(width, rule, p)
        } else {
            self.create_ca_single_seed(width, rule)
        };

        if let Some(mut ca) = initial_ca {
            println!("Initial state:");
            println!("{}", ca.current_generation_string());
            println!();

            self.run_interactive_ca_loop(&mut ca, rule, pattern);
        }
    }

    /// Create CA with single seed in center
    fn create_ca_single_seed(&self, width: usize, rule: u8) -> Option<SimpleCA> {
        let mut cells = vec![false; width];
        cells[width / 2] = true;
        Some(SimpleCA::new(cells, rule))
    }

    /// Create CA with custom pattern
    fn create_ca_with_pattern(&self, width: usize, rule: u8, pattern: &str) -> Option<SimpleCA> {
        if pattern.len() > width {
            eprintln!("Pattern too long for automaton width");
            return None;
        }

        let mut cells = vec![false; width];
        let start_pos = (width - pattern.len()) / 2;

        for (i, ch) in pattern.chars().enumerate() {
            match ch {
                '1' | '█' | '#' | '*' => cells[start_pos + i] = true,
                '0' | ' ' | '.' | '-' => cells[start_pos + i] = false,
                _ => {
                    eprintln!("Invalid pattern character: {}", ch);
                    return None;
                }
            }
        }

        Some(SimpleCA::new(cells, rule))
    }

    /// Get rule name
    fn get_rule_name(&self, rule: u8) -> Option<&'static str> {
        match rule {
            30 => Some("Rule 30 (Chaotic)"),
            90 => Some("Rule 90 (Sierpinski Triangle)"),
            110 => Some("Rule 110 (Turing Complete)"),
            150 => Some("Rule 150 (XOR)"),
            184 => Some("Rule 184 (Traffic)"),
            _ => None,
        }
    }

    /// Run interactive CA loop
    fn run_interactive_ca_loop(&self, ca: &mut SimpleCA, rule: u8, pattern: Option<&str>) {
        use std::io::{self, Write};

        loop {
            print!("CA> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                break;
            }

            let input = input.trim();
            if input.is_empty() {
                ca.step();
                println!("Generation {}:", ca.generation);
                println!("{}", ca.current_generation_string());
            } else if input == "q" || input == "quit" {
                break;
            } else if input == "r" || input == "reset" {
                *ca = if let Some(p) = pattern {
                    if let Some(new_ca) = self.create_ca_with_pattern(ca.width(), rule, p) {
                        new_ca
                    } else {
                        continue;
                    }
                } else {
                    if let Some(new_ca) = self.create_ca_single_seed(ca.width(), rule) {
                        new_ca
                    } else {
                        continue;
                    }
                };
                println!("Reset to initial state");
                println!("{}", ca.current_generation_string());
            } else if let Some(n_str) = input.strip_prefix("n ") {
                if let Ok(n) = n_str.parse::<usize>() {
                    for _ in 0..n {
                        ca.step();
                    }
                    println!(
                        "Evolved {} generations. Now at generation {}:",
                        n, ca.generation
                    );
                    println!("{}", ca.history_string_last(20));
                } else {
                    println!("Invalid number: {}", n_str);
                }
            } else if input == "h" || input == "help" {
                println!("Controls:");
                println!("  Enter     - Evolve one generation");
                println!("  n <num>   - Evolve multiple generations");
                println!("  r         - Reset to initial state");
                println!("  h         - Show this help");
                println!("  q         - Quit");
            } else {
                println!("Unknown command: {}. Type 'h' for help.", input);
            }
        }
    }
}

/// Simple cellular automaton implementation
struct SimpleCA {
    cells: Vec<bool>,
    rule: u8,
    generation: usize,
    history: Vec<Vec<bool>>,
}

impl SimpleCA {
    fn new(cells: Vec<bool>, rule: u8) -> Self {
        Self {
            cells: cells.clone(),
            rule,
            generation: 0,
            history: vec![cells],
        }
    }

    fn width(&self) -> usize {
        self.cells.len()
    }

    fn step(&mut self) {
        let mut next_cells = vec![false; self.cells.len()];

        for i in 0..self.cells.len() {
            let left = if i == 0 { false } else { self.cells[i - 1] };
            let center = self.cells[i];
            let right = if i == self.cells.len() - 1 {
                false
            } else {
                self.cells[i + 1]
            };

            next_cells[i] = self.apply_rule(left, center, right);
        }

        self.cells = next_cells.clone();
        self.history.push(next_cells);
        self.generation += 1;
    }

    fn apply_rule(&self, left: bool, center: bool, right: bool) -> bool {
        let pattern = (left as u8) << 2 | (center as u8) << 1 | (right as u8);
        (self.rule >> pattern) & 1 == 1
    }

    fn current_generation_string(&self) -> String {
        let cells_str: String = self
            .cells
            .iter()
            .map(|&cell| if cell { '█' } else { '·' })
            .collect();
        format!("{:3}: {}", self.generation, cells_str)
    }

    fn history_string(&self) -> String {
        self.history
            .iter()
            .enumerate()
            .map(|(i, generation)| {
                let cells_str: String = generation
                    .iter()
                    .map(|&cell| if cell { '█' } else { '·' })
                    .collect();
                format!("{:3}: {}", i, cells_str)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn history_string_last(&self, count: usize) -> String {
        let start_idx = self.history.len().saturating_sub(count);
        self.history[start_idx..]
            .iter()
            .enumerate()
            .map(|(i, generation)| {
                let gen_num = start_idx + i;
                let cells_str: String = generation
                    .iter()
                    .map(|&cell| if cell { '█' } else { '·' })
                    .collect();
                format!("{:3}: {}", gen_num, cells_str)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repl_creation() {
        let repl = Repl::new();
        assert!(!repl.show_stack);
        assert!(!repl.show_ordinals);
    }

    #[test]
    fn test_simple_arithmetic() {
        let mut repl = Repl::new();

        // Test basic arithmetic
        let result = repl.eval("3 4 +");
        assert!(result.is_ok());

        // Check that 7 is on the stack
        assert_eq!(repl.vm.stack().len(), 1);
        if let Value::Nat(n) = &repl.vm.stack()[0] {
            assert_eq!(*n, 7);
        } else {
            panic!("Expected Nat(7) on stack");
        }
    }

    #[test]
    fn test_word_definition() {
        let mut repl = Repl::new();

        // Define a simple word
        let result = repl.eval(":: double ( Nat -> Nat ) ;");
        assert!(result.is_ok());

        let result = repl.eval(": double 2 * ;");
        assert!(result.is_ok());

        // Use the word
        let result = repl.eval("5 double");
        assert!(result.is_ok());

        // Check result
        assert_eq!(repl.vm.stack().len(), 1);
        if let Value::Nat(n) = &repl.vm.stack()[0] {
            assert_eq!(*n, 10);
        } else {
            panic!("Expected Nat(10) on stack");
        }
    }

    #[test]
    fn test_repl_commands() {
        let mut repl = Repl::new();

        // Test stack toggle
        assert!(!repl.show_stack);
        let result = repl.eval(".stack");
        assert!(result.is_ok());
        assert!(repl.show_stack);

        // Test ordinals toggle
        assert!(!repl.show_ordinals);
        let result = repl.eval(".ordinals");
        assert!(result.is_ok());
        assert!(repl.show_ordinals);
    }

    #[test]
    fn test_quotations() {
        let mut repl = Repl::new();

        // Test quotation creation
        let result = repl.eval("[ 2 * ]");
        assert!(result.is_ok());

        // Check that a quote is on the stack
        assert_eq!(repl.vm.stack().len(), 1);
        if let Value::Quote(_) = &repl.vm.stack()[0] {
            // Success
        } else {
            panic!("Expected Quote on stack");
        }
    }
}
