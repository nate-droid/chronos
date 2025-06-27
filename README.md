# Chronos: An Evolving Axiomatic Programming Language

**C∀O (Kao)** - *Categorical ∀xiomatic Ordinal*

## Vision

Chronos was originally designed as an attempt to create a new sort of environment to explore topics like evolutionary 
algorithms, metaprogramming, and grammar design that would potentially lend itself for AI agents to digest and parse 
better. This will be built on the foundation of topics like Category Theory, Ordinal Analysis, and 
concatenative programming paradigms.


## Core Principles

- **Categorical Foundation**: Types are objects, functions are morphisms
- **Metaprogramming**: Programs can manipulate and evolve themselves
- **Ordinal Analysis**: Every definition has a proof of termination and consistency
- **Ordinal Verification**: Guaranteed termination and consistency through proof theory
- **Concatenative Core**: Stack-based, postfix syntax promoting interactive development
- **Axiomatic Extensibility**: User-defined types and axioms with proof verification

## Quick Start

TODO!

## Crate layout

- `chronos-core` - The core library containing the C∀O language implementation, including the parser, stack machine, and core types/words.
- `chronos-repl` - The REPL (Read-Eval-Print Loop) for interactive development, allowing users to enter C∀O code and see results immediately.

## The C∀O Language Specification

### Execution Model

C∀O is a concatenative (postfix) language operating on a data stack. Words are space-delimited and executed left-to-right.

```
3 4 +    ( pushes 3, then 4, then executes +, resulting in 7 )
```

### Syntax Elements

#### Comments
```
( This is a comment )
( Comments are enclosed in parentheses )
```

#### Word Definition
```
: square ( n -> n*n ) dup * ;
```

#### Type Signatures
Every user-defined word requires an explicit type signature:
```
:: square ( Nat -> Nat ) ;
: square dup * ;
```

#### Type Definition
New types are defined using the `type` keyword:
```
:: Point ( Nat Nat -> Point ) ;
type Point { x::Nat, y::Nat }
```

#### Axiom Definition
Unproven assertions accepted as true:
```
:: oracle ( -> Nat ) ;
axiom oracle
```

### Core Types

| Type | Description | Examples |
|------|-------------|----------|
| `Unit` | Terminal object | `()` |
| `Bool` | Boolean values | `true`, `false` |
| `Nat` | Natural numbers | `0`, `1`, `2`, ... |
| `Ordinal` | Proof-theoretic ordinals | Used internally by system |
| `Quote` | Code blocks as data | `[ dup * ]` |

### Core Words

#### Stack Manipulation
- `dup` - Duplicate top stack element
- `drop` - Remove top stack element
- `swap` - Exchange top two elements
- `over` - Copy second element to top
- `rot` - Rotate three elements

#### Control Flow
- `if` - Conditional execution (consumes Bool and two Quotes)

#### Arithmetic
- `+`, `-`, `*` - Basic arithmetic for Nat

#### System Words
- `--ordinal` - Calculate ordinal cost of definition
- `submit-to-hypervisor` - Submit definition for global verification

### Example Programs

#### Basic Arithmetic
```
: double ( n -> 2n ) 2 * ;
:: double ( Nat -> Nat ) ;

5 double    ( Result: 10 )
```

#### Conditional Logic
```
: abs ( n -> |n| )
  dup 0 < [ 0 swap - ] [ ] if ;
:: abs ( Int -> Nat ) ;
```

#### Working with Types
```
:: Point ( Nat Nat -> Point ) ;
type Point { x::Nat, y::Nat }

: distance-squared ( p1 p2 -> dist² )
  ( Extract coordinates and calculate (x2-x1)² + (y2-y1)² )
  ...implementation... ;
:: distance-squared ( Point Point -> Nat ) ;
```

## System Architecture

### The Ordinal Hypervisor (`chronosd`)

The central authority ensuring global consistency:

- **Master Theory Database**: Canonical store of verified definitions
- **Validation Engine**: Ordinal Analysis proof checker

### Evolutionary Cells (`kao-shell`)

Interactive development environments:

- **C∀O Runtime**: Parser, stack machine, core library
- **Local Theory State**: Cached master theory + local definitions

### Workflow Example

1. **Bootstrap**: Cell registers with hypervisor, downloads master theory
2. **Local Development**: User defines new functions locally
   ```
   : fibonacci ( n -> fib(n) )
     dup 2 < [ ] [ dup 1 - fibonacci swap 2 - fibonacci + ] if ;
   :: fibonacci ( Nat -> Nat ) ;
   ```
3. **Verification**: Local ordinal analysis ensures termination
4. **Submission**:
   ```
   [ :: fibonacci ( Nat -> Nat ) ;
     : fibonacci dup 2 < [ ] [ dup 1 - fibonacci swap 2 - fibonacci + ] if ;
   ] submit-to-hypervisor
   ```
5. **Global Integration**: Hypervisor validates and broadcasts to all cells

## Language Philosophy

### Mathematical Foundations

C∀O is built on solid mathematical principles:

- **Category Theory**: Provides the structural foundation for types and functions
- **Ordinal Analysis**: Ensures all programs terminate and the system remains consistent
- **Constructive Logic**: Every proof corresponds to a computable function

### Evolutionary Properties

- **Collaborative Growth**: The language evolves through community contributions
- **Verified Extensions**: All additions must pass rigorous proof requirements
- **Backward Compatibility**: New definitions cannot break existing theory

### Interactive Development

The concatenative nature promotes:

- **Immediate Feedback**: Every word can be tested interactively
- **Compositional Thinking**: Complex programs built from simple, verified components
- **Bottom-up Design**: Start with primitives, build complexity gradually

## Implementation Roadmap

### Phase 1: Standalone Core (`kao-core`)
- [x] Basic project structure
- [ ] Postfix parser implementation
- [ ] Stack machine VM
- [ ] Genesis axioms (core types/words)
- [ ] Type checker for signatures
- [ ] Mock ordinal verifier

### Phase 2: Network Architecture
- [ ] Hypervisor (`chronosd`) implementation
- [ ] Cell communication integration
- [ ] Definition submission system

### Phase 3: Ordinal Verification
- [ ] Ordinal notation system
- [ ] Termination proof algorithm
- [ ] True ordinal verifier
- [ ] Integration with core system

### Phase 4: Self-Evolution
- [ ] First-class code manipulation
- [ ] Metaprogramming capabilities
- [ ] Automated evolution tools
- [ ] Goal-oriented cells

## Getting Started

### Prerequisites
- Rust 2024 edition
- Basic understanding of stack-based languages

### Installation
```bash
git clone <repository-url>
cd chronos
cargo build
```

### Running Your First C∀O Session
```bash
cargo run
```

### Hello World Example
```
: hello ( -> ) "Hello, Chronos!" print ;
:: hello ( -> Unit ) ;
hello
```

## Contributing

Chronos thrives on community contributions. To contribute:

1. Study the ordinal verification requirements
2. Propose new axioms with mathematical justification
3. Submit definitions through the verification process
4. Help expand the core library

## License

Apache License 2.0

## Inspiration Topics

- Category Theory for Programmers
- Proof Theory and Ordinal Analysis
- Concatenative Programming Languages
- The Lean Theorem Prover

---

*"In the beginning was the Word, and the Word was with Math, and the Word was Math."*
