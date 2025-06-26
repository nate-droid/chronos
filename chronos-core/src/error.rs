//! Error handling for Chronos Core
//!
//! This module provides comprehensive error types and utilities for the
//! C∀O language runtime, covering lexical analysis, parsing, type checking,
//! and runtime execution errors.

use std::fmt;

/// Main error type for all Chronos Core operations
#[derive(Debug, Clone)]
pub enum ChronosError {
    /// Lexical analysis errors (tokenization)
    LexError {
        message: String,
        position: Option<usize>,
        source: Option<String>,
    },

    /// Parsing errors (syntax analysis)
    ParseError {
        message: String,
        token_position: Option<usize>,
        expected: Option<String>,
        found: Option<String>,
    },

    /// Type checking and inference errors
    TypeError {
        message: String,
        expected_type: Option<String>,
        found_type: Option<String>,
        word_name: Option<String>,
    },

    /// Runtime execution errors
    RuntimeError {
        message: String,
        word_name: Option<String>,
        stack_trace: Vec<String>,
    },

    /// Stack manipulation errors
    StackError {
        message: String,
        operation: String,
        stack_depth: usize,
        required_depth: usize,
    },

    /// Undefined words or operations
    UndefinedError {
        name: String,
        suggestion: Option<String>,
    },

    /// Ordinal verification and termination analysis errors
    OrdinalError {
        message: String,
        word_name: Option<String>,
        ordinal_context: Option<String>,
    },

    /// System-level errors (IO, memory, etc.)
    SystemError {
        message: String,
        source_message: Option<String>,
    },

    /// Arithmetic errors (division by zero, overflow, etc.)
    ArithmeticError {
        message: String,
        operation: String,
        operands: Vec<String>,
    },

    /// Definition errors (invalid word definitions, circular dependencies)
    DefinitionError {
        message: String,
        word_name: String,
        context: Option<String>,
    },
}

impl fmt::Display for ChronosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChronosError::LexError {
                message,
                position,
                source,
            } => {
                write!(f, "Lexical error: {}", message)?;
                if let Some(pos) = position {
                    write!(f, " at position {}", pos)?;
                }
                if let Some(src) = source {
                    write!(f, " in '{}'", src)?;
                }
                Ok(())
            }

            ChronosError::ParseError {
                message,
                token_position,
                expected,
                found,
            } => {
                write!(f, "Parse error: {}", message)?;
                if let Some(pos) = token_position {
                    write!(f, " at token {}", pos)?;
                }
                if let Some(exp) = expected {
                    write!(f, " (expected {})", exp)?;
                }
                if let Some(fnd) = found {
                    write!(f, " (found {})", fnd)?;
                }
                Ok(())
            }

            ChronosError::TypeError {
                message,
                expected_type,
                found_type,
                word_name,
            } => {
                write!(f, "Type error: {}", message)?;
                if let Some(word) = word_name {
                    write!(f, " in word '{}'", word)?;
                }
                if let Some(exp) = expected_type {
                    write!(f, " (expected {})", exp)?;
                }
                if let Some(fnd) = found_type {
                    write!(f, " (found {})", fnd)?;
                }
                Ok(())
            }

            ChronosError::RuntimeError {
                message,
                word_name,
                stack_trace,
            } => {
                write!(f, "Runtime error: {}", message)?;
                if let Some(word) = word_name {
                    write!(f, " in word '{}'", word)?;
                }
                if !stack_trace.is_empty() {
                    write!(f, "\nStack trace:")?;
                    for (i, frame) in stack_trace.iter().enumerate() {
                        write!(f, "\n  {}: {}", i, frame)?;
                    }
                }
                Ok(())
            }

            ChronosError::StackError {
                message,
                operation,
                stack_depth,
                required_depth,
            } => {
                write!(
                    f,
                    "Stack error: {} during '{}' (stack depth: {}, required: {})",
                    message, operation, stack_depth, required_depth
                )
            }

            ChronosError::UndefinedError { name, suggestion } => {
                write!(f, "Undefined word: '{}'", name)?;
                if let Some(sugg) = suggestion {
                    write!(f, " (did you mean '{}'?)", sugg)?;
                }
                Ok(())
            }

            ChronosError::OrdinalError {
                message,
                word_name,
                ordinal_context,
            } => {
                write!(f, "Ordinal error: {}", message)?;
                if let Some(word) = word_name {
                    write!(f, " in word '{}'", word)?;
                }
                if let Some(context) = ordinal_context {
                    write!(f, " ({})", context)?;
                }
                Ok(())
            }

            ChronosError::SystemError {
                message,
                source_message,
            } => {
                write!(f, "System error: {}", message)?;
                if let Some(src) = source_message {
                    write!(f, " ({})", src)?;
                }
                Ok(())
            }

            ChronosError::ArithmeticError {
                message,
                operation,
                operands,
            } => {
                write!(
                    f,
                    "Arithmetic error: {} during '{}' with operands [{}]",
                    message,
                    operation,
                    operands.join(", ")
                )
            }

            ChronosError::DefinitionError {
                message,
                word_name,
                context,
            } => {
                write!(f, "Definition error: {} for word '{}'", message, word_name)?;
                if let Some(ctx) = context {
                    write!(f, " ({})", ctx)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for ChronosError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            _ => None,
        }
    }
}

