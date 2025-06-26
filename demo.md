# C∀O (Kao) Demo - Categorical ∀xiomatic Ordinal Programming Language

This demo showcases the initial implementation of C∀O, an evolving axiomatic programming language that combines Category Theory, Ordinal Analysis, and concatenative programming.

## Running the Demo

```bash
cd chronos
cargo run
```

## Basic Arithmetic

Try these basic arithmetic operations:

```
C∀O> 3 4 +
C∀O> .
7
C∀O> 5 2 *
C∀O> .
10
C∀O> 15 3 /
C∀O> .
5
```

## Stack Manipulation

C∀O is a stack-based language. Try these stack operations:

```
C∀O> 5
C∀O> dup
C∀O> .s
<2> 5 5
C∀O> *
C∀O> .
25
C∀O> 10 20
C∀O> swap
C∀O> .s
<2> 20 10
```

## Defining Words

Define your own functions using type signatures and word definitions:

```
C∀O> :: square ( Nat -> Nat ) ;
Type signature declared for 'square'
C∀O> : square dup * ;
Defined word 'square'
C∀O> 7 square .
49
```

More complex example:

```
C∀O> :: cube ( Nat -> Nat ) ;
Type signature declared for 'cube'
C∀O> : cube dup dup * * ;
Defined word 'cube'
C∀O> 4 cube .
64
```

## Conditionals with Quotations

C∀O supports conditional execution using quotations (code blocks):

```
C∀O> :: abs ( Nat -> Nat ) ;
Type signature declared for 'abs'
C∀O> : abs dup 0 < [ 0 swap - ] [ ] if ;
Defined word 'abs'
C∀O> 5 abs .
5
```

## Boolean Operations

```
C∀O> true false and .
false
C∀O> true false or .
true
C∀O> true not .
false
```

## Comparison Operations

```
C∀O> 5 3 > .
true
C∀O> 2 7 < .
true
C∀O> 4 4 = .
true
```

## REPL Commands

Use dot commands to interact with the REPL:

```
C∀O> .s          # Show stack contents
C∀O> .clear      # Clear the stack
C∀O> .words      # List all available words
C∀O> .help       # Show help information
C∀O> .about      # About C∀O
```

## Example: Fibonacci Function

Here's a more complex example showing recursive function definition:

```
C∀O> :: fib ( Nat -> Nat ) ;
Type signature declared for 'fib'
C∀O> : fib dup 2 < [ ] [ dup 1 - fib swap 2 - fib + ] if ;
Defined word 'fib'
C∀O> 8 fib .
21
```

## Key Features Demonstrated

1. **Concatenative Programming**: Postfix notation with stack-based execution
2. **Type Signatures**: Explicit type declarations using `::` syntax
3. **Ordinal Analysis**: Mock verification ensures termination (Phase 1)
4. **Category Theory**: Types as objects, functions as morphisms
5. **Interactive Development**: REPL with immediate feedback

## What's Next?

This is Phase 1 of the Chronos project. Future phases will add:

- **Phase 2**: Network architecture with Hypervisor and Cells
- **Phase 3**: True ordinal verification using proof theory
- **Phase 4**: Self-evolution and metaprogramming capabilities

## Language Philosophy

C∀O embodies several key principles:

- **Mathematical Foundation**: Built on solid Category Theory and Ordinal Analysis
- **Collaborative Evolution**: Language grows through community verification
- **Provable Correctness**: All definitions must pass termination proofs
- **Interactive Development**: Immediate feedback promotes exploratory programming

## Exit

To exit the C∀O REPL:

```
C∀O> quit
Farewell! May your axioms remain consistent.
```

---

*"In the beginning was the Word, and the Word was with Math, and the Word was Math."*