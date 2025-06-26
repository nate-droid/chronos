//! Shell Manager for Coordinating Multiple Autonomous Shells
//!
//! This module provides coordination and management capabilities for multiple
//! shell instances, enabling them to work collaboratively or independently
//! while sharing resources and knowledge.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::shell::{Goal, Shell, ShellError};

/// Coordination strategy for multiple shells
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CoordinationStrategy {
    /// Shells work independently without coordination
    Independent,
    /// Shells share discoveries and collaborate on goals
    Collaborative {
        knowledge_sharing: bool,
        load_balancing: bool,
    },
    /// Shells work in a hierarchical manner with leader coordination
    Hierarchical {
        leader_id: String,
        delegation_rules: Vec<String>,
    },
    /// Shells compete to solve goals first
    Competitive {
        reward_system: bool,
        performance_tracking: bool,
    },
}

/// Shared knowledge base between shells
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedKnowledge {
    /// Successful solution patterns discovered by any shell
    pub solution_patterns: HashMap<String, (String, f64)>, // pattern -> (solution, confidence)
    /// Failed approaches to avoid
    pub failed_approaches: HashMap<String, Vec<String>>,
    /// Discovered theorems and axioms
    pub theorems: HashMap<String, String>,
    /// Performance benchmarks for different operations
    pub benchmarks: HashMap<String, Duration>,
    /// Inter-shell communication logs
    pub communication_log: Vec<CommunicationEntry>,
}

/// Communication between shells
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationEntry {
    pub timestamp: u64, // timestamp in seconds since epoch
    pub from_shell: String,
    pub to_shell: Option<String>, // None for broadcast
    pub message_type: MessageType,
    pub content: String,
}

/// Types of messages shells can send
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    /// Request help with a specific problem
    HelpRequest,
    /// Share a discovery or solution
    Discovery,
    /// Report completion of a goal
    Completion,
    /// Request coordination for a complex task
    CoordinationRequest,
    /// Status update
    StatusUpdate,
    /// Resource sharing request
    ResourceRequest,
}

/// Resource allocation and limits across shells
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePool {
    pub total_memory: usize,
    pub total_cpu_time: Duration,
    pub allocated_memory: HashMap<String, usize>, // shell_id -> allocated memory
    pub allocated_cpu_time: HashMap<String, Duration>, // shell_id -> allocated time
    pub max_shells: usize,
}

/// Manager for coordinating multiple shells
pub struct ShellManager {
    /// Active shell instances
    shells: HashMap<String, Arc<Mutex<Shell>>>,
    /// Coordination strategy
    strategy: CoordinationStrategy,
    /// Shared knowledge base
    knowledge: Arc<Mutex<SharedKnowledge>>,
    /// Resource pool and allocation
    resources: Arc<Mutex<ResourcePool>>,
    /// Manager configuration
    config: ManagerConfig,
    /// Performance metrics
    metrics: ManagerMetrics,
    /// Active coordination threads
    coordination_handles: Vec<thread::JoinHandle<()>>,
}

/// Configuration for the shell manager
#[derive(Debug, Clone)]
pub struct ManagerConfig {
    pub max_shells: usize,
    pub coordination_interval: Duration,
    pub knowledge_sync_interval: Duration,
    pub resource_check_interval: Duration,
    pub auto_load_balance: bool,
    pub enable_learning: bool,
}

/// Performance metrics for the manager
#[derive(Debug, Clone)]
pub struct ManagerMetrics {
    pub total_goals_completed: u64,
    pub total_goals_failed: u64,
    pub average_completion_time: Duration,
    pub shells_created: u64,
    pub shells_destroyed: u64,
    pub coordination_events: u64,
    pub knowledge_shares: u64,
}

/// Errors specific to shell management
#[derive(Debug)]
pub enum ManagerError {
    ShellError(ShellError),
    ResourceError(String),
    CoordinationError(String),
    ConfigurationError(String),
    ConcurrencyError(String),
}

impl std::fmt::Display for ManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManagerError::ShellError(e) => write!(f, "Shell error: {}", e),
            ManagerError::ResourceError(msg) => write!(f, "Resource error: {}", msg),
            ManagerError::CoordinationError(msg) => write!(f, "Coordination error: {}", msg),
            ManagerError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            ManagerError::ConcurrencyError(msg) => write!(f, "Concurrency error: {}", msg),
        }
    }
}

impl std::error::Error for ManagerError {}

impl From<ShellError> for ManagerError {
    fn from(error: ShellError) -> Self {
        ManagerError::ShellError(error)
    }
}

impl Default for ManagerConfig {
    fn default() -> Self {
        Self {
            max_shells: 10,
            coordination_interval: Duration::from_millis(100),
            knowledge_sync_interval: Duration::from_secs(5),
            resource_check_interval: Duration::from_secs(1),
            auto_load_balance: true,
            enable_learning: true,
        }
    }
}

