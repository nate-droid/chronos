//! Enhanced Game Features for Codd's Cellular Automata
//!
//! This module provides game-like enhancements to the Codd's CA system including:
//! - Enhanced visual patterns with educational content
//! - Pattern library with complex structures
//! - Simple demo functionality that works with existing implementation
//! - Educational tutorials and descriptions

use crate::codd_ca::{CoddCA, CoddState};
use crate::error::Result;

/// Enhanced pattern with educational content
#[derive(Debug, Clone)]
pub struct Pattern {
    pub name: String,
    pub description: String,
    pub grid: Vec<Vec<CoddState>>,
    pub difficulty: u8,
    pub category: String,
    pub educational_note: String,
}

/// Challenge definition
#[derive(Debug, Clone)]
pub struct Challenge {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub goal: String,
    pub difficulty: u8,
    pub hint: String,
}

/// Game statistics
#[derive(Debug, Clone, Default)]
pub struct GameStats {
    pub total_generations: u64,
    pub patterns_created: u32,
    pub challenges_completed: u32,
    pub max_signal_count: usize,
}

/// Drawing tool for interactive editing
#[derive(Debug, Clone, PartialEq)]
pub enum DrawTool {
    Paint(CoddState),
    Erase,
}

/// Game mode
#[derive(Debug, Clone, PartialEq)]
pub enum GameMode {
    FreePlay,
    Challenge(usize),
    Tutorial,
}

/// Enhanced game environment (simplified)
pub struct CoddGameEnvironment {
    pub patterns: Vec<Pattern>,
    pub challenges: Vec<Challenge>,
    pub stats: GameStats,
}

impl CoddGameEnvironment {
    /// Create new game environment
    pub fn new(_width: usize, _height: usize) -> Self {
        let patterns = Self::create_pattern_library();
        let challenges = Self::create_challenges();

        Self {
            patterns,
            challenges,
            stats: GameStats::default(),
        }
    }

    /// Create pattern library
    fn create_pattern_library() -> Vec<Pattern> {
        vec![
            Pattern {
                name: "Signal Highway".to_string(),
                description: "Long-distance signal transmission".to_string(),
                grid: vec![
                    vec![CoddState::Empty, CoddState::Empty, CoddState::Empty, CoddState::Empty, CoddState::Empty],
                    vec![CoddState::OrdinaryTransmission, CoddState::Conductor, CoddState::Conductor, CoddState::Conductor, CoddState::Empty],
                    vec![CoddState::Empty, CoddState::Empty, CoddState::Empty, CoddState::Empty, CoddState::Empty],
                ],
                difficulty: 1,
                category: "Basic Patterns".to_string(),
                educational_note: "Signals propagate through conductors. Watch how the signal moves from left to right.".to_string(),
            },
            Pattern {
                name: "Cross Junction".to_string(),
                description: "Four-way signal intersection".to_string(),
                grid: vec![
                    vec![CoddState::Empty, CoddState::Conductor, CoddState::Empty],
                    vec![CoddState::Conductor, CoddState::Confluence, CoddState::Conductor],
                    vec![CoddState::Empty, CoddState::Conductor, CoddState::Empty],
                ],
                difficulty: 2,
                category: "Junctions".to_string(),
                educational_note: "Confluence cells manage signal flow from multiple directions.".to_string(),
            },
            Pattern {
                name: "Signal Splitter".to_string(),
                description: "Split one signal into multiple paths".to_string(),
                grid: vec![
                    vec![CoddState::Empty, CoddState::Empty, CoddState::Conductor, CoddState::Empty, CoddState::Empty],
                    vec![CoddState::Empty, CoddState::Conductor, CoddState::Confluence, CoddState::Conductor, CoddState::Empty],
                    vec![CoddState::OrdinaryTransmission, CoddState::Conductor, CoddState::Confluence, CoddState::Conductor, CoddState::Empty],
                    vec![CoddState::Empty, CoddState::Conductor, CoddState::Confluence, CoddState::Conductor, CoddState::Empty],
                    vec![CoddState::Empty, CoddState::Empty, CoddState::Conductor, CoddState::Empty, CoddState::Empty],
                ],
                difficulty: 3,
                category: "Logic Elements".to_string(),
                educational_note: "Signals can be split and directed to multiple outputs using confluence points.".to_string(),
            },
            Pattern {
                name: "Memory Cell".to_string(),
                description: "Basic signal storage pattern".to_string(),
                grid: vec![
                    vec![CoddState::SheathedConductor, CoddState::Conductor, CoddState::SheathedConductor],
                    vec![CoddState::Conductor, CoddState::Confluence, CoddState::Conductor],
                    vec![CoddState::SheathedConductor, CoddState::OrdinaryTransmission, CoddState::SheathedConductor],
                ],
                difficulty: 4,
                category: "Memory Systems".to_string(),
                educational_note: "Sheathed conductors provide isolation for signal storage.".to_string(),
            },
        ]
    }

