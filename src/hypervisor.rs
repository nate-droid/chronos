//! Hypervisor: OS-like Environment for Shell Management
//!
//! The hypervisor provides a higher-level abstraction over shell management,
//! similar to how Docker manages containers or how an OS manages processes.
//! It provides isolation, resource management, networking, and a TUI interface.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::shell::GoalType;
use crate::shell_manager::{ManagerError, ShellManager};

/// Represents a shell instance managed by the hypervisor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualShell {
    pub id: String,
    pub name: String,
    pub image: String, // Template or base configuration
    pub status: ShellStatus,
    pub created_at: u64,    // Unix timestamp
    pub last_activity: u64, // Unix timestamp
    pub resource_usage: ResourceUsage,
    pub network_config: NetworkConfig,
    pub volumes: Vec<VolumeMount>,
    pub environment: HashMap<String, String>,
    pub restart_policy: RestartPolicy,
}

/// Status of a shell instance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ShellStatus {
    Creating,
    Running,
    Paused,
    Stopped,
    Failed { error: String },
    Restarting,
}

/// Resource usage statistics for a shell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_percent: f64,
    pub memory_bytes: u64,
    pub operations_count: u64,
    pub goal_completions: u32,
    pub uptime: Duration,
}

/// Network configuration for shell communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub network_id: String,
    pub ip_address: String,
    pub exposed_ports: Vec<u16>,
    pub connected_shells: Vec<String>,
}

/// Volume mount for persistent storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMount {
    pub source: String,
    pub target: String,
    pub read_only: bool,
}

/// Restart policy for shells
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestartPolicy {
    Never,
    OnFailure { max_retries: u32 },
    Always,
    UnlessStopped,
}

/// Hypervisor configuration
#[derive(Debug, Clone)]
pub struct HypervisorConfig {
    pub max_shells: usize,
    pub default_cpu_limit: f64,
    pub default_memory_limit: u64,
    pub network_pool_size: u16,
    pub auto_cleanup: bool,
    pub monitoring_interval: Duration,
    pub log_retention_days: u32,
}

/// Network for shell communication
#[derive(Debug, Clone)]
pub struct VirtualNetwork {
    pub id: String,
    pub name: String,
    pub subnet: String,
    pub gateway: String,
    pub shells: Vec<String>,
    pub isolated: bool,
}

/// Shell image/template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellImage {
    pub name: String,
    pub version: String,
    pub base_config: HashMap<String, String>,
    pub default_goals: Vec<GoalType>,
    pub resource_requirements: ResourceRequirements,
}

/// Resource requirements for a shell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub min_memory: u64,
    pub max_memory: u64,
    pub cpu_limit: f64,
    pub max_operations: u64,
}

/// Hypervisor errors
#[derive(Debug)]
pub enum HypervisorError {
    ShellNotFound(String),
    NetworkError(String),
    ResourceLimitExceeded(String),
    ImageNotFound(String),
    InvalidConfiguration(String),
    ManagerError(ManagerError),
    IoError(io::Error),
}

impl std::fmt::Display for HypervisorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HypervisorError::ShellNotFound(id) => write!(f, "Shell not found: {}", id),
            HypervisorError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            HypervisorError::ResourceLimitExceeded(msg) => {
                write!(f, "Resource limit exceeded: {}", msg)
            }
            HypervisorError::ImageNotFound(name) => write!(f, "Image not found: {}", name),
            HypervisorError::InvalidConfiguration(msg) => {
                write!(f, "Invalid configuration: {}", msg)
            }
            HypervisorError::ManagerError(e) => write!(f, "Manager error: {}", e),
            HypervisorError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for HypervisorError {}

impl From<ManagerError> for HypervisorError {
    fn from(error: ManagerError) -> Self {
        HypervisorError::ManagerError(error)
    }
}

impl From<io::Error> for HypervisorError {
    fn from(error: io::Error) -> Self {
        HypervisorError::IoError(error)
    }
}

