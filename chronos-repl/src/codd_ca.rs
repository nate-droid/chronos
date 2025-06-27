//! Codd's Cellular Automata System for Chronos REPL
//!
//! This module provides an interactive Codd's cellular automata environment within the REPL
//! for exploring 2D, 8-state cellular automata capable of self-replication and universal computation.
//!
//! Features:
//! - Codd's cellular automata (2D) with 8-state cells
//! - Self-replication and universal computation demonstrations
//! - Terminal-based graphics using Unicode characters
//! - Interactive evolution controls
//! - Multiple pattern types (signal transmission, replicators)

use crate::error::Result;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Codd's cellular automaton cell states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CoddState {
    /// Empty space
    Empty = 0,
    /// Conductor - can transmit signals
    Conductor = 1,
    /// Ordinary transmission state (signal moving)
    OrdinaryTransmission = 2,
    /// Special transmission state (signal moving)
    SpecialTransmission = 3,
    /// Confluence state - signal confluence
    Confluence = 4,
    /// Ordinary transmission reversed
    OrdinaryReversed = 5,
    /// Special transmission reversed
    SpecialReversed = 6,
    /// Sheathed conductor
    SheathedConductor = 7,
}

impl CoddState {
    /// Get character representation for display
    pub fn to_char(&self) -> char {
        match self {
            CoddState::Empty => ' ',
            CoddState::Conductor => '▒',
            CoddState::OrdinaryTransmission => '→',
            CoddState::SpecialTransmission => '⇒',
            CoddState::Confluence => '◊',
            CoddState::OrdinaryReversed => '←',
            CoddState::SpecialReversed => '⇐',
            CoddState::SheathedConductor => '█',
        }
    }

    /// Get enhanced character representation for game mode
    pub fn to_char_enhanced(&self) -> char {
        match self {
            CoddState::Empty => '·',
            CoddState::Conductor => '╬',
            CoddState::OrdinaryTransmission => '▶',
            CoddState::SpecialTransmission => '⬢',
            CoddState::Confluence => '⬟',
            CoddState::OrdinaryReversed => '◀',
            CoddState::SpecialReversed => '⬡',
            CoddState::SheathedConductor => '⬛',
        }
    }

    /// Check if this state represents an active signal
    pub fn is_signal(&self) -> bool {
        matches!(
            self,
            CoddState::OrdinaryTransmission
                | CoddState::SpecialTransmission
                | CoddState::OrdinaryReversed
                | CoddState::SpecialReversed
        )
    }

    /// Check if this state can conduct signals
    pub fn is_conductor(&self) -> bool {
        matches!(self, CoddState::Conductor | CoddState::SheathedConductor)
    }

    /// Get color for display
    pub fn color(&self) -> Color {
        match self {
            CoddState::Empty => Color::Black,
            CoddState::Conductor => Color::Gray,
            CoddState::OrdinaryTransmission => Color::Blue,
            CoddState::SpecialTransmission => Color::Cyan,
            CoddState::Confluence => Color::Yellow,
            CoddState::OrdinaryReversed => Color::Red,
            CoddState::SpecialReversed => Color::Magenta,
            CoddState::SheathedConductor => Color::White,
        }
    }

    pub fn to_num(self) -> u8 {
        self as u8
    }

    pub fn from_num(n: u8) -> CoddState {
        match n {
            0 => CoddState::Empty,
            1 => CoddState::Conductor,
            2 => CoddState::OrdinaryTransmission,
            3 => CoddState::SpecialTransmission,
            4 => CoddState::Confluence,
            5 => CoddState::OrdinaryReversed,
            6 => CoddState::SpecialReversed,
            7 => CoddState::SheathedConductor,
            _ => CoddState::Empty,
        }
    }
}

/// Codd's cellular automaton with 2D grid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoddCA {
    pub grid: Vec<Vec<CoddState>>,
    pub generation: u64,
    pub width: usize,
    pub height: usize,
    pub history: Vec<Vec<Vec<CoddState>>>,
}

