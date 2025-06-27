//! Enhanced Game-like Codd's Cellular Automata System for Chronos REPL
//!
//! This module provides an enhanced interactive Codd's cellular automata environment
//! with game-like features including:
//! - Interactive cell editing with mouse support
//! - Enhanced visual graphics and animations
//! - Pattern library and challenges
//! - Statistics tracking and achievements
//! - Save/load functionality
//! - Educational tutorials

use crate::error::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEvent, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Wrap,
    },
    Frame, Terminal,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::time::{Duration, Instant};

/// Enhanced Codd's cellular automaton cell states with game-like properties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CoddState {
    Empty = 0,
    Conductor = 1,
    OrdinaryTransmission = 2,
    SpecialTransmission = 3,
    Confluence = 4,
    OrdinaryReversed = 5,
    SpecialReversed = 6,
    SheathedConductor = 7,
}

impl CoddState {
    /// Get enhanced display character
    pub fn to_char(self, enhanced: bool) -> char {
        if enhanced {
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
        } else {
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
    }

    /// Get vibrant color for this state
    pub fn color(self) -> Color {
        match self {
            CoddState::Empty => Color::Rgb(20, 20, 20),
            CoddState::Conductor => Color::Rgb(180, 180, 180),
            CoddState::OrdinaryTransmission => Color::Rgb(0, 255, 100),
            CoddState::SpecialTransmission => Color::Rgb(100, 200, 255),
            CoddState::Confluence => Color::Rgb(255, 255, 100),
            CoddState::OrdinaryReversed => Color::Rgb(255, 100, 100),
            CoddState::SpecialReversed => Color::Rgb(255, 100, 255),
            CoddState::SheathedConductor => Color::Rgb(220, 220, 220),
        }
    }

    /// Get background color for highlighting
    pub fn bg_color(self) -> Color {
        match self {
            CoddState::Empty => Color::Rgb(5, 5, 5),
            CoddState::Conductor => Color::Rgb(30, 30, 30),
            CoddState::OrdinaryTransmission => Color::Rgb(0, 40, 20),
            CoddState::SpecialTransmission => Color::Rgb(20, 40, 50),
            CoddState::Confluence => Color::Rgb(40, 40, 20),
            CoddState::OrdinaryReversed => Color::Rgb(40, 20, 20),
            CoddState::SpecialReversed => Color::Rgb(40, 20, 40),
            CoddState::SheathedConductor => Color::Rgb(35, 35, 35),
        }
    }

    /// Get descriptive name
    pub fn name(self) -> &'static str {
        match self {
            CoddState::Empty => "Empty",
            CoddState::Conductor => "Conductor",
            CoddState::OrdinaryTransmission => "O-Signal",
            CoddState::SpecialTransmission => "S-Signal",
            CoddState::Confluence => "Confluence",
            CoddState::OrdinaryReversed => "O-Reversed",
            CoddState::SpecialReversed => "S-Reversed",
            CoddState::SheathedConductor => "Sheathed",
        }
    }

