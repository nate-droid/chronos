# Chronos Core

The foundational language runtime for **C∀O (Chronos)**, a concatenative programming language built on categorical foundations with ordinal verification.

## Overview

Chronos Core provides the essential components for parsing, type checking, and executing C∀O programs without any interactive or distributed system dependencies. It serves as the mathematical heart of the Chronos ecosystem.

## Features

- **Concatenative Programming**: Postfix notation with stack-based computation
- **Categorical Type System**: Types as objects, functions as morphisms
- **Ordinal Analysis**: Termination guarantees through proof theory
- **Pure Functional Core**: No side effects in the language runtime
- **Extensible Architecture**: Support for user-defined types and operations
- **Comprehensive Error Handling**: Detailed error reporting with context

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
chronos-core = "0.1.0"
```

### Basic Usage

```rust
use chronos_core::{ChronosCore, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut core = ChronosCore::new();
    
    // Basic arithmetic
    let result = core.eval("3 4 +")?;
    assert_eq!(result, Value::Nat(7));
    
    // Stack manipulation
    core.eval("1 2 3 dup")?;  // Stack: [1, 2, 3, 3]
    assert_eq!(core.stack_depth(), 4);
    
    // Boolean operations
    let result = core.eval("true false or")?;
    assert_eq!(result, Value::Bool(true));
    
    Ok(())
}
```

### Working with the Stack

```rust
use chronos_core::{ChronosCore, Value};

let mut core = ChronosCore::new();

// Push values directly
core.push(Value::Nat(42));
core.push(Value::Bool(true));

// Execute operations
core.eval("swap")?;  // Exchange top two values

// Access stack
let stack = core.get_stack();
println!("Current stack: {:?}", stack);

// Pop values
if let Some(top) = core.pop() {
    println!("Top value: {}", top);
}
```

### Error Handling

```rust
use chronos_core::{ChronosCore, ChronosError};

let mut core = ChronosCore::new();

