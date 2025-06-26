//! Execution tracing for the Chronos REPL
//!
//! This module provides detailed tracing capabilities for monitoring and
//! debugging program execution, including stack state changes, performance
//! metrics, and execution flow analysis.

use chronos_core::{Token, Value};
use serde::{Deserialize, Serialize};

use std::time::{Duration, Instant};

/// A single execution trace entry
#[derive(Debug, Clone)]
pub struct TraceEntry {
    /// Unique trace entry ID
    pub id: u64,

    /// Timestamp when the operation started
    pub timestamp: Instant,

    /// The token being executed
    pub token: Token,

    /// Stack state before execution
    pub stack_before: Vec<Value>,

    /// Stack state after execution
    pub stack_after: Vec<Value>,

    /// Execution duration
    pub duration: Duration,

    /// Stack depth change (positive = pushed, negative = popped)
    pub stack_depth_change: i32,

    /// Any error that occurred during execution
    pub error: Option<String>,

    /// Additional context information
    pub context: TraceContext,
}

/// Additional context for trace entries
#[derive(Debug, Clone)]
pub struct TraceContext {
    /// Current word being defined (if any)
    pub current_word: Option<String>,

    /// Nesting level (for quotes and control structures)
    pub nesting_level: usize,

    /// Operation category
    pub category: OperationCategory,

    /// Performance impact classification
    pub impact: PerformanceImpact,
}

/// Classification of operation types
#[derive(Debug, Clone, PartialEq)]
pub enum OperationCategory {
    /// Stack manipulation (dup, drop, swap, etc.)
    StackOp,

    /// Arithmetic operations
    Arithmetic,

    /// Comparison operations
    Comparison,

    /// Control flow operations
    ControlFlow,

    /// Word definition or execution
    Word,

    /// Quote operations
    Quote,

    /// System operations (printing, etc.)
    System,

    /// Type operations
    Type,

    /// Unknown or custom operation
    Other,
}

/// Performance impact classification
#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceImpact {
    /// Very fast operations (< 1μs)
    Minimal,

    /// Fast operations (1-10μs)
    Low,

    /// Moderate operations (10-100μs)
    Medium,

    /// Slow operations (100μs-1ms)
    High,

    /// Very slow operations (> 1ms)
    Critical,
}

/// Complete execution trace with analysis capabilities
#[derive(Debug, Clone)]
pub struct ExecutionTrace {
    /// All trace entries
    pub entries: Vec<TraceEntry>,

    /// Summary statistics
    pub summary: TraceSummary,

    /// Maximum number of entries to keep
    max_entries: usize,

    /// Next entry ID
    next_id: u64,
}

/// Summary statistics for a trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceSummary {
    /// Total number of operations
    pub total_operations: usize,

    /// Total execution time
    pub total_time: Duration,

    /// Average execution time per operation
    pub average_time: Duration,

    /// Maximum stack depth reached
    pub max_stack_depth: usize,

    /// Number of errors encountered
    pub error_count: usize,

    /// Operations by category
    pub operations_by_category: std::collections::HashMap<String, usize>,

    /// Performance distribution
    pub performance_distribution: std::collections::HashMap<String, usize>,

    /// Slowest operations
    pub slowest_operations: Vec<SlowOperation>,
}

/// Information about slow operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlowOperation {
    /// The token that was slow
    pub token: String,

    /// Execution time
    pub duration_us: u64,

    /// Stack depth at time of execution
    pub stack_depth: usize,

    /// Entry ID for reference
    pub entry_id: u64,
}

/// Trace filter for selective tracing
#[derive(Debug, Clone)]
pub struct TraceFilter {
    /// Include only these operation categories
    pub include_categories: Option<Vec<OperationCategory>>,

    /// Exclude these operation categories
    pub exclude_categories: Vec<OperationCategory>,

    /// Minimum execution time to trace (in microseconds)
    pub min_duration_us: u64,

    /// Only trace operations that change stack depth
    pub stack_changes_only: bool,

    /// Only trace errors
    pub errors_only: bool,

    /// Maximum nesting level to trace
    pub max_nesting_level: Option<usize>,
}

