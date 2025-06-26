//! Chronos REPL Binary
//!
//! Command-line executable for the Chronos Enhanced REPL

use chronos_repl::{EnhancedRepl, ReplConfig, ReplError};
use std::env;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<(), ReplError> {
    // Parse command line arguments
    let mut config = ReplConfig::default();
    let mut session_file: Option<String> = None;
    let mut show_help = false;
    let mut show_version = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                show_help = true;
                break;
            }
            "-v" | "--version" => {
                show_version = true;
                break;
            }
            "--no-color" => {
                config.use_colors = false;
            }
            "--no-welcome" => {
                config.show_welcome = false;
            }
            "--show-stack" => {
                config.show_stack = true;
            }
            "--show-timing" => {
                config.show_timing = true;
            }
            "--auto-save" => {
                config.auto_save = true;
            }
            "--session" | "-s" => {
                if i + 1 < args.len() {
                    session_file = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --session requires a filename");
                    process::exit(1);
                }
            }
            "--prompt" | "-p" => {
                if i + 1 < args.len() {
                    config.prompt = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Error: --prompt requires a prompt string");
                    process::exit(1);
                }
            }
            arg if arg.starts_with('-') => {
                eprintln!("Error: Unknown option: {}", arg);
                print_usage(&args[0]);
                process::exit(1);
            }
            _ => {
                // Treat as session file if no session specified yet
                if session_file.is_none() {
                    session_file = Some(args[i].clone());
                } else {
                    eprintln!("Error: Multiple session files specified");
                    process::exit(1);
                }
            }
        }
        i += 1;
    }

    if show_help {
        print_help(&args[0]);
        return Ok(());
    }

    if show_version {
        print_version();
        return Ok(());
    }

    // Create REPL instance
    let mut repl = EnhancedRepl::with_config(config);

    // Load session file if specified
    if let Some(file_path) = session_file {
        if Path::new(&file_path).exists() {
            println!("Loading session from: {}", file_path);
            repl.load_session(&file_path)?;
            println!("Session loaded successfully.");
        } else {
            eprintln!(
                "Warning: Session file '{}' not found, starting with empty session",
                file_path
            );
        }
    }

    // Start interactive REPL
    repl.start_interactive()?;

    Ok(())
}

fn print_help(program_name: &str) {
    println!("Chronos Enhanced REPL v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("USAGE:");
    println!("    {} [OPTIONS] [SESSION_FILE]", program_name);
    println!();
    println!("OPTIONS:");
    println!("    -h, --help              Show this help message");
    println!("    -v, --version           Show version information");
    println!("    -s, --session <FILE>    Load session from file");
    println!("    -p, --prompt <PROMPT>   Set custom prompt string");
    println!("        --no-color          Disable colored output");
    println!("        --no-welcome        Skip welcome message");
    println!("        --show-stack        Show stack after each operation");
    println!("        --show-timing       Show execution timing");
    println!("        --auto-save         Enable automatic session saving");
    println!();
    println!("EXAMPLES:");
    println!(
        "    {}                      Start with default settings",
        program_name
    );
    println!(
        "    {} my_session.json      Load and start with session file",
        program_name
    );
    println!("    {} --show-stack --show-timing", program_name);
    println!("    {} --prompt \">>> \" --no-color", program_name);
    println!();
    println!("REPL COMMANDS:");
    println!("    .help                   Show REPL command help");
    println!("    .stack                  Show current stack");
    println!("    .save [file]            Save current session");
    println!("    .load <file>            Load session from file");
    println!("    .trace                  Toggle execution tracing");
    println!("    .quit                   Exit the REPL");
    println!();
    println!("For language documentation, visit: https://github.com/chronos-lang/chronos");
}

fn print_version() {
    println!("Chronos Enhanced REPL v{}", env!("CARGO_PKG_VERSION"));
    println!("Built with chronos-core");
    println!();
    println!("C∀O (Kao) - Categorical ∀xiomatic Ordinal Programming Language");
    println!("An evolving axiomatic programming language");
}

fn print_usage(program_name: &str) {
    eprintln!("Usage: {} [OPTIONS] [SESSION_FILE]", program_name);
    eprintln!("Try '{} --help' for more information.", program_name);
}
