# DECENTRALIZED-APP

[![CI](https://github.com/attakdefand/DECENTRALIZED-APP/actions/workflows/ci.yml/badge.svg)](https://github.com/attakdefand/DECENTRALIZED-APP/actions/workflows/ci.yml)
[![Release](https://github.com/attakdefand/DECENTRALIZED-APP/actions/workflows/release.yml/badge.svg)](https://github.com/attakdefand/DECENTRALIZED-APP/actions/workflows/release.yml)
[![Web UI](https://github.com/attakdefand/DECENTRALIZED-APP/actions/workflows/web-ui.yml/badge.svg)](https://github.com/attakdefand/DECENTRALIZED-APP/actions/workflows/web-ui.yml)
[![GitHub release](https://img.shields.io/github/release/attakdefand/DECENTRALIZED-APP.svg)](https://github.com/attakdefand/DECENTRALIZED-APP/releases)
[![GitHub packages](https://img.shields.io/badge/packages-GitHub%20Packages-blue)](https://github.com/attakdefand/DECENTRALIZED-APP/packages)

A comprehensive decentralized application built with Rust, featuring smart contracts, indexing services, API endpoints, and a WebAssembly-based user interface.

## Features

- **Smart Contracts**: Ethereum-compatible smart contracts for decentralized exchange functionality
- **Indexing Services**: High-performance data indexing for blockchain events
- **API Services**: RESTful API for interacting with the decentralized application
- **WebAssembly UI**: Modern, responsive web interface built with Rust and Yew
- **Keeper Systems**: Automated systems for maintaining protocol health
- **Monitoring**: IPFS and MEV monitoring services
- **Account Abstraction**: Built-in account abstraction bundler
- **Security**: Comprehensive security layers and testing matrices

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│   Smart        │    │   Indexing      │    │   API           │
│   Contracts    │◄──►│   Services      │◄──►│   Services      │
└─────────────────┘    └──────────────────┘    └──────────────────┘
        ▲                       ▲                       ▲
        │                       │                       │
        ▼                       ▼                       ▼
┌─────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│   Keeper       │    │   IPFS Monitor  │    │   MEV Monitor   │
│   Systems      │    │   Service       │    │   Service       │
└─────────────────┘    └──────────────────┘    └──────────────────┘
        ▲                       ▲                       ▲
        │                       │                       │
        ▼                       ▼                       ▼
┌──────────────────────────────────────────────────────────────────┐
│                    WebAssembly UI                               │
└──────────────────────────────────────────────────────────────────┘
```

## Components Overview

### Smart Contracts (`contracts/`)
- Ethereum-compatible smart contracts implementing decentralized exchange functionality
- Built with Solidity and verified with comprehensive testing

### Core Services (`services/`)
1. **Indexer Service** (`services/indexer-rs/`): Real-time blockchain event indexing
2. **API Service** (`services/api-rs/`): RESTful API for frontend interactions
3. **Keeper Systems** (`services/keepers-rs/`): Automated protocol maintenance
4. **IPFS Monitor** (`services/ipfs-rs/`): Distributed storage monitoring
5. **MEV Monitor** (`services/mev-monitor/`): Miner Extractable Value detection
6. **AA Bundler** (`services/aa-bundler/`): Account Abstraction transaction bundling

### Web Interface (`web-ui/`)
- Modern WebAssembly-based UI built with Rust and Yew framework
- Responsive design with Tailwind CSS
- Client-side routing and state management

### Core Libraries (`crates/`)
- Shared Rust libraries for common functionality across services
- Blockchain interaction utilities
- Data structures and algorithms

### Infrastructure (`infra/`)
- Deployment configurations
- Docker compose files
- Kubernetes manifests

## Complete Cross-Platform Installation

The application provides a seamless installation experience across Windows, macOS, and Linux platforms.

### Quick Installation

#### Windows

1. Download or clone this repository
2. Open PowerShell as Administrator
3. Navigate to the project directory
4. Run the installer:
   ```powershell
   .\install.ps1
   ```
5. Restart your terminal or log out and back in for PATH changes to take effect

#### Linux/macOS

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

### Using Pre-built Packages

Download pre-built packages from the [Releases page](https://github.com/attakdefand/DECENTRALIZED-APP/releases):

1. Download the package for your platform
2. Extract the archive
3. Run the installer in the extracted directory

## Usage

After installation, you can use the `dex` command to manage the application:

```bash
# Initialize the application
dex init

# Start all services
dex start

# Start specific services
dex start --services api,indexer

# Start services with custom ports
dex start --api-port 3000 --indexer-port 3001

# Check service status
dex status

# Stop all services
dex stop

# Get help
dex --help
```

### Detailed CLI Options

The CLI provides comprehensive control over all services in the DECENTRALIZED-APP:

```bash
Usage: dex-cli.exe <COMMAND>

Commands:
  start   Start all services
  stop    Stop all services
  status  Check the status of services
  init    Initialize the application
  help    Print this message or the help of the given subcommand(s)

Start all services

Usage: dex-cli.exe start [OPTIONS]

Options:
  -s, --services <SERVICES>          Services to start (api, indexer, keepers, ipfs, mev, aa)
      --api-port <API_PORT>          Port for the API service [default: 3000]
      --indexer-port <INDEXER_PORT>  Port for the indexer service [default: 3001]
  -h, --help                         Print help
```

## Getting Started

### Prerequisites

- Rust toolchain (latest stable)
- Node.js (for development tools)
- Docker (for containerized services)
- Ethereum development network (Hardhat, Ganache, or similar)

### Quick Start

1. Clone the repository:
   ```bash
   git clone https://github.com/attakdefand/DECENTRALIZED-APP.git
   cd DECENTRALIZED-APP
   ```

2. Build all components:
   ```bash
   cargo build --workspace
   ```

3. Run tests:
   ```bash
   cargo test --workspace
   ```

4. Start development services:
   ```bash
   docker-compose up
   ```

5. Start the WebAssembly UI:
   ```bash
   cd web-ui
   wasm-pack build --target web
   # Serve the pkg directory with any static file server
   ```

### Complete Cross-Platform Setup

For detailed installation instructions for Windows, macOS, Linux, and Harmony OS, see the [Complete Cross-Platform Setup Guide](COMPLETE-CROSS-PLATFORM-SETUP.md).

## Releases and Packages

### GitHub Releases

We provide pre-built binaries and Docker images for each release:

- View all [releases](https://github.com/attakdefand/DECENTRALIZED-APP/releases)
- Download signed binaries for your platform
- Use our Docker images: `decentralized-app/api:latest`, `decentralized-app/indexer:latest`, etc.

### GitHub Packages

All crates and WebAssembly packages are published to GitHub Packages:

- Rust crates: Available through the GitHub registry
- WebAssembly packages: Published for direct browser usage
- Docker images: Hosted on GitHub Container Registry

To use our packages, see the [packages documentation](docs/packages.md) and [package usage examples](docs/package_usage_example.md).

## Development Tools

The project includes several tools to streamline development:

- **Release Scripts**: Automated release creation ([create-release.sh](scripts/create-release.sh), [create-release.ps1](scripts/create-release.ps1))
- **Package Scripts**: Create distributable packages ([package.sh](scripts/package.sh), [package.ps1](scripts/package.ps1))
- **Makefile**: Common development commands (`make build`, `make test`, etc.)
- **PowerShell Makefile**: Windows equivalent of Makefile ([Makefile.ps1](Makefile.ps1))

## Documentation

- [Project Summary](docs/project_summary.md)
- [Getting Started Guide](docs/getting_started.md)
- [Deployment Guide](docs/deployment.md)
- [Security Policy](docs/security_policy.md)
- [Branching Strategy](docs/branching_strategy.md)
- [Releases](docs/releases.md)
- [Packages](docs/packages.md)
- [Package Usage Examples](docs/package_usage_example.md)
- [Installation Guide](install/README.md)
- [Complete Installation Guide](docs/complete_installation_guide.md)
- [CLI Usage Examples](docs/cli_usage_example.md)
- [Cross-Platform Installation System](docs/cross_platform_installation.md)
- [Contributing Guidelines](docs/contributing.md)
- [Code of Conduct](docs/code_of_conduct.md)

## Security

We take security seriously. Please review our [security policy](docs/security_policy.md) and [hall of fame](docs/hall_of_fame.md) for responsible disclosure.

## Contributing

Contributions are welcome! Please read our [contributing guidelines](docs/contributing.md) and [code of conduct](docs/code_of_conduct.md).

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.