/// Result type alias for Chronos Core operations
pub type Result<T> = std::result::Result<T, ChronosError>;

/// Error construction helpers
impl ChronosError {
    /// Create a lexical error with position information
    pub fn lex_error(message: impl Into<String>, position: Option<usize>) -> Self {
        ChronosError::LexError {
            message: message.into(),
            position,
            source: None,
        }
    }

    /// Create a parse error with token information
    pub fn parse_error(
        message: impl Into<String>,
        expected: Option<String>,
        found: Option<String>,
    ) -> Self {
        ChronosError::ParseError {
            message: message.into(),
            token_position: None,
            expected,
            found,
        }
    }

    /// Create a type error with type information
    pub fn type_error(
        message: impl Into<String>,
        expected: Option<String>,
        found: Option<String>,
    ) -> Self {
        ChronosError::TypeError {
            message: message.into(),
            expected_type: expected,
            found_type: found,
            word_name: None,
        }
    }

    /// Create a runtime error with context
    pub fn runtime_error(message: impl Into<String>, word_name: Option<String>) -> Self {
        ChronosError::RuntimeError {
            message: message.into(),
            word_name,
            stack_trace: Vec::new(),
        }
    }

    /// Create a stack error with operation details
    pub fn stack_error(
        message: impl Into<String>,
        operation: impl Into<String>,
        stack_depth: usize,
        required_depth: usize,
    ) -> Self {
        ChronosError::StackError {
            message: message.into(),
            operation: operation.into(),
            stack_depth,
            required_depth,
        }
    }

    /// Create an undefined word error
    pub fn undefined_error(name: impl Into<String>) -> Self {
        ChronosError::UndefinedError {
            name: name.into(),
            suggestion: None,
        }
    }

    /// Create an arithmetic error
    pub fn arithmetic_error(
        message: impl Into<String>,
        operation: impl Into<String>,
        operands: Vec<String>,
    ) -> Self {
        ChronosError::ArithmeticError {
            message: message.into(),
            operation: operation.into(),
            operands,
        }
    }

    /// Add a word name context to this error
    pub fn with_word_context(mut self, word_name: impl Into<String>) -> Self {
        let word = word_name.into();
        match &mut self {
            ChronosError::TypeError {
                word_name: ref mut w,
                ..
            } => *w = Some(word),
            ChronosError::RuntimeError {
                word_name: ref mut w,
                ..
            } => *w = Some(word),
            ChronosError::OrdinalError {
                word_name: ref mut w,
                ..
            } => *w = Some(word),
            _ => {}
        }
        self
    }

    /// Add a stack trace frame to runtime errors
    pub fn with_stack_frame(mut self, frame: impl Into<String>) -> Self {
        if let ChronosError::RuntimeError {
            ref mut stack_trace,
            ..
        } = self
        {
            stack_trace.push(frame.into());
        }
        self
    }