    /// Get detailed description
    pub fn description(self) -> &'static str {
        match self {
            CoddState::Empty => "Empty space - signals cannot propagate",
            CoddState::Conductor => "Conductor - transmits signals in all directions",
            CoddState::OrdinaryTransmission => "Ordinary signal moving through conductor",
            CoddState::SpecialTransmission => "Special signal with enhanced properties",
            CoddState::Confluence => "Signal junction point for complex interactions",
            CoddState::OrdinaryReversed => "Reversed ordinary signal",
            CoddState::SpecialReversed => "Reversed special signal",
            CoddState::SheathedConductor => "Protected conductor with isolation",
        }
    }

    /// Check if this is an active signal
    pub fn is_signal(self) -> bool {
        matches!(
            self,
            CoddState::OrdinaryTransmission
                | CoddState::SpecialTransmission
                | CoddState::OrdinaryReversed
                | CoddState::SpecialReversed
        )
    }

    /// Check if this can conduct signals
    pub fn is_conductor(self) -> bool {
        matches!(self, CoddState::Conductor | CoddState::SheathedConductor)
    }

    /// Get all states for cycling
    pub fn all_states() -> Vec<CoddState> {
        vec![
            CoddState::Empty,
            CoddState::Conductor,
            CoddState::OrdinaryTransmission,
            CoddState::SpecialTransmission,
            CoddState::Confluence,
            CoddState::OrdinaryReversed,
            CoddState::SpecialReversed,
            CoddState::SheathedConductor,
        ]
    }

    /// Get next state in cycle
    pub fn next(self) -> CoddState {
        let states = Self::all_states();
        let current_idx = states.iter().position(|&s| s == self).unwrap_or(0);
        states[(current_idx + 1) % states.len()]
    }

    /// Get previous state in cycle
    pub fn prev(self) -> CoddState {
        let states = Self::all_states();
        let current_idx = states.iter().position(|&s| s == self).unwrap_or(0);
        states[(current_idx + states.len() - 1) % states.len()]
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

/// Game statistics and achievements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStats {
    pub total_generations: u64,
    pub total_playtime: Duration,
    pub patterns_created: u32,
    pub challenges_completed: u32,
    pub max_signal_count: usize,
    pub max_active_cells: usize,
    pub sessions_played: u32,
}

impl Default for GameStats {
    fn default() -> Self {
        Self {
            total_generations: 0,
            total_playtime: Duration::new(0, 0),
            patterns_created: 0,
            challenges_completed: 0,
            max_signal_count: 0,
            max_active_cells: 0,
            sessions_played: 0,
        }
    }
}

/// Pattern template for creating structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub name: String,
    pub description: String,
    pub grid: Vec<Vec<CoddState>>,
    pub difficulty: u8, // 1-5
    pub category: String,
}

/// Challenge or puzzle to solve
#[derive(Debug, Clone)]
pub struct Challenge {
    pub name: String,
    pub description: String,
    pub initial_state: Vec<Vec<CoddState>>,
    pub goal: String,
    pub difficulty: u8,
    pub max_generations: Option<u32>,
}

/// Enhanced Codd CA with game features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedCoddCA {
    pub grid: Vec<Vec<CoddState>>,
    pub width: usize,
    pub height: usize,
    pub generation: u64,
    pub history: Vec<Vec<Vec<CoddState>>>,
    pub max_history: usize,
}