impl Default for ManagerMetrics {
    fn default() -> Self {
        Self {
            total_goals_completed: 0,
            total_goals_failed: 0,
            average_completion_time: Duration::from_secs(0),
            shells_created: 0,
            shells_destroyed: 0,
            coordination_events: 0,
            knowledge_shares: 0,
        }
    }
}

impl Default for SharedKnowledge {
    fn default() -> Self {
        Self {
            solution_patterns: HashMap::new(),
            failed_approaches: HashMap::new(),
            theorems: HashMap::new(),
            benchmarks: HashMap::new(),
            communication_log: Vec::new(),
        }
    }
}

impl Default for ResourcePool {
    fn default() -> Self {
        Self {
            total_memory: 1024 * 1024 * 1024,          // 1GB
            total_cpu_time: Duration::from_secs(3600), // 1 hour
            allocated_memory: HashMap::new(),
            allocated_cpu_time: HashMap::new(),
            max_shells: 10,
        }
    }
}

impl ShellManager {
    /// Create a new shell manager
    pub fn new() -> Self {
        Self {
            shells: HashMap::new(),
            strategy: CoordinationStrategy::Independent,
            knowledge: Arc::new(Mutex::new(SharedKnowledge::default())),
            resources: Arc::new(Mutex::new(ResourcePool::default())),
            config: ManagerConfig::default(),
            metrics: ManagerMetrics::default(),
            coordination_handles: Vec::new(),
        }
    }

    /// Create a new shell manager with custom configuration
    pub fn with_config(config: ManagerConfig) -> Self {
        Self {
            shells: HashMap::new(),
            strategy: CoordinationStrategy::Independent,
            knowledge: Arc::new(Mutex::new(SharedKnowledge::default())),
            resources: Arc::new(Mutex::new(ResourcePool::default())),
            config,
            metrics: ManagerMetrics::default(),
            coordination_handles: Vec::new(),
        }
    }

    /// Create a new shell instance
    pub fn create_shell(&mut self, shell_id: String) -> Result<(), ManagerError> {
        if self.shells.len() >= self.config.max_shells {
            return Err(ManagerError::ResourceError(
                "Maximum number of shells reached".to_string(),
            ));
        }

        if self.shells.contains_key(&shell_id) {
            return Err(ManagerError::ConfigurationError(format!(
                "Shell {} already exists",
                shell_id
            )));
        }

        let shell = Shell::new(shell_id.clone());
        self.shells.insert(shell_id, Arc::new(Mutex::new(shell)));
        self.metrics.shells_created += 1;

        Ok(())
    }

    /// Remove a shell instance
    pub fn remove_shell(&mut self, shell_id: &str) -> Result<(), ManagerError> {
        if let Some(_) = self.shells.remove(shell_id) {
            // TODO: Clean up resources allocated to this shell
            self.metrics.shells_destroyed += 1;
            Ok(())
        } else {
            Err(ManagerError::ConfigurationError(format!(
                "Shell {} not found",
                shell_id
            )))
        }
    }

    /// Get a reference to a shell
    pub fn get_shell(&self, shell_id: &str) -> Option<Arc<Mutex<Shell>>> {
        self.shells.get(shell_id).cloned()
    }

    /// List all active shell IDs
    pub fn list_shells(&self) -> Vec<String> {
        self.shells.keys().cloned().collect()
    }

    /// Assign a goal to a specific shell
    pub fn assign_goal(&mut self, shell_id: &str, goal: Goal) -> Result<(), ManagerError> {
        if let Some(shell_arc) = self.shells.get(shell_id) {
            let mut shell = shell_arc
                .lock()
                .map_err(|_| ManagerError::ConcurrencyError("Failed to lock shell".to_string()))?;
            shell.add_goal(goal)?;
            Ok(())
        } else {
            Err(ManagerError::ConfigurationError(format!(
                "Shell {} not found",
                shell_id
            )))
        }
    }

    /// Assign a goal to the best available shell based on current strategy
    pub fn assign_goal_auto(&mut self, goal: Goal) -> Result<String, ManagerError> {
        let best_shell_id = self.select_best_shell_for_goal(&goal)?;
        self.assign_goal(&best_shell_id, goal)?;
        Ok(best_shell_id)
    }