/// Main hypervisor struct
pub struct Hypervisor {
    shell_manager: Arc<Mutex<ShellManager>>,
    virtual_shells: Arc<Mutex<HashMap<String, VirtualShell>>>,
    networks: Arc<Mutex<HashMap<String, VirtualNetwork>>>,
    images: Arc<Mutex<HashMap<String, ShellImage>>>,
    config: HypervisorConfig,
    monitoring_handle: Option<thread::JoinHandle<()>>,
    next_shell_id: Arc<Mutex<u64>>,
}

impl Hypervisor {
    /// Create a new hypervisor instance
    pub fn new() -> Self {
        Self::with_config(HypervisorConfig::default())
    }

    /// Create hypervisor with custom configuration
    pub fn with_config(config: HypervisorConfig) -> Self {
        let mut hypervisor = Self {
            shell_manager: Arc::new(Mutex::new(ShellManager::new())),
            virtual_shells: Arc::new(Mutex::new(HashMap::new())),
            networks: Arc::new(Mutex::new(HashMap::new())),
            images: Arc::new(Mutex::new(HashMap::new())),
            config,
            monitoring_handle: None,
            next_shell_id: Arc::new(Mutex::new(1)),
        };

        // Create default network
        hypervisor.create_default_network();
        // Load default images
        hypervisor.load_default_images();
        // Start monitoring
        hypervisor.start_monitoring();

        hypervisor
    }

    /// Run a new shell instance
    pub fn run(&mut self, image: &str, name: Option<String>) -> Result<String, HypervisorError> {
        let shell_id = self.generate_shell_id();
        let shell_name = name.unwrap_or_else(|| format!("shell-{}", shell_id));

        // Get image configuration
        let image_config = {
            let images = self.images.lock().unwrap();
            images
                .get(image)
                .ok_or_else(|| HypervisorError::ImageNotFound(image.to_string()))?
                .clone()
        };

        // Create virtual shell
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let virtual_shell = VirtualShell {
            id: shell_id.clone(),
            name: shell_name.clone(),
            image: image.to_string(),
            status: ShellStatus::Creating,
            created_at: now,
            last_activity: now,
            resource_usage: ResourceUsage::default(),
            network_config: self.assign_network_config(&shell_id)?,
            volumes: Vec::new(),
            environment: image_config.base_config.clone(),
            restart_policy: RestartPolicy::OnFailure { max_retries: 3 },
        };

        // Register with shell manager
        {
            let mut manager = self.shell_manager.lock().unwrap();
            manager.create_shell(shell_id.clone())?;
        }

        // Update status to running
        let mut vs = virtual_shell;
        vs.status = ShellStatus::Running;

        // Store virtual shell
        {
            let mut shells = self.virtual_shells.lock().unwrap();
            shells.insert(shell_id.clone(), vs);
        }

        println!("Shell {} ({}) started successfully", shell_name, shell_id);
        Ok(shell_id)
    }

    /// Stop a running shell
    pub fn stop(&mut self, shell_id: &str) -> Result<(), HypervisorError> {
        // Update virtual shell status
        {
            let mut shells = self.virtual_shells.lock().unwrap();
            if let Some(shell) = shells.get_mut(shell_id) {
                shell.status = ShellStatus::Stopped;
                shell.last_activity = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
            } else {
                return Err(HypervisorError::ShellNotFound(shell_id.to_string()));
            }
        }

        // Remove from shell manager
        {
            let mut manager = self.shell_manager.lock().unwrap();
            manager.remove_shell(shell_id)?;
        }

        println!("Shell {} stopped", shell_id);
        Ok(())
    }

    /// Pause a running shell
    pub fn pause(&mut self, shell_id: &str) -> Result<(), HypervisorError> {
        let mut shells = self.virtual_shells.lock().unwrap();
        if let Some(shell) = shells.get_mut(shell_id) {
            shell.status = ShellStatus::Paused;
            shell.last_activity = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            println!("Shell {} paused", shell_id);
            Ok(())
        } else {
            Err(HypervisorError::ShellNotFound(shell_id.to_string()))
        }
    }

    /// Resume a paused shell
    pub fn resume(&mut self, shell_id: &str) -> Result<(), HypervisorError> {
        let mut shells = self.virtual_shells.lock().unwrap();
        if let Some(shell) = shells.get_mut(shell_id) {
            shell.status = ShellStatus::Running;
            shell.last_activity = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            println!("Shell {} resumed", shell_id);
            Ok(())
        } else {
            Err(HypervisorError::ShellNotFound(shell_id.to_string()))
        }
    }

