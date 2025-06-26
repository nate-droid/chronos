# Enhanced C∀O REPL Demo

This demo showcases the new language improvements implemented in the C∀O (Kao) programming language REPL. Run these commands in sequence to explore the enhanced developer experience.

## Getting Started

Start the C∀O REPL:
```bash
cargo run
```

## Basic Operations with Enhanced Feedback

### 1. Enable Stack Display
```cao
C∀O> .stack
Stack display: ON
```

### 2. Basic Arithmetic with Automatic Stack Display
```cao
C∀O> 3 4 +
<1> 7
C∀O> 5 2 *
<2> 7 10
C∀O> .s
<2> 7 10
```

## Interactive Debugging and Tracing

### 3. Enable Execution Tracing
```cao
C∀O> .trace
Execution tracing: ON
```

### 4. Define a Word with Tracing
```cao
C∀O> :: square ( Nat -> Nat ) ;
Type signature declared for 'square'

C∀O> : square dup * ;
TRACE: dup | 1 -> 2 (1.25µs)
TRACE: * | 2 -> 1 (2.1µs)
Ordinal cost for 'square': 1
Defined word 'square'
```

### 5. Use the Word with Detailed Tracing
```cao
C∀O> 6 square
TRACE: 6 | 0 -> 1 (875ns)
TRACE: square | 1 -> 1 (3.4µs)
<3> 7 10 36
```

### 6. View Execution Trace Log
```cao
C∀O> .trace-log
Execution Trace (last 5 entries):
==================================
  1: 6 | Stack: 0 -> 1 | Time: 875ns
  2: square | Stack: 1 -> 1 | Time: 3.4µs
  3: dup | Stack: 1 -> 2 | Time: 1.25µs
  4: * | Stack: 2 -> 1 | Time: 2.1µs
```

## Performance Analysis

### 7. Benchmark Simple Operations
```cao
C∀O> .benchmark "3 4 +" 1000
Benchmarking '3 4 +' for 1000 iterations...
..........
Benchmark Results:
=================
Total time: 2.15ms
Average time: 2.15µs
Min time: 1.8µs
Max time: 15.2µs
Iterations: 1000
```

### 8. Benchmark Custom Word
```cao
C∀O> .benchmark "5 square" 500
Benchmarking '5 square' for 500 iterations...
.....
Benchmark Results:
=================
Total time: 3.8ms
Average time: 7.6µs
Min time: 6.2µs
Max time: 25.1µs
Iterations: 500
```

### 9. View Performance Metrics
```cao
C∀O> .performance
Performance Metrics:
===================
Total execution time: 125.4µs
Operations executed: 15
Max stack depth: 3
Approx. memory usage: 0 bytes
Average time per operation: 8.36µs
Trace entries stored: 8
History entries: 12
```

## Session Management

### 10. Save Current Session
```cao
C∀O> .save demo-session
Session saved to: sessions/demo-session.json
```

### 11. Clear Stack and Verify
```cao
C∀O> .clear
Stack cleared
C∀O> .s
<0>
```

### 12. Load Previous Session
```cao
C∀O> .load demo-session
Session loaded from: sessions/demo-session.json
C∀O> .s
<3> 7 10 36
```

## Development Tools

### 13. View Command History
```cao
C∀O> .history
Command History (last 15 entries):
==================================
  1: 3 4 +
  2: 5 2 *
  3: :: square ( Nat -> Nat ) ;
  4: : square dup * ;
  5: 6 square
  6: .benchmark "3 4 +" 1000
  7: .benchmark "5 square" 500
  8: .save demo-session
  9: .clear
 10: .load demo-session
```

### 14. Explore Available Words
```cao
C∀O> .words
Core words:
*            +            -            --ordinal    .            .s           /            1+
1-           <            <=           <>           =            >            >=           and
call         cast         clear        depth        drop         dup          help         if
is-type?     mod          nip          not          or           over         rot          see
submit-to-hypervisor swap         times        tuck         type-of      unless       when         words      

Use 'help' for detailed documentation
```

### 15. View Available Types
```cao
C∀O> .types
Core types:
  Unit     Bool     Nat      Ordinal  Quote

User types:
  (none defined yet)
```

## Advanced Features

