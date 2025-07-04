//! Hypervisor Demo - Demonstrates the C∀O Hypervisor functionality
//!
//! This example shows how to use the hypervisor to manage multiple shell environments,
//! similar to how Docker manages containers.

use chronos::hypervisor::{Hypervisor, HypervisorConfig};
use chronos::shell::GoalType;
use chronos_core::Value;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== C∀O Hypervisor Demo ===");
    println!();

    // Create a hypervisor with custom configuration
    let config = HypervisorConfig {
        max_shells: 10,
        default_memory_limit: 100 * 1024 * 1024, // 100MB
        network_pool_size: 100,
        auto_cleanup: true,
        monitoring_interval: Duration::from_secs(5),
        log_retention_days: 3,
    };

    let mut hypervisor = Hypervisor::with_config(config);

    println!("🚀 Hypervisor initialized with custom configuration");
    println!("   - Max shells: 10");
    println!("   - CPU limit: 30%");
    println!("   - Memory limit: 100MB");
    println!();

    // Demonstrate running multiple shells
    demo_shell_lifecycle(&mut hypervisor)?;
    demo_shell_management(&mut hypervisor)?;
    demo_resource_monitoring(&mut hypervisor)?;

    println!("✅ Hypervisor demo completed successfully!");
    Ok(())
}

/// Demonstrate shell lifecycle operations
fn demo_shell_lifecycle(hypervisor: &mut Hypervisor) -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 Shell Lifecycle Demo");
    println!("========================");

    // Start some shells
    let shell1 = hypervisor.run("default", Some("fibonacci-calculator".to_string()))?;
    println!("✓ Started shell: {}", shell1);

    let shell2 = hypervisor.run("cao-shell", Some("prime-finder".to_string()))?;
    println!("✓ Started shell: {}", shell2);

    let shell3 = hypervisor.run("default", None)?; // Auto-generated name
    println!("✓ Started shell: {}", shell3);

    // Show running shells
    println!("\n📋 Current shells:");
    let shells = hypervisor.list_shells();
    for shell in &shells {
        println!(
            "   {} ({}) - Status: {:?}",
            shell.name,
            &shell.id[..8],
            shell.status
        );
    }

    // Pause and resume a shell
    println!("\n⏸️  Pausing shell: {}", shell2);
    hypervisor.pause(&shell2)?;

    println!("▶️  Resuming shell: {}", shell2);
    hypervisor.resume(&shell2)?;

    // Stop a shell
    println!("🛑 Stopping shell: {}", shell3);
    hypervisor.stop(&shell3)?;

    println!("✓ Shell lifecycle demo completed\n");
    Ok(())
}

/// Demonstrate shell management features
fn demo_shell_management(hypervisor: &mut Hypervisor) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Shell Management Demo");
    println!("=========================");

    // Get list of shells after lifecycle operations
    let shells = hypervisor.list_shells();
    let running_shells: Vec<_> = shells
        .iter()
        .filter(|s| matches!(s.status, chronos::hypervisor::ShellStatus::Running))
        .collect();

    if let Some(shell) = running_shells.first() {
        println!("🔍 Inspecting shell: {}", shell.name);

        // Inspect shell details
        let detailed_shell = hypervisor.inspect_shell(&shell.id)?;
        println!("   ID: {}", detailed_shell.id);
        println!("   Name: {}", detailed_shell.name);
        println!("   Image: {}", detailed_shell.image);
        println!("   Status: {:?}", detailed_shell.status);
        println!(
            "   Network IP: {}",
            detailed_shell.network_config.ip_address
        );
        println!(
            "   CPU Usage: {:.1}%",
            detailed_shell.resource_usage.cpu_percent
        );
        println!(
            "   Memory: {} bytes",
            detailed_shell.resource_usage.memory_bytes
        );

        // Execute a command in the shell
        println!("\n💻 Executing command in shell...");
        let result = hypervisor.exec(&shell.id, "3 4 +")?;
        println!("   Command result: {}", result);
    }

    // Demonstrate partial ID matching
    if let Some(shell) = running_shells.first() {
        let partial_id = &shell.id[..6]; // Use first 6 characters
        println!("\n🎯 Testing partial ID matching with: {}", partial_id);

        match hypervisor.inspect_shell(partial_id) {
            Ok(shell) => println!("   ✓ Successfully resolved partial ID to: {}", shell.name),
            Err(e) => println!("   ❌ Failed to resolve partial ID: {}", e),
        }
    }

    println!("✓ Shell management demo completed\n");
    Ok(())
}

