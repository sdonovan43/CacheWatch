# Installing Rust on Linux for CacheWatch

This guide provides instructions for installing Rust on Linux systems to develop and run the CacheWatch-Rust-SQLite project.

## Prerequisites

- A Linux distribution (Ubuntu, Debian, Fedora, CentOS, etc.)
- Internet connection
- Sudo privileges for system package management

## Method 1: Using rustup (Recommended)

The easiest way to install Rust is using `rustup`, the official Rust toolchain installer:

```bash
# Download and run the rustup installer
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source the environment variables (or restart your terminal)
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

## Method 2: Using Package Manager

### Ubuntu/Debian:
```bash
sudo apt update
sudo apt install rustc cargo
```

### Fedora:
```bash
sudo dnf install rust cargo
```

### CentOS/RHEL:
```bash
sudo yum install rust cargo
```

## Verify Installation

After installation, verify that Rust is properly installed:

```bash
rustc --version
cargo --version
```

## Install Additional Dependencies

For building CacheWatch, you may need additional system dependencies:

### Ubuntu/Debian:
```bash
sudo apt install build-essential libsqlite3-dev
```

### Fedora:
```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install sqlite-devel
```

### CentOS/RHEL:
```bash
sudo yum groupinstall "Development Tools"
sudo yum install sqlite-devel
```

## Build CacheWatch for Linux

Once Rust is installed, you can build the CacheWatch project:

```bash
# Clone or navigate to your CacheWatch directory
cd /path/to/CacheWatch-Rust-SQLite

# Build the project
cargo build --bin cli-display

# Run the CLI tool
./target/debug/cli-display
```

## Cross-compilation (Optional)

If you want to cross-compile for different platforms:

```bash
# Install target for different platforms
rustup target add x86_64-unknown-linux-gnu

# Build for Linux
cargo build --bin cli-display --target x86_64-unknown-linux-gnu
```

## Troubleshooting

### If `cargo` command is not found:
1. Make sure you've sourced the environment: `source $HOME/.cargo/env`
2. Check if rustup is installed: `which rustup`

### If you get permission errors:
```bash
# Fix permissions for Cargo directory
chmod -R 755 ~/.cargo
```

## Next Steps

After installing Rust, you can proceed with:

1. Building the CacheWatch CLI tool: `cargo build --bin cli-display`
2. Running the tool to see telemetry data
3. Preparing for Tauri GUI development (see INSTALL_RUST_TAURI.md)

This installation guide ensures your Linux system is properly configured to develop and run the CacheWatch-Rust-SQLite project.