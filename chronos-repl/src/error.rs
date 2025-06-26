//! Error handling for the Chronos REPL
//!
//! This module defines error types and handling for the REPL environment,
//! building on top of chronos-core's error system.

use chronos_core::ChronosError;
use std::fmt;
use std::io;

/// Result type for REPL operations
pub type Result<T> = std::result::Result<T, ReplError>;

/// Errors that can occur in the REPL environment
#[derive(Debug, Clone)]
pub enum ReplError {
    /// Error from the core Chronos language system
    Core(ChronosError),

    /// Session management error
    Session(SessionError),

    /// I/O error (file operations, etc.)
    Io(String),

    /// Command parsing error
    Command(String),

    /// Configuration error
    Config(String),

    /// Trace buffer overflow or other tracing issues
    Trace(String),

    /// Performance monitoring error
    Performance(String),
}

/// Session-specific errors
#[derive(Debug, Clone)]
pub enum SessionError {
    /// Session file not found
    FileNotFound(String),

    /// Invalid session format
    InvalidFormat(String),

    /// Serialization/deserialization error
    Serialization(String),

    /// Session version mismatch
    VersionMismatch { expected: String, found: String },

    /// Missing required session data
    MissingData(String),

    /// Session corruption detected
    Corrupted(String),
}

impl fmt::Display for ReplError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReplError::Core(e) => write!(f, "Core language error: {}", e),
            ReplError::Session(e) => write!(f, "Session error: {}", e),
            ReplError::Io(msg) => write!(f, "I/O error: {}", msg),
            ReplError::Command(msg) => write!(f, "Command error: {}", msg),
            ReplError::Config(msg) => write!(f, "Configuration error: {}", msg),
            ReplError::Trace(msg) => write!(f, "Trace error: {}", msg),
            ReplError::Performance(msg) => write!(f, "Performance monitoring error: {}", msg),
        }
    }
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionError::FileNotFound(path) => write!(f, "Session file not found: {}", path),
            SessionError::InvalidFormat(msg) => write!(f, "Invalid session format: {}", msg),
            SessionError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            SessionError::VersionMismatch { expected, found } => {
                write!(
                    f,
                    "Session version mismatch: expected {}, found {}",
                    expected, found
                )
            }
            SessionError::MissingData(field) => write!(f, "Missing session data: {}", field),
            SessionError::Corrupted(msg) => write!(f, "Session corrupted: {}", msg),
        }
    }
}

impl std::error::Error for ReplError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ReplError::Core(e) => Some(e),
            _ => None,
        }
    }
}

impl std::error::Error for SessionError {}

// Conversions from other error types

impl From<ChronosError> for ReplError {
    fn from(error: ChronosError) -> Self {
        ReplError::Core(error)
    }
}

impl From<SessionError> for ReplError {
    fn from(error: SessionError) -> Self {
        ReplError::Session(error)
    }
}

impl From<io::Error> for ReplError {
    fn from(error: io::Error) -> Self {
        ReplError::Io(error.to_string())
    }
}

impl From<serde_json::Error> for ReplError {
    fn from(error: serde_json::Error) -> Self {
        ReplError::Session(SessionError::Serialization(error.to_string()))
    }
}

impl From<serde_json::Error> for SessionError {
    fn from(error: serde_json::Error) -> Self {
        SessionError::Serialization(error.to_string())
    }
}

// Helper functions for creating specific errors

impl ReplError {
    /// Create a command error
    pub fn command<S: Into<String>>(msg: S) -> Self {
        ReplError::Command(msg.into())
    }

    /// Create a configuration error
    pub fn config<S: Into<String>>(msg: S) -> Self {
        ReplError::Config(msg.into())
    }

    /// Create a trace error
    pub fn trace<S: Into<String>>(msg: S) -> Self {
        ReplError::Trace(msg.into())
    }

    /// Create a performance error
    pub fn performance<S: Into<String>>(msg: S) -> Self {
        ReplError::Performance(msg.into())
    }

    /// Create an I/O error
    pub fn io<S: Into<String>>(msg: S) -> Self {
        ReplError::Io(msg.into())
    }
}

impl SessionError {
    /// Create a file not found error
    pub fn file_not_found<S: Into<String>>(path: S) -> Self {
        SessionError::FileNotFound(path.into())
    }

    /// Create an invalid format error
    pub fn invalid_format<S: Into<String>>(msg: S) -> Self {
        SessionError::InvalidFormat(msg.into())
    }

    /// Create a version mismatch error
    pub fn version_mismatch<S: Into<String>>(expected: S, found: S) -> Self {
        SessionError::VersionMismatch {
            expected: expected.into(),
            found: found.into(),
        }
    }

    /// Create a missing data error
    pub fn missing_data<S: Into<String>>(field: S) -> Self {
        SessionError::MissingData(field.into())
    }

    /// Create a corrupted session error
    pub fn corrupted<S: Into<String>>(msg: S) -> Self {
        SessionError::Corrupted(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = ReplError::command("Invalid command syntax");
        assert!(error.to_string().contains("Command error"));
        assert!(error.to_string().contains("Invalid command syntax"));
    }

    #[test]
    fn test_session_error_display() {
        let error = SessionError::version_mismatch("1.0", "0.9");
        assert!(error.to_string().contains("version mismatch"));
        assert!(error.to_string().contains("expected 1.0"));
        assert!(error.to_string().contains("found 0.9"));
    }

    #[test]
    fn test_error_conversions() {
        // Test conversion from ChronosError
        let core_error = ChronosError::runtime_error("Test error", None);
        let repl_error: ReplError = core_error.into();
        matches!(repl_error, ReplError::Core(_));

        // Test conversion from SessionError
        let session_error = SessionError::file_not_found("test.json");
        let repl_error: ReplError = session_error.into();
        matches!(repl_error, ReplError::Session(_));
    }

    #[test]
    fn test_error_creation_helpers() {
        let error = ReplError::command("test");
        matches!(error, ReplError::Command(_));

        let error = SessionError::missing_data("stack");
        matches!(error, SessionError::MissingData(_));
    }
}
