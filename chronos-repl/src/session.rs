//! Session management for the Chronos REPL
//!
//! This module handles saving and loading REPL sessions, including user-defined
//! words, types, stack state, and configuration settings.

use crate::error::{Result, SessionError};
use chronos_core::{Token, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use uuid::Uuid;

/// Current session format version
pub const SESSION_VERSION: &str = "0.1.0";

/// Session data that can be serialized and persisted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    /// Session format version
    pub version: String,

    /// Unique session identifier
    pub session_id: String,

    /// Session metadata
    pub metadata: SessionMetadata,

    /// Current stack contents
    pub stack: Vec<Value>,

    /// User-defined words (stored as token sequences)
    pub user_words: HashMap<String, Vec<Token>>,

    /// Session configuration
    pub config: SessionConfig,

    /// Command history
    pub history: Vec<String>,

    /// Performance statistics
    pub stats: SessionStats,
}

/// Session metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    /// When the session was created
    pub created_at: String,

    /// When the session was last modified
    pub modified_at: String,

    /// Session name/description
    pub name: Option<String>,

    /// Session tags for organization
    pub tags: Vec<String>,

    /// Notes about the session
    pub notes: Option<String>,
}

/// Session configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Whether to show stack after each operation
    pub show_stack: bool,

    /// Whether to show ordinal costs
    pub show_ordinals: bool,

    /// Whether to trace execution
    pub trace_execution: bool,

    /// Maximum trace entries to keep
    pub max_trace_entries: usize,

    /// Maximum command history entries
    pub max_history_entries: usize,

    /// Whether to auto-save sessions
    pub auto_save: bool,

    /// Auto-save interval in seconds
    pub auto_save_interval: u64,
}

/// Session performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStats {
    /// Total operations executed
    pub operations_count: usize,

    /// Total execution time in milliseconds
    pub total_time_ms: u64,

    /// Maximum stack depth reached
    pub max_stack_depth: usize,

    /// Number of words defined
    pub words_defined: usize,

    /// Number of errors encountered
    pub errors_count: usize,
}

/// Session manager for handling persistence
pub struct Session {
    /// Current session data
    data: SessionData,

    /// File path for auto-saving
    file_path: Option<std::path::PathBuf>,

    /// Whether the session has unsaved changes
    dirty: bool,

    /// Last auto-save timestamp
    last_auto_save: std::time::Instant,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            show_stack: false,
            show_ordinals: false,
            trace_execution: false,
            max_trace_entries: 1000,
            max_history_entries: 100,
            auto_save: false,
            auto_save_interval: 300, // 5 minutes
        }
    }
}

impl Default for SessionStats {
    fn default() -> Self {
        Self {
            operations_count: 0,
            total_time_ms: 0,
            max_stack_depth: 0,
            words_defined: 0,
            errors_count: 0,
        }
    }
}

impl Default for SessionData {
    fn default() -> Self {
        let now = chrono::Utc::now().to_rfc3339();

        Self {
            version: SESSION_VERSION.to_string(),
            session_id: Uuid::new_v4().to_string(),
            metadata: SessionMetadata {
                created_at: now.clone(),
                modified_at: now,
                name: None,
                tags: Vec::new(),
                notes: None,
            },
            stack: Vec::new(),
            user_words: HashMap::new(),
            config: SessionConfig::default(),
            history: Vec::new(),
            stats: SessionStats::default(),
        }
    }
}

impl Session {
    /// Create a new empty session
    pub fn new() -> Self {
        Self {
            data: SessionData::default(),
            file_path: None,
            dirty: false,
            last_auto_save: std::time::Instant::now(),
        }
    }

    /// Create a session with a name
    pub fn new_with_name<S: Into<String>>(name: S) -> Self {
        let mut session = Self::new();
        session.data.metadata.name = Some(name.into());
        session.dirty = true;
        session
    }

