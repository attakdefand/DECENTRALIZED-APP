# MONDOL Decentralized Exchange

A comprehensive decentralized application (dApp) implementing a full-featured decentralized exchange with multiple financial primitives.

[![CI](https://github.com/attakdefand/MONDOL-DECENTRALIZED-EXCHANGE/actions/workflows/ci.yml/badge.svg)](https://github.com/attakdefand/MONDOL-DECENTRALIZED-EXCHANGE/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/attakdefand/MONDOL-DECENTRALIZED-EXCHANGE#license)

## Overview

MONDOL Decentralized Exchange is a next-generation decentralized finance (DeFi) platform that combines the best of Automated Market Makers (AMMs), Orderbooks, and advanced financial primitives. Built with security, scalability, and composability in mind, it provides a complete ecosystem for decentralized trading, lending, and cross-chain asset management.

## Key Features

- **Hybrid Trading**: AMM and Orderbook implementations for optimal liquidity
- **Advanced Financial Primitives**: Lending protocols, oracles, and governance mechanisms
- **Cross-Chain Compatibility**: Multiple bridge implementations for seamless asset transfers
- **Account Abstraction**: EIP-4337 support for enhanced user experience
- **Comprehensive Observability**: Full monitoring stack with Prometheus and Grafana
- **Enterprise-Grade Security**: Multi-layered security architecture with formal verification

## Architecture

```
├── 📦 Smart Contracts (Solidity/Vyper)
├── ⚙️ Core Services (Rust)
├── 🌐 Web Interface (WebAssembly/Rust)
├── 🏗️ Infrastructure (Docker/Kubernetes)
└── 📊 Monitoring (Prometheus/Grafana)
```

### Core Components

1. **Automated Market Maker (AMM)**
   - Constant Product (x*y=k)
   - StableSwap with amplification factor
   - Concentrated Liquidity (Uniswap V3 style)

2. **Orderbook System**
   - Central Limit Order Book
   - Price-time priority matching

3. **Lending Protocol**
   - Interest rate models
   - Deposit/withdraw functionality
   - Borrow/repay with health factor calculations

4. **Oracle Infrastructure**
   - Price feed aggregation
   - TWAP calculations
   - Bounds checking

5. **Cross-Chain Bridges**
   - Light-client verification
   - Optimistic bridges
   - ZK-proof bridges

6. **Account Abstraction**
   - EIP-4337 implementation
   - Smart contract wallets
   - Session key management

## Technology Stack

- **Smart Contracts**: Solidity, Vyper
- **Backend Services**: Rust (Tokio, Axum)
- **Frontend**: WebAssembly (Yew Framework)
- **Infrastructure**: Docker, Kubernetes, Helm
- **Monitoring**: Prometheus, Grafana
- **Testing**: Foundry, Property-based testing, Fuzzing
- **Security**: OPA, Cedar, Formal Verification

## Quick Start

### Prerequisites

- Rust 1.60+
- Docker and Docker Compose
- wasm-pack (for Web UI)
- Node.js (for development server)

### Building the Project

```bash
# Build all components
cargo build

# Build Web UI
cd web-ui && wasm-pack build --target web
```

### Running Services

```powershell
# Start all infrastructure
.\scripts\start-infra.ps1

# Start all services
.\scripts\start-services.ps1

# Start everything
.\scripts\start-all.ps1
```

### Running Tests

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --features integration

# Run all tests
cargo test --workspace
```

### Development Web UI

```powershell
# Navigate to web-ui directory
cd web-ui

# Start development server
.\dev-server.ps1

# Access at http://localhost:8080
```

## Project Structure

```
├── Cargo.toml              # Workspace configuration
├── crates/                 # Core Rust libraries
│   ├── core/               # Shared utilities
│   ├── amm/                # AMM implementations
│   ├── orderbook/          # Orderbook system
│   ├── lending/            # Lending protocol
│   ├── oracle/             # Oracle systems
│   ├── indexer/            # Blockchain indexer
│   ├── keeper/             # Keeper bots
│   ├── bridge/             # Cross-chain bridges
│   ├── aa/                 # Account abstraction
│   └── governance/         # Governance mechanisms
├── services/               # Off-chain services
│   ├── api-rs/             # REST API service
│   ├── indexer-rs/         # Event indexer
│   ├── keepers-rs/         # Keeper bots service
│   ├── ipfs-rs/            # IPFS monitoring
│   ├── mev-monitor/        # MEV monitoring
│   └── aa-bundler/         # AA bundler service
├── web-ui/                 # WebAssembly frontend
├── contracts/              # Smart contracts
├── infra/                  # Infrastructure configs
├── docs/                   # Documentation
└── scripts/                # Utility scripts
```

## Documentation

Comprehensive documentation is available in the [docs/](docs/) directory:

- [Project Guidelines](docs/project_guidelines.md)
- [Security Model](docs/threat-model.md)
- [Testing Strategy](docs/testing_strategy.md)
- [Deployment Guide](docs/deployment.md)
- [API Reference](docs/api/)

## Security

This project follows industry best practices for security:

- Multi-layered security architecture
- Formal verification of critical components
- Continuous security scanning
- Bug bounty program
- Supply chain security measures

For security-related issues, please refer to our [Security Policy](docs/security_policy.md).

## Contributing

We welcome contributions from the community! Please see our [Contributing Guidelines](docs/contributing.md) for details on how to get started.

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Project Status

This project follows the [DApp Maturity Model](docs/dapp_maturity_model_implementation.md):

- ✅ L1: Prototype Ready
- ✅ L2: Testnet Ready
- 🚧 L3: Production Ready
- 🔜 L4: Mainnet Ready
- 🔜 L5: Cross-Chain Ready

## Repository

**GitHub**: https://github.com/attakdefand/MONDOL-DECENTRALIZED-EXCHANGE

**Issues**: https://github.com/attakdefand/MONDOL-DECENTRALIZED-EXCHANGE/issues

**Discussions**: https://github.com/attakdefand/MONDOL-DECENTRALIZED-EXCHANGE/discussions