impl EnhancedCoddCA {
    /// Create new CA with given dimensions
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![CoddState::Empty; width]; height];
        Self {
            grid: grid.clone(),
            width,
            height,
            generation: 0,
            history: vec![grid],
            max_history: 100,
        }
    }

    /// Create with signal demonstration
    pub fn new_with_signal(width: usize, height: usize) -> Self {
        let mut ca = Self::new(width, height);

        // Create a signal transmission demo
        let center_x = width / 2;
        let center_y = height / 2;

        // Horizontal conductor line
        for x in center_x.saturating_sub(5)..=(center_x + 5).min(width - 1) {
            ca.grid[center_y][x] = CoddState::Conductor;
        }

        // Add signal at the start
        if center_x >= 3 {
            ca.grid[center_y][center_x - 3] = CoddState::OrdinaryTransmission;
        }

        ca.save_state();
        ca
    }

    /// Create with more complex pattern
    pub fn new_with_complex_pattern(width: usize, height: usize) -> Self {
        let mut ca = Self::new(width, height);

        let center_x = width / 2;
        let center_y = height / 2;

        // Create a cross pattern with different conductors
        for i in 0..5 {
            if center_x >= 2 && center_x + 2 < width {
                ca.grid[center_y][center_x - 2 + i] = CoddState::Conductor;
            }
            if center_y >= 2 && center_y + 2 < height {
                ca.grid[center_y - 2 + i][center_x] = CoddState::Conductor;
            }
        }

        // Add confluence at center
        ca.grid[center_y][center_x] = CoddState::Confluence;

        // Add signals from different directions
        if center_x >= 3 {
            ca.grid[center_y][center_x - 3] = CoddState::OrdinaryTransmission;
        }
        if center_y >= 3 {
            ca.grid[center_y - 3][center_x] = CoddState::SpecialTransmission;
        }

        ca.save_state();
        ca
    }

    /// Save current state to history
    fn save_state(&mut self) {
        self.history.push(self.grid.clone());
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }

    /// Step one generation forward
    pub fn step(&mut self) {
        let mut new_grid = self.grid.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                new_grid[y][x] = self.apply_codd_rule(x, y);
            }
        }

        self.grid = new_grid;
        self.generation += 1;
        self.save_state();
    }

    /// Apply Codd's rules (simplified version)
    fn apply_codd_rule(&self, x: usize, y: usize) -> CoddState {
        let current = self.grid[y][x];
        let neighbors = self.get_neighbors(x, y);

        match current {
            CoddState::Empty => {
                // Empty cells stay empty unless specific conditions
                CoddState::Empty
            }
            CoddState::Conductor => {
                // Check for incoming signals
                let signal_count = neighbors.iter()
                    .filter(|&&state| state.is_signal())
                    .count();

                if signal_count > 0 {
                    // Become the type of signal that's most common
                    let ordinary_count = neighbors.iter()
                        .filter(|&&state| matches!(state, CoddState::OrdinaryTransmission))
                        .count();
                    let special_count = neighbors.iter()
                        .filter(|&&state| matches!(state, CoddState::SpecialTransmission))
                        .count();

                    if ordinary_count > special_count {
                        CoddState::OrdinaryTransmission
                    } else if special_count > 0 {
                        CoddState::SpecialTransmission
                    } else {
                        CoddState::Conductor
                    }
                } else {
                    CoddState::Conductor
                }
            }
            CoddState::OrdinaryTransmission => {
                // Signals decay back to conductor or continue propagating
                let conductor_neighbors = neighbors.iter()
                    .filter(|&&state| state.is_conductor())
                    .count();

                if conductor_neighbors > 2 {
                    CoddState::Conductor
                } else {
                    CoddState::OrdinaryTransmission
                }
            }
            CoddState::SpecialTransmission => {
                let conductor_neighbors = neighbors.iter()
                    .filter(|&&state| state.is_conductor())
                    .count();

                if conductor_neighbors > 1 {
                    CoddState::Conductor
                } else {
                    CoddState::SpecialTransmission
                }
            }
            CoddState::Confluence => {
                // Confluence can create or destroy signals
                let signal_neighbors = neighbors.iter()
                    .filter(|&&state| state.is_signal())
                    .count();

                if signal_neighbors >= 2 {
                    CoddState::SpecialTransmission
                } else if signal_neighbors == 1 {
                    CoddState::OrdinaryTransmission
                } else {
                    CoddState::Confluence
                }
            }
            _ => current, // Other states for now
        }
    }

    /// Get Moore neighborhood
    fn get_neighbors(&self, x: usize, y: usize) -> [CoddState; 8] {
        let mut neighbors = [CoddState::Empty; 8];
        let directions = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1),
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

    /// Set cell at position
    pub fn set_cell(&mut self, x: usize, y: usize, state: CoddState) {
        if x < self.width && y < self.height {
            self.grid[y][x] = state;
        }
    }

    /// Get cell at position
    pub fn get_cell(&self, x: usize, y: usize) -> CoddState {
        if x < self.width && y < self.height {
            self.grid[y][x]
        } else {
            CoddState::Empty
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

    /// Count cells by state
    pub fn count_state(&self, state: CoddState) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell == state)
            .count()
    }

    /// Get active cell count
    pub fn active_count(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| *cell != CoddState::Empty)
            .count()
    }

    /// Get signal count
    pub fn signal_count(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell.is_signal())
            .count()
    }

    /// Get density of active cells
    pub fn density(&self) -> f64 {
        self.active_count() as f64 / (self.width * self.height) as f64
    }
}

