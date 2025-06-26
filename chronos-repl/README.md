# chronos-repl

Enhanced REPL (Read-Eval-Print Loop) for the Chronos programming language.

This crate provides an interactive development environment for Chronos that builds on top of `chronos-core` to offer advanced features like session management, execution tracing, performance monitoring, and enhanced user experience.

## Features

### 🎯 Core REPL Functionality
- **Interactive evaluation** of Chronos code with immediate feedback
- **Stack visualization** with configurable display options
- **Error handling** with helpful error messages and suggestions
- **Command history** with persistent storage across sessions

### 💾 Session Management
- **Save and restore** complete REPL sessions including:
  - User-defined words and types
  - Current stack state
  - Configuration settings
  - Command history
- **Auto-save** functionality with configurable intervals
- **Session metadata** with timestamps, tags, and notes

### 🔍 Execution Tracing & Debugging
- **Detailed execution tracing** showing:
  - Stack state before/after each operation
  - Execution timing for performance analysis
  - Operation categorization (stack ops, arithmetic, etc.)
  - Error tracking and reporting
- **Selective tracing** with configurable filters
- **Performance metrics** including operations per second

### ⚡ Performance Monitoring
- **Real-time metrics** tracking:
  - Total operations executed
  - Execution time statistics
  - Maximum stack depth reached
  - Error rates and patterns
- **Performance analysis** tools for optimization

### 🛠 Enhanced User Experience
- **Customizable prompts** and display formatting
- **Colored output** with configurable themes
- **Rich command system** with shortcuts and aliases
- **Context-aware help** system

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
chronos-repl = "0.1.0"
```

Or install the binary:

```bash
cargo install chronos-repl
```

## Quick Start

### Library Usage

```rust
use chronos_repl::EnhancedRepl;

let mut repl = EnhancedRepl::new();

// Evaluate some code
repl.eval("2 3 +").unwrap();

// Check the result
let stack = repl.stack();
assert_eq!(stack.len(), 1);
```

### Interactive REPL

```bash
# Start the REPL
chronos-repl

# Or with options
chronos-repl --show-stack --show-timing my_session.json
```

## REPL Commands

### Basic Commands
- `.help` - Show help information
- `.stack` - Display current stack contents
- `.words` - List all defined words
- `.clear` - Clear the stack
- `.reset` - Reset REPL to initial state
- `.quit` - Exit the REPL

### Session Management
- `.save [file]` - Save current session
- `.load <file>` - Load session from file
- `.history` - Show command history

### Debugging & Analysis
- `.trace` - Toggle execution tracing
- `.metrics` - Show performance metrics
- `.showtrace [n]` - Show last n trace entries

### Configuration
- `.set <key> <value>` - Set configuration option

## Examples

### Basic Usage

```chronos
C∀O> 2 3 +
C∀O> .stack
[0] 5

C∀O> dup *
C∀O> .stack
[0] 25
```

### Word Definition

```chronos
C∀O> square : dup *
Defined word 'square'

C∀O> 5 square
C∀O> .stack
[0] 25
```

### Session Management

```chronos
C∀O> double : dup +
C∀O> 5 double
C∀O> .save my_session.json
Session saved to my_session.json

# Later...
C∀O> .load my_session.json
Session loaded from my_session.json
C∀O> 3 double
C∀O> .stack
[0] 6
```

### Execution Tracing

```chronos
C∀O> .trace on
Execution tracing enabled

C∀O> 2 3 + dup *
TRACE: 2 | 0 -> 1 (15μs)
TRACE: 3 | 1 -> 2 (12μs)
TRACE: + | 2 -> 1 (28μs)
TRACE: dup | 1 -> 2 (18μs)
TRACE: * | 2 -> 1 (31μs)

C∀O> .showtrace 3
1: 2 (15μs)
2: 3 (12μs)
3: + (28μs)
```

## Configuration

### Command Line Options

```bash
chronos-repl [OPTIONS] [SESSION_FILE]

OPTIONS:
    -h, --help              Show help message
    -v, --version           Show version information
    -s, --session <FILE>    Load session from file
    -p, --prompt <PROMPT>   Set custom prompt string
        --no-color          Disable colored output
        --no-welcome        Skip welcome message
        --show-stack        Show stack after each operation
        --show-timing       Show execution timing
        --auto-save         Enable automatic session saving
```

### Programmatic Configuration

```rust
use chronos_repl::{EnhancedRepl, ReplConfig};

let config = ReplConfig {
    show_stack: true,
    show_timing: true,
    prompt: ">>> ".to_string(),
    use_colors: false,
    auto_save: true,
    ..Default::default()
};

let mut repl = EnhancedRepl::with_config(config);
```

## Session Format

Sessions are stored as JSON files with the following structure:

```json
{
  "version": "0.1.0",
  "session_id": "uuid-here",
  "metadata": {
    "created_at": "2024-12-19T10:30:00Z",
    "modified_at": "2024-12-19T11:15:00Z",
    "name": "My Session",
    "tags": ["tutorial", "math"],
    "notes": "Working on arithmetic examples"
  },
  "stack": [{"Nat": 42}],
  "user_words": {
    "double": [{"Word": "dup"}, {"Word": "+"}]
  },
  "config": {
    "show_stack": false,
    "trace_execution": false
  },
  "history": ["2 3 +", "dup *"],
  "stats": {
    "operations_count": 15,
    "total_time_ms": 245,
    "max_stack_depth": 3,
    "words_defined": 2,
    "errors_count": 0
  }
}
```

## API Reference

### Core Types

- `EnhancedRepl` - Main REPL interface
- `ReplConfig` - Configuration options
- `Session` - Session management
- `ExecutionTrace` - Tracing system
- `PerformanceMetrics` - Performance monitoring

### Key Methods

```rust
impl EnhancedRepl {
    pub fn new() -> Self;
    pub fn with_config(config: ReplConfig) -> Self;
    pub fn start_interactive(&mut self) -> Result<()>;
    pub fn eval(&mut self, input: &str) -> Result<()>;
    pub fn eval_with_trace(&mut self, input: &str) -> Result<EvalResult>;
    pub fn save_session<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;
    pub fn load_session<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;
    pub fn stack(&self) -> &[Value];
    pub fn performance_metrics(&self) -> &PerformanceMetrics;
}
```

## Development

### Building

```bash
cd chronos-repl
cargo build
```

### Testing

```bash
cargo test
```

### Running Examples

```bash
cargo run --example basic_usage
```

## Contributing

Contributions are welcome! Please see the main Chronos repository for contribution guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.