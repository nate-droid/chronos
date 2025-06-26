//! Chronos: C∀O (Kao) - Categorical ∀xiomatic Ordinal Programming Language
//!
//! An evolving axiomatic programming language that combines:
//! - Category Theory foundations
//! - Ordinal Analysis for termination proofs
//! - Concatenative (stack-based) programming
//! - Collaborative verification and evolution

mod core_lib;
mod lexer;
mod ordinal;
mod parser;
mod repl;
mod type_inference;
mod types;
mod vm;

use std::io::{self, Write};

use crate::repl::Repl;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("C∀O (Kao) - Categorical ∀xiomatic Ordinal Language v0.1.0");
    println!("An Evolving Axiomatic Programming Language");
    println!("Type 'help' for available commands, 'quit' to exit");
    println!();

    let mut repl = Repl::new();

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