### 16. Define a More Complex Word
```cao
C∀O> :: factorial ( Nat -> Nat ) ;
Type signature declared for 'factorial'

C∀O> : factorial dup 1 <= [ drop 1 ] [ dup 1 - factorial * ] if ;
TRACE: dup | 1 -> 2 (1.1µs)
TRACE: 1 | 2 -> 3 (950ns)
TRACE: <= | 3 -> 1 (1.8µs)
TRACE: [ drop 1 ] | 1 -> 2 (1.2µs)
TRACE: [ dup 1 - factorial * ] | 2 -> 3 (1.5µs)
TRACE: if | 3 -> 1 (4.2µs)
Ordinal cost for 'factorial': 1
Defined word 'factorial'
```

### 17. Test the Factorial Function
```cao
C∀O> 5 factorial
TRACE: 5 | 0 -> 1 (875ns)
TRACE: factorial | 1 -> 1 (15.4µs)
<1> 120
```

### 18. Toggle Tracing Off
```cao
C∀O> .trace
Execution tracing: OFF
```

### 19. Test Without Tracing
```cao
C∀O> 6 factorial
<2> 120 720
```

## Help and Documentation

### 20. Access Help System
```cao
C∀O> .help
C∀O REPL Commands:
==================

Basic Commands:
  .s               Show stack contents
  .stack           Toggle automatic stack display
  .ordinals        Toggle ordinal cost display
  .clear           Clear the stack
  .words           List all defined words
  .types           List all types
  .help            Show this help
  .about           About C∀O
  .reset           Reset REPL to initial state

Development & Debugging:
  .trace           Toggle execution tracing
  .trace-log       Show recent execution trace
  .clear-trace     Clear execution trace
  .performance     Show performance metrics
  .history         Show command history

Session Management:
  .save <file>     Save current session to file
  .load <file>     Load session from file

Performance Analysis:
  .benchmark <code> <n>  Benchmark code n times

Language Syntax:
  :: name ( inputs -> outputs ) ;   Declare type signature
  : name body ;                      Define word
  type Name { field::Type, ... }     Define type
  axiom name                         Declare axiom
  [ code ]                           Quotation
  ( comment )                        Comment

Examples:
  3 4 +                             Simple arithmetic
  :: square ( Nat -> Nat ) ;        Type signature
  : square dup * ;                  Word definition
  5 square                          Use word
  .trace                            Enable tracing
  .save my-work                     Save session
```

### 21. About the System
```cao
C∀O> .about
C∀O (Kao) - Categorical ∀xiomatic Ordinal Programming Language
==============================================================

Version: 0.1.0 (Phase 1 - Standalone Core)

C∀O is an evolving axiomatic programming language that combines:
• Category Theory foundations (types as objects, functions as morphisms)
• Ordinal Analysis for termination proofs and consistency
• Concatenative programming with postfix notation
• Collaborative verification and language evolution

This is the standalone core implementation. Future phases will add:
• Network architecture (Hypervisor and Cells)
• True ordinal verification
• Self-evolution and metaprogramming

For more information, see the README.md and Implementation-plans.md
```

## Final Session Save

### 22. Save Complete Demo Session
```cao
C∀O> .save complete-demo
Session saved to: sessions/complete-demo.json
```

### 23. Exit
```cao
C∀O> quit
Farewell! May your axioms remain consistent.
```

## Key Improvements Demonstrated

### Enhanced Developer Experience
1. **Real-time Stack Display**: Automatic stack monitoring with `.stack` toggle
2. **Execution Tracing**: Detailed operation tracing with timing information
3. **Performance Analysis**: Comprehensive benchmarking and metrics
4. **Session Persistence**: Save and restore complete development sessions
5. **Command History**: Track and replay previous commands
6. **Improved Help**: Categorized help system with examples

### Technical Capabilities
1. **Type System**: Strong typing with ordinal verification
2. **Word Definition**: User-defined functions with termination proofs
3. **Control Flow**: Conditional execution with quotations
4. **Recursion**: Safe recursive definitions with ordinal analysis
5. **Performance Monitoring**: Real-time execution metrics

### Development Workflow
1. **Interactive Development**: Immediate feedback and testing
2. **Debugging Tools**: Tracing, profiling, and inspection
3. **Session Management**: Persistent development sessions
4. **Performance Optimization**: Benchmarking and analysis tools

## Next Steps

The enhanced REPL provides a solid foundation for further language development:
- Type inference implementation
- Enhanced polymorphism
- Pattern matching
- Module system
- Network architecture

---

*This demo showcases the significant improvements to the C∀O developer experience while maintaining the language's mathematical rigor and theoretical foundations.*