impl Default for TraceFilter {
    fn default() -> Self {
        Self {
            include_categories: None,
            exclude_categories: Vec::new(),
            min_duration_us: 0,
            stack_changes_only: false,
            errors_only: false,
            max_nesting_level: None,
        }
    }
}

impl ExecutionTrace {
    /// Create a new execution trace
    pub fn new() -> Self {
        Self::with_capacity(1000)
    }

    /// Create a new execution trace with specified capacity
    pub fn with_capacity(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            summary: TraceSummary::new(),
            max_entries,
            next_id: 1,
        }
    }

    /// Add a new trace entry
    pub fn add_entry(&mut self, entry: TraceEntry) {
        let mut entry = entry;
        entry.id = self.next_id;
        self.next_id += 1;

        self.entries.push(entry);

        // Maintain size limit
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }

        // Update summary statistics
        self.update_summary();
    }

    /// Create and add a trace entry from execution data
    pub fn trace_execution(
        &mut self,
        token: Token,
        stack_before: Vec<Value>,
        stack_after: Vec<Value>,
        duration: Duration,
        error: Option<String>,
        context: TraceContext,
    ) {
        let stack_depth_change = stack_after.len() as i32 - stack_before.len() as i32;

        let entry = TraceEntry {
            id: 0, // Will be set by add_entry
            timestamp: Instant::now(),
            token,
            stack_before,
            stack_after,
            duration,
            stack_depth_change,
            error,
            context,
        };

        self.add_entry(entry);
    }

    /// Get entries matching a filter
    pub fn filter_entries(&self, filter: &TraceFilter) -> Vec<&TraceEntry> {
        self.entries
            .iter()
            .filter(|entry| self.matches_filter(entry, filter))
            .collect()
    }

    /// Check if an entry matches a filter
    fn matches_filter(&self, entry: &TraceEntry, filter: &TraceFilter) -> bool {
        // Check category inclusion
        if let Some(ref include_cats) = filter.include_categories {
            if !include_cats.contains(&entry.context.category) {
                return false;
            }
        }

        // Check category exclusion
        if filter.exclude_categories.contains(&entry.context.category) {
            return false;
        }

        // Check minimum duration
        if entry.duration.as_micros() < filter.min_duration_us as u128 {
            return false;
        }

        // Check stack changes only
        if filter.stack_changes_only && entry.stack_depth_change == 0 {
            return false;
        }

        // Check errors only
        if filter.errors_only && entry.error.is_none() {
            return false;
        }

        // Check nesting level
        if let Some(max_level) = filter.max_nesting_level {
            if entry.context.nesting_level > max_level {
                return false;
            }
        }

        true
    }

    /// Update summary statistics
    fn update_summary(&mut self) {
        let mut summary = TraceSummary::new();

        summary.total_operations = self.entries.len();

        if !self.entries.is_empty() {
            // Calculate timing statistics
            let total_time: Duration = self.entries.iter().map(|e| e.duration).sum();
            summary.total_time = total_time;
            summary.average_time = total_time / self.entries.len() as u32;

            // Find maximum stack depth
            summary.max_stack_depth = self
                .entries
                .iter()
                .map(|e| e.stack_after.len())
                .max()
                .unwrap_or(0);

            // Count errors
            summary.error_count = self.entries.iter().filter(|e| e.error.is_some()).count();

            // Count operations by category
            for entry in &self.entries {
                let category = format!("{:?}", entry.context.category);
                *summary.operations_by_category.entry(category).or_insert(0) += 1;
            }

            // Count performance distribution
            for entry in &self.entries {
                let impact = format!("{:?}", entry.context.impact);
                *summary.performance_distribution.entry(impact).or_insert(0) += 1;
            }

            // Find slowest operations
            let mut slow_ops: Vec<_> = self
                .entries
                .iter()
                .map(|e| SlowOperation {
                    token: e.token.to_string(),
                    duration_us: e.duration.as_micros() as u64,
                    stack_depth: e.stack_after.len(),
                    entry_id: e.id,
                })
                .collect();

            slow_ops.sort_by_key(|op| std::cmp::Reverse(op.duration_us));
            summary.slowest_operations = slow_ops.into_iter().take(10).collect();
        }

        self.summary = summary;
    }

    /// Clear all trace entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.summary = TraceSummary::new();
        self.next_id = 1;
    }

    /// Get the most recent entries
    pub fn recent_entries(&self, count: usize) -> &[TraceEntry] {
        let start = if self.entries.len() > count {
            self.entries.len() - count
        } else {
            0
        };
        &self.entries[start..]
    }

    /// Find entries by token pattern
    pub fn find_by_token(&self, pattern: &str) -> Vec<&TraceEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.token.to_string().contains(pattern))
            .collect()
    }

    /// Get performance statistics for a specific token
    pub fn token_performance(&self, token_str: &str) -> Option<TokenPerformance> {
        let matching_entries: Vec<_> = self
            .entries
            .iter()
            .filter(|e| e.token.to_string() == token_str)
            .collect();

        if matching_entries.is_empty() {
            return None;
        }

        let durations: Vec<Duration> = matching_entries.iter().map(|e| e.duration).collect();
        let total_time: Duration = durations.iter().sum();
        let avg_time = total_time / durations.len() as u32;
        let min_time = durations.iter().min().unwrap();
        let max_time = durations.iter().max().unwrap();

        Some(TokenPerformance {
            token: token_str.to_string(),
            call_count: matching_entries.len(),
            total_time,
            average_time: avg_time,
            min_time: *min_time,
            max_time: *max_time,
            error_count: matching_entries
                .iter()
                .filter(|e| e.error.is_some())
                .count(),
        })
    }
}

