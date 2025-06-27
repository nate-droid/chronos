# Cellular Automata Demo for Chronos REPL

This document demonstrates the cellular automata system that has been successfully integrated into the Chronos REPL. The system provides an interactive way to explore elementary cellular automata without cluttering the main runtime.

## What We've Built

### 1. **Non-Intrusive Design**
- CA functionality is isolated in the REPL layer
- Core Chronos runtime remains clean and focused
- Can be disabled/removed without affecting language functionality

### 2. **Text-Based Visualization**
- Beautiful Unicode visualization in terminal
- Works with any terminal that supports Unicode
- No external graphics dependencies

### 3. **Interactive and Batch Modes**
- Quick text output for exploration
- Interactive mode for step-by-step analysis
- Pattern specification support

## Quick Demo Commands

### List Famous Rules
```
C∀O> .ca-rules
Famous Cellular Automaton Rules:

Rule 30: Rule 30 (Chaotic)
Rule 90: Rule 90 (Sierpinski Triangle) 
Rule 110: Rule 110 (Turing Complete)
Rule 150: Rule 150 (XOR)
Rule 184: Rule 184 (Traffic)

Use '.ca <rule>' to run interactively or '.ca-simple <rule> <generations>' for text output
```

### Show Rule Truth Table
```
C∀O> .ca-rule 30
Elementary Cellular Automaton Rule 30
Rule 30 (Chaotic)

Pattern -> Output
111 -> 0
110 -> 0
101 -> 0
100 -> 1
011 -> 1
010 -> 1
001 -> 1
000 -> 0
```

### Generate Sierpinski Triangle (Rule 90)
```
C∀O> .ca-simple 90 8
Running Elementary CA Rule 90 for 8 generations
  0: ·······································█·······································
  1: ······