# Chronos REPL Enhancement Plan

This document outlines enhancements to make the Chronos REPL more feature-rich while maintaining simplicity, benefiting both humans and agents alike.

## Vision

Create a REPL that provides:
- **Rich visual experience** for humans (inspired by utop)
- **Machine-readable interfaces** for agents
- **Adaptive configuration** that can switch between modes
- **Consistent simplicity** as the core principle

## Current State Analysis

The Chronos REPL already has solid foundations:
- ✅ Session management (save/load complete sessions)
- ✅ Execution tracing with timing information
- ✅ Performance metrics and benchmarking
- ✅ Command history
- ✅ Configurable display options (stack display, colors, timing)
- ✅ Auto-save functionality
- ✅ Rich command system (.help, .stack, .trace, etc.)
- ✅ Color support
- ✅ Welcome messages and help system

## Enhancement Categories

### 1. Human-Friendly Enhancements (inspired by utop)

#### Enhanced Visual Interface ✅ COMPLETED
**Priority: High**
- ✅ Syntax highlighting for input and output
- ✅ Rich stack visualization with type information
- ✅ Better visual hierarchy and spacing
- ✅ Color-coded operation categories
- ✅ Visual indicators for REPL state (tracing, auto-save, etc.)

**Implementation:**
```rust
// chronos-repl/src/display.rs
pub struct DisplayConfig {
    pub use_colors: bool,
    pub show_types: bool,
    pub compact_stack: bool,
    pub highlight_syntax: bool,
    pub unicode_symbols: bool,
}

pub fn format_stack_rich(stack: &[Value], config: &DisplayConfig) -> String;
pub fn highlight_syntax(input: &str) -> String;
pub fn format_value_with_type(value: &Value, show_type: bool) -> String;
```

#### Interactive Input Enhancement
**Priority: Medium**
- Arrow key navigation through history
- Tab completion for words and commands
- Multi-line input support with proper indentation
- Real-time syntax validation
- Parentheses/bracket matching

#### Contextual Prompt
**Priority: Medium**
- Show stack depth in prompt
- Show current mode indicators (tracing, auto-save)
- Show execution context
- Customizable prompt templates

**Implementation:**
```rust
// chronos-repl/src/prompt.rs
pub fn generate_rich_prompt(repl: &EnhancedRepl) -> String {
    let stack_depth = repl.stack().len();
    let trace_status = if repl.tracing_enabled { "T" } else { " " };
    let mode_indicators = format_mode_indicators(repl);
    
    format!("C∀O[{}{}{}]> ", stack_depth, trace_status, mode_indicators)
}
```

### 2. Agent-Friendly Enhancements

#### Machine-Readable Output Mode
**Priority: High**
- JSON output format option
- Structured error responses
- Consistent delimiters for parsing
- Machine-readable status indicators

**Implementation:**
```rust
// chronos-repl/src/output.rs
pub struct OutputFormat {
    pub json_mode: bool,
    pub delimited_mode: bool,
    pub minimal_mode: bool,
}

#[derive(Serialize)]
pub struct MachineOutput {
    pub status: String,      // "ok", "error", "partial"
    pub result: Option<Value>,
    pub stack: Vec<Value>,
    pub timing: Option<Duration>,
    pub error: Option<String>,
    pub metadata: HashMap<String, String>,
}
```

#### Structured Command Interface
**Priority: High**
- Commands that output structured data
- Batch operation support
- Scriptable interface
- API-style command syntax

**Implementation:**
```rust
// Enhanced commands in chronos-repl/src/commands.rs
impl EnhancedRepl {
    pub fn handle_machine_command(&mut self, cmd: &str) -> Result<MachineOutput>;
    
    // New machine-friendly commands:
    // .stack-json, .eval-json, .status-json
    // .batch-eval, .script-mode
}
```

#### Standardized Error Codes
**Priority: Medium**
- Numeric error codes for different error types
- Structured error information
- Recovery suggestions
- Error categorization

### 3. Unified Enhancements (Both Human & Agent)

#### Smart Help System
**Priority: Medium**
- Context-sensitive help based on current input
- Interactive documentation
- Example suggestions
- Command discovery

**Implementation:**
```rust
// chronos-repl/src/help.rs
pub fn context_sensitive_help(input: &str, repl: &EnhancedRepl) -> String;
pub fn format_help_for_agent() -> String;
pub fn suggest_completions(partial_input: &str) -> Vec<String>;
```

