# C∀O Architecture Guide

**Purpose**: Comprehensive guide to understanding the C∀O (Chronos) codebase structure  
**Audience**: New contributors, maintainers, and developers extending the language  
**Last Updated**: Enhanced REPL implementation phase  

## 🏗️ High-Level Architecture

### System Overview

The C∀O language implementation follows a modular architecture with clear separation of concerns:

```
User Input → Lexer → Parser → Type Checker → VM → Output
              ↓        ↓          ↓         ↓
           Tokens   AST      TypedAST   Execution
```

### Core Components

1. **REPL** (`repl.rs`) - Enhanced interactive environment
2. **Virtual Machine** (`vm.rs`) - Stack-based execution engine
3. **Parser** (`parser.rs`) - Syntax analysis and AST generation
4. **Type System** (`types.rs`) - Core type definitions and operations
5. **Lexer** (`lexer.rs`) - Tokenization of source code
6. **Core Library** (`core_lib.rs`) - Built-in words and operations
7. **Ordinal Verifier** (`ordinal.rs`) - Termination analysis

## 📁 File-by-File Architecture

### `src/main.rs` - Entry Point
```rust
// Simple entry point that starts the REPL
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut repl = Repl::new();
    // Main REPL loop
}
```
**Responsibilities**:
- Application initialization
- REPL startup
- Error handling at top level

**Key Functions**:
- `main()` - Application entry point

---

### `src/repl.rs` - Enhanced REPL ✅ RECENTLY ENHANCED
```rust
pub struct Repl {
    vm: VirtualMachine,
    core_lib: CoreLibrary,
    verifier: OrdinalVerifier,
    // NEW: Enhanced features
    execution_trace: Vec<TraceEntry>,
    performance_metrics: PerformanceMetrics,
    command_history: Vec<String>,
}
```

**Responsibilities**:
- Interactive command processing
- Session management (save/load)
- Execution tracing and debugging
- Performance benchmarking
- Command history tracking

**Key Functions**:
- `eval()` - Process user input
- `save_session()` / `load_session()` - Session persistence
- `execute_tokens_with_trace()` - Traced execution
- `benchmark_code()` - Performance analysis
- `handle_repl_command()` - Dot commands (.help, .trace, etc.)

**New Features** (Enhanced):
- 15+ REPL commands
- Session persistence with JSON serialization
- Real-time execution tracing
- Performance metrics and benchmarking
- Command history with replay

---

### `src/vm.rs` - Virtual Machine
```rust
pub struct VirtualMachine {
    stack: Vec<Value>,
    dictionary: HashMap<String, WordDefinition>,
    type_signatures: HashMap<String, TypeSignature>,
    call_stack: Vec<String>,
}
```

**Responsibilities**:
- Stack-based program execution
- Word definition storage and lookup
- Built-in operation implementation
- Recursion depth tracking

**Key Functions**:
- `execute_token()` - Execute single token
- `execute_tokens()` - Execute token sequence
- `define_word()` - Register new word definition
- `builtin_*()` - Implementation of core operations

**Core Operations**:
- Stack manipulation (dup, drop, swap, over, rot)
- Arithmetic (+, -, *, /, mod)
- Comparison (=, <, >, <=, >=)
- Control flow (if, when, unless)
- System operations (., .s, --ordinal)

---

### `src/types.rs` - Type System ✅ ENHANCED WITH SERDE
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Unit, Bool(bool), Nat(u64), 
    Ordinal(OrdinalValue), Quote(Vec<Token>),
    Composite { type_name: String, fields: HashMap<String, Value> }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Unit, Bool, Nat, Ordinal, Quote,
    Composite { name: String, fields: HashMap<String, Type> },
    Variable(String)  // For polymorphism
}
```

**Responsibilities**:
- Core value representation
- Type definition and checking
- Serialization support for session persistence
- Ordinal value representation

**Key Types**:
- `Value` - Runtime values
- `Type` - Type system representation
- `TypeSignature` - Function type signatures
- `WordDefinition` - User-defined word metadata
- `OrdinalValue` - Termination analysis values

**Recent Enhancements**:
- Full serde serialization support
- Session persistence compatibility
- Enhanced error types

---

### `src/parser.rs` - Syntax Analysis
```rust
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