    /// Add a suggestion to undefined errors
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        if let ChronosError::UndefinedError {
            suggestion: ref mut s,
            ..
        } = self
        {
            *s = Some(suggestion.into());
        }
        self
    }

    /// Check if this is a recoverable error (doesn't indicate system failure)
    pub fn is_recoverable(&self) -> bool {
        match self {
            ChronosError::LexError { .. }
            | ChronosError::ParseError { .. }
            | ChronosError::TypeError { .. }
            | ChronosError::UndefinedError { .. }
            | ChronosError::StackError { .. }
            | ChronosError::ArithmeticError { .. }
            | ChronosError::DefinitionError { .. } => true,

            ChronosError::RuntimeError { .. } | ChronosError::OrdinalError { .. } => true, // Usually recoverable

            ChronosError::SystemError { .. } => false, // System errors are not recoverable
        }
    }

    /// Get a short error category for logging/metrics
    pub fn category(&self) -> &'static str {
        match self {
            ChronosError::LexError { .. } => "lex",
            ChronosError::ParseError { .. } => "parse",
            ChronosError::TypeError { .. } => "type",
            ChronosError::RuntimeError { .. } => "runtime",
            ChronosError::StackError { .. } => "stack",
            ChronosError::UndefinedError { .. } => "undefined",
            ChronosError::OrdinalError { .. } => "ordinal",
            ChronosError::SystemError { .. } => "system",
            ChronosError::ArithmeticError { .. } => "arithmetic",
            ChronosError::DefinitionError { .. } => "definition",
        }
    }
}

/// Helper trait for converting string errors to ChronosError
pub trait IntoChronosError<T> {
    fn into_chronos_error(self) -> Result<T>;
}

impl<T> IntoChronosError<T> for std::result::Result<T, String> {
    fn into_chronos_error(self) -> Result<T> {
        self.map_err(|e| ChronosError::runtime_error(e, None))
    }
}

/// Error context management for better error reporting
pub struct ErrorContext {
    word_stack: Vec<String>,
    current_source: Option<String>,
    current_position: Option<usize>,
}

impl ErrorContext {
    pub fn new() -> Self {
        Self {
            word_stack: Vec::new(),
            current_source: None,
            current_position: None,
        }
    }

    pub fn push_word(&mut self, word: impl Into<String>) {
        self.word_stack.push(word.into());
    }

    pub fn pop_word(&mut self) -> Option<String> {
        self.word_stack.pop()
    }

    pub fn set_source(&mut self, source: impl Into<String>) {
        self.current_source = Some(source.into());
    }

    pub fn set_position(&mut self, position: usize) {
        self.current_position = Some(position);
    }

    pub fn enhance_error(&self, mut error: ChronosError) -> ChronosError {
        // Add word context from the call stack
        if let Some(current_word) = self.word_stack.last() {
            error = error.with_word_context(current_word);
        }

        // Add stack trace for runtime errors
        if let ChronosError::RuntimeError {
            ref mut stack_trace,
            ..
        } = error
        {
            for word in self.word_stack.iter().rev() {
                stack_trace.push(word.clone());
            }
        }

        error
    }
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = ChronosError::stack_error("Not enough values", "dup", 0, 1);
        let display = format!("{}", error);
        assert!(display.contains("Stack error"));
        assert!(display.contains("dup"));
    }

    #[test]
    fn test_error_context() {
        let error = ChronosError::undefined_error("foo").with_suggestion("bar");

        if let ChronosError::UndefinedError { suggestion, .. } = error {
            assert_eq!(suggestion, Some("bar".to_string()));
        } else {
            panic!("Expected UndefinedError");
        }
    }

    #[test]
    fn test_error_category() {
        assert_eq!(ChronosError::lex_error("test", None).category(), "lex");
        assert_eq!(
            ChronosError::type_error("test", None, None).category(),
            "type"
        );
        assert_eq!(
            ChronosError::runtime_error("test", None).category(),
            "runtime"
        );
    }

    #[test]
    fn test_error_context_manager() {
        let mut ctx = ErrorContext::new();
        ctx.push_word("outer");
        ctx.push_word("inner");

        let error = ChronosError::runtime_error("test error", None);
        let enhanced = ctx.enhance_error(error);

        if let ChronosError::RuntimeError { stack_trace, .. } = enhanced {
            assert_eq!(stack_trace, vec!["inner", "outer"]);
        } else {
            panic!("Expected RuntimeError");
        }
    }
}
