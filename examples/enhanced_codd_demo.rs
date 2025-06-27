//! Enhanced Codd's Cellular Automata Demo
//!
//! This demo showcases enhanced patterns and visual improvements for Codd's Cellular Automata
//! while working with the existing implementation. It demonstrates complex patterns,
//! educational content, and game-like features.

use chronos_repl::codd_ca::{
    codd_patterns, run_simple_codd_ca, CoddCA, CoddPatternType, CoddState,
};
use std::error::Error;

/// Enhanced pattern definitions with educational content
#[derive(Debug, Clone)]
pub struct EnhancedPattern {
    pub name: String,
    pub description: String,
    pub grid: Vec<Vec<CoddState>>,
    pub difficulty: u8,
    pub category: String,
    pub educational_note: String,
}

/// Create enhanced pattern library
fn create_enhanced_patterns() -> Vec<EnhancedPattern> {
    vec![
        EnhancedPattern {
            name: "Signal Highway".to_string(),
            description: "Long-distance signal transmission corridor".to_string(),
            grid: vec![
                vec![CoddState::Empty, CoddState::Empty, CoddState::Empty, CoddState::Empty, CoddState::Empty, CoddState::Empty, CoddState::Empty],
                vec![CoddState::OrdinaryTransmission, CoddState::Conductor, CoddState::Conductor, CoddState::Conductor, CoddState::Conductor, CoddState::Conductor, CoddState::Empty],
                vec![CoddState::Empty, CoddState::Empty, CoddState::Empty, CoddState::Empty, CoddState::Empty, CoddState::Empty, CoddState::Empty],
            ],
            difficulty: 1,
            category: "Basic Transmission".to_string(),
            educational_note: "Signals propagate through conductors in a straight line. Watch how the signal moves from left to right.".to_string(),
        },
        EnhancedPattern {
            name: "Cross Junction Plus".to_string(),
            description: "Advanced four-way signal intersection with confluence control".to_string(),
            grid: vec![
                vec![CoddState::Empty, CoddState::OrdinaryTransmission, CoddState::Empty],
                vec![CoddState::SpecialTransmission, CoddState::Confluence, CoddState::Conductor],
                vec![CoddState::Empty, CoddState::Conductor, CoddState::Empty],
            ],
            difficulty: 3,
            category: "Signal Processing".to_string(),
            educational_note: "Confluence cells can handle multiple signal types simultaneously, creating complex interactions.".to_string(),
        },
        EnhancedPattern {
            name: "Signal Amplifier".to_string(),
            description: "Pattern that strengthens and redirects signals".to_string(),
            grid: vec![
                vec![CoddState::Empty, CoddState::Empty, CoddState::Conductor, CoddState::Empty, CoddState::Empty],
                vec![CoddState::Empty, CoddState::Conductor, CoddState::Confluence, CoddState::Conductor, CoddState::Empty],
                vec![CoddState::OrdinaryTransmission, CoddState::Conductor, CoddState::Confluence, CoddState::Conductor, CoddState::SpecialTransmission],
                vec![CoddState::Empty, CoddState::Conductor, CoddState::Confluence, CoddState::Conductor, CoddState::Empty],
                vec![CoddState::Empty, CoddState::Empty, CoddState::Conductor, CoddState::Empty, CoddState::Empty],
            ],
            difficulty: 4,
            category: "Signal Enhancement".to_string(),
            educational_note: "This pattern demonstrates signal amplification and type conversion using confluence networks.".to_string(),
        },
        EnhancedPattern {
            name: "Logic Gate Array".to_string(),
            description: "Multiple logic gates working in parallel".to_string(),
            grid: vec![
                vec![CoddState::OrdinaryTransmission, CoddState::Conductor, CoddState::Empty, CoddState::SpecialTransmission, CoddState::Conductor],
                vec![CoddState::Empty, CoddState::Conductor, CoddState::Confluence, CoddState::Conductor, CoddState::Empty],
                vec![CoddState::Empty, CoddState::Empty, CoddState::Conductor, CoddState::Empty, CoddState::Empty],
                vec![CoddState::Empty, CoddState::Conductor, CoddState::Confluence, CoddState::Conductor, CoddState::Empty],
                vec![CoddState::OrdinaryReversed, CoddState::Conductor, CoddState::Empty, CoddState::SpecialReversed, CoddState::Conductor],
            ],
            difficulty: 5,
            category: "Computational Logic".to_string(),
            educational_note: "Multiple logic operations can be performed in parallel using carefully designed conductor networks.".to_string(),
        },
        EnhancedPattern {
            name: "Signal Memory Cell".to_string(),
            description: "Pattern that can store and recall signal states".to_string(),
            grid: vec![
                vec![CoddState::SheathedConductor, CoddState::Conductor, CoddState::SheathedConductor],
                vec![CoddState::Conductor, CoddState::Confluence, CoddState::Conductor],
                vec![CoddState::SheathedConductor, CoddState::OrdinaryTransmission, CoddState::SheathedConductor],
            ],
            difficulty: 4,
            category: "Memory Systems".to_string(),
            educational_note: "Sheathed conductors provide isolation, allowing signals to be stored and retrieved.".to_string(),
        },
    ]
}