    /// Create challenges
    fn create_challenges() -> Vec<Challenge> {
        vec![
            Challenge {
                id: 1,
                name: "First Signal".to_string(),
                description: "Create a signal that travels at least 5 cells".to_string(),
                goal: "Signal propagation".to_string(),
                difficulty: 1,
                hint: "Place conductors in a line and add a signal at one end".to_string(),
            },
            Challenge {
                id: 2,
                name: "Signal Split".to_string(),
                description: "Create a signal that splits into two paths".to_string(),
                goal: "Signal bifurcation".to_string(),
                difficulty: 2,
                hint: "Use confluence cells to split signals".to_string(),
            },
            Challenge {
                id: 3,
                name: "Signal Loop".to_string(),
                description: "Create a circulating signal".to_string(),
                goal: "Persistent oscillation".to_string(),
                difficulty: 3,
                hint: "Create a circular path of conductors".to_string(),
            },
        ]
    }

    /// Show pattern library
    pub fn show_pattern_library(&self) {
        println!("Enhanced Codd's CA Pattern Library:");
        println!("====================================");
        for (i, pattern) in self.patterns.iter().enumerate() {
            let stars = "★".repeat(pattern.difficulty as usize);
            println!("{}. {} {}", i + 1, pattern.name, stars);
            println!(
                "   Category: {} | {}",
                pattern.category, pattern.description
            );
            println!("   {}", pattern.educational_note);
            println!();
        }
    }

    /// Show challenges
    pub fn show_challenges(&self) {
        println!("Enhanced Codd's CA Challenges:");
        println!("==============================");
        for challenge in &self.challenges {
            let stars = "★".repeat(challenge.difficulty as usize);
            println!("{}. {} {}", challenge.id, challenge.name, stars);
            println!("   Goal: {}", challenge.goal);
            println!("   Description: {}", challenge.description);
            println!("   Hint: {}", challenge.hint);
            println!();
        }
    }

    /// Create CA from pattern
    fn create_ca_from_pattern(
        &self,
        pattern_index: usize,
        width: usize,
        height: usize,
    ) -> Option<CoddCA> {
        if pattern_index >= self.patterns.len() {
            return None;
        }

        let pattern = &self.patterns[pattern_index];
        let mut ca = CoddCA::new(width, height);

        // Center the pattern
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

        Some(ca)
    }

    /// Run enhanced demo
    pub fn run_simple_demo(&mut self, pattern: &str, generations: u32) {
        let pattern_index = match pattern {
            "highway" => 0,
            "junction" => 1,
            "splitter" => 2,
            "memory" => 3,
            _ => 0,
        };

        if let Some(mut ca) = self.create_ca_from_pattern(pattern_index, 15, 8) {
            let pattern_info = &self.patterns[pattern_index];

            println!("Enhanced Codd's CA Demo: {}", pattern_info.name);
            println!(
                "Category: {} ({}★)",
                pattern_info.category,
                "★".repeat(pattern_info.difficulty as usize)
            );
            println!("Description: {}", pattern_info.description);
            println!("Educational: {}", pattern_info.educational_note);
            println!(
                "========================{}",
                "=".repeat(pattern_info.name.len())
            );

            for gen in 0..=generations {
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
                    "\nGeneration {}: (Active: {}, Signals: {})",
                    gen, active_count, signal_count
                );

                // Print grid with enhanced characters
                for row in &ca.grid {
                    for &cell in row {
                        let symbol = match cell {
                            CoddState::Empty => '·',
                            CoddState::Conductor => '╬',
                            CoddState::OrdinaryTransmission => '▶',
                            CoddState::SpecialTransmission => '⬢',
                            CoddState::Confluence => '⬟',
                            CoddState::OrdinaryReversed => '◀',
                            CoddState::SpecialReversed => '⬡',
                            CoddState::SheathedConductor => '⬛',
                        };
                        print!("{}", symbol);
                    }
                    println!();
                }

                if gen < generations {
                    ca.step();
                    self.stats.total_generations += 1;
                }
            }

            println!("\nDemo completed! Enhanced graphics showcase the pattern evolution.");
        }
    }
}

/// Public interface functions for REPL integration
pub fn run_enhanced_codd_demo(
    pattern: &str,
    width: usize,
    height: usize,
    generations: u32,
) -> Result<()> {
    let mut env = CoddGameEnvironment::new(width, height);
    env.run_simple_demo(pattern, generations);
    Ok(())
}

pub fn show_enhanced_patterns() -> Result<()> {
    let env = CoddGameEnvironment::new(20, 15);
    env.show_pattern_library();
    Ok(())
}

pub fn show_enhanced_challenges() -> Result<()> {
    let env = CoddGameEnvironment::new(20, 15);
    env.show_challenges();
    Ok(())
}