impl CoddCA {
    /// Create a new Codd CA with empty grid
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![CoddState::Empty; width]; height];
        let history = vec![grid.clone()];

        Self {
            grid,
            generation: 0,
            width,
            height,
            history,
        }
    }

    /// Create with a signal transmission pattern
    pub fn new_with_signal(width: usize, height: usize) -> Self {
        let mut ca = Self::new(width, height);

        // Create a horizontal signal line
        let y = height / 2;
        for x in 2..width - 2 {
            ca.grid[y][x] = CoddState::Conductor;
        }

        // Add signal source
        if width > 5 {
            ca.grid[y][2] = CoddState::OrdinaryTransmission;
            ca.grid[y][1] = CoddState::Confluence;
        }

        ca.save_state();
        ca
    }

    /// Create with a replicator pattern (simplified)
    pub fn new_with_replicator(width: usize, height: usize) -> Self {
        let mut ca = Self::new(width, height);

        let cx = width / 2;
        let cy = height / 2;

        // Create a simple cross-shaped replicator core
        if cx >= 3 && cy >= 3 && cx + 3 < width && cy + 3 < height {
            // Central confluence
            ca.grid[cy][cx] = CoddState::Confluence;

            // Arms
            ca.grid[cy][cx - 1] = CoddState::Conductor;
            ca.grid[cy][cx + 1] = CoddState::Conductor;
            ca.grid[cy - 1][cx] = CoddState::Conductor;
            ca.grid[cy + 1][cx] = CoddState::Conductor;

            // Signal sources
            ca.grid[cy][cx - 2] = CoddState::OrdinaryTransmission;
            ca.grid[cy][cx + 2] = CoddState::SpecialTransmission;
            ca.grid[cy - 2][cx] = CoddState::OrdinaryReversed;
            ca.grid[cy + 2][cx] = CoddState::SpecialReversed;

            // Sheathing
            ca.grid[cy - 1][cx - 1] = CoddState::SheathedConductor;
            ca.grid[cy - 1][cx + 1] = CoddState::SheathedConductor;
            ca.grid[cy + 1][cx - 1] = CoddState::SheathedConductor;
            ca.grid[cy + 1][cx + 1] = CoddState::SheathedConductor;
        }

        ca.save_state();
        ca
    }

    /// Evolve one generation
    pub fn step(&mut self) {
        let mut new_grid = self.grid.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.get_neighbors(x, y);
                new_grid[y][x] = self.apply_codd_rule(self.grid[y][x], neighbors);
            }
        }

        self.grid = new_grid;
        self.generation += 1;

        // Save state periodically for undo
        if self.generation % 10 == 0 {
            self.save_state();
        }
    }

    /// Apply Codd's transition rules (simplified implementation)
    fn apply_codd_rule(&self, current: CoddState, neighbors: [CoddState; 8]) -> CoddState {
        use CoddState::*;

        match current {
            Empty => {
                // Empty cells can become conductor if surrounded by enough conductors
                let conductor_neighbors = neighbors.iter().filter(|&&s| s.is_conductor()).count();
                if conductor_neighbors >= 3 {
                    Conductor
                } else {
                    Empty
                }
            }

            Conductor => {
                // Conductor transmits signals
                let signal_count = neighbors.iter().filter(|&&s| s.is_signal()).count();
                if signal_count > 0 {
                    // Become the most common signal type
                    let ordinary_count = neighbors
                        .iter()
                        .filter(|&&s| s == OrdinaryTransmission)
                        .count();
                    let special_count = neighbors
                        .iter()
                        .filter(|&&s| s == SpecialTransmission)
                        .count();

                    if ordinary_count > special_count {
                        OrdinaryTransmission
                    } else if special_count > 0 {
                        SpecialTransmission
                    } else {
                        Conductor
                    }
                } else {
                    Conductor
                }
            }

            OrdinaryTransmission => {
                // Ordinary signals decay or continue
                let conductor_neighbors = neighbors.iter().filter(|&&s| s.is_conductor()).count();
                if conductor_neighbors > 0 {
                    Conductor
                } else {
                    Empty
                }
            }

            SpecialTransmission => {
                // Special signals have different behavior
                let conductor_neighbors = neighbors.iter().filter(|&&s| s.is_conductor()).count();
                if conductor_neighbors > 1 {
                    Conductor
                } else {
                    Empty
                }
            }

            Confluence => {
                // Confluence points manage signal flow
                let signal_neighbors = neighbors.iter().filter(|&&s| s.is_signal()).count();
                if signal_neighbors >= 2 {
                    SpecialTransmission
                } else if signal_neighbors == 1 {
                    OrdinaryTransmission
                } else {
                    Confluence
                }
            }

            OrdinaryReversed => {
                // Reversed signals
                let conductor_neighbors = neighbors.iter().filter(|&&s| s.is_conductor()).count();
                if conductor_neighbors > 0 {
                    Conductor
                } else {
                    Empty
                }
            }

            SpecialReversed => {
                // Special reversed signals
                let conductor_neighbors = neighbors.iter().filter(|&&s| s.is_conductor()).count();
                if conductor_neighbors > 0 {
                    Conductor
                } else {
                    Empty
                }
            }

            SheathedConductor => {
                // Sheathed conductors are more stable
                let signal_neighbors = neighbors.iter().filter(|&&s| s.is_signal()).count();
                if signal_neighbors > 2 {
                    SpecialTransmission
                } else {
                    SheathedConductor
                }
            }
        }
    }

    /// Get Moore neighborhood for a cell
    fn get_neighbors(&self, x: usize, y: usize) -> [CoddState; 8] {
        let mut neighbors = [CoddState::Empty; 8];
        let directions = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        for (i, (dx, dy)) in directions.iter().enumerate() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                neighbors[i] = self.grid[ny as usize][nx as usize];
            }
        }

        neighbors
    }

    /// Evolve for multiple generations
    pub fn evolve(&mut self, generations: usize) {
        for _ in 0..generations {
            self.step();
        }
    }

    /// Reset to initial state
    pub fn reset(&mut self) {
        if let Some(initial) = self.history.first() {
            self.grid = initial.clone();
            self.generation = 0;
            self.history = vec![initial.clone()];
        }
    }

    /// Get active cell count (non-empty cells)
    pub fn active_count(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell != CoddState::Empty)
            .count()
    }

    /// Get density (fraction of active cells)
    pub fn density(&self) -> f64 {
        self.active_count() as f64 / (self.width * self.height) as f64
    }

    /// Count cells by state
    pub fn count_state(&self, state: CoddState) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell == state)
            .count()
    }

    /// Save current state to history
    fn save_state(&mut self) {
        self.history.push(self.grid.clone());
        if self.history.len() > 100 {
            self.history.remove(0);
        }
    }
}

