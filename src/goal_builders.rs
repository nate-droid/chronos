//! Goal Builders for Easy Goal Creation and Common Patterns
//!
//! This module provides convenient builders and templates for creating
//! common types of goals that shells can work toward autonomously.

use crate::shell::{CompletionStatus, Goal, GoalType};
use chronos_core::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Builder for creating puzzle goals
pub struct PuzzleBuilder {
    name: String,
    description: String,
    inputs: Vec<Value>,
    expected_outputs: Vec<Value>,
    max_attempts: u32,
    priority: u32,
    context: HashMap<String, Value>,
}

/// Builder for creating computation goals
pub struct ComputationBuilder {
    name: String,
    target: String,
    max_time: Duration,
    max_operations: u64,
    priority: u32,
    context: HashMap<String, Value>,
}

/// Builder for creating axiom exploration goals
pub struct AxiomBuilder {
    name: String,
    statement: String,
    constraints: Vec<String>,
    proof_steps: Vec<String>,
    priority: u32,
    context: HashMap<String, Value>,
}

/// Builder for creating exploration goals
pub struct ExplorationBuilder {
    name: String,
    domain: String,
    discovery_criteria: Vec<String>,
    max_iterations: u32,
    priority: u32,
    context: HashMap<String, Value>,
}

/// Pre-defined goal templates for common mathematical problems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalTemplate {
    /// Basic arithmetic challenges
    ArithmeticPuzzle {
        operation: ArithmeticOperation,
        difficulty: DifficultyLevel,
    },
    /// Number theory explorations
    NumberTheory {
        topic: NumberTheoryTopic,
        range: (u64, u64),
    },
    /// Logic puzzles
    LogicPuzzle {
        puzzle_type: LogicPuzzleType,
        size: usize,
    },
    /// Algorithm implementation challenges
    Algorithm {
        algorithm_type: AlgorithmType,
        input_size: usize,
    },
    /// Mathematical proof attempts
    ProofChallenge {
        theorem: String,
        difficulty: DifficultyLevel,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ArithmeticOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modular,
    Power,
    Factorial,
    Fibonacci,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NumberTheoryTopic {
    PrimeFactorization,
    GCD,
    LCM,
    PrimalityTesting,
    CollatzConjecture,
    PerfectNumbers,
    AmicableNumbers,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogicPuzzleType {
    Sudoku,
    NQueens,
    KnightsTour,
    TowerOfHanoi,
    GraphColoring,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlgorithmType {
    Sorting,
    Searching,
    GraphTraversal,
    DynamicProgramming,
    Recursion,
    Backtracking,
}

impl PuzzleBuilder {
    /// Create a new puzzle builder
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            inputs: Vec::new(),
            expected_outputs: Vec::new(),
            max_attempts: 50,
            priority: 1,
            context: HashMap::new(),
        }
    }

    /// Set the puzzle description
    pub fn description<S: Into<String>>(mut self, desc: S) -> Self {
        self.description = desc.into();
        self
    }

    /// Add an input value
    pub fn input(mut self, value: Value) -> Self {
        self.inputs.push(value);
        self
    }

    /// Add multiple input values
    pub fn inputs(mut self, values: Vec<Value>) -> Self {
        self.inputs.extend(values);
        self
    }

    /// Set expected output
    pub fn expected_output(mut self, value: Value) -> Self {
        self.expected_outputs.push(value);
        self
    }

    /// Set multiple expected outputs
    pub fn expected_outputs(mut self, values: Vec<Value>) -> Self {
        self.expected_outputs = values;
        self
    }

    /// Set maximum attempts
    pub fn max_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = attempts;
        self
    }

    /// Set priority
    pub fn priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// Add context value
    pub fn context<S: Into<String>>(mut self, key: S, value: Value) -> Self {
        self.context.insert(key.into(), value);
        self
    }

    /// Build the goal
    pub fn build(self) -> Goal {
        let id = format!(
            "puzzle_{}",
            uuid::Uuid::new_v4().to_string()[..8].to_string()
        );

        Goal {
            id,
            goal_type: GoalType::Puzzle {
                name: self.name,
                description: self.description,
                inputs: self.inputs,
                expected_outputs: self.expected_outputs,
                max_attempts: self.max_attempts,
            },
            status: CompletionStatus::InProgress {
                attempts: 0,
                last_attempt: None,
                partial_results: vec![],
            },
            priority: self.priority,
            created_at: current_timestamp(),
            context: self.context,
            progress_log: vec![],
        }
    }
}