    /// List all shells
    pub fn list_shells(&self) -> Vec<VirtualShell> {
        let shells = self.virtual_shells.lock().unwrap();
        shells.values().cloned().collect()
    }

    /// Get shell details
    pub fn inspect_shell(&self, shell_id: &str) -> Result<VirtualShell, HypervisorError> {
        let shells = self.virtual_shells.lock().unwrap();
        shells
            .get(shell_id)
            .cloned()
            .ok_or_else(|| HypervisorError::ShellNotFound(shell_id.to_string()))
    }

    /// Execute command in shell
    pub fn exec(&mut self, shell_id: &str, command: &str) -> Result<String, HypervisorError> {
        // Update last activity
        {
            let mut shells = self.virtual_shells.lock().unwrap();
            if let Some(shell) = shells.get_mut(shell_id) {
                shell.last_activity = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
            } else {
                return Err(HypervisorError::ShellNotFound(shell_id.to_string()));
            }
        }

        // Execute via shell manager
        let manager = self.shell_manager.lock().unwrap();
        if manager.get_shell(shell_id).is_some() {
            // This would need to be implemented in the Shell struct
            // For now, return a placeholder
            Ok(format!("Executed '{}' in shell {}", command, shell_id))
        } else {
            Err(HypervisorError::ShellNotFound(shell_id.to_string()))
        }
    }

    /// Create a new network
    pub fn create_network(&mut self, name: &str, subnet: &str) -> Result<String, HypervisorError> {
        let network_id = format!("net-{}", self.generate_network_id());
        let network = VirtualNetwork {
            id: network_id.clone(),
            name: name.to_string(),
            subnet: subnet.to_string(),
            gateway: format!(
                "{}.1",
                subnet.split('.').take(3).collect::<Vec<_>>().join(".")
            ),
            shells: Vec::new(),
            isolated: false,
        };

        let mut networks = self.networks.lock().unwrap();
        networks.insert(network_id.clone(), network);
        println!("Network {} ({}) created", name, network_id);
        Ok(network_id)
    }

    /// Start the TUI interface
    pub fn start_tui(&mut self) -> Result<(), HypervisorError> {
        println!("C∀O Hypervisor - Shell Environment Manager");
        println!("=========================================");
        println!("Type 'help' for commands, 'quit' to exit");
        println!();

        loop {
            print!("hypervisor> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            if input == "quit" || input == "exit" {
                println!("Shutting down hypervisor...");
                break;
            }

            if let Err(e) = self.handle_tui_command(input) {
                eprintln!("Error: {}", e);
            }
        }

        Ok(())
    }

    /// Handle TUI commands
    fn handle_tui_command(&mut self, input: &str) -> Result<(), HypervisorError> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        match parts[0] {
            "help" | "h" => self.show_help(),
            "ps" | "list" => self.show_shells(),
            "images" => self.show_images(),
            "networks" => self.show_networks(),
            "run" => self.handle_run_command(&parts[1..])?,
            "stop" => self.handle_stop_command(&parts[1..])?,
            "pause" => self.handle_pause_command(&parts[1..])?,
            "resume" => self.handle_resume_command(&parts[1..])?,
            "exec" => self.handle_exec_command(&parts[1..])?,
            "inspect" => self.handle_inspect_command(&parts[1..])?,
            "logs" => self.handle_logs_command(&parts[1..])?,
            "stats" => self.show_stats(),
            "network" => self.handle_network_command(&parts[1..])?,
            "cleanup" => self.cleanup_stopped_shells(),
            _ => println!(
                "Unknown command: {}. Type 'help' for available commands.",
                parts[0]
            ),
        }