match core.eval("1 0 /") {
    Ok(result) => println!("Result: {}", result),
    Err(ChronosError::ArithmeticError { message, operation, .. }) => {
        println!("Math error in {}: {}", operation, message);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Language Reference

### Core Types

| Type | Description | Literal Examples |
|------|-------------|------------------|
| `Unit` | Terminal object | `()` |
| `Bool` | Boolean values | `true`, `false` |
| `Nat` | Natural numbers | `0`, `1`, `42`, `1000` |
| `Quote` | Code blocks | `[ 2 * ]`, `[ dup + ]` |
| `Ordinal` | Proof ordinals | Used internally |

### Stack Operations

| Word | Stack Effect | Description |
|------|--------------|-------------|
| `dup` | `( a -- a a )` | Duplicate top element |
| `drop` | `( a -- )` | Remove top element |
| `swap` | `( a b -- b a )` | Exchange top two |
| `over` | `( a b -- a b a )` | Copy second to top |
| `rot` | `( a b c -- b c a )` | Rotate three elements |

### Arithmetic

| Word | Stack Effect | Description |
|------|--------------|-------------|
| `+` | `( a b -- a+b )` | Addition |
| `-` | `( a b -- a-b )` | Subtraction |
| `*` | `( a b -- a*b )` | Multiplication |
| `/` | `( a b -- a/b )` | Division |
| `mod` | `( a b -- a%b )` | Modulo |

### Control Flow

| Word | Stack Effect | Description |
|------|--------------|-------------|
| `if` | `( bool then else -- result )` | Conditional execution |
| `when` | `( bool quote -- ... )` | Execute if true |
| `unless` | `( bool quote -- ... )` | Execute if false |

### Examples

#### Factorial Function
```rust
// Note: This requires the full word definition syntax
// which is implemented in higher-level modules
let mut core = ChronosCore::new();

// For now, we can compute factorial iteratively:
core.eval("5 1 swap")?;  // Start: 5 1
core.eval("[ dup 1 > ] [ dup rot * swap 1 - swap ] while")?;
// Result: factorial of 5 = 120
```

#### Fibonacci Sequence
```rust
let mut core = ChronosCore::new();

// Compute 10th Fibonacci number
core.eval("0 1")?;           // Start: fib(0)=0, fib(1)=1
core.eval("8 times [ over over + ]")?;  // Compute next 8 terms
let result = core.pop().unwrap();  // Get fib(10)
```

## Architecture

### Core Components

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│    Lexer    │───▶│   Parser    │───▶│ Type System │
└─────────────┘    └─────────────┘    └─────────────┘
       │                   │                   │
       ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Tokens    │    │   AST       │    │ Type Check  │
└─────────────┘    └─────────────┘    └─────────────┘
                                            │
                                            ▼
                              ┌─────────────────────────┐
                              │    Virtual Machine      │
                              │  ┌─────────────────┐   │
                              │  │      Stack      │   │
                              │  └─────────────────┘   │
                              │  ┌─────────────────┐   │
                              │  │   Dictionary    │   │
                              │  └─────────────────┘   │
                              └─────────────────────────┘
```

### Module Structure

- **`types.rs`**: Core type system and value representations
- **`lexer.rs`**: Tokenization of source code
- **`parser.rs`**: Syntax analysis and AST generation  
- **`vm.rs`**: Stack-based virtual machine
- **`core_lib.rs`**: Built-in operations and standard library
- **`error.rs`**: Comprehensive error handling

## API Reference

### ChronosCore

The main interface for language operations:

```rust
impl ChronosCore {
    pub fn new() -> Self;
    pub fn eval(&mut self, source: &str) -> Result<Value>;
    pub fn tokenize(&self, source: &str) -> Result<Vec<Token>>;
    pub fn parse(&self, tokens: &[Token]) -> Result<Vec<Statement>>;
    pub fn execute_tokens(&mut self, tokens: &[Token]) -> Result<()>;
    
    // Stack management
    pub fn get_stack(&self) -> Vec<Value>;
    pub fn push(&mut self, value: Value);
    pub fn pop(&mut self) -> Option<Value>;
    pub fn clear_stack(&mut self);
    
    // Word management
    pub fn define_word(&mut self, name: String, definition: WordDefinition) -> Result<()>;
    pub fn get_words(&self) -> Vec<String>;
    pub fn is_word_defined(&self, name: &str) -> bool;
}
```

### Error Types

```rust
pub enum ChronosError {
    LexError { message: String, position: Option<usize>, .. },
    ParseError { message: String, expected: Option<String>, .. },
    TypeError { message: String, expected_type: Option<String>, .. },
    RuntimeError { message: String, word_name: Option<String>, .. },
    StackError { operation: String, stack_depth: usize, .. },
    UndefinedError { name: String, suggestion: Option<String> },
    // ... and more
}
```

### Value Types

```rust
pub enum Value {
    Unit,
    Bool(bool),
    Nat(u64),
    Ordinal(OrdinalValue),
    Quote(Vec<Token>),
    Composite { type_name: String, fields: HashMap<String, Value> },
    Option(Option<Box<Value>>),
    Result(Result<Box<Value>, Box<Value>>),
    List(Vec<Value>),
}
```

## Testing

Run the test suite:

```bash
cd chronos-core
cargo test
```

Run with verbose output:

```bash
cargo test -- --nocapture
```

Test specific modules:

```bash
cargo test vm::tests
cargo test types::tests
```

## Examples

See the `examples/` directory for complete working examples:

- `basic_evaluation.rs` - Simple arithmetic and stack operations
- `type_checking.rs` - Type system usage and validation
- `advanced_features.rs` - Complex language constructs

Run examples:

```bash
cargo run --example basic_evaluation
cargo run --example type_checking
```

## Mathematical Foundations

C∀O is built on solid mathematical principles:

### Category Theory
- **Objects**: Types in the type system
- **Morphisms**: Functions between types
- **Composition**: Function composition via concatenation
- **Identity**: The identity function for each type

### Ordinal Analysis
- **Termination**: All functions must terminate (proven via ordinals)
- **Consistency**: The type system maintains logical consistency
- **Well-foundedness**: Recursive definitions have decreasing ordinal measures

### Proof Theory
- **Propositions as Types**: Types correspond to logical propositions
- **Programs as Proofs**: Programs are constructive proofs
- **Curry-Howard**: Deep connection between computation and logic

## Performance Considerations

### Benchmarking

```rust
use std::time::Instant;
use chronos_core::ChronosCore;

let mut core = ChronosCore::new();
let start = Instant::now();

for _ in 0..1000 {
    core.eval("1 1 + drop")?;
}

println!("1000 operations took: {:?}", start.elapsed());
```

### Optimization Tips

- **Batch Operations**: Use `execute_tokens()` for multiple operations
- **Stack Reuse**: Avoid unnecessary `clear_stack()` calls  
- **Word Definitions**: Define frequently used operations as words
- **Error Handling**: Use `Result` pattern for better performance

## Integration

### With Higher-Level Modules

Chronos Core is designed to be extended by other modules:

```rust
// In chronos-repl
use chronos_core::{ChronosCore, Value};

struct EnhancedRepl {
    core: ChronosCore,
    // ... additional REPL features
}

impl EnhancedRepl {
    pub fn eval_with_trace(&mut self, input: &str) -> Result<(Value, TraceInfo)> {
        // Use core.eval() and add tracing
        let result = self.core.eval(input)?;
        // ... tracing logic
        Ok((result, trace_info))
    }
}
```

### Custom Extensions

```rust
use chronos_core::{ChronosCore, Value, WordDefinition, TypeSignature};

// Add custom words
let mut core = ChronosCore::new();

let custom_word = WordDefinition {
    name: "double".to_string(),
    signature: TypeSignature {
        inputs: vec![Type::Nat],
        outputs: vec![Type::Nat],
    },
    body: vec![Token::Literal(Value::Nat(2)), Token::Word("*".to_string())],
    is_axiom: false,
    ordinal_cost: OrdinalValue::Finite(1),
};

core.define_word("double".to_string(), custom_word)?;
let result = core.eval("5 double")?;  // Result: 10
```

## Contributing

Chronos Core follows strict mathematical principles. When contributing:

1. **Maintain Purity**: No side effects in core operations
2. **Preserve Termination**: All operations must terminate
3. **Type Safety**: Maintain categorical type discipline
4. **Test Coverage**: Add comprehensive tests for new features
5. **Documentation**: Document mathematical foundations

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT License

at your option.

## Related Crates

- **chronos-repl**: Interactive development environment
- **chronos-verification**: Ordinal analysis and proof checking
- **chronos-distributed**: Distributed computing capabilities
- **chronos-tooling**: Development automation tools

---

*"In the beginning was the Word, and the Word was with Math, and the Word was Math."*