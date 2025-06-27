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
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use serde::{Deserialize, Serialize};
use std::io;
use std::time::{Duration, Instant};

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

    /// Get numeric value for state
    pub fn to_num(&self) -> u8 {
        *self as u8
    }

    /// Create state from numeric value
    pub fn from_num(num: u8) -> Option<Self> {
        match num {
            0 => Some(CoddState::Empty),
            1 => Some(CoddState::Conductor),
            2 => Some(CoddState::OrdinaryTransmission),
            3 => Some(CoddState::SpecialTransmission),
            4 => Some(CoddState::Confluence),
            5 => Some(CoddState::OrdinaryReversed),
            6 => Some(CoddState::SpecialReversed),
            7 => Some(CoddState::SheathedConductor),
            _ => None,
        }
    }
}

/// Codd's cellular automaton (2D, 8-state)
#[derive(Debug, Clone)]
pub struct CoddCA {
    /// 2D grid of cells
    pub grid: Vec<Vec<CoddState>>,
    /// Generation number
    pub generation: usize,
    /// Width of the grid
    pub width: usize,
    /// Height of the grid
    pub height: usize,
    /// History of generations (limited for memory)
    pub history: Vec<Vec<Vec<CoddState>>>,
}

impl CoddCA {
    /// Create a new Codd CA with empty grid
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![CoddState::Empty; width]; height];

        Self {
            grid: grid.clone(),
            generation: 0,
            width,
            height,
            history: vec![grid],
        }
    }

    /// Create a new Codd CA with a simple signal pattern
    pub fn new_with_signal(width: usize, height: usize) -> Self {
        let mut ca = Self::new(width, height);

        // Create a simple signal transmission pattern
        if width >= 10 && height >= 5 {
            let mid_y = height / 2;
            // Horizontal conductor line
            for x in 2..8 {
                ca.grid[mid_y][x] = CoddState::Conductor;
            }
            // Signal at the start
            ca.grid[mid_y][2] = CoddState::OrdinaryTransmission;
        }

        ca.history = vec![ca.grid.clone()];
        ca
    }

    /// Create a new Codd CA with a replicator pattern (simplified)
    pub fn new_with_replicator(width: usize, height: usize) -> Self {
        let mut ca = Self::new(width, height);

        if width >= 10 && height >= 10 {
            let start_x = width / 4;
            let start_y = height / 4;

            // Create a simplified self-replicating pattern
            // This is a basic example - full Codd replicators are much more complex
            for i in 0..6 {
                ca.grid[start_y][start_x + i] = CoddState::SheathedConductor;
                ca.grid[start_y + 5][start_x + i] = CoddState::SheathedConductor;
            }
            for i in 1..5 {
                ca.grid[start_y + i][start_x] = CoddState::SheathedConductor;
                ca.grid[start_y + i][start_x + 5] = CoddState::SheathedConductor;
            }

            // Add some internal structure
            ca.grid[start_y + 2][start_x + 2] = CoddState::Conductor;
            ca.grid[start_y + 2][start_x + 3] = CoddState::OrdinaryTransmission;
        }

        ca.history = vec![ca.grid.clone()];
        ca
    }

    /// Apply Codd's transition rules to evolve one generation
    pub fn step(&mut self) {
        let mut new_grid = vec![vec![CoddState::Empty; self.width]; self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                new_grid[y][x] = self.apply_codd_rule(x, y);
            }
        }

        self.grid = new_grid.clone();
        self.generation += 1;

        // Keep limited history
        self.history.push(new_grid);
        if self.history.len() > 10 {
            self.history.remove(0);
        }
    }

    /// Apply Codd's cellular automaton rules (simplified version)
    fn apply_codd_rule(&self, x: usize, y: usize) -> CoddState {
        let current = self.grid[y][x];
        let neighbors = self.get_neighbors(x, y);

        match current {
            CoddState::Empty => {
                // Empty cells can become conductors if surrounded by enough conductors
                let conductor_count = neighbors
                    .iter()
                    .filter(|&&s| s == CoddState::Conductor || s == CoddState::SheathedConductor)
                    .count();
                if conductor_count >= 2 {
                    CoddState::Conductor
                } else {
                    CoddState::Empty
                }
            }

            CoddState::Conductor => {
                // Conductors can transmit signals
                let has_signal = neighbors.iter().any(|&s| {
                    matches!(
                        s,
                        CoddState::OrdinaryTransmission | CoddState::SpecialTransmission
                    )
                });

                if has_signal {
                    CoddState::OrdinaryTransmission
                } else {
                    CoddState::Conductor
                }
            }

            CoddState::OrdinaryTransmission => {
                // Signals move and can create new patterns
                let conductor_neighbors = neighbors
                    .iter()
                    .filter(|&&s| s == CoddState::Conductor)
                    .count();

                if conductor_neighbors > 0 {
                    CoddState::Conductor
                } else {
                    CoddState::Empty
                }
            }

            CoddState::SpecialTransmission => {
                // Special signals have different propagation rules
                CoddState::Conductor
            }

            CoddState::Confluence => {
                // Confluence states handle signal merging
                CoddState::OrdinaryTransmission
            }

            CoddState::OrdinaryReversed => CoddState::Conductor,

            CoddState::SpecialReversed => CoddState::Conductor,

            CoddState::SheathedConductor => {
                // Sheathed conductors are stable structures
                CoddState::SheathedConductor
            }
        }
    }

    /// Get the 8 neighbors of a cell (Moore neighborhood)
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

    /// Get a text representation of the current state
    pub fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.iter().map(|cell| cell.to_char()).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Count cells by state
    pub fn count_state(&self, state: CoddState) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell == state)
            .count()
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
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_loop(&mut terminal);

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }

    /// Main event loop for Codd CA environment
    fn run_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|f| self.ui(f))?;

            let timeout = self
                .evolve_delay
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char(' ') => {
                            self.paused = !self.paused;
                        }
                        KeyCode::Char('s') => {
                            if self.paused {
                                self.ca.step();
                            }
                        }
                        KeyCode::Char('r') => {
                            self.ca.reset();
                        }
                        KeyCode::Char('h') => {
                            self.show_help = !self.show_help;
                        }
                        KeyCode::Char('1') => {
                            self.ca.step();
                        }
                        KeyCode::Char('5') => {
                            self.ca.evolve(5);
                        }
                        KeyCode::F(1) => {
                            self.ca.evolve(10);
                        }
                        KeyCode::F(2) => {
                            self.ca.evolve(50);
                        }
                        KeyCode::Char('c') => {
                            self.use_colors = !self.use_colors;
                        }
                        _ => {}
                    }
                }
            }

            if !self.paused && last_tick.elapsed() >= self.evolve_delay {
                self.ca.step();
                last_tick = Instant::now();
            }
        }

        Ok(())
    }

    /// Draw the UI for Codd CA
    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(if self.show_help { 8 } else { 3 }),
            ])
            .split(f.size());

        // Status bar
        let status_text = format!(
            "Codd's CA | Pattern: {:?} | Gen: {} | Active: {} | Density: {:.2}% | {}",
            self.pattern_type,
            self.ca.generation,
            self.ca.active_count(),
            self.ca.density() * 100.0,
            if self.paused { "PAUSED" } else { "RUNNING" }
        );

        let status = Paragraph::new(status_text)
            .block(Block::default().borders(Borders::ALL).title("Status"))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(status, chunks[0]);

        // CA grid display
        let grid_lines: Vec<Line> = self
            .ca
            .grid
            .iter()
            .map(|row| {
                let spans: Vec<Span> = row
                    .iter()
                    .map(|&cell| {
                        Span::styled(
                            cell.to_char().to_string(),
                            Style::default().fg(if self.use_colors {
                                cell.color()
                            } else {
                                Color::White
                            }),
                        )
                    })
                    .collect();
                Line::from(spans)
            })
            .collect();

        let grid_widget = Paragraph::new(grid_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Codd's Cellular Automaton"),
            )
            .wrap(Wrap { trim: false });
        f.render_widget(grid_widget, chunks[1]);

        // Help or info panel
        if self.show_help {
            let help_text = vec![
                Line::from("Controls:"),
                Line::from("Space: Play/Pause | s: Step | r: Reset | h: Toggle help"),
                Line::from("1: Step 1 | 5: Step 5 | F1: Step 10 | F2: Step 50"),
                Line::from("c: Toggle colors | q: Quit"),
                Line::from(""),
                Line::from("Cell States:"),
                Line::from("  ▒ Conductor  → Ordinary Signal  ⇒ Special Signal"),
                Line::from("  ◊ Confluence  ← Reversed  █ Sheathed  (space) Empty"),
            ];

            let help = Paragraph::new(help_text)
                .block(Block::default().borders(Borders::ALL).title("Help"))
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(help, chunks[2]);
        } else {
            let info_text = format!(
                "States: Empty({}), Conductor({}), Signals({}), Sheathed({}) | Press 'h' for help",
                self.ca.count_state(CoddState::Empty),
                self.ca.count_state(CoddState::Conductor),
                self.ca.count_state(CoddState::OrdinaryTransmission)
                    + self.ca.count_state(CoddState::SpecialTransmission),
                self.ca.count_state(CoddState::SheathedConductor)
            );

            let info = Paragraph::new(info_text)
                .block(Block::default().borders(Borders::ALL).title("Info"))
                .style(Style::default().fg(Color::Green));
            f.render_widget(info, chunks[2]);
        }
    }
}

/// Run simple Codd CA and return text output
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

    result.push_str(&format!(
        "Codd's Cellular Automaton - Pattern: {:?}\n",
        pattern_type
    ));
    result.push_str(&format!(
        "Size: {}x{}, Generations: {}\n\n",
        width, height, generations
    ));

    for gen in 0..=generations {
        result.push_str(&format!("Generation {}:\n", gen));
        result.push_str(&ca.to_string());
        result.push_str(&format!(
            "\nActive cells: {}, Density: {:.2}%\n\n",
            ca.active_count(),
            ca.density() * 100.0
        ));

        if gen < generations {
            ca.step();
        }
    }

    Ok(result)
}

/// Get famous Codd CA patterns
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
