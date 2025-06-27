# Chronos (C∀O) Language Examples

This directory contains a comprehensive set of examples demonstrating the Chronos programming language capabilities. These examples are designed to teach you the fundamentals of C∀O and showcase its unique features.

## Quick Start

To run any example file:

```bash
# From the chronos-repl directory
cargo run --bin chronos-repl < examples/filename.cao
```

Or start the interactive REPL and load examples manually:

```bash
cargo run --bin chronos-repl
# Then in the REPL, you can type the contents or use .load if supported
```

## Example Files Overview

### 00_overview.cao - **Start Here!**
A comprehensive introduction to the Chronos language with:
- Language fundamentals and philosophy
- Quick start tutorial
- Index of all example files
- Interactive demo
- Learning path recommendations

### 01_basic_arithmetic.cao - **Beginner Level**
Foundation concepts including:
- Basic arithmetic operations (+, -, *)
- Stack manipulation (dup, drop, swap, over, rot)
- Simple word definitions with type signatures
- Stack inspection commands (.s, .clear)
- Building blocks for all C∀O programs

### 02_conditionals.cao - **Beginner to Intermediate**
Control flow and decision making:
- Boolean values and operations
- Conditional execution with `if`
- Comparison operators (=, <, >)
- Recursive function definitions
- Pattern matching concepts

### 03_algorithms.cao - **Intermediate to Advanced**
Complex algorithmic patterns:
- Mathematical sequences (Fibonacci, factorial)
- Number theory algorithms (GCD, prime testing)
- Recursive and iterative approaches
- Performance considerations
- Advanced mathematical computations

### 04_data_types.cao - **Intermediate**
Working with structured data:
- Custom type definitions
- Algebraic data types (Option, Result)
- Composite data structures
- Type validation and checking
- Real-world data modeling

### 05_repl_features.cao - **All Levels**
Interactive development tools:
- REPL commands and debugging
- Execution tracing and profiling
- Session management
- Performance monitoring
- Development workflow optimization

### 06_real_world_app.cao - **Advanced**
Complete application example:
- Scientific calculator implementation
- Unit conversion system
- Financial calculations
- Physics and engineering formulas
- Application architecture patterns

## Learning Path

### For Complete Beginners
1. **Start with**: `00_overview.cao` - Get the big picture
2. **Practice with**: `01_basic_arithmetic.cao` - Learn the basics
3. **Experiment**: Try the examples interactively in the REPL
4. **Understand**: Stack-based thinking and postfix notation

### For Programmers New to Concatenative Languages
1. **Quick start**: `00_overview.cao` - Understand the paradigm
2. **Core concepts**: `01_basic_arithmetic.cao` - Master stack operations
3. **Control flow**: `02_conditionals.cao` - Learn C∀O conditionals
4. **Data structures**: `04_data_types.cao` - Type system exploration

### For Advanced Users
1. **Algorithms**: `03_algorithms.cao` - Complex problem solving
2. **Tooling**: `05_repl_features.cao` - Master the development environment
3. **Applications**: `06_real_world_app.cao` - Complete project patterns

## Key Concepts Demonstrated

### Stack-Based Computing
- Postfix notation: `2 3 +` instead of `2 + 3`
- Stack manipulation: `dup`, `swap`, `drop`, `over`, `rot`
- Data flow visualization with `.s` command

### Type System
- Explicit type signatures: `:: function_name ( input -> output ) ;`
- Type inference and checking
- Custom type definitions with `type` keyword

### Functional Programming
- Immutable data structures
- Function composition
- Higher-order functions with quotes `[ code ]`

### Mathematical Foundations
- Category theory principles
- Ordinal analysis for termination guarantees
- Provably correct programs

## Interactive Commands Reference

Essential REPL commands used in the examples:

```
.s              Show stack contents
.clear          Clear the stack
.words          List all defined words
.types          List all defined types
.help           Show help information
.trace on/off   Enable/disable execution tracing
.save filename  Save current session
.load filename  Load saved session
.reset          Reset to initial state
```

## Language Syntax Quick Reference

```cao
( This is a comment )

:: word_name ( InputType -> OutputType ) ;  ( Type signature )
: word_name definition ;                     ( Implementation )

( Basic operations )
2 3 +          ( Addition: 2 + 3 = 5 )
dup            ( Duplicate top stack element )
swap           ( Exchange top two elements )

( Conditionals )
condition [ true-branch ] [ false-branch ] if

( Custom types )
type Point { x::Nat, y::Nat }
```

## Tips for Success

1. **Think in terms of data flow** - Visualize values moving through a stack
2. **Use .s frequently** - Always check what's on the stack
3. **Start simple** - Build complexity gradually from basic operations
4. **Practice stack manipulation** - Master dup, swap, drop, over, rot
5. **Provide type signatures** - Always declare types for clarity
6. **Test interactively** - Experiment with functions before saving
7. **Use tracing** - Enable `.trace on` to understand execution
8. **Save your work** - Use `.save` to preserve interesting sessions

## Common Patterns

### Stack Juggling
```cao
dup          ( a -> a a )
swap         ( a b -> b a )
over         ( a b -> a b a )
rot          ( a b c -> b c a )
```

### Conditional Execution
```cao
( Simple conditional )
x 0 > [ positive ] [ non-positive ] if

( Recursive pattern )
: factorial dup 1 = [ ] [ dup 1 - factorial * ] if ;
```

### Type-Safe Development
```cao
:: square ( Nat -> Nat ) ;
: square dup * ;
```

## Troubleshooting

### Common Issues
1. **Stack underflow** - Not enough values on stack for operation
2. **Type mismatches** - Ensure type signatures match implementation
3. **Infinite recursion** - Check termination conditions in recursive functions

### Getting Help
- Use `.help` in the REPL for command information
- Use `.trace on` to see execution step-by-step
- Check the stack with `.s` before and after operations
- Refer to the main Chronos documentation

## Contributing

These examples are part of the Chronos language ecosystem. Contributions are welcome:

1. **Improve existing examples** - Add clarity, fix issues
2. **Add new examples** - Demonstrate additional language features
3. **Update documentation** - Keep information current
4. **Share patterns** - Contribute useful programming patterns

## Next Steps

After working through these examples:

1. **Explore the main Chronos repository** - Understand the full ecosystem
2. **Join the community** - Participate in language development
3. **Build your own applications** - Apply what you've learned
4. **Contribute to the language** - Help expand the core library

Happy coding in Chronos! The future of provably correct software awaits.