# Testing Guide for Chronos Project

This document explains how to run tests across the entire Chronos project workspace.

## Quick Start

### Option 1: Using Make (Recommended)
```bash
# Run all tests across the entire project
make test         # Runs unit tests + demo verification

# Quick testing (minimal output)
make test-quick   # Fast unit tests only

# Other useful commands
make test-core    # Test only chronos-core
make test-repl    # Test only chronos-repl  
make demo         # Run the enhanced display demo
make dev-check    # Format, lint, and quick test
```

### Option 2: Using Cargo Directly
```bash
# Test individual components (recommended)
cd chronos-core && cargo test --lib
cd chronos-repl && cargo test --lib

# Run the enhanced display demo
cd chronos-repl && cargo run --example demo_display

# Quick workspace test (may have warnings)
cargo test --workspace --lib
```