pub enum Statement {
    Expression(Vec<Token>),
    TypeSignatureDecl { name: String, signature: TypeSignature },
    WordDefinition(WordDefinition),
    TypeDefinition(TypeDefinition),
    AxiomDeclaration { name: String, signature: TypeSignature },
}
```

**Responsibilities**:
- Token sequence parsing
- AST generation
- Syntax validation
- Statement classification

**Key Functions**:
- `parse_all()` - Parse complete input
- `parse_statement()` - Parse individual statements
- `parse_type_signature()` - Parse type declarations
- `parse_word_definition()` - Parse function definitions

**🚧 NEXT ENHANCEMENT TARGET**: Type inference integration

---

### `src/lexer.rs` - Tokenization
```rust
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    current_char: Option<char>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    Literal(Value), Word(String), QuoteStart, QuoteEnd, Comment(String)
}
```

**Responsibilities**:
- String to token conversion
- Comment handling
- Number and literal parsing
- Symbol recognition

**Key Functions**:
- `tokenize()` - Convert string to token stream
- `next_token()` - Get next token from input
- `read_number()` - Parse numeric literals
- `read_word()` - Parse identifiers and operators

---

### `src/core_lib.rs` - Built-in Library
```rust
pub struct CoreLibrary {
    core_words: HashMap<String, WordDefinition>,
    core_signatures: HashMap<String, TypeSignature>,
    word_docs: HashMap<String, String>,
}
```

**Responsibilities**:
- Built-in word definitions
- Core type signatures
- Documentation for built-in operations
- Standard library functionality

**Key Functions**:
- `new()` - Initialize core library
- `get_core_words()` - Access built-in words
- `define_core_word()` - Register built-in operations

**Built-in Categories**:
- Stack manipulation (dup, drop, swap, over, rot, nip, tuck)
- Arithmetic (+, -, *, /, mod, 1+, 1-)
- Comparison (=, <, >, <=, >=, <>)
- Logic (and, or, not)
- Control flow (if, when, unless, times)
- System (., .s, words, help, clear, depth, type-of)

---

### `src/ordinal.rs` - Termination Analysis
```rust
pub struct OrdinalVerifier {
    max_analysis_depth: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrdinalValue {
    Zero, Finite(u64), Omega, 
    OmegaPower(Box<OrdinalValue>),
    Sum(Vec<OrdinalValue>), 
    Product(Box<OrdinalValue>, Box<OrdinalValue>)
}
```

**Responsibilities**:
- Termination proof verification
- Ordinal arithmetic
- Recursive definition analysis
- Well-foundedness checking

**Key Functions**:
- `verify_termination()` - Check word termination
- `analyze_recursion()` - Analyze recursive calls
- `calculate_ordinal()` - Compute ordinal complexity

**Status**: Currently mock implementation, will be enhanced in Phase 3

## 🔄 Data Flow Architecture

### REPL Command Processing Flow
```
User Input
    ↓
.command? → REPL Command Handler → Execute REPL Command
    ↓                                       ↓
Regular Code → Parser → Statements → VM Execution → Stack Update
    ↓              ↓         ↓           ↓              ↓
Lexer → Tokens → AST → Type Check → Bytecode → Result
```

### Session Management Flow
```
.save command → Collect State → Serialize to JSON → Write to File
                    ↓
    [Stack, Types, Words, Settings] → SessionData → JSON → sessions/file.json

.load command → Read File → Deserialize JSON → Restore State
                    ↓
    sessions/file.json → JSON → SessionData → [Stack, Types, Words, Settings]
```

### Tracing and Performance Flow
```
Execute with .trace enabled:
    Token → Pre-execution State → Execute → Post-execution State → TraceEntry
              ↓                     ↓              ↓                  ↓
         Stack Snapshot → Timing → Stack Snapshot → Performance Metrics
                                      ↓
                              Store in execution_trace
```

## 🎯 Extension Points for New Features

### Adding New REPL Commands
1. Add command pattern in `handle_repl_command()`
2. Implement command handler function
3. Update `.help` documentation
4. Add to command completion if needed

### Adding New Types
1. Extend `Value` enum in `types.rs`
2. Extend `Type` enum for type system
3. Add serialization support
4. Update display implementations
5. Add VM operations for new type

### Adding New Built-in Words
1. Add to `CoreLibrary` initialization
2. Implement operation in VM
3. Add type signature
4. Add documentation
5. Update help system

### Adding Language Syntax
1. Extend `Token` enum in `lexer.rs`
2. Update lexer to recognize new syntax
3. Extend `Statement` enum in `parser.rs`
4. Add parsing logic
5. Update VM execution

## 🧪 Testing Architecture

### Current Testing Strategy
- **Manual Testing**: Through REPL commands and demo scripts
- **Integration Testing**: Full REPL command sequences
- **Performance Testing**: Using `.benchmark` command
- **Session Testing**: Save/load functionality

### Testing Files and Locations
- `demo_enhanced_repl.md` - Comprehensive feature demonstration
- `examples/enhanced_repl_demo.txt` - Command sequences
- Inline tests in source files (some modules have `#[cfg(test)]`)

### Adding Tests for New Features
1. Add unit tests in relevant module
2. Add integration test in demo scripts
3. Add performance benchmark if applicable
4. Update testing documentation

## 🔧 Development Workflow

### Making Changes
1. **Understand Impact**: Check dependencies between modules
2. **Plan Changes**: Consider backward compatibility
3. **Implement Incrementally**: Small, testable changes
4. **Test Thoroughly**: Use REPL commands and demos
5. **Update Documentation**: Keep guides current

### Common Patterns
- **Error Handling**: Use Result types, comprehensive error messages
- **Type Safety**: Leverage Rust's type system
- **Performance**: Measure with `.benchmark` command
- **Backward Compatibility**: Never break existing functionality

## 📊 Performance Characteristics

### Current Performance Profile
- **Token Execution**: ~1-10µs per operation
- **Type Checking**: ~1-5µs per type operation
- **Session Save/Load**: ~1-10ms depending on state size
- **Tracing Overhead**: ~2-5µs per traced operation

### Optimization Opportunities
- **Type Inference**: Pre-compute common type patterns
- **VM Optimization**: Bytecode compilation for complex words
- **Memory Management**: Pool allocation for frequently used values
- **Serialization**: Binary format for faster session save/load

## 🚀 Future Architecture Evolution

### Phase 2A: Type System Enhancement
- Add `src/type_inference.rs` module
- Extend parser for optional type signatures
- Enhance error reporting with type information

### Phase 2B: Syntax Improvements
- Pattern matching compiler
- Infix operator parser
- Collection literal syntax

### Phase 3: Module System
- Module definition and loading
- Namespace resolution
- Import/export mechanisms

### Phase 4: Network Architecture
- Hypervisor communication layer
- Cell registration and management
- Distributed verification system

---

**Key Principle**: Each component has a single responsibility and clear interfaces, making the system maintainable and extensible while preserving C∀O's mathematical foundations.

*"Understanding the architecture is the foundation for contributing effectively to C∀O's evolution."*