/// Enhanced visual representation using Unicode symbols
fn display_enhanced_grid(grid: &Vec<Vec<CoddState>>, title: &str) {
    println!("🎨 {} (Enhanced View):", title);
    println!("┌{}┐", "─".repeat(grid[0].len() * 2));

    for row in grid {
        print!("│");
        for &cell in row {
            let symbol = match cell {
                CoddState::Empty => "· ",
                CoddState::Conductor => "╬ ",
                CoddState::OrdinaryTransmission => "▶ ",
                CoddState::SpecialTransmission => "⬢ ",
                CoddState::Confluence => "⬟ ",
                CoddState::OrdinaryReversed => "◀ ",
                CoddState::SpecialReversed => "⬡ ",
                CoddState::SheathedConductor => "⬛ ",
            };
            print!("{}", symbol);
        }
        println!("│");
    }
    println!("└{}┘", "─".repeat(grid[0].len() * 2));
}

/// Create a Codd CA from a pattern
fn create_ca_from_pattern(pattern: &EnhancedPattern, width: usize, height: usize) -> CoddCA {
    let mut ca = CoddCA::new(width, height);

    // Center the pattern in the grid
    let start_x = (width.saturating_sub(pattern.grid[0].len())) / 2;
    let start_y = (height.saturating_sub(pattern.grid.len())) / 2;

    for (py, row) in pattern.grid.iter().enumerate() {
        for (px, &cell) in row.iter().enumerate() {
            let x = start_x + px;
            let y = start_y + py;
            if x < width && y < height {
                ca.grid[y][x] = cell;
            }
        }
    }

    ca
}