impl ComputationBuilder {
    /// Create a new computation builder
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            target: String::new(),
            max_time: Duration::from_secs(60),
            max_operations: 1000,
            priority: 1,
            context: HashMap::new(),
        }
    }

    /// Set the target computation
    pub fn target<S: Into<String>>(mut self, target: S) -> Self {
        self.target = target.into();
        self
    }

    /// Set maximum execution time
    pub fn max_time(mut self, time: Duration) -> Self {
        self.max_time = time;
        self
    }

    /// Set maximum operations
    pub fn max_operations(mut self, ops: u64) -> Self {
        self.max_operations = ops;
        self
    }

    /// Set priority
    pub fn priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// Add context value
    pub fn context<S: Into<String>>(mut self, key: S, value: Value) -> Self {
        self.context.insert(key.into(), value);
        self
    }

    /// Build the goal
    pub fn build(self) -> Goal {
        let id = format!(
            "computation_{}",
            uuid::Uuid::new_v4().to_string()[..8].to_string()
        );

        Goal {
            id,
            goal_type: GoalType::Computation {
                name: self.name,
                target: self.target,
                max_time: self.max_time,
                max_operations: self.max_operations,
            },
            status: CompletionStatus::InProgress {
                attempts: 0,
                last_attempt: None,
                partial_results: vec![],
            },
            priority: self.priority,
            created_at: current_timestamp(),
            context: self.context,
            progress_log: vec![],
        }
    }
}

impl AxiomBuilder {
    /// Create a new axiom builder
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            statement: String::new(),
            constraints: Vec::new(),
            proof_steps: Vec::new(),
            priority: 1,
            context: HashMap::new(),
        }
    }

    /// Set the axiom statement
    pub fn statement<S: Into<String>>(mut self, statement: S) -> Self {
        self.statement = statement.into();
        self
    }

    /// Add a constraint
    pub fn constraint<S: Into<String>>(mut self, constraint: S) -> Self {
        self.constraints.push(constraint.into());
        self
    }

    /// Add multiple constraints
    pub fn constraints(mut self, constraints: Vec<String>) -> Self {
        self.constraints.extend(constraints);
        self
    }

    /// Add a proof step
    pub fn proof_step<S: Into<String>>(mut self, step: S) -> Self {
        self.proof_steps.push(step.into());
        self
    }

    /// Set priority
    pub fn priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// Add context value
    pub fn context<S: Into<String>>(mut self, key: S, value: Value) -> Self {
        self.context.insert(key.into(), value);
        self
    }

    /// Build the goal
    pub fn build(self) -> Goal {
        let id = format!(
            "axiom_{}",
            uuid::Uuid::new_v4().to_string()[..8].to_string()
        );

        Goal {
            id,
            goal_type: GoalType::Axiom {
                name: self.name,
                statement: self.statement,
                constraints: self.constraints,
                proof_steps: self.proof_steps,
            },
            status: CompletionStatus::InProgress {
                attempts: 0,
                last_attempt: None,
                partial_results: vec![],
            },
            priority: self.priority,
            created_at: current_timestamp(),
            context: self.context,
            progress_log: vec![],
        }
    }
}

