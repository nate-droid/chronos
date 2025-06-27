//! Cellular Automata System for Chronos REPL
//!
//! This module provides an interactive cellular automata environment within the REPL
//! for exploring elementary cellular automata and eventually more complex systems
//! like Codd's cellular automata.
//!
//! Features:
//! - Elementary cellular automata (1D) visualization
//! - Rule specification using Wolfram notation
//! - Terminal-based graphics using Unicode characters
//! - Interactive evolution controls
//! - Extensible framework for 2D systems

use crate::error::{ReplError, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use serde::{Deserialize, Serialize};
use std::io;
use std::time::{Duration, Instant};

/// Elementary cellular automaton rule (Wolfram notation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ElementaryRule(pub u8);

impl ElementaryRule {
    /// Create a new elementary rule
    pub fn new(rule: u8) -> Self {
        Self(rule)
    }

    /// Apply the rule to a neighborhood (left, center, right)
    pub fn apply(&self, left: bool, center: bool, right: bool) -> bool {
        let pattern = (left as u8) << 2 | (center as u8) << 1 | (right as u8);
        (self.0 >> pattern) & 1 == 1
    }

    /// Get the rule table as a human-readable string
    pub fn rule_table(&self) -> String {
        let mut result = String::new();
        result.push_str("Pattern -> Output\n");
        for i in (0..8).rev() {
            let left = (i & 4) != 0;
            let center = (i & 2) != 0;
            let right = (i & 1) != 0;
            let output = self.apply(left, center, right);
            result.push_str(&format!(
                "{}{}{} -> {}\n",
                if left { "1" } else { "0" },
                if center { "1" } else { "0" },
                if right { "1" } else { "0" },
                if output { "1" } else { "0" }
            ));
        }
        result
    }

    /// Get famous rule names
    pub fn name(&self) -> Option<&'static str> {
        match self.0 {
            30 => Some("Rule 30 (Chaotic)"),
            90 => Some("Rule 90 (Sierpinski Triangle)"),
            110 => Some("Rule 110 (Turing Complete)"),
            150 => Some("Rule 150 (XOR)"),
            184 => Some("Rule 184 (Traffic)"),
            _ => None,
        }
    }
}

/// 1D cellular automaton state
#[derive(Debug, Clone)]
pub struct ElementaryCA {
    /// Current generation
    pub cells: Vec<bool>,
    /// Rule being applied
    pub rule: ElementaryRule,
    /// Generation number
    pub generation: usize,
    /// History of generations
    pub history: Vec<Vec<bool>>,
    /// Width of the automaton
    pub width: usize,
}

impl ElementaryCA {
    /// Create a new elementary CA with a single active cell in the center
    pub fn new(width: usize, rule: ElementaryRule) -> Self {
        let mut cells = vec![false; width];
        cells[width / 2] = true; // Single seed in center

        Self {
            cells: cells.clone(),
            rule,
            generation: 0,
            history: vec![cells],
            width,
        }
    }

    /// Create a new elementary CA with random initial state
    pub fn new_random(width: usize, rule: ElementaryRule, density: f64) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let cells: Vec<bool> = (0..width).map(|_| rng.gen::<f64>() < density).collect();

