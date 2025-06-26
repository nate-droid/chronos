//! Chronos: C∀O (Kao) - Categorical ∀xiomatic Ordinal Programming Language
//!
//! An evolving axiomatic programming language that combines:
//! - Category Theory foundations
//! - Ordinal Analysis for termination proofs
//! - Concatenative (stack-based) programming
//! - Collaborative verification and evolution
//! - Autonomous goal-oriented shell environments

pub mod core_lib;
pub mod goal_builders;
pub mod lexer;
pub mod ordinal;
pub mod parser;
pub mod repl;
pub mod shell;
pub mod shell_manager;
pub mod type_inference;
pub mod types;
pub mod vm;

// Re-export commonly used types and functions for convenience
pub use goal_builders::{
    AxiomBuilder, ComputationBuilder, ExplorationBuilder, GoalTemplate, GoalTemplateFactory,
    PuzzleBuilder, quick,
};
pub use repl::Repl;
pub use shell::{CompletionStatus, Goal, GoalType, Shell, ShellMode};
pub use shell_manager::{CoordinationStrategy, ShellManager};
pub use types::{Type, TypeSignature, Value};