    /// Select the best shell for a given goal based on current strategy
    fn select_best_shell_for_goal(&self, _goal: &Goal) -> Result<String, ManagerError> {
        if self.shells.is_empty() {
            return Err(ManagerError::ResourceError(
                "No shells available".to_string(),
            ));
        }

        match &self.strategy {
            CoordinationStrategy::Independent => {
                // Select shell with fewest active goals
                let mut best_shell = None;
                let mut min_goals = usize::MAX;

                for (shell_id, shell_arc) in &self.shells {
                    if let Ok(shell) = shell_arc.lock() {
                        let goal_count = shell.goal_status().len();
                        if goal_count < min_goals {
                            min_goals = goal_count;
                            best_shell = Some(shell_id.clone());
                        }
                    }
                }

                best_shell.ok_or_else(|| {
                    ManagerError::CoordinationError("No available shell found".to_string())
                })
            }
            CoordinationStrategy::Collaborative { .. } => {
                // Select shell with most relevant experience
                // TODO: Implement experience-based selection
                self.shells.keys().next().cloned().ok_or_else(|| {
                    ManagerError::CoordinationError("No available shell found".to_string())
                })
            }
            CoordinationStrategy::Hierarchical { leader_id, .. } => {
                // Delegate to leader or find available subordinate
                if self.shells.contains_key(leader_id) {
                    Ok(leader_id.clone())
                } else {
                    self.shells.keys().next().cloned().ok_or_else(|| {
                        ManagerError::CoordinationError("No available shell found".to_string())
                    })
                }
            }
            CoordinationStrategy::Competitive { .. } => {
                // Assign to multiple shells for competition
                // For now, just pick the first available
                self.shells.keys().next().cloned().ok_or_else(|| {
                    ManagerError::CoordinationError("No available shell found".to_string())
                })
            }
        }
    }

    /// Start autonomous execution for all shells
    pub fn start_all_autonomous(&mut self) -> Result<(), ManagerError> {
        for (shell_id, shell_arc) in &self.shells {
            if let Ok(mut shell) = shell_arc.lock() {
                if let Err(e) = shell.start_autonomous() {
                    eprintln!(
                        "Failed to start autonomous mode for shell {}: {}",
                        shell_id, e
                    );
                }
            }
        }
        Ok(())
    }

    /// Stop autonomous execution for all shells
    pub fn stop_all_autonomous(&mut self) -> Result<(), ManagerError> {
        for (_shell_id, shell_arc) in &self.shells {
            if let Ok(mut shell) = shell_arc.lock() {
                shell.stop_autonomous();
            }
        }
        Ok(())
    }

    /// Execute one coordination step across all shells
    pub fn coordination_step(&mut self) -> Result<(), ManagerError> {
        self.metrics.coordination_events += 1;

        // Check shell statuses
        self.check_shell_statuses()?;

        // Perform strategy-specific coordination
        let strategy = self.strategy.clone();
        match strategy {
            CoordinationStrategy::Independent => {
                // No coordination needed
            }
            CoordinationStrategy::Collaborative {
                knowledge_sharing,
                load_balancing,
            } => {
                if knowledge_sharing {
                    self.share_knowledge()?;
                }
                if load_balancing {
                    self.balance_load()?;
                }
            }
            CoordinationStrategy::Hierarchical { .. } => {
                self.coordinate_hierarchical()?;
            }
            CoordinationStrategy::Competitive { .. } => {
                self.coordinate_competitive()?;
            }
        }

        Ok(())
    }

    /// Check the status of all shells and handle completed goals
    fn check_shell_statuses(&mut self) -> Result<(), ManagerError> {
        let shell_ids: Vec<String> = self.shells.keys().cloned().collect();
        let is_collaborative = matches!(
            self.strategy,
            CoordinationStrategy::Collaborative {
                knowledge_sharing: true,
                ..
            }
        );

        let mut completions = Vec::new();

        for shell_id in shell_ids {
            if let Some(shell_arc) = self.shells.get(&shell_id) {
                if let Ok(mut shell) = shell_arc.lock() {
                    let completed_goals = shell.check_completion();
                    for goal_id in completed_goals {
                        self.metrics.total_goals_completed += 1;
                        // TODO: Record completion time and update metrics

                        completions.push((shell_id.clone(), goal_id));
                    }
                }
            }
        }

        // Handle completions after releasing shell locks
        if is_collaborative {
            for (shell_id, goal_id) in completions {
                self.broadcast_completion(&shell_id, &goal_id)?;
            }
        }

        Ok(())
    }

    /// Share knowledge between shells
    fn share_knowledge(&mut self) -> Result<(), ManagerError> {
        self.metrics.knowledge_shares += 1;
        // TODO: Implement knowledge sharing logic
        // - Collect successful patterns from all shells
        // - Distribute to shells that could benefit
        // - Update shared knowledge base
        Ok(())
    }

    /// Balance load across shells
    fn balance_load(&mut self) -> Result<(), ManagerError> {
        // TODO: Implement load balancing logic
        // - Identify overloaded shells
        // - Find underutilized shells
        // - Migrate goals between shells
        Ok(())
    }

    /// Coordinate hierarchical strategy
    fn coordinate_hierarchical(&mut self) -> Result<(), ManagerError> {
        // TODO: Implement hierarchical coordination
        // - Check leader status
        // - Handle delegation requests
        // - Coordinate subordinate shells
        Ok(())
    }