        Ok(())
    }

    /// Show help information
    fn show_help(&self) {
        println!("C∀O Hypervisor Commands:");
        println!("========================");
        println!("  ps, list              List all shells");
        println!("  run <image> [name]     Run a new shell");
        println!("  stop <shell_id>        Stop a shell");
        println!("  pause <shell_id>       Pause a shell");
        println!("  resume <shell_id>      Resume a paused shell");
        println!("  exec <shell_id> <cmd>  Execute command in shell");
        println!("  inspect <shell_id>     Show detailed shell info");
        println!("  logs <shell_id>        Show shell logs");
        println!("  images                 List available images");
        println!("  networks               List networks");
        println!("  network <subcommand>   Network management");
        println!("  stats                  Show resource statistics");
        println!("  cleanup                Remove stopped shells");
        println!("  help, h                Show this help");
        println!("  quit, exit             Exit hypervisor");
    }

    /// Show all shells in a formatted table
    fn show_shells(&self) {
        let shells = self.list_shells();
        if shells.is_empty() {
            println!("No shells running");
            return;
        }

        println!(
            "{:<12} {:<15} {:<12} {:<10} {:<8} {:<15}",
            "SHELL ID", "NAME", "IMAGE", "STATUS", "CPU%", "UPTIME"
        );
        println!("{}", "-".repeat(80));

        for shell in shells {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let uptime_secs = now.saturating_sub(shell.created_at);
            let uptime_str = format!("{}s", uptime_secs);

            println!(
                "{:<12} {:<15} {:<12} {:<10} {:<8.1} {:<15}",
                &shell.id[..8.min(shell.id.len())],
                shell.name,
                shell.image,
                format!("{:?}", shell.status),
                shell.resource_usage.cpu_percent,
                uptime_str
            );
        }
    }

    /// Show available images
    fn show_images(&self) {
        let images = self.images.lock().unwrap();
        if images.is_empty() {
            println!("No images available");
            return;
        }

        println!("{:<20} {:<10} {:<30}", "IMAGE", "VERSION", "DESCRIPTION");
        println!("{}", "-".repeat(60));

        for (name, image) in images.iter() {
            println!(
                "{:<20} {:<10} {:<30}",
                name, image.version, "Shell environment"
            );
        }
    }

    /// Show networks
    fn show_networks(&self) {
        let networks = self.networks.lock().unwrap();
        if networks.is_empty() {
            println!("No networks available");
            return;
        }

        println!(
            "{:<15} {:<20} {:<15} {:<10}",
            "NETWORK ID", "NAME", "SUBNET", "SHELLS"
        );
        println!("{}", "-".repeat(60));

        for (id, network) in networks.iter() {
            println!(
                "{:<15} {:<20} {:<15} {:<10}",
                &id[..12.min(id.len())],
                network.name,
                network.subnet,
                network.shells.len()
            );
        }
    }

    /// Handle run command
    fn handle_run_command(&mut self, args: &[&str]) -> Result<(), HypervisorError> {
        if args.is_empty() {
            println!("Usage: run <image> [name]");
            return Ok(());
        }

        let image = args[0];
        let name = args.get(1).map(|s| s.to_string());

        match self.run(image, name) {
            Ok(shell_id) => println!("Started shell {}", shell_id),
            Err(e) => println!("Failed to start shell: {}", e),
        }

        Ok(())
    }

    /// Handle stop command
    fn handle_stop_command(&mut self, args: &[&str]) -> Result<(), HypervisorError> {
        if args.is_empty() {
            println!("Usage: stop <shell_id>");
            return Ok(());
        }

        let shell_id = self.resolve_shell_id(args[0])?;
        self.stop(&shell_id)
    }

    /// Handle pause command
    fn handle_pause_command(&mut self, args: &[&str]) -> Result<(), HypervisorError> {
        if args.is_empty() {
            println!("Usage: pause <shell_id>");
            return Ok(());
        }

        let shell_id = self.resolve_shell_id(args[0])?;
        self.pause(&shell_id)
    }

    /// Handle resume command
    fn handle_resume_command(&mut self, args: &[&str]) -> Result<(), HypervisorError> {
        if args.is_empty() {
            println!("Usage: resume <shell_id>");
            return Ok(());
        }

        let shell_id = self.resolve_shell_id(args[0])?;
        self.resume(&shell_id)
    }

    /// Handle exec command
    fn handle_exec_command(&mut self, args: &[&str]) -> Result<(), HypervisorError> {
        if args.len() < 2 {
            println!("Usage: exec <shell_id> <command>");
            return Ok(());
        }

        let shell_id = self.resolve_shell_id(args[0])?;
        let command = args[1..].join(" ");

        match self.exec(&shell_id, &command) {
            Ok(output) => println!("{}", output),
            Err(e) => println!("Execution failed: {}", e),
        }

        Ok(())
    }

    /// Handle inspect command
    fn handle_inspect_command(&mut self, args: &[&str]) -> Result<(), HypervisorError> {
        if args.is_empty() {
            println!("Usage: inspect <shell_id>");
            return Ok(());
        }

        let shell_id = self.resolve_shell_id(args[0])?;
        let shell = self.inspect_shell(&shell_id)?;

        println!("Shell Details:");
        println!("  ID: {}", shell.id);
        println!("  Name: {}", shell.name);
        println!("  Image: {}", shell.image);
        println!("  Status: {:?}", shell.status);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let uptime_secs = now.saturating_sub(shell.created_at);
        println!("  Created: {} (Unix timestamp)", shell.created_at);
        println!("  Uptime: {}s", uptime_secs);
        println!("  CPU: {:.1}%", shell.resource_usage.cpu_percent);
        println!("  Memory: {} bytes", shell.resource_usage.memory_bytes);
        println!("  Operations: {}", shell.resource_usage.operations_count);
        println!(
            "  Goal Completions: {}",
            shell.resource_usage.goal_completions
        );
        println!("  Network: {}", shell.network_config.ip_address);

        Ok(())
    }

    /// Handle logs command (placeholder)
    fn handle_logs_command(&mut self, args: &[&str]) -> Result<(), HypervisorError> {
        if args.is_empty() {
            println!("Usage: logs <shell_id>");
            return Ok(());
        }

        let shell_id = self.resolve_shell_id(args[0])?;
        println!("Logs for shell {}:", shell_id);
        println!("  [Logs would be displayed here]");
        println!("  [This is a placeholder implementation]");

        Ok(())
    }

    /// Handle network commands
    fn handle_network_command(&mut self, args: &[&str]) -> Result<(), HypervisorError> {
        if args.is_empty() {
            println!("Usage: network <create|list|remove> [args...]");
            return Ok(());
        }

        match args[0] {
            "create" => {
                if args.len() < 3 {
                    println!("Usage: network create <name> <subnet>");
                    return Ok(());
                }
                match self.create_network(args[1], args[2]) {
                    Ok(net_id) => println!("Created network {}", net_id),
                    Err(e) => println!("Failed to create network: {}", e),
                }
            }
            "list" => self.show_networks(),
            _ => println!("Unknown network command: {}", args[0]),
        }

        Ok(())
    }

    /// Show resource statistics
    fn show_stats(&self) {
        let shells = self.virtual_shells.lock().unwrap();
        let total_shells = shells.len();
        let running_shells = shells
            .values()
            .filter(|s| s.status == ShellStatus::Running)
            .count();
        let paused_shells = shells
            .values()
            .filter(|s| s.status == ShellStatus::Paused)
            .count();
        let stopped_shells = shells
            .values()
            .filter(|s| s.status == ShellStatus::Stopped)
            .count();

        let total_cpu: f64 = shells.values().map(|s| s.resource_usage.cpu_percent).sum();
        let total_memory: u64 = shells.values().map(|s| s.resource_usage.memory_bytes).sum();
        let total_operations: u64 = shells
            .values()
            .map(|s| s.resource_usage.operations_count)
            .sum();

        println!("Hypervisor Statistics:");
        println!("=====================");
        println!("  Total Shells: {}", total_shells);
        println!("  Running: {}", running_shells);
        println!("  Paused: {}", paused_shells);
        println!("  Stopped: {}", stopped_shells);
        println!("  Total CPU Usage: {:.1}%", total_cpu);
        println!("  Total Memory: {} bytes", total_memory);
        println!("  Total Operations: {}", total_operations);
        println!("  Max Shells: {}", self.config.max_shells);
    }

    /// Cleanup stopped shells
    fn cleanup_stopped_shells(&mut self) {
        let mut shells = self.virtual_shells.lock().unwrap();
        let stopped_ids: Vec<String> = shells
            .iter()
            .filter(|(_, shell)| shell.status == ShellStatus::Stopped)
            .map(|(id, _)| id.clone())
            .collect();

        for id in stopped_ids {
            shells.remove(&id);
        }

        println!("Cleaned up stopped shells");
    }

    /// Resolve shell ID (supports partial matching)
    fn resolve_shell_id(&self, partial_id: &str) -> Result<String, HypervisorError> {
        let shells = self.virtual_shells.lock().unwrap();

        // Try exact match first
        if shells.contains_key(partial_id) {
            return Ok(partial_id.to_string());
        }

        // Try partial match
        let matches: Vec<String> = shells
            .keys()
            .filter(|id| id.starts_with(partial_id))
            .cloned()
            .collect();

        match matches.len() {
            0 => Err(HypervisorError::ShellNotFound(partial_id.to_string())),
            1 => Ok(matches[0].clone()),
            _ => {
                println!("Ambiguous shell ID '{}'. Matches:", partial_id);
                for id in matches {
                    println!("  {}", id);
                }
                Err(HypervisorError::ShellNotFound(format!(
                    "Ambiguous ID: {}",
                    partial_id
                )))
            }
        }
    }

    /// Generate unique shell ID
    fn generate_shell_id(&self) -> String {
        let mut counter = self.next_shell_id.lock().unwrap();
        let id = format!("shell-{:08x}", *counter);
        *counter += 1;
        id
    }

    /// Generate network ID
    fn generate_network_id(&self) -> u32 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        SystemTime::now().hash(&mut hasher);
        (hasher.finish() % 10000) as u32
    }

    /// Create default network
    fn create_default_network(&mut self) {
        let default_network = VirtualNetwork {
            id: "bridge0".to_string(),
            name: "default".to_string(),
            subnet: "172.20.0.0/16".to_string(),
            gateway: "172.20.0.1".to_string(),
            shells: Vec::new(),
            isolated: false,
        };

        let mut networks = self.networks.lock().unwrap();
        networks.insert("bridge0".to_string(), default_network);
    }

    /// Load default shell images
    fn load_default_images(&mut self) {
        let default_image = ShellImage {
            name: "cao-shell".to_string(),
            version: "latest".to_string(),
            base_config: HashMap::new(),
            default_goals: Vec::new(),
            resource_requirements: ResourceRequirements {
                min_memory: 1024 * 1024,       // 1MB
                max_memory: 100 * 1024 * 1024, // 100MB
                cpu_limit: 50.0,               // 50%
                max_operations: 1000000,       // 1M operations
            },
        };

        let mut images = self.images.lock().unwrap();
        let default_clone = default_image.clone();
        images.insert("cao-shell".to_string(), default_image);
        images.insert("default".to_string(), default_clone);
    }

    /// Assign network configuration to a shell
    fn assign_network_config(&self, shell_id: &str) -> Result<NetworkConfig, HypervisorError> {
        // Simple IP assignment for now
        let ip_suffix = (shell_id.len() % 254) + 2; // Avoid .0 and .1
        Ok(NetworkConfig {
            network_id: "bridge0".to_string(),
            ip_address: format!("172.20.0.{}", ip_suffix),
            exposed_ports: Vec::new(),
            connected_shells: Vec::new(),
        })
    }

    /// Start monitoring thread
    fn start_monitoring(&mut self) {
        // Placeholder for monitoring implementation
        // In a real implementation, this would start a background thread
        // to collect metrics and update resource usage
    }
}

impl Default for HypervisorConfig {
    fn default() -> Self {
        Self {
            max_shells: 50,
            default_cpu_limit: 25.0,
            default_memory_limit: 50 * 1024 * 1024, // 50MB
            network_pool_size: 1000,
            auto_cleanup: true,
            monitoring_interval: Duration::from_secs(10),
            log_retention_days: 7,
        }
    }
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            cpu_percent: 0.0,
            memory_bytes: 0,
            operations_count: 0,
            goal_completions: 0,
            uptime: Duration::from_secs(0),
        }
    }
}
