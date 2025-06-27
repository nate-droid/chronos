//! Standalone Cellular Automata Binary for Chronos
//!
//! This binary provides cellular automata functionality that can be called
//! from the main Chronos REPL without cluttering the core runtime.

use std::env;
use std::process;

/// Elementary cellular automaton rule (Wolfram notation)
#[derive(Debug, Clone, Copy)]
struct ElementaryRule(u8);

impl ElementaryRule {
    fn new(rule: u8) -> Self {
        Self(rule)
    }

    fn apply(&self, left: bool, center: bool, right: bool) -> bool {
        let pattern = (left as u8) << 2 | (center as u8) << 1 | (right as u8);
        (self.0 >> pattern) & 1 == 1
    }

    fn rule_table(&self) -> String {
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

    fn name(&self) -> Option<&'static str> {
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
struct ElementaryCA {
    cells: Vec<bool>,
    rule: ElementaryRule,
    generation: usize,
    history: Vec<Vec<bool>>,
    width: usize,
}

impl ElementaryCA {
    fn new(width: usize, rule: ElementaryRule) -> Self {
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

    fn new_with_pattern(width: usize, rule: ElementaryRule, pattern: &str) -> Result<Self, String> {
        if pattern.len() > width {
            return Err("Pattern too long for automaton width".to_string());
        }

        let mut cells = vec![false; width];
        let start_pos = (width - pattern.len()) / 2;

        for (i, ch) in pattern.chars().enumerate() {
            match ch {
                '1' | '█' | '#' | '*' => cells[start_pos + i] = true,
                '0' | ' ' | '.' | '-' => cells[start_pos + i] = false,
                _ => return Err("Invalid pattern character".to_string()),
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

    fn step(&mut self) {
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

    fn evolve(&mut self, generations: usize) {
        for _ in 0..generations {
            self.step();
        }
    }

    fn history_string(&self, max_generations: Option<usize>) -> String {
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
                    .map(|&cell| if cell { '█' } else { '·' })
                    .collect();
                format!("{:3}: {}", gen_num, cells_str)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn show_usage() {
    println!("Chronos Cellular Automata");
    println!("Usage:");
    println!("  cellular_automata rules                    - List famous rules");
    println!("  cellular_automata rule <number>            - Show rule table");
    println!("  cellular_automata simple <rule> <gens>     - Run simple CA");
    println!("  cellular_automata simple <rule> <gens> <pattern> - Run with pattern");
    println!("  cellular_automata interactive <rule>       - Interactive CA");
    println!("  cellular_automata interactive <rule> <pattern> - Interactive with pattern");
}

fn show_famous_rules() {
    println!("Famous Cellular Automaton Rules:");
    println!();
    println!("Rule 30: Rule 30 (Chaotic)");
    println!("Rule 90: Rule 90 (Sierpinski Triangle)");
    println!("Rule 110: Rule 110 (Turing Complete)");
    println!("Rule 150: Rule 150 (XOR)");
    println!("Rule 184: Rule 184 (Traffic)");
    println!();
    println!("Use 'cellular_automata simple <rule> <generations>' for text output");
    println!("Use 'cellular_automata interactive <rule>' for interactive mode");
}

fn show_rule_table(rule: u8) {
    let ca_rule = ElementaryRule::new(rule);
    println!("Elementary Cellular Automaton Rule {}", rule);
    if let Some(name) = ca_rule.name() {
        println!("{}", name);
    }
    println!();
    print!("{}", ca_rule.rule_table());
}

fn run_simple_ca(rule: u8, generations: usize, pattern: Option<&str>) -> Result<(), String> {
    let ca_rule = ElementaryRule::new(rule);
    let mut ca = if let Some(p) = pattern {
        ElementaryCA::new_with_pattern(79, ca_rule, p)?
    } else {
        ElementaryCA::new(79, ca_rule)
    };

    ca.evolve(generations);
    println!("{}", ca.history_string(None));
    Ok(())
}

fn run_interactive_ca(rule: u8, pattern: Option<&str>) -> Result<(), String> {
    println!("Interactive Cellular Automaton (Rule {})", rule);
    if let Some(name) = ElementaryRule::new(rule).name() {
        println!("{}", name);
    }
    println!();
    println!("Controls:");
    println!("  Press Enter to evolve one generation");
    println!("  Type 'n <number>' to evolve multiple generations");
    println!("  Type 'r' to reset");
    println!("  Type 'q' to quit");
    println!();

    let ca_rule = ElementaryRule::new(rule);
    let mut ca = if let Some(p) = pattern {
        ElementaryCA::new_with_pattern(79, ca_rule, p)?
    } else {
        ElementaryCA::new(79, ca_rule)
    };

    println!("Initial state:");
    println!("{}", ca.history_string(None));
    println!();

    loop {
        print!("CA> ");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            ca.step();
            println!("Generation {}:", ca.generation);
            let cells_str: String = ca
                .cells
                .iter()
                .map(|&cell| if cell { '█' } else { '·' })
                .collect();
            println!("{:3}: {}", ca.generation, cells_str);
        } else if input == "q" || input == "quit" {
            break;
        } else if input == "r" || input == "reset" {
            ca = if let Some(p) = pattern {
                ElementaryCA::new_with_pattern(79, ca_rule, p)?
            } else {
                ElementaryCA::new(79, ca_rule)
            };
            println!("Reset to initial state");
            println!("{}", ca.history_string(None));
        } else if let Some(n_str) = input.strip_prefix("n ") {
            if let Ok(n) = n_str.parse::<usize>() {
                ca.evolve(n);
                println!(
                    "Evolved {} generations. Now at generation {}:",
                    n, ca.generation
                );
                println!("{}", ca.history_string(Some(20))); // Show last 20 generations
            } else {
                println!("Invalid number: {}", n_str);
            }
        } else if input == "h" || input == "help" {
            println!("Controls:");
            println!("  Enter     - Evolve one generation");
            println!("  n <num>   - Evolve multiple generations");
            println!("  r         - Reset to initial state");
            println!("  h         - Show this help");
            println!("  q         - Quit");
        } else {
            println!("Unknown command: {}. Type 'h' for help.", input);
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        show_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "rules" => {
            show_famous_rules();
        }
        "rule" => {
            if args.len() < 3 {
                eprintln!("Error: Missing rule number");
                show_usage();
                process::exit(1);
            }
            match args[2].parse::<u8>() {
                Ok(rule) => show_rule_table(rule),
                Err(_) => {
                    eprintln!("Error: Invalid rule number: {}", args[2]);
                    process::exit(1);
                }
            }
        }
        "simple" => {
            if args.len() < 4 {
                eprintln!("Error: Missing rule number or generations");
                show_usage();
                process::exit(1);
            }
            match (args[2].parse::<u8>(), args[3].parse::<usize>()) {
                (Ok(rule), Ok(gens)) => {
                    let pattern = args.get(4).map(|s| s.as_str());
                    if let Err(e) = run_simple_ca(rule, gens, pattern) {
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                }
                _ => {
                    eprintln!("Error: Invalid rule number or generations");
                    process::exit(1);
                }
            }
        }
        "interactive" => {
            if args.len() < 3 {
                eprintln!("Error: Missing rule number");
                show_usage();
                process::exit(1);
            }
            match args[2].parse::<u8>() {
                Ok(rule) => {
                    let pattern = args.get(3).map(|s| s.as_str());
                    if let Err(e) = run_interactive_ca(rule, pattern) {
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                }
                Err(_) => {
                    eprintln!("Error: Invalid rule number: {}", args[2]);
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Error: Unknown command: {}", args[1]);
            show_usage();
            process::exit(1);
        }
    }
}