impl ExplorationBuilder {
    /// Create a new exploration builder
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            domain: String::new(),
            discovery_criteria: Vec::new(),
            max_iterations: 1000,
            priority: 1,
            context: HashMap::new(),
        }
    }

    /// Set the exploration domain
    pub fn domain<S: Into<String>>(mut self, domain: S) -> Self {
        self.domain = domain.into();
        self
    }

    /// Add a discovery criterion
    pub fn discovery_criterion<S: Into<String>>(mut self, criterion: S) -> Self {
        self.discovery_criteria.push(criterion.into());
        self
    }

    /// Add multiple discovery criteria
    pub fn discovery_criteria(mut self, criteria: Vec<String>) -> Self {
        self.discovery_criteria.extend(criteria);
        self
    }

    /// Set maximum iterations
    pub fn max_iterations(mut self, iterations: u32) -> Self {
        self.max_iterations = iterations;
        self
    }

    /// Set priority
    pub fn priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// Add context value
    pub fn context<S: Into<String>>(mut self, key: S, value: Value) -> Self {
        self.context.insert(key.into(), value);
        self
    }

    /// Build the goal
    pub fn build(self) -> Goal {
        let id = format!(
            "exploration_{}",
            uuid::Uuid::new_v4().to_string()[..8].to_string()
        );

        Goal {
            id,
            goal_type: GoalType::Exploration {
                name: self.name,
                domain: self.domain,
                discovery_criteria: self.discovery_criteria,
                max_iterations: self.max_iterations,
            },
            status: CompletionStatus::InProgress {
                attempts: 0,
                last_attempt: None,
                partial_results: vec![],
            },
            priority: self.priority,
            created_at: current_timestamp(),
            context: self.context,
            progress_log: vec![],
        }
    }
}

/// Template-based goal creation
pub struct GoalTemplateFactory;

impl GoalTemplateFactory {
    /// Create a goal from a template
    pub fn from_template(template: GoalTemplate) -> Goal {
        match template {
            GoalTemplate::ArithmeticPuzzle {
                operation,
                difficulty,
            } => Self::create_arithmetic_puzzle(operation, difficulty),
            GoalTemplate::NumberTheory { topic, range } => {
                Self::create_number_theory_goal(topic, range)
            }
            GoalTemplate::LogicPuzzle { puzzle_type, size } => {
                Self::create_logic_puzzle(puzzle_type, size)
            }
            GoalTemplate::Algorithm {
                algorithm_type,
                input_size,
            } => Self::create_algorithm_goal(algorithm_type, input_size),
            GoalTemplate::ProofChallenge {
                theorem,
                difficulty,
            } => Self::create_proof_challenge(theorem, difficulty),
        }
    }

    fn create_arithmetic_puzzle(
        operation: ArithmeticOperation,
        difficulty: DifficultyLevel,
    ) -> Goal {
        let (inputs, expected, max_attempts) = match (&operation, &difficulty) {
            (ArithmeticOperation::Addition, DifficultyLevel::Beginner) => {
                (vec![Value::Nat(5), Value::Nat(3)], vec![Value::Nat(8)], 10)
            }
            (ArithmeticOperation::Multiplication, DifficultyLevel::Intermediate) => (
                vec![Value::Nat(12), Value::Nat(7)],
                vec![Value::Nat(84)],
                20,
            ),
            (ArithmeticOperation::Fibonacci, DifficultyLevel::Advanced) => {
                (vec![Value::Nat(10)], vec![Value::Nat(55)], 50)
            }
            _ => (vec![Value::Nat(2), Value::Nat(3)], vec![Value::Nat(5)], 25),
        };

        PuzzleBuilder::new(format!("{:?} Puzzle", operation))
            .description(format!(
                "Solve {:?} problem with {:?} difficulty",
                operation, difficulty
            ))
            .inputs(inputs)
            .expected_outputs(expected)
            .max_attempts(max_attempts)
            .priority(match difficulty {
                DifficultyLevel::Beginner => 1,
                DifficultyLevel::Intermediate => 2,
                DifficultyLevel::Advanced => 3,
                DifficultyLevel::Expert => 4,
            })
            .build()
    }

    fn create_number_theory_goal(topic: NumberTheoryTopic, range: (u64, u64)) -> Goal {
        let target = match topic {
            NumberTheoryTopic::PrimeFactorization => {
                format!("{} prime-factors .", range.1)
            }
            NumberTheoryTopic::GCD => {
                format!("{} {} gcd .", range.0, range.1)
            }
            NumberTheoryTopic::PrimalityTesting => {
                format!("{} prime? .", range.1)
            }
            _ => format!("{} {} number-theory-operation", range.0, range.1),
        };

        ComputationBuilder::new(format!("{:?} Exploration", topic))
            .target(target)
            .max_time(Duration::from_secs(120))
            .max_operations(5000)
            .priority(2)
            .build()
    }