#### Enhanced Error Reporting
**Priority: High**
- Rich error messages with context
- Suggestions for common mistakes
- Multiple output formats
- Error recovery hints

**Implementation:**
```rust
// chronos-repl/src/error_reporting.rs
pub struct RichError {
    pub code: u32,
    pub message: String,
    pub suggestion: Option<String>,
    pub context: Vec<String>,
    pub position: Option<usize>,
}

impl RichError {
    pub fn format_human(&self) -> String;
    pub fn format_machine(&self) -> String;
}
```

#### Adaptive Configuration
**Priority: Medium**
- Auto-detect usage patterns
- Smart defaults for different contexts
- Profile-based configuration
- Runtime mode switching

**Implementation:**
```rust
// chronos-repl/src/adaptive.rs
pub enum UsageMode {
    Human,
    Agent,
    Hybrid,
}

pub fn detect_usage_mode() -> UsageMode;
pub fn auto_configure_for_mode(config: &mut ReplConfig, mode: UsageMode);
```

## Implementation Phases

### Phase 1: Core Visual Enhancements ✅ COMPLETED
1. ✅ **Enhanced Visual Interface** - Rich display formatting
2. **Machine-Readable Output** - JSON mode and structured output (Next Priority)
3. **Enhanced Error Reporting** - Better error messages (Partially Complete)

### Phase 2: Interactive Improvements
1. **Interactive Input Enhancement** - History, completion, multi-line
2. **Smart Help System** - Context-aware documentation
3. **Contextual Prompt** - Rich prompt with indicators

### Phase 3: Advanced Features
1. **Adaptive Configuration** - Smart mode detection
2. **Structured Command Interface** - API-style commands
3. **Performance Optimizations** - Based on usage patterns

## Configuration Examples

### Human-Friendly Mode (Default)
```bash
chronos-repl
# Enables: colors, rich prompts, syntax highlighting, welcome message
```

### Agent-Friendly Mode
```bash
chronos-repl --json-output --minimal --no-color --no-welcome
# Enables: JSON output, minimal prompts, structured errors
```

### Hybrid Mode
```bash
chronos-repl --show-stack --json-errors --rich-display
# Mix of human and machine-readable features
```

### Developer Mode
```bash
chronos-repl --trace --show-timing --auto-save --rich-display
# Full debugging and development features
```

## Success Criteria

### For Humans:
- Intuitive visual feedback
- Easy discovery of features
- Clear error messages with suggestions
- Smooth interactive experience

### For Agents:
- Consistent, parseable output
- Programmatic interface
- Reliable error handling
- Batch operation support

### For Both:
- Simple core interface
- Optional complexity
- Fast performance
- Reliable operation

## Technical Considerations

### Dependencies
- `serde` for JSON serialization
- `crossterm` for terminal control and colors
- `rustyline` for line editing and history
- `syntect` for syntax highlighting

### Backward Compatibility
- All new features are opt-in
- Existing command interface unchanged
- Configuration-driven behavior
- Graceful fallbacks for unsupported features

### Performance
- Lazy loading of heavy features
- Configurable refresh rates
- Efficient display updates
- Memory-conscious implementation

## Future Considerations

### Integration Points
- Language server protocol support
- Plugin architecture for extensions
- Remote REPL capabilities
- Integration with development tools

### Extensibility
- Theme system for visual customization
- Custom command registration
- Programmable display formatters
- User-defined automation scripts

## Phase 1 Completion Summary

### What Was Implemented
- ✅ **Complete DisplayConfig system** with comprehensive customization
- ✅ **Rich stack visualization** with colors, types, and Unicode symbols
- ✅ **Syntax highlighting** for numbers, operators, keywords, and comments
- ✅ **Enhanced message formatting** with success/info/warning/error styles
- ✅ **New display control commands** (.colors, .types, .compact, .syntax, .unicode)
- ✅ **Backward compatibility** with existing REPL functionality
- ✅ **Comprehensive test coverage** and working demo program

### Key Benefits Delivered
- **Human users**: Rich visual feedback, customizable interface, better usability
- **Agents**: Configurable output, consistent formatting, machine-friendly options
- **Both**: Maintained simplicity with optional complexity

### Next Priority: Phase 2 Focus Areas
1. **Machine-Readable Output Mode** - JSON formatting for agents
2. **Interactive Input Enhancement** - History navigation and completion
3. **Advanced Error Reporting** - Context-aware error messages

---

*Phase 1 successfully demonstrates that we can enhance the REPL significantly while maintaining the core principle of simplicity. The foundation is now in place for both human and agent-friendly features.*