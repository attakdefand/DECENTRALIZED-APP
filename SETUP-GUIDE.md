# DEX Application Setup and Configuration Guide

This document explains how to use all the setup and configuration files in this project.

## Setup Scripts

### 1. Automated Setup Scripts

#### Windows PowerShell Script: `setup-environment.ps1`
A comprehensive PowerShell script that automatically configures all required credentials and environment settings for Windows users.

**Usage:**
```powershell
# Run with default settings
.\setup-environment.ps1

# Skip specific steps
.\setup-environment.ps1 -SkipTools
.\setup-environment.ps1 -SkipGitConfig
.\setup-environment.ps1 -SkipGpgImport
.\setup-environment.ps1 -SkipEnvFile
```

**What it does:**
1. Installs required tools (Rust, Node.js, Docker, GPG) using Chocolatey
2. Configures Git with proper user settings and GPG signing
3. Imports GPG keys if available
4. Creates a .env file with all required environment variables
5. Verifies all installations

#### Unix Shell Script: `setup-environment.sh`
A bash script that performs the same setup process for Linux and macOS users.

**Usage:**
```bash
# Make executable
chmod +x setup-environment.sh

# Run with default settings
./setup-environment.sh

# Skip specific steps
./setup-environment.sh --skip-tools
./setup-environment.sh --skip-git-config
./setup-environment.sh --skip-gpg-import
./setup-environment.sh --skip-env-file
```

### 2. Verification Scripts

#### Python Verification Script: `verify-credentials.py`
A cross-platform Python script that verifies all credentials and configurations are properly set up.

**Usage:**
```bash
# Run the verification
python verify-credentials.py
```

#### Windows Batch Script: `verify-credentials.bat`
A Windows batch file that runs the Python verification script with proper error handling.

**Usage:**
```cmd
# Run the verification
verify-credentials.bat
```

## Configuration Files

### 1. Environment File: `.env`
Contains all required environment variables for the application. This file is automatically created by the setup scripts but can also be created manually.

**Key variables:**
- Database configuration (DATABASE_URL, DATABASE_NAME, etc.)
- Redis configuration (REDIS_URL)
- Ethereum RPC configuration (ETHEREUM_RPC_URL)
- IPFS configuration (IPFS_API_URL, IPFS_GATEWAY_URL)
- API service configuration (API_PORT, API_HOST)
- Security settings (SERVICE_CONTRACT_ALLOWLIST_ENABLED, etc.)

### 2. Documentation: `CREDENTIALS-AND-CONFIGURATION.md`
A comprehensive guide that documents all credentials and configuration requirements in detail.

## Build Verification Scripts

### 1. PowerShell: `verify-build.ps1`
Verifies that the Rust project builds correctly with all dependencies.

### 2. Shell Script: `verify-build.sh`
Unix equivalent of the build verification script.

## Troubleshooting Guide

### `TROUBLESHOOTING-RUST-EDITION2024.md`
Detailed solutions for common issues related to the Rust edition2024 requirement.

## How to Use These Files

### For New Users (Recommended Approach)

1. **Run the automated setup:**
   - **Windows:** `.\setup-environment.ps1`
   - **Linux/macOS:** `./setup-environment.sh`

2. **Verify the setup:**
   - **Windows:** `verify-credentials.bat`
   - **All platforms:** `python verify-credentials.py`

3. **Run build verification:**
   - **Windows:** `.\verify-build.ps1`
   - **Linux/macOS:** `./verify-build.sh`

### For Advanced Users

1. **Manual configuration:**
   - Review `CREDENTIALS-AND-CONFIGURATION.md` for detailed requirements
   - Set up tools manually as needed
   - Create `.env` file with required variables

2. **Specific troubleshooting:**
   - Refer to `TROUBLESHOOTING-RUST-EDITION2024.md` for Rust-related issues
   - Use individual verification scripts to check specific components

### For Development

1. **Environment verification:**
   - Run `python verify-credentials.py` before starting development
   - Ensure all checks pass for consistent development experience

2. **Continuous integration:**
   - These scripts can be integrated into CI/CD pipelines
   - The verification script returns appropriate exit codes for automation

## File Summary

| File | Purpose | Platform |
|------|---------|----------|
| `setup-environment.ps1` | Automated setup | Windows |
| `setup-environment.sh` | Automated setup | Linux/macOS |
| `verify-credentials.py` | Configuration verification | All |
| `verify-credentials.bat` | Windows verification runner | Windows |
| `verify-build.ps1` | Build verification | Windows |
| `verify-build.sh` | Build verification | Linux/macOS |
| `.env` | Environment variables | All |
| `CREDENTIALS-AND-CONFIGURATION.md` | Documentation | All |
| `TROUBLESHOOTING-RUST-EDITION2024.md` | Troubleshooting guide | All |

These files provide a complete solution for setting up, configuring, verifying, and troubleshooting the DEX application development environment.