    fn create_logic_puzzle(puzzle_type: LogicPuzzleType, size: usize) -> Goal {
        ExplorationBuilder::new(format!("{:?} Puzzle", puzzle_type))
            .domain(format!("{:?} of size {}", puzzle_type, size))
            .discovery_criterion("valid_solution".to_string())
            .discovery_criterion("optimal_path".to_string())
            .max_iterations((size * size * 10) as u32)
            .priority(3)
            .build()
    }

    fn create_algorithm_goal(algorithm_type: AlgorithmType, input_size: usize) -> Goal {
        ComputationBuilder::new(format!("{:?} Implementation", algorithm_type))
            .target(format!("implement-{:?}-algorithm", algorithm_type))
            .max_time(Duration::from_secs(300))
            .max_operations((input_size * input_size) as u64)
            .priority(2)
            .context("input_size".to_string(), Value::Nat(input_size as u64))
            .build()
    }

    fn create_proof_challenge(theorem: String, difficulty: DifficultyLevel) -> Goal {
        AxiomBuilder::new("Proof Challenge")
            .statement(theorem)
            .constraint("formal_logic".to_string())
            .constraint("constructive_proof".to_string())
            .proof_step("establish_assumptions".to_string())
            .proof_step("apply_logical_rules".to_string())
            .proof_step("reach_conclusion".to_string())
            .priority(match difficulty {
                DifficultyLevel::Beginner => 2,
                DifficultyLevel::Intermediate => 3,
                DifficultyLevel::Advanced => 4,
                DifficultyLevel::Expert => 5,
            })
            .build()
    }

    /// Create a batch of goals for systematic exploration
    pub fn create_exploration_batch(domain: &str, count: usize) -> Vec<Goal> {
        let mut goals = Vec::new();

        for i in 0..count {
            let goal = ExplorationBuilder::new(format!("Exploration {} in {}", i + 1, domain))
                .domain(domain.to_string())
                .discovery_criterion("novel_pattern".to_string())
                .discovery_criterion("unexpected_result".to_string())
                .max_iterations(500)
                .priority(1)
                .context("batch_index".to_string(), Value::Nat(i as u64))
                .build();

            goals.push(goal);
        }

        goals
    }

    /// Create a progressive difficulty series
    pub fn create_progressive_series(operation: ArithmeticOperation) -> Vec<Goal> {
        vec![
            Self::from_template(GoalTemplate::ArithmeticPuzzle {
                operation: operation.clone(),
                difficulty: DifficultyLevel::Beginner,
            }),
            Self::from_template(GoalTemplate::ArithmeticPuzzle {
                operation: operation.clone(),
                difficulty: DifficultyLevel::Intermediate,
            }),
            Self::from_template(GoalTemplate::ArithmeticPuzzle {
                operation: operation.clone(),
                difficulty: DifficultyLevel::Advanced,
            }),
            Self::from_template(GoalTemplate::ArithmeticPuzzle {
                operation,
                difficulty: DifficultyLevel::Expert,
            }),
        ]
    }
}

/// Convenience functions for quick goal creation
pub mod quick {
    use super::*;

    /// Create a simple arithmetic puzzle
    pub fn arithmetic_puzzle(a: u64, b: u64, expected: u64) -> Goal {
        PuzzleBuilder::new("Quick Arithmetic")
            .input(Value::Nat(a))
            .input(Value::Nat(b))
            .expected_output(Value::Nat(expected))
            .build()
    }

    /// Create a simple computation goal
    pub fn compute<S: Into<String>>(name: S, target: S) -> Goal {
        ComputationBuilder::new(name.into())
            .target(target.into())
            .build()
    }

    /// Create a quick exploration goal
    pub fn explore<S: Into<String>>(domain: S) -> Goal {
        ExplorationBuilder::new("Quick Exploration")
            .domain(domain.into())
            .discovery_criterion("interesting_result".to_string())
            .build()
    }

