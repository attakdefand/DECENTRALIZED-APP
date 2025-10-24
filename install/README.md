# Installation Guide

This guide explains how to install and use the Decentralized Exchange Application on different platforms.

## Prerequisites

Before installing, ensure you have the following:

- Git
- Rust toolchain (latest stable version)

If Rust is not installed, the installer will automatically install it for you.

## Installation

### Windows

1. Download or clone this repository
2. Open PowerShell as Administrator
3. Navigate to the project directory
4. Run the installer:
   ```powershell
   .\install.ps1
   ```
5. Restart your terminal or log out and back in for PATH changes to take effect

### Linux/macOS

1. Download or clone this repository
2. Open a terminal
3. Navigate to the project directory
4. Make the installer executable:
   ```bash
   chmod +x install.sh
   ```
5. Run the installer:
   ```bash
   ./install.sh
   ```

## Usage

After installation, you can use the `dex` command to manage the application:

```bash
# Start all services
dex start

# Start specific services
dex start --services api,indexer

# Check service status
dex status

# Stop all services
dex stop

# Initialize the application
dex init

# Get help
dex --help
```

## Uninstallation

### Windows

1. Delete the binary from `%USERPROFILE%\.dex\bin\dex.exe`
2. Remove `%USERPROFILE%\.dex\bin` from your PATH environment variable
3. Delete the config directory at `%ProgramData%\dex`

### Linux/macOS

1. Remove the binary:
   ```bash
   sudo rm /usr/local/bin/dex
   ```
2. Remove the config directory:
   ```bash
   sudo rm -rf /etc/dex
   ```

## Troubleshooting

### Rust Installation Issues

If you encounter issues with Rust installation, try installing it manually:

1. Visit [rustup.rs](https://rustup.rs/)
2. Follow the instructions for your platform
3. Restart your terminal
4. Run the installer again

### Permission Issues

On Linux/macOS, if you encounter permission issues:

1. Ensure you're running the installer with sufficient privileges
2. Check that `/usr/local/bin` is writable or change the installation path

### PATH Issues

If the `dex` command is not found after installation:

1. Restart your terminal
2. Log out and back in (on Linux/macOS)
3. Manually add the installation directory to your PATH