/// Performance statistics for a specific token
#[derive(Debug, Clone)]
pub struct TokenPerformance {
    pub token: String,
    pub call_count: usize,
    pub total_time: Duration,
    pub average_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub error_count: usize,
}

impl TraceSummary {
    /// Create a new empty summary
    pub fn new() -> Self {
        Self {
            total_operations: 0,
            total_time: Duration::from_secs(0),
            average_time: Duration::from_secs(0),
            max_stack_depth: 0,
            error_count: 0,
            operations_by_category: std::collections::HashMap::new(),
            performance_distribution: std::collections::HashMap::new(),
            slowest_operations: Vec::new(),
        }
    }

    /// Get operations per second
    pub fn operations_per_second(&self) -> f64 {
        if self.total_time.as_secs_f64() > 0.0 {
            self.total_operations as f64 / self.total_time.as_secs_f64()
        } else {
            0.0
        }
    }

    /// Get error rate as percentage
    pub fn error_rate(&self) -> f64 {
        if self.total_operations > 0 {
            (self.error_count as f64 / self.total_operations as f64) * 100.0
        } else {
            0.0
        }
    }
}

impl TraceContext {
    /// Create a new trace context
    pub fn new(category: OperationCategory) -> Self {
        let impact = Self::classify_performance_impact(&category);

        Self {
            current_word: None,
            nesting_level: 0,
            category,
            impact,
        }
    }

    /// Create context for a word definition
    pub fn for_word<S: Into<String>>(word_name: S, category: OperationCategory) -> Self {
        let mut context = Self::new(category);
        context.current_word = Some(word_name.into());
        context
    }

    /// Create context with nesting level
    pub fn with_nesting(category: OperationCategory, level: usize) -> Self {
        let mut context = Self::new(category);
        context.nesting_level = level;
        context
    }

    /// Classify performance impact based on operation category
    fn classify_performance_impact(category: &OperationCategory) -> PerformanceImpact {
        match category {
            OperationCategory::StackOp => PerformanceImpact::Minimal,
            OperationCategory::Arithmetic => PerformanceImpact::Low,
            OperationCategory::Comparison => PerformanceImpact::Low,
            OperationCategory::Quote => PerformanceImpact::Medium,
            OperationCategory::Word => PerformanceImpact::Medium,
            OperationCategory::System => PerformanceImpact::High,
            OperationCategory::Type => PerformanceImpact::Medium,
            OperationCategory::ControlFlow => PerformanceImpact::Medium,
            OperationCategory::Other => PerformanceImpact::Low,
        }
    }
}

impl PerformanceImpact {
    /// Classify based on duration
    pub fn from_duration(duration: Duration) -> Self {
        let micros = duration.as_micros();
        match micros {
            0..=1 => PerformanceImpact::Minimal,
            2..=10 => PerformanceImpact::Low,
            11..=100 => PerformanceImpact::Medium,
            101..=1000 => PerformanceImpact::High,
            _ => PerformanceImpact::Critical,
        }
    }