    /// Coordinate competitive strategy
    fn coordinate_competitive(&mut self) -> Result<(), ManagerError> {
        // TODO: Implement competitive coordination
        // - Track performance metrics
        // - Award points for successful completions
        // - Handle competition results
        Ok(())
    }

    /// Broadcast a goal completion to all shells
    fn broadcast_completion(
        &mut self,
        completing_shell: &str,
        goal_id: &str,
    ) -> Result<(), ManagerError> {
        let message = CommunicationEntry {
            timestamp: current_timestamp(),
            from_shell: completing_shell.to_string(),
            to_shell: None, // Broadcast
            message_type: MessageType::Completion,
            content: goal_id.to_string(),
        };

        if let Ok(mut knowledge) = self.knowledge.lock() {
            knowledge.communication_log.push(message);
        }

        Ok(())
    }

    /// Set the coordination strategy
    pub fn set_strategy(&mut self, strategy: CoordinationStrategy) {
        self.strategy = strategy;
    }

    /// Get current manager statistics
    pub fn get_statistics(&self) -> String {
        format!(
            "Shell Manager Statistics:\n\
             - Active Shells: {}\n\
             - Total Goals Completed: {}\n\
             - Total Goals Failed: {}\n\
             - Shells Created: {}\n\
             - Coordination Events: {}\n\
             - Knowledge Shares: {}",
            self.shells.len(),
            self.metrics.total_goals_completed,
            self.metrics.total_goals_failed,
            self.metrics.shells_created,
            self.metrics.coordination_events,
            self.metrics.knowledge_shares
        )
    }

    /// Save manager state to file
    pub fn save_state(&self, _path: &str) -> Result<(), ManagerError> {
        // TODO: Implement state serialization
        // - Save all shell states
        // - Save shared knowledge
        // - Save coordination strategy
        // - Save metrics
        Ok(())
    }

    /// Load manager state from file
    pub fn load_state(&mut self, _path: &str) -> Result<(), ManagerError> {
        // TODO: Implement state deserialization
        Ok(())
    }

    /// Shutdown the manager and all shells gracefully
    pub fn shutdown(&mut self) -> Result<(), ManagerError> {
        // Stop all autonomous execution
        self.stop_all_autonomous()?;

        // Wait for coordination threads to finish
        for handle in self.coordination_handles.drain(..) {
            if let Err(_) = handle.join() {
                eprintln!("Warning: Coordination thread did not shut down cleanly");
            }
        }

        // Clear all shells
        self.shells.clear();

        Ok(())
    }
}

/// Helper function to get current timestamp in seconds since epoch
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

impl Default for ShellManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::{CompletionStatus, GoalType};

    #[test]
    fn test_manager_creation() {
        let manager = ShellManager::new();
        assert_eq!(manager.shells.len(), 0);
        assert!(matches!(
            manager.strategy,
            CoordinationStrategy::Independent
        ));
    }

    #[test]
    fn test_shell_creation_and_removal() {
        let mut manager = ShellManager::new();

        // Create a shell
        assert!(manager.create_shell("test_shell".to_string()).is_ok());
        assert_eq!(manager.shells.len(), 1);
        assert_eq!(manager.metrics.shells_created, 1);

        // Remove the shell
        assert!(manager.remove_shell("test_shell").is_ok());
        assert_eq!(manager.shells.len(), 0);
        assert_eq!(manager.metrics.shells_destroyed, 1);
    }

    #[test]
    fn test_goal_assignment() {
        let mut manager = ShellManager::new();
        manager.create_shell("test_shell".to_string()).unwrap();

        let goal = Goal {
            id: "test_goal".to_string(),
            goal_type: GoalType::Computation {
                name: "Simple addition".to_string(),
                target: "2 3 +".to_string(),
                max_time: Duration::from_secs(60),
                max_operations: 100,
            },
            status: CompletionStatus::InProgress {
                attempts: 0,
                last_attempt: None,
                partial_results: vec![],
            },
            priority: 1,
            created_at: current_timestamp(),
            context: HashMap::new(),
            progress_log: vec![],
        };

        assert!(manager.assign_goal("test_shell", goal).is_ok());
    }

    #[test]
    fn test_coordination_strategies() {
        let mut manager = ShellManager::new();

        // Test setting different strategies
        manager.set_strategy(CoordinationStrategy::Collaborative {
            knowledge_sharing: true,
            load_balancing: true,
        });

        manager.set_strategy(CoordinationStrategy::Hierarchical {
            leader_id: "leader".to_string(),
            delegation_rules: vec!["rule1".to_string()],
        });

        manager.set_strategy(CoordinationStrategy::Competitive {
            reward_system: true,
            performance_tracking: true,
        });
    }
}