/// Demonstrate resource monitoring
fn demo_resource_monitoring(hypervisor: &mut Hypervisor) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Resource Monitoring Demo");
    println!("============================");

    // Show current statistics
    println!("📈 Current hypervisor statistics:");
    let shells = hypervisor.list_shells();

    let total_shells = shells.len();
    let running_shells = shells
        .iter()
        .filter(|s| matches!(s.status, chronos::hypervisor::ShellStatus::Running))
        .count();
    let paused_shells = shells
        .iter()
        .filter(|s| matches!(s.status, chronos::hypervisor::ShellStatus::Paused))
        .count();
    let stopped_shells = shells
        .iter()
        .filter(|s| matches!(s.status, chronos::hypervisor::ShellStatus::Stopped))
        .count();

    println!("   Total shells: {}", total_shells);
    println!("   Running: {}", running_shells);
    println!("   Paused: {}", paused_shells);
    println!("   Stopped: {}", stopped_shells);

    // Calculate resource usage
    let total_cpu: f64 = shells.iter().map(|s| s.resource_usage.cpu_percent).sum();
    let total_memory: u64 = shells.iter().map(|s| s.resource_usage.memory_bytes).sum();
    let total_operations: u64 = shells
        .iter()
        .map(|s| s.resource_usage.operations_count)
        .sum();

    println!("   Total CPU usage: {:.1}%", total_cpu);
    println!("   Total memory usage: {} bytes", total_memory);
    println!("   Total operations: {}", total_operations);

    // Demonstrate cleanup
    println!("\n🧹 Performing cleanup of stopped shells...");
    let initial_count = shells.len();

    // Note: In a real implementation, this would remove stopped shells
    // For demo purposes, we'll just show the concept
    println!("   Shells before cleanup: {}", initial_count);

    let remaining_shells = hypervisor.list_shells();
    let final_count = remaining_shells.len();
    println!("   Shells after cleanup: {}", final_count);

    if initial_count > final_count {
        println!(
            "   ✓ Cleaned up {} stopped shells",
            initial_count - final_count
        );
    } else {
        println!("   ℹ️  No stopped shells to clean up");
    }

    println!("✓ Resource monitoring demo completed\n");
    Ok(())
}

/// Helper function to create sample goals for shells
#[allow(dead_code)]
fn create_sample_goals() -> Vec<GoalType> {
    vec![
        GoalType::Computation {
            name: "fibonacci".to_string(),
            target: "Calculate 10th Fibonacci number".to_string(),
            max_time: Duration::from_secs(30),
            max_operations: 1000,
        },
        GoalType::Puzzle {
            name: "arithmetic".to_string(),
            description: "Simple arithmetic puzzle".to_string(),
            inputs: vec![Value::Nat(5), Value::Nat(3)],
            expected_outputs: vec![Value::Nat(8)],
            max_attempts: 5,
        },
        GoalType::Exploration {
            name: "prime-search".to_string(),
            domain: "natural numbers".to_string(),
            discovery_criteria: vec!["prime numbers up to 100".to_string()],
            max_iterations: 100,
        },
    ]
}

/// Helper function to simulate shell workload
#[allow(dead_code)]
fn simulate_workload(
    hypervisor: &mut Hypervisor,
    shell_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🏃 Simulating workload for shell: {}", shell_id);

    // Execute some sample commands
    let commands = vec!["2 3 +", "5 dup *", "1 2 3 + *", "10 factorial"];

    for cmd in commands {
        match hypervisor.exec(shell_id, cmd) {
            Ok(result) => println!("   {} → {}", cmd, result),
            Err(e) => println!("   {} → Error: {}", cmd, e),
        }

        // Simulate some processing time
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