        Self {
            cells: cells.clone(),
            rule,
            generation: 0,
            history: vec![cells],
            width,
        }
    }

    /// Create a new elementary CA with custom initial pattern
    pub fn new_with_pattern(width: usize, rule: ElementaryRule, pattern: &str) -> Result<Self> {
        if pattern.len() > width {
            return Err(ReplError::command("Pattern too long for automaton width"));
        }

        let mut cells = vec![false; width];
        let start_pos = (width - pattern.len()) / 2;

        for (i, ch) in pattern.chars().enumerate() {
            match ch {
                '1' | '█' | '#' | '*' => cells[start_pos + i] = true,
                '0' | ' ' | '.' | '-' => cells[start_pos + i] = false,
                _ => return Err(ReplError::command("Invalid pattern character")),
            }
        }

        Ok(Self {
            cells: cells.clone(),
            rule,
            generation: 0,
            history: vec![cells],
            width,
        })
    }

    /// Evolve to the next generation
    pub fn step(&mut self) {
        let mut next_cells = vec![false; self.width];

        for i in 0..self.width {
            let left = if i == 0 { false } else { self.cells[i - 1] };
            let center = self.cells[i];
            let right = if i == self.width - 1 {
                false
            } else {
                self.cells[i + 1]
            };

            next_cells[i] = self.rule.apply(left, center, right);
        }

        self.cells = next_cells.clone();
        self.history.push(next_cells);
        self.generation += 1;
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
            self.cells = initial.clone();
            self.generation = 0;
            self.history = vec![initial.clone()];
        }
    }

    /// Get a text representation of the current state
    pub fn to_string(&self) -> String {
        self.cells
            .iter()
            .map(|&cell| if cell { '█' } else { ' ' })
            .collect()
    }

    /// Get a text representation of the entire history
    pub fn history_string(&self, max_generations: Option<usize>) -> String {
        let generations_to_show = if let Some(max) = max_generations {
            self.history.len().min(max)
        } else {
            self.history.len()
        };

        let start_idx = self.history.len().saturating_sub(generations_to_show);

        self.history[start_idx..]
            .iter()
            .enumerate()
            .map(|(i, generation)| {
                let gen_num = start_idx + i;
                let cells_str: String = generation
                    .iter()
                    .map(|&cell| if cell { '█' } else { ' ' })
                    .collect();
                format!("{:3}: {}", gen_num, cells_str)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Count active cells
    pub fn active_count(&self) -> usize {
        self.cells.iter().filter(|&&cell| cell).count()
    }

    /// Get density (fraction of active cells)
    pub fn density(&self) -> f64 {
        self.active_count() as f64 / self.width as f64
    }
}

/// Configuration for CA visualization
#[derive(Debug, Clone)]
pub struct CAConfig {
    pub width: usize,
    pub max_history: usize,
    pub auto_evolve: bool,
    pub evolve_delay: Duration,
    pub show_stats: bool,
    pub use_colors: bool,
}

impl Default for CAConfig {
    fn default() -> Self {
        Self {
            width: 79,
            max_history: 50,
            auto_evolve: false,
            evolve_delay: Duration::from_millis(100),
            show_stats: true,
            use_colors: true,
        }
    }
}

/// Interactive cellular automata environment
pub struct CAEnvironment {
    pub ca: ElementaryCA,
    pub config: CAConfig,
    pub paused: bool,
    pub step_mode: bool,
    pub show_help: bool,
}

impl CAEnvironment {
    /// Create new CA environment
    pub fn new(rule: ElementaryRule, config: CAConfig) -> Self {
        let ca = ElementaryCA::new(config.width, rule);
        Self {
            ca,
            config,
            paused: true,
            step_mode: true,
            show_help: false,
        }
    }

    /// Create new CA environment with random initial state
    pub fn new_random(rule: ElementaryRule, config: CAConfig, density: f64) -> Self {
        let ca = ElementaryCA::new_random(config.width, rule, density);
        Self {
            ca,
            config,
            paused: true,
            step_mode: true,
            show_help: false,
        }
    }

    /// Create new CA environment with pattern
    pub fn new_with_pattern(rule: ElementaryRule, config: CAConfig, pattern: &str) -> Result<Self> {
        let ca = ElementaryCA::new_with_pattern(config.width, rule, pattern)?;
        Ok(Self {
            ca,
            config,
            paused: true,
            step_mode: true,
            show_help: false,
        })
    }

    /// Run the interactive CA environment
    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()
            .map_err(|e| ReplError::command(format!("Failed to enable raw mode: {}", e)))?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .map_err(|e| ReplError::command(format!("Failed to setup terminal: {}", e)))?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)
            .map_err(|e| ReplError::command(format!("Failed to create terminal: {}", e)))?;

        let app_result = self.run_app(&mut terminal);

        // Restore terminal
        disable_raw_mode()
            .map_err(|e| ReplError::command(format!("Failed to disable raw mode: {}", e)))?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .map_err(|e| ReplError::command(format!("Failed to restore terminal: {}", e)))?;
        terminal
            .show_cursor()
            .map_err(|e| ReplError::command(format!("Failed to show cursor: {}", e)))?;

        app_result
    }

    /// Run the main application loop
    fn run_app(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        let mut last_tick = Instant::now();
        let mut last_evolve = Instant::now();
        let tick_rate = Duration::from_millis(50);

        loop {
            terminal
                .draw(|f| self.ui(f))
                .map_err(|e| ReplError::command(format!("Draw error: {}", e)))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)
                .map_err(|e| ReplError::command(format!("Poll error: {}", e)))?
            {
                if let Event::Key(key) =
                    event::read().map_err(|e| ReplError::command(format!("Read error: {}", e)))?
                {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char(' ') => self.paused = !self.paused,
                        KeyCode::Char('s') => {
                            if self.paused {
                                self.ca.step();
                            }
                        }
                        KeyCode::Char('r') => self.ca.reset(),
                        KeyCode::Char('h') => self.show_help = !self.show_help,
                        KeyCode::Char('a') => self.config.auto_evolve = !self.config.auto_evolve,
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            if self.config.evolve_delay.as_millis() > 10 {
                                self.config.evolve_delay = Duration::from_millis(
                                    self.config.evolve_delay.as_millis() as u64 - 10,
                                );
                            }
                        }
                        KeyCode::Char('-') => {
                            self.config.evolve_delay = Duration::from_millis(
                                self.config.evolve_delay.as_millis() as u64 + 10,
                            );
                        }
                        KeyCode::Char('c') => self.config.use_colors = !self.config.use_colors,
                        KeyCode::Char('1') => self.ca.evolve(1),
                        KeyCode::Char('5') => self.ca.evolve(5),
                        KeyCode::F(1) => self.ca.evolve(10),
                        KeyCode::F(2) => self.ca.evolve(50),
                        KeyCode::F(3) => self.ca.evolve(100),
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }

            // Auto-evolve if enabled and not paused
            if self.config.auto_evolve
                && !self.paused
                && last_evolve.elapsed() >= self.config.evolve_delay
            {
                self.ca.step();
                last_evolve = Instant::now();
            }
        }
    }

    /// Render the user interface
    fn ui(&mut self, f: &mut Frame) {
        let size = f.size();

        // Split into main area and help area
        let chunks = if self.show_help {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(size)
        } else {
            Layout::default()
                .constraints([Constraint::Percentage(100)])
                .split(size)
        };

        // Main area layout
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Status bar
                Constraint::Min(10),   // CA display
                Constraint::Length(5), // Info panel
            ])
            .split(chunks[0]);

        // Render status bar
        self.render_status_bar(f, main_chunks[0]);

        // Render CA display
        self.render_ca_display(f, main_chunks[1]);

        // Render info panel
        self.render_info_panel(f, main_chunks[2]);

        // Render help if shown
        if self.show_help {
            self.render_help(f, chunks[1]);
        }
    }

    fn render_status_bar(&self, f: &mut Frame, area: Rect) {
        let status_text = format!(
            "Rule: {} | Gen: {} | Cells: {}/{} ({:.1}%) | {}",
            self.ca.rule.0,
            self.ca.generation,
            self.ca.active_count(),
            self.ca.width,
            self.ca.density() * 100.0,
            if self.paused { "PAUSED" } else { "RUNNING" }
        );

        let style = if self.config.use_colors {
            Style::default().fg(if self.paused {
                Color::Yellow
            } else {
                Color::Green
            })
        } else {
            Style::default()
        };

        let paragraph = Paragraph::new(status_text).style(style).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Cellular Automaton"),
        );

        f.render_widget(paragraph, area);
    }

    fn render_ca_display(&self, f: &mut Frame, area: Rect) {
        let inner = area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        });
        let display_height = inner.height as usize;

        // Show recent history
        let history_to_show = self.ca.history.len().min(display_height);
        let start_idx = self.ca.history.len().saturating_sub(history_to_show);

        let lines: Vec<Line> = self.ca.history[start_idx..]
            .iter()
            .enumerate()
            .map(|(i, generation)| {
                let gen_num = start_idx + i;
                let cells_str: String = generation
                    .iter()
                    .map(|&cell| if cell { '█' } else { '·' })
                    .collect();

                let spans = if self.config.use_colors {
                    vec![
                        Span::styled(format!("{:3}: ", gen_num), Style::default().fg(Color::Cyan)),
                        Span::styled(cells_str, Style::default().fg(Color::White)),
                    ]
                } else {
                    vec![Span::raw(format!("{:3}: {}", gen_num, cells_str))]
                };

                Line::from(spans)
            })
            .collect();

        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("Evolution"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn render_info_panel(&self, f: &mut Frame, area: Rect) {
        let rule_name = self.ca.rule.name().unwrap_or("Custom Rule");
        let auto_status = if self.config.auto_evolve { "ON" } else { "OFF" };

        let info_text = format!(
            "{}\nDelay: {}ms | Auto: {} | Press 'h' for help",
            rule_name,
            self.config.evolve_delay.as_millis(),
            auto_status
        );

        let paragraph = Paragraph::new(info_text)
            .block(Block::default().borders(Borders::ALL).title("Info"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn render_help(&self, f: &mut Frame, area: Rect) {
        let help_text = vec![
            Line::from("Cellular Automata Controls:"),
            Line::from(""),
            Line::from("Space - Play/Pause"),
            Line::from("s     - Single step (when paused)"),
            Line::from("r     - Reset to initial state"),
            Line::from("a     - Toggle auto-evolution"),
            Line::from(""),
            Line::from("Evolution:"),
            Line::from("1     - Evolve 1 generation"),
            Line::from("5     - Evolve 5 generations"),
            Line::from("F1    - Evolve 10 generations"),
            Line::from("F2    - Evolve 50 generations"),
            Line::from("F3    - Evolve 100 generations"),
            Line::from(""),
            Line::from("Display:"),
            Line::from("c     - Toggle colors"),
            Line::from("+/-   - Speed up/slow down"),
            Line::from(""),
            Line::from("h     - Toggle this help"),
            Line::from("q     - Quit"),
        ];

        let paragraph = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title("Help"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }
}

/// Simple text-based CA runner for non-interactive use
pub fn run_simple_ca(
    rule: ElementaryRule,
    generations: usize,
    width: usize,
    pattern: Option<&str>,
) -> Result<String> {
    let mut ca = if let Some(p) = pattern {
        ElementaryCA::new_with_pattern(width, rule, p)?
    } else {
        ElementaryCA::new(width, rule)
    };

    ca.evolve(generations);
    Ok(ca.history_string(None))
}

/// Get a summary of famous rules
pub fn famous_rules() -> Vec<(u8, &'static str)> {
    vec![
        (30, "Rule 30 (Chaotic)"),
        (90, "Rule 90 (Sierpinski Triangle)"),
        (110, "Rule 110 (Turing Complete)"),
        (150, "Rule 150 (XOR)"),
        (184, "Rule 184 (Traffic)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elementary_rule() {
        let rule30 = ElementaryRule::new(30);

        // Test specific patterns for Rule 30
        assert_eq!(rule30.apply(true, true, true), false);
        assert_eq!(rule30.apply(false, false, true), true);
        assert_eq!(rule30.apply(false, true, false), true);
    }

    #[test]
    fn test_ca_evolution() {
        let rule30 = ElementaryRule::new(30);
        let mut ca = ElementaryCA::new(5, rule30);

        ca.step();

        assert_eq!(ca.generation, 1);
        assert_eq!(ca.history.len(), 2);
    }

    #[test]
    fn test_ca_with_pattern() {
        let rule90 = ElementaryRule::new(90);
        let ca = ElementaryCA::new_with_pattern(7, rule90, "111").unwrap();

        assert_eq!(ca.active_count(), 3);
        assert_eq!(ca.generation, 0);
    }

    #[test]
    fn test_rule_names() {
        assert_eq!(ElementaryRule::new(30).name(), Some("Rule 30 (Chaotic)"));
        assert_eq!(
            ElementaryRule::new(90).name(),
            Some("Rule 90 (Sierpinski Triangle)")
        );
        assert_eq!(ElementaryRule::new(255).name(), None);
    }

    #[test]
    fn test_simple_ca_run() {
        let result = run_simple_ca(ElementaryRule::new(90), 5, 11, Some("1")).unwrap();
        assert!(result.contains("0:"));
        assert!(result.contains("5:"));
    }
}
