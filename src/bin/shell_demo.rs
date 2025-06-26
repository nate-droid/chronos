//! Shell Framework Demonstration
//!
//! This program demonstrates the autonomous shell framework capabilities,
//! showing how shells can work toward goals independently and collaboratively.

use chronos::goal_builders::{
    ArithmeticOperation, DifficultyLevel, GoalTemplate, GoalTemplateFactory, quick,
};
use chronos::shell::Shell;
use chronos::shell_manager::{CoordinationStrategy, ShellManager};
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 C∀O Shell Framework Demonstration");
    println!("===================================\n");

    // Demo 1: Single Shell Autonomous Operation
    demo_single_shell_autonomous()?;

    println!("\n{}\n", "=".repeat(50));

    // Demo 2: Multiple Shells with Coordination
    demo_multiple_shells_coordination()?;

    println!("\n{}\n", "=".repeat(50));

    // Demo 3: Goal Templates and Builders
    demo_goal_templates()?;

    println!("\n{}\n", "=".repeat(50));

    // Demo 4: Interactive Shell Session
    demo_interactive_session()?;

    println!("\n✨ Demo completed! Shell framework is ready for hypervisor integration.");
    Ok(())
}

/// Demonstrate a single shell working autonomously toward goals
fn demo_single_shell_autonomous() -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 Demo 1: Single Shell Autonomous Operation");
    println!("--------------------------------------------");

    let mut shell = Shell::new("autonomous_worker".to_string());

    // Create some goals for the shell to work on
    let goals = vec![
        quick::arithmetic_puzzle(5, 3, 8),
        quick::compute("Simple Addition", "2 3 +"),
        quick::fibonacci(10),
        quick::factorial(5),
    ];

    println!("📝 Adding {} goals to shell...", goals.len());
    for goal in goals {
        println!(
            "   • Goal: {} (Priority: {})",
            match &goal.goal_type {
                chronos::shell::GoalType::Puzzle { name, .. } => name,
                chronos::shell::GoalType::Computation { name, .. } => name,
                chronos::shell::GoalType::Axiom { name, .. } => name,
                chronos::shell::GoalType::Exploration { name, .. } => name,
            },
            goal.priority
        );
        shell.add_goal(goal)?;
    }

    println!("\n🔄 Starting autonomous execution...");
    shell.start_autonomous()?;

    // Let the shell work for a few iterations
    let start_time = Instant::now();
    let mut iterations = 0;

    while start_time.elapsed() < Duration::from_secs(5) && iterations < 20 {
        match shell.autonomous_step() {
            Ok(true) => {
                iterations += 1;
                println!("   Step {}: Shell working on goals...", iterations);

                // Show current goal status
                let status = shell.goal_status();
                println!("     Active goals: {}", status.len());

                // Simulate some processing time
                thread::sleep(Duration::from_millis(200));
            }
            Ok(false) => {
                println!("   Shell completed all goals or paused.");
                break;
            }
            Err(e) => {
                println!("   Error during execution: {}", e);
                break;
            }
        }
    }

    shell.stop_autonomous();
    println!("⏹️  Autonomous execution stopped.");
    println!("📊 Shell info: {}", shell.info());

    Ok(())
}

/// Demonstrate multiple shells with coordination
fn demo_multiple_shells_coordination() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤝 Demo 2: Multiple Shells with Coordination");
    println!("-------------------------------------------");

    let mut manager = ShellManager::new();

    // Create multiple shells
    let shell_ids = vec!["worker_1", "worker_2", "explorer_1"];
    for id in &shell_ids {
        manager.create_shell(id.to_string())?;
        println!("🔧 Created shell: {}", id);
    }

    // Set collaborative strategy
    manager.set_strategy(CoordinationStrategy::Collaborative {
        knowledge_sharing: true,
        load_balancing: true,
    });
    println!("🔗 Set collaborative coordination strategy");

    // Assign different types of goals to different shells
    let computational_goals = vec![
        quick::compute("Addition Task", "5 7 +"),
        quick::compute("Multiplication Task", "6 9 *"),
        quick::fibonacci(8),
    ];

    let exploration_goals = vec![
        quick::explore("Prime Numbers"),
        quick::explore("Fibonacci Patterns"),
    ];

    println!("\n📋 Assigning goals to shells...");

    // Assign computational goals
    for (i, goal) in computational_goals.into_iter().enumerate() {
        let shell_id = &shell_ids[i % 2]; // Distribute between worker_1 and worker_2
        manager.assign_goal(shell_id, goal)?;
        println!("   • Assigned computational goal to {}", shell_id);
    }

    // Assign exploration goals
    for goal in exploration_goals {
        manager.assign_goal("explorer_1", goal)?;
        println!("   • Assigned exploration goal to explorer_1");
    }

    println!("\n🚀 Starting coordinated autonomous execution...");
    manager.start_all_autonomous()?;

    // Run coordination steps
    for i in 1..=10 {
        println!("   Coordination step {}", i);
        manager.coordination_step()?;

        // Show manager statistics
        if i % 3 == 0 {
            println!("   📊 {}", manager.get_statistics());
        }

        thread::sleep(Duration::from_millis(300));
    }

    manager.stop_all_autonomous()?;
    println!("⏹️  All shells stopped.");
    println!("📈 Final statistics:\n{}", manager.get_statistics());

    Ok(())
}

