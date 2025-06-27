# Cellular Automata in Chronos REPL

The Chronos REPL includes a powerful cellular automata system for exploring elementary cellular automata and eventually more complex systems like Codd's cellular automata. This system provides both text-based and interactive graphical interfaces for studying emergent computational patterns.

## Overview

Cellular automata are discrete mathematical models consisting of a grid of cells, each in one of a finite number of states. The state of each cell evolves according to a fixed rule based on the states of neighboring cells. Elementary cellular automata are one-dimensional systems where each cell has two possible states (0 or 1) and the rule depends on the cell and its two immediate neighbors.

## Getting Started

### Basic Commands

The cellular automata system is integrated into the REPL command system. All CA commands start with `.ca`:

```cao
C∀O> .ca-rules                    # List famous rules
C∀O> .ca-rule 30                  # Show Rule 30 truth table
C∀O> .ca-simple 30 20             # Run Rule 30 for 20 generations (text)
C∀O> .ca 30                       # Interactive Rule 30 environment
C∀O> .ca 90 "111"                 # Rule 90 with custom initial pattern
```

### Famous Rules

Several cellular automaton rules are particularly interesting:

- **Rule 30**: Produces chaotic, seemingly random patterns despite deterministic rules
- **Rule 90**: Generates the famous Sierpinski triangle fractal pattern
- **Rule 110**: Proven to be Turing complete, capable of universal computation
- **Rule 150**: Implements XOR logic, creating symmetric patterns
- **Rule 184**: Models traffic flow dynamics

## Command Reference

### `.ca-rules`
Lists all famous cellular automaton rules with descriptions.

```cao
C∀O> .ca-rules
Famous Cellular Automaton Rules:

Rule 30: Rule 30 (Chaotic)
Rule 90: Rule 90 (Sierpinski Triangle)
Rule 110: Rule 110 (Turing Complete)
Rule 150: Rule 150 (XOR)
Rule 184: Rule 184 (Traffic)
```

### `.ca-rule <number>`
Shows the truth table for a specific rule number (0-255).

```cao
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

### `.ca-simple <rule> <generations> [pattern]`
Runs a cellular automaton in text mode and displays the evolution.

```cao
C∀O> .ca-simple 90 15
  0:                    █
  1:                   █ █
  2:                  █   █
  3:                 █ █ █ █
  4:                █       █
  5:               █ █     █ █
  6:              █   █   █   █
  7:             █ █ █ █ █ █ █ █
  ...
```

With custom pattern:
```cao
C∀O> .ca-simple 150 10 "111"
  0:                 ███
  1:                █   █
  2:               █ █ █ █
  3:              █       █
  4:             █ █     █ █
  ...
