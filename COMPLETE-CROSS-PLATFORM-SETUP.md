# Complete Cross-Platform Setup Guide for DECENTRALIZED-APP

This comprehensive guide provides detailed instructions for installing and running the DECENTRALIZED-APP on Windows, macOS, Linux, and Harmony OS operating systems.

## System Requirements

### Minimum Requirements
- **RAM**: 8GB
- **Disk Space**: 10GB free space
- **Internet Connection**: Required for downloading dependencies

### Recommended Requirements
- **RAM**: 16GB or more
- **Disk Space**: 20GB free space
- **CPU**: Multi-core processor

## Prerequisites

Before installation, ensure you have:
- Git (for downloading the source code)
- Internet connection (for downloading dependencies)

The installer will automatically install Rust if it's not present.

## Installation Methods

### Method 1: Source Installation (Recommended)

#### Windows

1. **Install Git** (if not already installed):
   - Download from https://git-scm.com/download/win
   - Follow the installation wizard with default settings

2. **Open PowerShell as Administrator**:
   - Press Win + X and select "Windows PowerShell (Admin)"
   - Navigate to your desired directory:
     ```powershell
     cd C:\Users\YourUsername\Documents
     ```

3. **Download the source code**:
   ```powershell
   git clone https://github.com/attakdefand/DECENTRALIZED-APP.git
   cd DECENTRALIZED-APP
   ```

4. **Run the installer**:
   ```powershell
   .\install.ps1
   ```

5. **Restart your terminal** or log out and back in for PATH changes to take effect.

#### macOS

1. **Install Git** (if not already installed):
   - Install Xcode Command Line Tools:
     ```bash
     xcode-select --install
     ```
   - Or install via Homebrew:
     ```bash
     brew install git
     ```

2. **Open Terminal**:
   - Press Cmd + Space and type "Terminal"
   - Navigate to your desired directory:
     ```bash
     cd ~/Documents
     ```

3. **Download the source code**:
   ```bash
   git clone https://github.com/attakdefand/DECENTRALIZED-APP.git
   cd DECENTRALIZED-APP
   ```

4. **Make the installer executable**:
   ```bash
   chmod +x install.sh
   ```

5. **Run the installer**:
   ```bash
   ./install.sh
   ```

#### Linux (Ubuntu/Debian)

1. **Install Git** (if not already installed):
   ```bash
   sudo apt update
   sudo apt install git
   ```

2. **Open Terminal**:
   - Press Ctrl + Alt + T
   - Navigate to your desired directory:
     ```bash
     cd ~/Documents
     ```

3. **Download the source code**:
   ```bash
   git clone https://github.com/attakdefand/DECENTRALIZED-APP.git
   cd DECENTRALIZED-APP
   ```

4. **Make the installer executable**:
   ```bash
   chmod +x install.sh
   ```

5. **Run the installer**:
   ```bash
   ./install.sh
   ```

#### Linux (Red Hat/CentOS/Fedora)

1. **Install Git** (if not already installed):
   ```bash
   # For CentOS/RHEL
   sudo yum install git
   
   # For Fedora
   sudo dnf install git
   ```

2. **Open Terminal** and follow steps 2-5 from the Ubuntu/Debian section above.

#### Harmony OS

1. **Install Git** (if not already installed):
   ```bash
   # For Harmony OS (OpenHarmony-based)
   pkg install git
   # Or if using a Debian-based Harmony OS variant:
   apt-get update && apt-get install git
   ```

2. **Open Terminal**:
   - Navigate to your desired directory:
     ```bash
     cd ~/Documents
     ```

3. **Download the source code**:
   ```bash
   git clone https://github.com/attakdefand/DECENTRALIZED-APP.git
   cd DECENTRALIZED-APP
   ```

4. **Make the installer executable**:
   ```bash
   chmod +x install.sh
   ```

5. **Run the installer**:
   ```bash
   ./install.sh
   ```

**Note**: Harmony OS support is experimental. Some features may require additional configuration or may not be fully supported depending on the specific Harmony OS variant and version.

### Method 2: Pre-built Packages

For each release, pre-built packages are available:

1. **Download** the appropriate package for your platform from the [Releases page](https://github.com/attakdefand/DECENTRALIZED-APP/releases)
2. **Extract** the archive
3. **Run** the installer in the extracted directory

## Verifying Installation

After installation, verify that the CLI is working:

```bash
# Check version
dex --version

# Show help
dex --help
```

## Running the Application

### Initialize the Application

```bash
dex init
```

### Start All Services

```bash
dex start
```

This command starts all services:
- API Service (http://localhost:3000)
- Indexer Service (http://localhost:3001)
- Keepers Service
- IPFS Monitor Service
- MEV Monitor Service
- AA Bundler Service

### Start Specific Services

```bash
# Start only API and Indexer services
dex start --services api,indexer

# Start with custom ports
dex start --api-port 8080 --indexer-port 8081
```

### Check Service Status

```bash
dex status
```

### Stop All Services

```bash
dex stop
```

## Service Endpoints

Once services are running, you can access them at:

- **API Service**: http://localhost:3000
  - Health check: `curl http://localhost:3000/health`
  - Metrics: `curl http://localhost:3000/metrics`

- **Indexer Service**: http://localhost:3001
  - Health check: `curl http://localhost:3001/health`

## Development Workflow

For developers, you can also run services directly without installation:

```bash
# Start all infrastructure
docker-compose -f infra/compose/docker-compose.yml up -d

# Start all services
cargo run --bin dex-cli -- start

# Or start individual services
cargo run -p api-service
cargo run -p indexer-service
# etc.
```

## Automation Scripts

The project includes PowerShell scripts for Windows and shell scripts for Unix-like systems:

- **Windows**: `scripts\start-all.ps1`, `scripts\start-infra.ps1`, `scripts\start-services.ps1`, `scripts\stop-infra.ps1`
- **macOS/Linux**: `scripts/start-all.sh`, `scripts/start-infra.sh`, `scripts/start-services.sh`, `scripts/stop-infra.sh`

## Troubleshooting

### Common Issues and Solutions

1. **Port Conflicts**
   ```bash
   # Start services with custom ports
   dex start --api-port 8080 --indexer-port 8081
   ```

2. **Permission Issues (Linux/macOS)**
   ```bash
   # Run with sudo if needed
   sudo dex start
   ```

3. **Service Not Responding**
   ```bash
   # Check service status
   dex status
   
   # Restart services
   dex stop
   dex start
   ```

4. **Rust Installation Issues**
   - Visit [rustup.rs](https://rustup.rs/) and follow manual installation instructions
   - Restart your terminal after installation

5. **PATH Issues**
   - Restart your terminal
   - Log out and back in (on Linux/macOS)
   - Manually add the installation directory to your PATH

### Platform-Specific Issues

#### Windows

1. **Execution Policy Errors**:
   ```powershell
   Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
   ```

2. **Antivirus Interference**:
   - Add the DECENTRALIZED-APP directory to your antivirus exclusions

#### macOS

1. **Gatekeeper Issues**:
   ```bash
   sudo xattr -rd com.apple.quarantine /path/to/dex
   ```

2. **Homebrew Issues**:
   - Ensure Homebrew is up to date: `brew update`

#### Linux

1. **Package Manager Issues**:
   - Update package lists: `sudo apt update` or `sudo yum update`

2. **Docker Permissions**:
   ```bash
   sudo usermod -aG docker $USER
   # Log out and back in
   ```

## Uninstallation

### Windows

1. Delete the binary from `%USERPROFILE%\.dex\bin\dex.exe`
2. Remove `%USERPROFILE%\.dex\bin` from your PATH environment variable:
   - Open System Properties → Advanced → Environment Variables
   - Edit the PATH variable and remove the entry
3. Delete the config directory at `%ProgramData%\dex`

### macOS/Linux

1. Remove the binary:
   ```bash
   sudo rm /usr/local/bin/dex
   ```

2. Remove the config directory:
   ```bash
   sudo rm -rf /etc/dex
   ```

## Security Considerations

- All binaries are signed and verified
- Installation requires appropriate permissions
- Configuration files are protected
- Secure communication between services

## Getting Help

```bash
# Show help for all commands
dex --help

# Show help for specific command
dex start --help

# View logs
dex status
```

## Next Steps

After successful installation:

1. **Start the application**: `dex start`
2. **Verify services are running**: `dex status`
3. **Access the Web UI**: Open a browser and navigate to the appropriate URL
4. **Explore the API**: Visit http://localhost:3000 for API documentation
5. **Check monitoring**: Access Grafana/Prometheus dashboards if configured

This setup provides a complete environment for running the DECENTRALIZED-APP with all security layers implemented and ready for use.