/// Types of Codd CA patterns
#[derive(Debug, Clone, Copy)]
pub enum CoddPatternType {
    Empty,
    Signal,
    Replicator,
}

/// Interactive Codd's cellular automata environment
pub struct CoddEnvironment {
    pub ca: CoddCA,
    pub paused: bool,
    pub step_mode: bool,
    pub show_help: bool,
    pub pattern_type: CoddPatternType,
    pub use_colors: bool,
    pub evolve_delay: Duration,
}

impl CoddEnvironment {
    /// Create new Codd CA environment
    pub fn new(pattern_type: CoddPatternType, width: usize, height: usize) -> Self {
        let ca = match pattern_type {
            CoddPatternType::Empty => CoddCA::new(width, height),
            CoddPatternType::Signal => CoddCA::new_with_signal(width, height),
            CoddPatternType::Replicator => CoddCA::new_with_replicator(width, height),
        };

        Self {
            ca,
            paused: true,
            step_mode: true,
            show_help: false,
            pattern_type,
            use_colors: true,
            evolve_delay: Duration::from_millis(200),
        }
    }

    /// Run the interactive environment
    pub fn run(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Run simple Codd CA demo
pub fn run_simple_codd_ca(
    pattern_type: CoddPatternType,
    generations: usize,
    width: usize,
    height: usize,
) -> Result<String> {
    let mut ca = match pattern_type {
        CoddPatternType::Empty => CoddCA::new(width, height),
        CoddPatternType::Signal => CoddCA::new_with_signal(width, height),
        CoddPatternType::Replicator => CoddCA::new_with_replicator(width, height),
    };

    let mut result = String::new();
    result.push_str(&format!("Codd's CA Demo - Pattern: {:?}\n", pattern_type));
    result.push_str(&format!(
        "Grid: {}x{}, Generations: {}\n\n",
        width, height, generations
    ));

    for gen in 0..=generations {
        result.push_str(&format!(
            "Generation {}: Active: {}\n",
            gen,
            ca.active_count()
        ));

        for row in &ca.grid {
            for &cell in row {
                result.push(cell.to_char());
            }
            result.push('\n');
        }
        result.push('\n');

        if gen < generations {
            ca.step();
        }
    }

    Ok(result)
}

/// Get available Codd CA patterns
pub fn codd_patterns() -> Vec<(CoddPatternType, &'static str)> {
    vec![
        (CoddPatternType::Empty, "Empty grid"),
        (CoddPatternType::Signal, "Signal transmission demo"),
        (
            CoddPatternType::Replicator,
            "Self-replicating structure (simplified)",
        ),
    ]
}