    /// Create a proof goal
    pub fn prove<S: Into<String>>(theorem: S) -> Goal {
        AxiomBuilder::new("Quick Proof")
            .statement(theorem.into())
            .constraint("logical_soundness".to_string())
            .build()
    }

    /// Create a Fibonacci goal
    pub fn fibonacci(n: u64) -> Goal {
        ComputationBuilder::new("Fibonacci Computation")
            .target(format!("{} fibonacci .", n))
            .max_operations(n * 10)
            .context("n".to_string(), Value::Nat(n))
            .build()
    }

    /// Create a factorial goal
    pub fn factorial(n: u64) -> Goal {
        ComputationBuilder::new("Factorial Computation")
            .target(format!("{} factorial .", n))
            .max_operations(n * 5)
            .context("n".to_string(), Value::Nat(n))
            .build()
    }

    /// Create a prime checking goal
    pub fn is_prime(n: u64) -> Goal {
        ComputationBuilder::new("Prime Check")
            .target(format!("{} prime? .", n))
            .max_operations(n / 2)
            .context("n".to_string(), Value::Nat(n))
            .build()
    }
}

/// Helper function to get current timestamp in seconds since epoch
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_builder() {
        let goal = PuzzleBuilder::new("Test Puzzle")
            .description("A test puzzle")
            .input(Value::Nat(5))
            .input(Value::Nat(3))
            .expected_output(Value::Nat(8))
            .max_attempts(10)
            .priority(2)
            .build();

        assert!(goal.id.starts_with("puzzle_"));
        if let GoalType::Puzzle {
            name, max_attempts, ..
        } = goal.goal_type
        {
            assert_eq!(name, "Test Puzzle");
            assert_eq!(max_attempts, 10);
        } else {
            panic!("Expected puzzle goal type");
        }
        assert_eq!(goal.priority, 2);
    }

    #[test]
    fn test_computation_builder() {
        let goal = ComputationBuilder::new("Test Computation")
            .target("2 3 +")
            .max_time(Duration::from_secs(30))
            .max_operations(100)
            .build();

        assert!(goal.id.starts_with("computation_"));
        if let GoalType::Computation { target, .. } = goal.goal_type {
            assert_eq!(target, "2 3 +");
        } else {
            panic!("Expected computation goal type");
        }
    }

    #[test]
    fn test_template_factory() {
        let goal = GoalTemplateFactory::from_template(GoalTemplate::ArithmeticPuzzle {
            operation: ArithmeticOperation::Addition,
            difficulty: DifficultyLevel::Beginner,
        });

        assert!(matches!(goal.goal_type, GoalType::Puzzle { .. }));
    }

    #[test]
    fn test_quick_functions() {
        let goal = quick::arithmetic_puzzle(5, 3, 8);
        assert!(matches!(goal.goal_type, GoalType::Puzzle { .. }));

        let goal = quick::compute("Test", "2 3 +");
        assert!(matches!(goal.goal_type, GoalType::Computation { .. }));

        let goal = quick::fibonacci(10);
        if let GoalType::Computation { target, .. } = goal.goal_type {
            assert!(target.contains("fibonacci"));
        }
    }

    #[test]
    fn test_progressive_series() {
        let goals = GoalTemplateFactory::create_progressive_series(ArithmeticOperation::Addition);
        assert_eq!(goals.len(), 4);

        // Each goal should have increasing priority (difficulty)
        assert_eq!(goals[0].priority, 1); // Beginner
        assert_eq!(goals[1].priority, 2); // Intermediate
        assert_eq!(goals[2].priority, 3); // Advanced
        assert_eq!(goals[3].priority, 4); // Expert
    }

    #[test]
    fn test_exploration_batch() {
        let goals = GoalTemplateFactory::create_exploration_batch("Number Theory", 3);
        assert_eq!(goals.len(), 3);

        for (i, goal) in goals.iter().enumerate() {
            assert!(goal.id.starts_with("exploration_"));
            assert!(goal.context.contains_key("batch_index"));
            if let Some(Value::Nat(index)) = goal.context.get("batch_index") {
                assert_eq!(*index, i as u64);
            }
        }
    }
}
