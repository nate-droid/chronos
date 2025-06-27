//! Codd's Cellular Automata Demo for Chronos REPL
//!
//! This example demonstrates the Codd's cellular automata functionality
//! available in the Chronos REPL environment.

use chronos_repl::codd_ca::{codd_patterns, run_simple_codd_ca, CoddPatternType};
use chronos_repl::{CoddCA, CoddEnvironment, CoddState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Chronos Codd's Cellular Automata Demo");
    println!("====================================\n");

    // Show available patterns
    println!("Available Codd CA Patterns:");
    let patterns = codd_patterns();
    for (pattern_type, description) in patterns {
        println!("  {:?}: {}", pattern_type, description);
    }
    println!();

    // Demo 1: Signal transmission pattern
    println!("Demo 1: Signal Transmission Pattern");
    println!("-----------------------------------");
    let signal_result = run_simple_codd_ca(CoddPatternType::Signal, 5, 15, 8)?;
    println!("{}", signal_result);

    // Demo 2: Show cell states and their representations
    println!("Demo 2: Cell State Representations");
    println!("----------------------------------");
    let states = [
        CoddState::Empty,
        CoddState::Conductor,
        CoddState::OrdinaryTransmission,
        CoddState::SpecialTransmission,
        CoddState::Confluence,
        CoddState::OrdinaryReversed,
        CoddState::SpecialReversed,
        CoddState::SheathedConductor,
    ];

    for state in &states {
        println!(
            "State {:?} ({}): '{}'",
            state,
            state.to_num(),
            state.to_char()
        );
    }
    println!();

    // Demo 3: Create and evolve a custom CA
    println!("Demo 3: Custom Codd CA Evolution");
    println!("--------------------------------");
    let mut ca = CoddCA::new_with_signal(12, 6);

    println!("Initial state:");
    println!("{}\n", ca.to_string());

    for gen in 1..=3 {
        ca.step();
        println!("Generation {}:", gen);
        println!("{}", ca.to_string());
        println!(
            "Active cells: {}, Density: {:.1}%\n",
            ca.active_count(),
            ca.density() * 100.0
        );
    }

    // Demo 4: Replicator pattern
    println!("Demo 4: Simplified Replicator Pattern");
    println!("------------------------------------");
    let replicator_result = run_simple_codd_ca(CoddPatternType::Replicator, 3, 20, 15)?;
    println!("{}", replicator_result);

    // Demo 5: Show how to use interactive mode (without actually running it)
    println!("Demo 5: Interactive Mode Usage");
    println!("------------------------------");
    println!("To run interactive Codd CA sessions in the REPL:");
    println!("  .codd signal 30 20        - Signal transmission (30x20 grid)");
    println!("  .codd replicator 40 30    - Replicator pattern (40x30 grid)");
    println!("  .codd empty 25 25         - Empty grid to experiment");
    println!();
    println!("Interactive controls:");
    println!("  Space: Play/Pause evolution");
    println!("  s: Single step when paused");
    println!("  r: Reset to initial state");
    println!("  h: Toggle help display");
    println!("  1/5: Evolve 1 or 5 generations");
    println!("  F1/F2: Evolve 10 or 50 generations");
    println!("  c: Toggle color display");
    println!("  q: Quit interactive mode");
    println!();

    println!("For text-mode output:");
    println!("  .codd-simple signal 20 15 10    - Run signal pattern for 10 generations");
    println!("  .codd-patterns                  - List all available patterns");

    println!("\nDemo completed! Try the interactive modes in the Chronos REPL.");
    Ok(())
}