/// Demonstrate goal templates and builders
fn demo_goal_templates() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Demo 3: Goal Templates and Builders");
    println!("-------------------------------------");

    let mut shell = Shell::new("template_demo".to_string());

    // Create goals from templates
    println!("🏗️  Creating goals from templates...");

    let arithmetic_goal = GoalTemplateFactory::from_template(GoalTemplate::ArithmeticPuzzle {
        operation: ArithmeticOperation::Fibonacci,
        difficulty: DifficultyLevel::Intermediate,
    });
    println!("   • Created Fibonacci puzzle (Intermediate)");

    let number_theory_goal = GoalTemplateFactory::from_template(GoalTemplate::NumberTheory {
        topic: chronos::goal_builders::NumberTheoryTopic::PrimalityTesting,
        range: (100, 200),
    });
    println!("   • Created prime number exploration");

    let logic_goal = GoalTemplateFactory::from_template(GoalTemplate::LogicPuzzle {
        puzzle_type: chronos::goal_builders::LogicPuzzleType::NQueens,
        size: 8,
    });
    println!("   • Created N-Queens puzzle (8x8)");

    // Create a progressive series
    let progressive_goals =
        GoalTemplateFactory::create_progressive_series(ArithmeticOperation::Multiplication);
    println!(
        "   • Created progressive multiplication series ({} goals)",
        progressive_goals.len()
    );

    // Add all goals to shell
    shell.add_goal(arithmetic_goal)?;
    shell.add_goal(number_theory_goal)?;
    shell.add_goal(logic_goal)?;

    for goal in progressive_goals {
        shell.add_goal(goal)?;
    }

    println!(
        "\n📊 Shell loaded with {} goals from templates",
        shell.goal_status().len()
    );
    println!(
        "🎯 Goal types cover: arithmetic, number theory, logic puzzles, and progressive series"
    );

    // Show goal information
    let status = shell.goal_status();
    for (goal_id, status) in status.iter().take(3) {
        println!("   • Goal {}: {:?}", goal_id, status);
    }

    Ok(())
}

/// Demonstrate interactive shell session
fn demo_interactive_session() -> Result<(), Box<dyn std::error::Error>> {
    println!("💬 Demo 4: Interactive Shell Session");
    println!("-----------------------------------");

    let mut shell = Shell::new("interactive_demo".to_string());

    println!("🔧 Shell created in interactive mode");
    println!("📝 Adding a simple computation goal...");

    let goal = quick::compute("Interactive Demo", "10 20 + 5 *");
    shell.add_goal(goal)?;

    println!("✅ Goal added. Shell can now:");
    println!("   • Work autonomously: shell.start_autonomous()");
    println!("   • Execute code directly: shell.eval(\"2 3 +\")");
    println!("   • Access REPL: shell.repl()");
    println!("   • Monitor progress: shell.goal_status()");
    println!("   • Save/load state: shell.save_shell_state()");

    // Demonstrate direct evaluation
    println!("\n🧮 Demonstrating direct evaluation:");
    if let Err(e) = shell.eval("2 3 +") {
        println!("   Note: REPL integration pending - got error: {}", e);
    } else {
        println!("   ✅ Successfully evaluated: 2 3 +");
    }

    // Show shell information
    println!("\n📋 Shell Information:");
    println!("   {}", shell.info());

    // Demonstrate resource limits checking
    let resource_violations = shell.check_resource_limits();
    if resource_violations.is_empty() {
        println!("   ✅ No resource limit violations");
    } else {
        println!("   ⚠️  Resource violations: {:?}", resource_violations);
    }

    Ok(())
}

/// Helper function to create a sample shell with various goals
#[allow(dead_code)]
fn create_sample_shell() -> Result<Shell, Box<dyn std::error::Error>> {
    let mut shell = Shell::new("sample_shell".to_string());

    // Add a variety of goals
    shell.add_goal(quick::arithmetic_puzzle(10, 15, 25))?;
    shell.add_goal(quick::fibonacci(12))?;
    shell.add_goal(quick::is_prime(97))?;
    shell.add_goal(quick::prove("∀n: n + 0 = n"))?;
    shell.add_goal(quick::explore("Collatz Conjecture"))?;

    Ok(shell)
}

/// Helper function to demonstrate error handling
#[allow(dead_code)]
fn demo_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚨 Demo: Error Handling");
    println!("----------------------");

    let mut shell = Shell::new("error_demo".to_string());

    // Try to start autonomous without goals
    match shell.start_autonomous() {
        Ok(_) => println!("   Unexpected: Should have failed without goals"),
        Err(e) => println!("   ✅ Correctly failed: {}", e),
    }

    // Add a goal and try again
    shell.add_goal(quick::compute("Test", "1 1 +"))?;
    match shell.start_autonomous() {
        Ok(_) => println!("   ✅ Successfully started with goals"),
        Err(e) => println!("   ❌ Unexpected failure: {}", e),
    }

    Ok(())
}
