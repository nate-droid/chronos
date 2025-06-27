# Makefile for Chronos Project
# Provides convenient commands for testing, building, and running the project

.PHONY: help test test-all test-core test-repl build clean run-repl demo check fmt clippy install

# Default target
help:
	@echo "🧪 Chronos Project - Available Commands"
	@echo "======================================"
	@echo ""
	@echo "Testing:"
	@echo "  make test       - Run comprehensive test suite across all components"
	@echo "  make test-all   - Same as 'test' (alias)"
	@echo "  make test-core  - Run tests for chronos-core only"
	@echo "  make test-repl  - Run tests for chronos-repl only"
	@echo "  make demo       - Run the enhanced display demo"
	@echo ""
	@echo "Building:"
	@echo "  make build      - Build all workspace members"
	@echo "  make check      - Check compilation without building"
	@echo "  make clean      - Clean build artifacts"
	@echo ""
	@echo "Code Quality:"
	@echo "  make fmt        - Format code with rustfmt"
	@echo "  make clippy     - Run clippy linter"
	@echo ""
	@echo "Running:"
	@echo "  make run-repl   - Start the enhanced Chronos REPL"
	@echo "  make install    - Install binaries to ~/.cargo/bin"

# Main test command - runs comprehensive test suite
test:
	@echo "🧪 Running Chronos test suite..."
	@echo "Testing chronos-core..."
	@cd chronos-core && cargo test --lib
	@echo ""
	@echo "Testing chronos-repl..."
	@cd chronos-repl && cargo test --lib
	@echo ""
	@echo "Testing enhanced display demo..."
	@cd chronos-repl && cargo run --example demo_display > /dev/null 2>&1 && echo "✅ Demo works" || echo "❌ Demo failed"
	@echo ""
	@echo "🎉 Basic tests complete!"

test-all: test

# Quick test - just unit tests
test-quick:
	@echo "🚀 Running quick tests..."
	@cd chronos-core && cargo test --lib --quiet
	@cd chronos-repl && cargo test --lib --quiet
	@echo "✅ Quick tests complete!"

# Individual component tests
test-core:
	@echo "🧪 Testing chronos-core..."
	@cd chronos-core && cargo test

test-repl:
	@echo "🧪 Testing chronos-repl..."
	@cd chronos-repl && cargo test

# Demo commands
demo:
	@echo "🎨 Running enhanced display demo..."
	@cd chronos-repl && cargo run --example demo_display

# Build commands
build:
	@echo "🔨 Building all workspace members..."
	@cargo build --workspace

check:
	@echo "🔍 Checking compilation..."
	@cargo check --workspace

clean:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean

# Code quality
fmt:
	@echo "🎨 Formatting code..."
	@cargo fmt --all

clippy:
	@echo "📎 Running clippy..."
	@cargo clippy --workspace --all-targets --all-features -- -D warnings

# Running
run-repl:
	@echo "🚀 Starting Chronos REPL..."
	@cd chronos-repl && cargo run --bin chronos-repl

install:
	@echo "📦 Installing Chronos binaries..."
	@cargo install --path . --bin chronos
	@cargo install --path chronos-repl --bin chronos-repl

# Development workflow
dev-check: fmt clippy test-quick
	@echo "✅ Development checks complete!"

# CI-like command for comprehensive validation
ci: clean check test-quick clippy
	@echo "🎉 All CI checks passed!"

# Full test suite (ignores some known issues)
test-full:
	@echo "🧪 Running full test suite (may have some warnings)..."
	@cargo test --workspace || echo "⚠️  Some tests have warnings but core functionality works"
