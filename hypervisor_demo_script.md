# C∀O Hypervisor Demo Script

This script demonstrates the hypervisor functionality that provides an OS-like environment for managing C∀O shell instances.

## What We Built

The C∀O Hypervisor is a comprehensive shell management system that provides:

1. **Container-like Shell Management** - Similar to Docker containers
2. **Resource Monitoring** - CPU, memory, and operation tracking
3. **Virtual Networking** - Isolated communication channels
4. **TUI Interface** - Interactive command-line management
5. **Lifecycle Management** - Start, stop, pause, resume shells

## Live Demo Commands

### 1. Start the Hypervisor

```bash
# Run the main C∀O program
cargo run --bin chronos

# Enter hypervisor mode
C∀O> hypervisor
```

### 2. Basic Shell Management

```bash
# Show help
hypervisor> help

# List available images
hypervisor> images

# Run new shells
hypervisor> run default fibonacci-calculator
hypervisor> run cao-shell prime-finder
hypervisor> run default theorem-prover

# List running shells
hypervisor> ps
```

### 3. Shell Operations

```bash
# Inspect a shell
hypervisor> inspect fibonacci-calculator

# Execute commands in shells
hypervisor> exec fibonacci-calculator "5 6 +"
hypervisor> exec prime-finder "17 19 *"

# Pause and resume
hypervisor> pause prime-finder
hypervisor> ps
hypervisor> resume prime-finder
```

### 4. Resource Monitoring

```bash
# Show system statistics
hypervisor> stats

# Monitor resource usage
hypervisor> ps

# Inspect detailed shell info
hypervisor> inspect prime-finder
```

### 5. Network Management

```bash
# Show networks
hypervisor> networks

# Create a new network
hypervisor> network create secure-net 10.0.0.0/24
hypervisor> networks
```

### 6. Cleanup and Exit

```bash
# Stop shells
hypervisor> stop fibonacci-calculator
hypervisor> stop prime-finder

# Clean up stopped shells
hypervisor> cleanup

# Check final status
hypervisor> ps
hypervisor> stats

# Exit hypervisor
hypervisor> quit
```

## Demo Output Examples

### Shell Listing
```
SHELL ID     NAME                IMAGE        STATUS     CPU%     UPTIME
------------ ------------------- ------------ ---------- -------- ---------------
shell-00     fibonacci-calculator default      Running    0.0      45s
shell-00     prime-finder        cao-shell    Running    0.0      32s
shell-00     theorem-prover      default      Running    0.0      15s
```

### Shell Inspection
```
Shell Details:
  ID: shell-00000001
  Name: fibonacci-calculator
  Image: default
  Status: Running
  Created: 1234567890 (Unix timestamp)
  Uptime: 67s
  CPU: 0.0%
  Memory: 0 bytes
  Operations: 0
  Goal Completions: 0
  Network: 172.20.0.2
```

### System Statistics
```
Hypervisor Statistics:
=====================
  Total Shells: 3
  Running: 2
  Paused: 0
  Stopped: 1
  Total CPU Usage: 0.0%
  Total Memory: 0 bytes
  Total Operations: 0
  Max Shells: 50
```

## Architecture Highlights

### Layered Design
```
┌─────────────────────────────────────────┐
│            Hypervisor TUI               │
├─────────────────────────────────────────┤
│         Virtual Shell Manager          │
├─────────────────────────────────────────┤
│        Physical Shell Manager          │
├─────────────────────────────────────────┤
│    Shell 1    Shell 2    Shell 3      │
│   ┌─────────┐ ┌─────────┐ ┌─────────┐    │
│   │  REPL   │ │  REPL   │ │  REPL   │    │
│   │   VM    │ │   VM    │ │   VM    │    │
│   └─────────┘ └─────────┘ └─────────┘    │
└─────────────────────────────────────────┘