```

### `.ca <rule> [pattern]`
Launches the interactive cellular automaton environment.

```cao
C∀O> .ca 30                       # Rule 30 with single center seed
C∀O> .ca 110 "000001000000"       # Rule 110 with custom pattern
```

## Interactive Environment

The interactive CA environment provides real-time visualization and control:

### Controls

- **Space**: Play/Pause evolution
- **s**: Single step (when paused)
- **r**: Reset to initial state
- **a**: Toggle auto-evolution mode
- **h**: Toggle help display
- **q**: Quit

### Evolution Controls

- **1**: Evolve 1 generation
- **5**: Evolve 5 generations
- **F1**: Evolve 10 generations
- **F2**: Evolve 50 generations
- **F3**: Evolve 100 generations

### Display Controls

- **c**: Toggle colors
- **+/-**: Speed up/slow down auto-evolution

### Interface Layout

The interactive environment displays:

1. **Status Bar**: Current rule, generation, active cell count, and status
2. **Evolution Display**: Visual history of generations with each row representing one generation
3. **Info Panel**: Rule description, timing information, and current settings
4. **Help Panel**: Available controls (toggle with 'h')

## Pattern Specification

When specifying initial patterns, you can use several formats:

- `"1"` or `"0"`: Single cell states
- `"111"`: Multiple adjacent active cells
- `"█"` or `"#"`: Alternative symbols for active cells
- `" "` or `"."` or `"-"`: Symbols for inactive cells

Examples:
```cao
.ca 30 "1"                        # Single active cell
.ca 90 "111"                      # Three adjacent active cells
.ca 110 "█ █ █"                   # Three separated active cells
.ca 150 "11011"                   # Complex pattern
```

## Understanding Elementary Rules

Elementary cellular automaton rules are numbered 0-255 based on their behavior. Each rule defines what happens to a cell based on its current state and the states of its immediate neighbors.

### Rule Encoding

The rule number is a binary encoding of the lookup table:

```
Neighborhood: 111 110 101 100 011 010 001 000
Rule 30:        0   0   0   1   1   1   1   0
Binary:      00011110 = 30 in decimal
```

### Rule Categories

- **Class 1**: Evolution leads to homogeneous state (all 0 or all 1)
- **Class 2**: Evolution leads to simple periodic patterns
- **Class 3**: Chaotic, aperiodic patterns (like Rule 30)
- **Class 4**: Complex, localized structures (like Rule 110)

## Programming Integration

The cellular automata system is designed to integrate with the Chronos language. You can use CA results in your computations:

```cao
# Future integration examples (planned)
C∀O> : rule30-gen 30 ca-step ;     # Define word for Rule 30 evolution
C∀O> : sierpinski 90 ca-step ;     # Define word for Sierpinski generation
C∀O> 5 rule30-gen                  # Apply Rule 30 for 5 generations
```

## Advanced Features

### Configuration

The CA system supports various configuration options:

- Width: Number of cells (default: 79 for terminal compatibility)
- History: Number of generations to keep in memory
- Auto-evolution: Automatic stepping with configurable delay
- Colors: Enhanced visualization with color coding

### Future Extensions

The system is designed to be extensible for more complex cellular automata:

- **2D Cellular Automata**: Conway's Game of Life, Codd's CA
- **Multi-state CA**: Rules with more than two states per cell
- **Probabilistic CA**: Rules with random elements
- **Quantum CA**: Quantum cellular automata models

## Examples and Experiments

### Exploring Chaos (Rule 30)
```cao
C∀O> .ca 30
# Watch how a simple initial condition creates complex, unpredictable patterns
# Rule 30 is used in some random number generators!
```

### Fractals (Rule 90)
```cao
C∀O> .ca-simple 90 20
# Observe the emergence of the Sierpinski triangle
# Perfect mathematical beauty from simple rules
```

### Universal Computation (Rule 110)
```cao
C∀O> .ca 110 "000000000000000100000000000000"
# Rule 110 can simulate any computer program
# Look for gliders and other complex structures
```

### Traffic Modeling (Rule 184)
```cao
C∀O> .ca 184 "1101001100101"
# Models traffic flow with cars (1) and empty spaces (0)
# Shows how traffic jams form and dissolve
```

## Tips for Exploration

1. **Start Simple**: Begin with famous rules to understand basic behaviors
2. **Vary Initial Conditions**: Same rule, different starting patterns = different outcomes
3. **Look for Patterns**: Many rules create periodic or eventually periodic behavior
4. **Study Boundaries**: How do patterns interact when they meet?
5. **Experiment**: Try random rule numbers - you might discover something interesting!

## Mathematical Insights

Cellular automata demonstrate several important computational and mathematical concepts:

- **Emergence**: Complex behavior arising from simple rules
- **Universality**: Some CA can simulate any computation
- **Entropy**: How randomness and order interact
- **Self-Organization**: Pattern formation without external control
- **Fractal Geometry**: Self-similar structures across scales

## References and Further Reading

- Stephen Wolfram's "A New Kind of Science"
- Matthew Cook's proof of Rule 110 universality
- John Conway's Game of Life
- Codd's self-replicating cellular automata
- Quantum cellular automata research

The cellular automata system in Chronos provides a playground for exploring these deep mathematical and computational concepts interactively, making abstract theory concrete and visual.