/// Enhanced game environment with interactive features
pub struct EnhancedCoddEnvironment {
    pub ca: EnhancedCoddCA,
    pub paused: bool,
    pub edit_mode: bool,
    pub enhanced_graphics: bool,
    pub show_help: bool,
    pub show_stats: bool,
    pub show_patterns: bool,
    pub current_tool: CoddState,
    pub mouse_x: u16,
    pub mouse_y: u16,
    pub grid_offset_x: u16,
    pub grid_offset_y: u16,
    pub stats: GameStats,
    pub session_start: Instant,
    pub last_update: Instant,
    pub evolution_speed: Duration,
    pub patterns: Vec<Pattern>,
    pub challenges: Vec<Challenge>,
    pub selected_pattern: usize,
    pub zoom_level: f32,
}

impl EnhancedCoddEnvironment {
    /// Create new enhanced environment
    pub fn new(width: usize, height: usize, pattern_type: &str) -> Self {
        let ca = match pattern_type {
            "signal" => EnhancedCoddCA::new_with_signal(width, height),
            "complex" => EnhancedCoddCA::new_with_complex_pattern(width, height),
            _ => EnhancedCoddCA::new(width, height),
        };

        let patterns = Self::create_pattern_library();
        let challenges = Self::create_challenges();

        Self {
            ca,
            paused: true,
            edit_mode: false,
            enhanced_graphics: true,
            show_help: false,
            show_stats: false,
            show_patterns: false,
            current_tool: CoddState::Conductor,
            mouse_x: 0,
            mouse_y: 0,
            grid_offset_x: 3,
            grid_offset_y: 4,
            stats: GameStats::default(),
            session_start: Instant::now(),
            last_update: Instant::now(),
            evolution_speed: Duration::from_millis(150),
            patterns,
            challenges,
            selected_pattern: 0,
            zoom_level: 1.0,
        }
    }

    /// Create pattern library
    fn create_pattern_library() -> Vec<Pattern> {
        vec![
            Pattern {
                name: "Signal Line".to_string(),
                description: "Basic horizontal signal transmission".to_string(),
                grid: vec![
                    vec![CoddState::Empty, CoddState::Empty, CoddState::Empty],
                    vec![CoddState::OrdinaryTransmission, CoddState::Conductor, CoddState::Conductor],
                    vec![CoddState::Empty, CoddState::Empty, CoddState::Empty],
                ],
                difficulty: 1,
                category: "Basic".to_string(),
            },
            Pattern {
                name: "Cross Junction".to_string(),
                description: "Signal intersection with confluence".to_string(),
                grid: vec![
                    vec![CoddState::Empty, CoddState::Conductor, CoddState::Empty],
                    vec![CoddState::Conductor, CoddState::Confluence, CoddState::Conductor],
                    vec![CoddState::Empty, CoddState::Conductor, CoddState::Empty],
                ],
                difficulty: 2,
                category: "Junctions".to_string(),
            },
            Pattern {
                name: "Signal Loop".to_string(),
                description: "Circular signal propagation".to_string(),
                grid: vec![
                    vec![CoddState::Conductor, CoddState::Conductor, CoddState::Conductor],
                    vec![CoddState::Conductor, CoddState::Empty, CoddState::Conductor],
                    vec![CoddState::Conductor, CoddState::OrdinaryTransmission, CoddState::Conductor],
                ],
                difficulty: 3,
                category: "Loops".to_string(),
            },
        ]
    }

    /// Create challenges
    fn create_challenges() -> Vec<Challenge> {
        vec![
            Challenge {
                name: "First Signal".to_string(),
                description: "Create a signal that travels at least 5 cells".to_string(),
                initial_state: vec![vec![CoddState::Empty; 10]; 5],
                goal: "Signal propagation".to_string(),
                difficulty: 1,
                max_generations: Some(20),
            },
            Challenge {
                name: "Signal Split".to_string(),
                description: "Create a signal that splits into two paths".to_string(),
                initial_state: vec![vec![CoddState::Empty; 8]; 8],
                goal: "Signal bifurcation".to_string(),
                difficulty: 2,
                max_generations: Some(50),
            },
        ]
    }

    /// Run the enhanced environment
    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
