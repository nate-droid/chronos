//! Chronos: C∀O (Kao) - Categorical ∀xiomatic Ordinal Programming Language
//!
//! An evolving axiomatic programming language that combines:
//! - Category Theory foundations
//! - Ordinal Analysis for termination proofs
//! - Concatenative (stack-based) programming
//! - Collaborative verification and evolution

mod goal_builders;
mod hypervisor;
mod ordinal;
mod parser;

mod shell;
mod shell_manager;
mod type_inference;
mod vm;

use std::io::{self, Write};

use crate::hypervisor::Hypervisor;

use chronos_repl::repl;
use chronos_core;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("C∀O (Kao) - Categorical ∀xiomatic Ordinal Language v0.1.0");
    println!("An Evolving Axiomatic Programming Language");
    println!("Type 'help' for available commands, 'quit' to exit");
    println!("Type 'hypervisor' to enter hypervisor mode");
    println!();

    // let mut repl = Repl::new();
    let mut repl = repl::EnhancedRepl::new();    

    loop {
        print!("C∀O> ");
        io::stdout().flush()?;

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                if input == "quit" || input == "exit" {
                    println!("Farewell! May your axioms remain consistent.");
                    break;
                }

                if input == "hypervisor" {
                    println!("Entering hypervisor mode...");
                    let mut hypervisor = Hypervisor::new();
                    if let Err(e) = hypervisor.start_tui() {
                        eprintln!("Hypervisor error: {}", e);
                    }
                    println!("Returned to C∀O REPL");
                    continue;
                }

                if let Err(e) = repl.eval(input) {
                    eprintln!("Error: {}", e);
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }

    Ok(())
}