    /// Load a session from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(SessionError::file_not_found(path.display().to_string()).into());
        }

        let contents =
            fs::read_to_string(path).map_err(|e| SessionError::Serialization(e.to_string()))?;

        let data: SessionData = serde_json::from_str(&contents)?;

        // Validate session version compatibility
        if data.version != SESSION_VERSION {
            return Err(SessionError::version_mismatch(SESSION_VERSION, &data.version).into());
        }

        Ok(Self {
            data,
            file_path: Some(path.to_path_buf()),
            dirty: false,
            last_auto_save: std::time::Instant::now(),
        })
    }

    /// Save the session to a file
    pub fn save<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();

        // Update modification timestamp
        self.data.metadata.modified_at = chrono::Utc::now().to_rfc3339();

        let contents = serde_json::to_string_pretty(&self.data)?;
        fs::write(path, contents)?;

        self.file_path = Some(path.to_path_buf());
        self.dirty = false;
        self.last_auto_save = std::time::Instant::now();

        Ok(())
    }

    /// Save to the current file path if set
    pub fn save_current(&mut self) -> Result<()> {
        if let Some(ref path) = self.file_path.clone() {
            self.save(path)
        } else {
            Err(SessionError::missing_data("file path").into())
        }
    }

    /// Check if auto-save should be performed
    pub fn should_auto_save(&self) -> bool {
        if !self.data.config.auto_save || !self.dirty {
            return false;
        }

        let elapsed = self.last_auto_save.elapsed().as_secs();
        elapsed >= self.data.config.auto_save_interval
    }

    /// Perform auto-save if needed
    pub fn try_auto_save(&mut self) -> Result<bool> {
        if self.should_auto_save() {
            self.save_current()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get session data
    pub fn data(&self) -> &SessionData {
        &self.data
    }

    /// Get mutable session data
    pub fn data_mut(&mut self) -> &mut SessionData {
        self.dirty = true;
        &mut self.data
    }

    /// Check if session has unsaved changes
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Get the current file path
    pub fn file_path(&self) -> Option<&Path> {
        self.file_path.as_deref()
    }

    /// Update stack contents
    pub fn update_stack(&mut self, stack: Vec<Value>) {
        self.data.stack = stack;
        self.dirty = true;

        // Update statistics
        self.data.stats.max_stack_depth =
            self.data.stats.max_stack_depth.max(self.data.stack.len());
    }

    /// Add a user-defined word
    pub fn define_word<S: Into<String>>(&mut self, name: S, tokens: Vec<Token>) {
        let name = name.into();
        let is_new = !self.data.user_words.contains_key(&name);

        self.data.user_words.insert(name, tokens);
        self.dirty = true;

        if is_new {
            self.data.stats.words_defined += 1;
        }
    }

    /// Remove a user-defined word
    pub fn undefine_word<S: AsRef<str>>(&mut self, name: S) -> bool {
        let removed = self.data.user_words.remove(name.as_ref()).is_some();
        if removed {
            self.dirty = true;
        }
        removed
    }

    /// Get all user-defined words
    pub fn user_words(&self) -> &HashMap<String, Vec<Token>> {
        &self.data.user_words
    }

    /// Add command to history
    pub fn add_to_history<S: Into<String>>(&mut self, command: S) {
        let command = command.into();
        if command.is_empty() {
            return;
        }

        // Avoid duplicate consecutive entries
        if let Some(last) = self.data.history.last() {
            if last == &command {
                return;
            }
        }

        self.data.history.push(command);

        // Limit history size
        let max_entries = self.data.config.max_history_entries;
        if self.data.history.len() > max_entries {
            self.data
                .history
                .drain(0..self.data.history.len() - max_entries);
        }

        self.dirty = true;
    }

    /// Get command history
    pub fn history(&self) -> &[String] {
        &self.data.history
    }

    /// Clear command history
    pub fn clear_history(&mut self) {
        self.data.history.clear();
        self.dirty = true;
    }

    /// Update session statistics
    pub fn update_stats(&mut self, operations: usize, time_ms: u64, errors: usize) {
        self.data.stats.operations_count += operations;
        self.data.stats.total_time_ms += time_ms;
        self.data.stats.errors_count += errors;
        self.dirty = true;
    }

    /// Get session configuration
    pub fn config(&self) -> &SessionConfig {
        &self.data.config
    }

    /// Get mutable session configuration
    pub fn config_mut(&mut self) -> &mut SessionConfig {
        self.dirty = true;
        &mut self.data.config
    }

    /// Export session data for external use
    pub fn export_data(&self) -> SessionData {
        self.data.clone()
    }

    /// Import session data from external source
    pub fn import_data(&mut self, data: SessionData) -> Result<()> {
        // Validate version compatibility
        if data.version != SESSION_VERSION {
            return Err(SessionError::version_mismatch(SESSION_VERSION, &data.version).into());
        }

        self.data = data;
        self.dirty = true;
        Ok(())
    }

    /// Reset session to default state
    pub fn reset(&mut self) {
        let session_id = self.data.session_id.clone();
        let name = self.data.metadata.name.clone();

        self.data = SessionData::default();
        self.data.session_id = session_id;
        self.data.metadata.name = name;
        self.dirty = true;
    }

    /// Add tags to session
    pub fn add_tags<I>(&mut self, tags: I)
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        for tag in tags {
            let tag = tag.into();
            if !self.data.metadata.tags.contains(&tag) {
                self.data.metadata.tags.push(tag);
            }
        }
        self.dirty = true;
    }

    /// Remove tags from session
    pub fn remove_tags<I>(&mut self, tags: I)
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        for tag in tags {
            self.data.metadata.tags.retain(|t| t != tag.as_ref());
        }
        self.dirty = true;
    }

    /// Set session notes
    pub fn set_notes<S: Into<String>>(&mut self, notes: Option<S>) {
        self.data.metadata.notes = notes.map(|s| s.into());
        self.dirty = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_session_creation() {
        let session = Session::new();
        assert_eq!(session.data.version, SESSION_VERSION);
        assert!(!session.data.session_id.is_empty());
        assert!(!session.is_dirty());
    }

    #[test]
    fn test_session_with_name() {
        let session = Session::new_with_name("Test Session");
        assert_eq!(session.data.metadata.name, Some("Test Session".to_string()));
        assert!(session.is_dirty());
    }

    #[test]
    fn test_session_save_load() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Create and save a session
        {
            let mut session = Session::new_with_name("Test Session");
            session.define_word("test", vec![Token::Literal(Value::Nat(42))]);
            session.add_to_history("test command");
            session.save(path).unwrap();
        }

        // Load the session
        {
            let session = Session::load(path).unwrap();
            assert_eq!(session.data.metadata.name, Some("Test Session".to_string()));
            assert_eq!(session.data.user_words.len(), 1);
            assert_eq!(session.data.history.len(), 1);
            assert!(!session.is_dirty());
        }
    }

    #[test]
    fn test_word_definition() {
        let mut session = Session::new();
        let tokens = vec![Token::Literal(Value::Nat(42))];

        session.define_word("test", tokens.clone());
        assert!(session.is_dirty());
        assert_eq!(session.data.stats.words_defined, 1);

        let stored_tokens = session.user_words().get("test").unwrap();
        assert_eq!(stored_tokens, &tokens);

        // Redefining doesn't increment count
        session.define_word("test", tokens);
        assert_eq!(session.data.stats.words_defined, 1);
    }

    #[test]
    fn test_history_management() {
        let mut session = Session::new();

        session.add_to_history("command1");
        session.add_to_history("command2");
        session.add_to_history("command2"); // Duplicate, should be ignored

        let history = session.history();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0], "command1");
        assert_eq!(history[1], "command2");
    }

    #[test]
    fn test_stack_update() {
        let mut session = Session::new();
        let stack = vec![Value::Nat(1), Value::Nat(2), Value::Nat(3)];

        session.update_stack(stack.clone());
        assert!(session.is_dirty());
        assert_eq!(session.data.stack, stack);
        assert_eq!(session.data.stats.max_stack_depth, 3);
    }

    #[test]
    fn test_auto_save_logic() {
        let mut session = Session::new();
        session.data.config.auto_save = true;
        session.data.config.auto_save_interval = 0; // Immediate

        // No auto-save if not dirty
        assert!(!session.should_auto_save());

        // Auto-save if dirty and interval elapsed
        session.define_word("test", vec![]);
        assert!(session.should_auto_save());
    }

    #[test]
    fn test_session_reset() {
        let mut session = Session::new_with_name("Test");
        session.define_word("test", vec![]);
        session.add_to_history("test");

        let original_id = session.data.session_id.clone();
        let original_name = session.data.metadata.name.clone();

        session.reset();

        assert_eq!(session.data.session_id, original_id);
        assert_eq!(session.data.metadata.name, original_name);
        assert!(session.data.user_words.is_empty());
        assert!(session.data.history.is_empty());
        assert!(session.is_dirty());
    }

    #[test]
    fn test_tags_management() {
        let mut session = Session::new();

        session.add_tags(vec!["math", "tutorial"]);
        assert_eq!(session.data.metadata.tags.len(), 2);

        session.add_tags(vec!["math"]); // Duplicate, should be ignored
        assert_eq!(session.data.metadata.tags.len(), 2);

        session.remove_tags(vec!["math"]);
        assert_eq!(session.data.metadata.tags.len(), 1);
        assert_eq!(session.data.metadata.tags[0], "tutorial");
    }
}