    /// Get threshold in microseconds
    pub fn threshold_us(&self) -> u64 {
        match self {
            PerformanceImpact::Minimal => 1,
            PerformanceImpact::Low => 10,
            PerformanceImpact::Medium => 100,
            PerformanceImpact::High => 1000,
            PerformanceImpact::Critical => u64::MAX,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_creation() {
        let mut trace = ExecutionTrace::new();
        assert_eq!(trace.entries.len(), 0);
        assert_eq!(trace.summary.total_operations, 0);
    }

    #[test]
    fn test_trace_entry() {
        let mut trace = ExecutionTrace::new();

        let entry = TraceEntry {
            id: 0,
            timestamp: Instant::now(),
            token: Token::Word("test".to_string()),
            stack_before: vec![],
            stack_after: vec![Value::Nat(42)],
            duration: Duration::from_micros(100),
            stack_depth_change: 1,
            error: None,
            context: TraceContext::new(OperationCategory::Word),
        };

        trace.add_entry(entry);

        assert_eq!(trace.entries.len(), 1);
        assert_eq!(trace.entries[0].id, 1);
        assert_eq!(trace.summary.total_operations, 1);
    }

    #[test]
    fn test_trace_filtering() {
        let mut trace = ExecutionTrace::new();

        // Add entries with different categories
        let entry1 = TraceEntry {
            id: 0,
            timestamp: Instant::now(),
            token: Token::Word("test".to_string()),
            stack_before: vec![],
            stack_after: vec![],
            duration: Duration::from_micros(50),
            stack_depth_change: 0,
            error: None,
            context: TraceContext::new(OperationCategory::StackOp),
        };

        let entry2 = TraceEntry {
            id: 0,
            timestamp: Instant::now(),
            token: Token::Word("slow".to_string()),
            stack_before: vec![],
            stack_after: vec![],
            duration: Duration::from_micros(200),
            stack_depth_change: 0,
            error: Some("Test error".to_string()),
            context: TraceContext::new(OperationCategory::System),
        };

        trace.add_entry(entry1);
        trace.add_entry(entry2);

        // Filter by minimum duration
        let filter = TraceFilter {
            min_duration_us: 100,
            ..Default::default()
        };

        let filtered = trace.filter_entries(&filter);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].token.to_string(), "slow");

        // Filter by errors only
        let filter = TraceFilter {
            errors_only: true,
            ..Default::default()
        };

        let filtered = trace.filter_entries(&filter);
        assert_eq!(filtered.len(), 1);
        assert!(filtered[0].error.is_some());
    }

    #[test]
    fn test_performance_classification() {
        assert_eq!(
            PerformanceImpact::from_duration(Duration::from_nanos(500)),
            PerformanceImpact::Minimal
        );

        assert_eq!(
            PerformanceImpact::from_duration(Duration::from_micros(5)),
            PerformanceImpact::Low
        );

        assert_eq!(
            PerformanceImpact::from_duration(Duration::from_micros(50)),
            PerformanceImpact::Medium
        );

        assert_eq!(
            PerformanceImpact::from_duration(Duration::from_micros(500)),
            PerformanceImpact::High
        );

        assert_eq!(
            PerformanceImpact::from_duration(Duration::from_millis(5)),
            PerformanceImpact::Critical
        );
    }

    #[test]
    fn test_token_performance() {
        let mut trace = ExecutionTrace::new();

        // Add multiple entries for the same token
        for i in 0..3 {
            let entry = TraceEntry {
                id: 0,
                timestamp: Instant::now(),
                token: Token::Word("test".to_string()),
                stack_before: vec![],
                stack_after: vec![],
                duration: Duration::from_micros(100 + i * 10),
                stack_depth_change: 0,
                error: if i == 2 {
                    Some("error".to_string())
                } else {
                    None
                },
                context: TraceContext::new(OperationCategory::Word),
            };
            trace.add_entry(entry);
        }

        let perf = trace.token_performance("test").unwrap();
        assert_eq!(perf.call_count, 3);
        assert_eq!(perf.error_count, 1);
        assert_eq!(perf.min_time, Duration::from_micros(100));
        assert_eq!(perf.max_time, Duration::from_micros(120));
    }
}