/// Enhanced demo runner with educational content
fn run_enhanced_demo(pattern: &EnhancedPattern, generations: usize) -> Result<(), Box<dyn Error>> {
    println!("\n🧪 Pattern: {}", pattern.name);
    println!(
        "🏷️  Category: {} (Difficulty: {}★)",
        pattern.category,
        "★".repeat(pattern.difficulty as usize)
    );
    println!("📝 Description: {}", pattern.description);
    println!("🎓 Educational Note: {}", pattern.educational_note);
    println!();

    // Show initial pattern
    display_enhanced_grid(&pattern.grid, "Initial State");

    // Create CA and run simulation
    let width = pattern.grid[0].len().max(15);
    let height = pattern.grid.len().max(8);
    let mut ca = create_ca_from_pattern(pattern, width, height);

    println!("\n🔄 Evolution (showing every 3rd generation):");
    println!("{}", "=".repeat(50));

    for gen in 0..=generations {
        if gen % 3 == 0 || gen == generations {
            let active_count = ca
                .grid
                .iter()
                .flat_map(|row| row.iter())
                .filter(|&&cell| cell != CoddState::Empty)
                .count();
            let signal_count = ca
                .grid
                .iter()
                .flat_map(|row| row.iter())
                .filter(|&&cell| {
                    matches!(
                        cell,
                        CoddState::OrdinaryTransmission
                            | CoddState::SpecialTransmission
                            | CoddState::OrdinaryReversed
                            | CoddState::SpecialReversed
                    )
                })
                .count();

            println!(
                "\n⏱️  Generation {}: Active: {}, Signals: {}",
                gen, active_count, signal_count
            );

            // Display grid with enhanced symbols
            for row in &ca.grid {
                for &cell in row {
                    let symbol = match cell {
                        CoddState::Empty => "·",
                        CoddState::Conductor => "╬",
                        CoddState::OrdinaryTransmission => "▶",
                        CoddState::SpecialTransmission => "⬢",
                        CoddState::Confluence => "⬟",
                        CoddState::OrdinaryReversed => "◀",
                        CoddState::SpecialReversed => "⬡",
                        CoddState::SheathedConductor => "⬛",
                    };
                    print!("{}", symbol);
                }
                println!();
            }
        }

        if gen < generations {
            ca.step();
        }
    }

    println!("\n✅ Pattern evolution completed!");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("🎮 Enhanced Codd's Cellular Automata Showcase");
    println!("==============================================\n");

    // Show visual legend
    println!("🎨 Enhanced Visual Legend:");
    println!("╬  Conductor (transmits signals)");
    println!("▶  Ordinary Signal (moving)");
    println!("⬢  Special Signal (enhanced properties)");
    println!("⬟  Confluence (signal junction)");
    println!("◀  Reversed Ordinary Signal");
    println!("⬡  Special Reversed Signal");
    println!("⬛  Sheathed Conductor (protected)");
    println!("·  Empty Space\n");

    // Show original patterns first
    println!("📚 Original Codd CA Patterns:");
    let original_patterns = codd_patterns();
    for (pattern_type, description) in original_patterns {
        println!("• {:?}: {}", pattern_type, description);
    }
    println!();

    // Create enhanced patterns
    let enhanced_patterns = create_enhanced_patterns();

    println!(
        "🌟 Enhanced Pattern Library ({} patterns):",
        enhanced_patterns.len()
    );
    for (i, pattern) in enhanced_patterns.iter().enumerate() {
        let stars = "★".repeat(pattern.difficulty as usize);
        println!(
            "{}. {} {} - {}",
            i + 1,
            pattern.name,
            stars,
            pattern.category
        );
    }
    println!();

    // Demo 1: Basic signal with standard CA
    println!("🔸 Demo 1: Standard Signal Transmission");
    println!("---------------------------------------");
    run_simple_codd_ca(CoddPatternType::Signal, 6, 20, 5)?;

    println!("\n{}", "=".repeat(60));

    // Demo 2: Enhanced signal highway
    println!("\n🔸 Demo 2: Enhanced Signal Highway");
    println!("----------------------------------");
    run_enhanced_demo(&enhanced_patterns[0], 8)?;

    println!("\n{}", "=".repeat(60));

    // Demo 3: Complex junction
    println!("\n🔸 Demo 3: Advanced Signal Junction");
    println!("-----------------------------------");
    run_enhanced_demo(&enhanced_patterns[1], 10)?;

    println!("\n{}", "=".repeat(60));

    // Demo 4: Signal amplifier
    println!("\n🔸 Demo 4: Signal Amplification Network");
    println!("---------------------------------------");
    run_enhanced_demo(&enhanced_patterns[2], 12)?;

    println!("\n{}", "=".repeat(60));

    // Educational content
    println!("\n🎓 Educational Insights:");
    println!("------------------------");
    println!("• Codd's CA demonstrates universal computation capability");
    println!("• Signals can be processed, stored, and transformed");
    println!("• Complex behaviors emerge from simple local rules");
    println!("• Confluence cells act as programmable logic elements");
    println!("• Sheathed conductors provide signal isolation");
    println!("• Pattern complexity correlates with computational power");

    println!("\n🚀 REPL Commands to Try:");
    println!("------------------------");
    println!("• .codd-patterns                    - List available patterns");
    println!("• .codd signal 25 15               - Interactive signal demo");
    println!("• .codd replicator 30 20           - Self-replication demo");
    println!("• .codd-simple signal 20 10 15     - Text-based evolution");

    println!("\n🔬 Research Directions:");
    println!("----------------------");
    println!("• Design custom logic circuits");
    println!("• Explore signal timing relationships");
    println!("• Create memory and storage patterns");
    println!("• Build computational networks");
    println!("• Study emergent pattern formation");

    println!("\n🏆 Challenges to Attempt:");
    println!("-------------------------");
    println!("1. Create a signal that travels in a complete loop");
    println!("2. Design an AND gate using confluence cells");
    println!("3. Build a signal splitter that creates 3 outputs");
    println!("4. Construct a memory cell that stores signals");
    println!("5. Create a pattern that exhibits self-modification");

    println!("\n🎉 Enhanced demo completed!");
    println!("Launch the Chronos REPL to explore Codd's CA interactively!");

    Ok(())
}
