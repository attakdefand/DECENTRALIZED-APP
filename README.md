# MONDOL Decentralized Exchange

A comprehensive decentralized application (dApp) implementing a full-featured decentralized exchange with multiple financial primitives.

## Overview

This project implements a complete decentralized exchange with the following components:

1. **Automated Market Maker (AMM)** - Constant Product, StableSwap, and Concentrated Liquidity variants
2. **Orderbook** - Central Limit Order Book implementation
3. **Lending Protocol** - Complete lending and borrowing functionality with interest rate models
4. **Oracles** - Price feeds and data aggregation mechanisms
5. **Indexer** - Blockchain event indexing and off-chain state management
6. **Keeper Bots** - Automated protocol maintenance bots
7. **Cross-chain Bridge** - Multiple bridge implementations (optimistic, ZK, light-client)
8. **Account Abstraction** - EIP-4337 smart contract wallets
9. **Governance** - Token voting, quadratic voting, and conviction voting mechanisms

## Project Structure

```
├── Cargo.toml                 # Workspace configuration
├── COMPLETE-DAPP.MD          # Production-grade dApp checklist
├── DBS-DAPP.MD               # Off-chain infrastructure specification
├── DECENTRALIZED-APP.MD       # Original specification
├── README.md                 # Project documentation
├── crates/
│   ├── core/                  # Shared components and utilities
│   ├── amm/                   # Automated Market Maker implementations
│   ├── orderbook/             # Orderbook implementation
│   ├── lending/              # Lending protocol
│   ├── oracle/               # Oracle implementations
│   ├── indexer/              # Blockchain indexer
│   ├── keeper/               # Keeper bots
│   ├── bridge/               # Cross-chain bridge
│   ├── aa/                   # Account Abstraction
│   ├── governance/           # Governance mechanisms
│   └── cli/                  # Command-line interface
├── services/                 # Off-chain services
│   ├── indexer-rs/           # Event indexer service
│   ├── api-rs/               # REST API service
│   ├── keepers-rs/           # Keeper bots service
│   ├── ipfs-rs/              # IPFS monitoring service
│   ├── mev-monitor/          # MEV monitoring service
│   └── aa-bundler/           # Account Abstraction bundler service
├── web-ui/                   # WebAssembly-based frontend
│   ├── Cargo.toml            # Web UI package configuration
│   ├── index.html            # Main HTML file
│   ├── styles.css            # Global styles
│   ├── build.ps1             # Build script
│   ├── dev-server.ps1        # Development server script
│   ├── src/                  # Rust source code
│   └── pkg/                  # Generated WebAssembly package
├── contracts/                # Smart contracts (Solidity/Vyper)
│   ├── src/                  # Smart contract source files
│   ├── script/               # Deployment and upgrade scripts
│   └── test/                 # Foundry tests
├── infra/                    # Infrastructure
│   ├── compose/              # Docker Compose configurations
│   ├── k8s/                  # Kubernetes deployments (Helm/Kustomize)
│   ├── policies/             # Policy definitions (OPA/Cedar)
│   ├── prometheus.yml        # Prometheus configuration
│   ├── prometheus_rules/     # Prometheus alerting rules
│   ├── grafana/              # Grafana dashboards and configuration
│   └── README.md             # Infrastructure documentation
├── docs/                     # Documentation
│   ├── threat-model.md       # Security threat model
│   ├── governance.md         # Governance documentation
│   ├── project_guidelines.md # Project guidelines and rules
│   ├── testing_strategy.md   # Comprehensive testing strategy
│   ├── security_policy.md    # Security policy
│   ├── guideline_role_implementation.md # GUIDELINE-ROLE-DAPP implementation
│   ├── guideline_role_framework_summary.md # Framework implementation summary
│   ├── guideline_role_quick_reference.md # Quick reference guide
│   ├── gate_checklist.md     # Gate 0 → Gate 3 checklist
│   ├── dapp_maturity_model_analysis.md # Maturity model analysis
│   ├── dapp_maturity_model_implementation.md # Maturity model implementation
│   ├── dapp_maturity_model_test_execution.md # Test execution plan
│   ├── dapp_maturity_model_integration_summary.md # Integration summary
│   ├── dapp_kpis_implementation_plan.md # KPI implementation plan
│   ├── dapp_kpi_test_execution.md # KPI test execution plan
│   ├── dapp_kpi_implementation_summary.md # KPI implementation summary
│   ├── dapp_release_gate_checklist_implementation.md # Release gate implementation
│   ├── dapp_release_gate_checklist_test_execution.md # Release gate test execution
│   ├── dapp_release_gate_checklist_summary.md # Release gate summary
│   ├── security_testing_strategy.md # Security testing approach
│   ├── supply_chain_security.md # Supply chain security measures
│   ├── observability_strategy.md # Observability implementation
│   ├── chaos_engineering.md  # Chaos engineering practices
│   ├── privacy_zk_testing.md # Privacy and ZK testing
│   ├── cross_chain_testing.md # Cross-chain testing approach
│   ├── e2e_testing.md        # End-to-end testing
│   ├── performance_scalability_testing.md # Performance testing
│   ├── bug_bounty_program.md # Bug bounty program
│   ├── shadow_fork_testing.md # Shadow fork testing approach
│   ├── adversarial_economic_testing.md # Economic testing
│   ├── bytecode_diff_signed_artifacts.md # Bytecode verification
│   ├── slo_implementation_plan.md # SLO implementation plan
│   ├── slo_implementation.md # SLO implementation documentation
│   ├── slo_test_execution.md # SLO test execution plan
│   ├── slo_implementation_summary.md # SLO implementation summary
│   ├── cicd_controls_implementation.md # CI/CD controls implementation
│   ├── cicd_implementation_summary.md # CI/CD implementation summary
│   ├── api/                  # API documentation and SDKs
│   └── runbooks/             # Operational procedures
│       ├── pause.md          # Pause procedure
│       ├── oracle.md         # Oracle failure handling
│       ├── bridge.md         # Bridge security incident
│       └── restore.md        # System restore
├── scripts/                  # Utility scripts
│   ├── build.ps1             # Build script
│   ├── test.ps1              # Test script
│   ├── test-build.ps1        # Build test script
│   ├── verify-cicd.bat       # CI/CD verification script
│   ├── run.ps1               # Run CLI script
│   ├── start-infra.ps1       # Start infrastructure
│   ├── stop-infra.ps1        # Stop infrastructure
│   ├── start-services.ps1    # Start off-chain services
│   └── start-all.ps1         # Start everything
├── tests/                    # Tests
│   ├── basic_test.rs         # Basic tests
│   ├── integration_test.rs   # Integration tests
│   ├── kpi_tests.rs          # KPI tests
│   ├── scaffold_tests.rs     # Scaffold implementation tests
│   ├── e2e/                  # End-to-end tests
│   ├── perf/                 # Performance tests
│   └── chaos/                # Chaos engineering tests
```

## Components

### Core
Shared utilities, error handling, configuration management, and logging.

### AMM (Automated Market Maker)
Implements various AMM algorithms:
- Constant Product (x*y=k)
- StableSwap with amplification factor
- Concentrated Liquidity (Uniswap V3 style)

### Orderbook
Central Limit Order Book implementation with price-time priority.

### Lending
Complete lending protocol with:
- Interest rate models (kinked model)
- Deposit/withdraw functionality
- Borrow/repay functionality
- Health factor calculations

### Oracle
Price oracle implementations:
- Medianizer for aggregating multiple feeds
- TWAP (Time Weighted Average Price) calculator
- Validation and bounds checking

### Indexer
Blockchain event indexing system:
- Event processing framework
- Database integration
- Re-indexing capabilities

### Keeper
Automated bots for protocol maintenance:
- Liquidation bots
- Oracle updaters
- Funding rate updaters

### Off-chain Services
Additional services that work with the core components:

**Indexer Service** - Streams blockchain events and materializes them into queryable database tables

**API Service** - Provides RESTful APIs for querying off-chain data

**Keepers Service** - Runs automated bots that maintain protocol health and execute time-sensitive operations

**IPFS Monitoring Service** - Monitors IPFS pin coverage to ensure data availability

**MEV Monitoring Service** - Monitors for Maximal Extractable Value incidents that could affect protocol fairness

**Account Abstraction Bundler Service** - Implements EIP-4337 bundler for processing user operations

### Web UI
WebAssembly-based frontend built with Rust and Yew framework:
- Responsive design using Tailwind CSS utility classes
- Component-based architecture
- Integration with backend API services
- Real-time market data visualization
- Order management interface
- Liquidity pool interaction

### Infrastructure
Complete infrastructure setup for all off-chain components:

**Docker Compose** - Local development and testing environment with all services

**Kubernetes** - Production deployment configurations using Helm charts and Kustomize overlays

**Policies** - Security policies implemented with OPA and Cedar

**Monitoring** - Comprehensive observability stack with Prometheus and Grafana

### Bridge
Cross-chain bridge implementations:
- Light-client verification
- Optimistic bridges with challenge periods
- ZK-proof bridges

### Account Abstraction (AA)
EIP-4337 implementation:
- User operations
- Smart contract wallets
- Session keys with limited permissions
- Paymaster integration

### Governance
Various governance mechanisms:
- Token-based voting
- Quadratic voting
- Conviction voting

## Getting Started

### Prerequisites
- Rust 1.60 or higher
- Cargo package manager
- Docker and Docker Compose for infrastructure
- wasm-pack for WebAssembly compilation

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running the CLI

```bash
cargo run --bin dex --help
```

### Building the Web UI

Navigate to the web-ui directory and run:

```powershell
cd web-ui
.\build.ps1
```

### Running the Web UI Development Server

```powershell
cd web-ui
.\dev-server.ps1
```

## Testing

Each module includes comprehensive unit tests covering core functionality. Integration tests are located in the `tests/` directory of each crate.

To run all tests:

```bash
cargo test --workspace
```

For more information about our testing strategy, see [docs/testing_strategy.md](docs/testing_strategy.md).

## Security

The project follows a comprehensive security approach with multiple layers of protection. For details, see:

- [docs/threat-model.md](docs/threat-model.md) - Security threat model
- [docs/security_policy.md](docs/security_policy.md) - Security policy
- [docs/runbooks/](docs/runbooks/) - Incident response procedures

## Project Guidelines

The project follows the guidelines outlined in [docs/project_guidelines.md](docs/project_guidelines.md), which implement the framework from GUIDELINE-ROLE-DAPP.MD.

For a detailed implementation of the GUIDELINE-ROLE-DAPP.MD framework, see:
- [docs/guideline_role_implementation.md](docs/guideline_role_implementation.md) - Complete implementation plan
- [docs/gate_checklist.md](docs/gate_checklist.md) - Gate 0 → Gate 3 checklist

## KPI Monitoring

The project implements comprehensive KPI monitoring as defined in dapp_kpis.csv. For details, see:

- [docs/dapp_kpis_implementation_plan.md](docs/dapp_kpis_implementation_plan.md) - KPI implementation plan
- [docs/dapp_kpi_test_execution.md](docs/dapp_kpi_test_execution.md) - KPI test execution plan
- [docs/dapp_kpi_implementation_summary.md](docs/dapp_kpi_implementation_summary.md) - KPI implementation summary

## Release Gate Checklist

The project implements the dApp Release Gate Checklist for progression from Canary to GA. For details, see:

- [docs/dapp_release_gate_checklist_implementation.md](docs/dapp_release_gate_checklist_implementation.md) - Implementation plan
- [docs/dapp_release_gate_checklist_test_execution.md](docs/dapp_release_gate_checklist_test_execution.md) - Test execution plan
- [docs/dapp_release_gate_checklist_summary.md](docs/dapp_release_gate_checklist_summary.md) - Implementation summary

## SLO Implementation

The project implements comprehensive Service Level Objectives (SLOs) as defined in dapp_slos.yaml. For details, see:

- [docs/slo_implementation_plan.md](docs/slo_implementation_plan.md) - SLO implementation plan
- [docs/slo_implementation.md](docs/slo_implementation.md) - SLO implementation documentation
- [docs/slo_test_execution.md](docs/slo_test_execution.md) - SLO test execution plan
- [docs/slo_implementation_summary.md](docs/slo_implementation_summary.md) - SLO implementation summary

## CI/CD Implementation

The project implements comprehensive CI/CD controls as defined in dapp_cicd_controls.md. For details, see:

- [docs/cicd_controls_implementation.md](docs/cicd_controls_implementation.md) - CI/CD controls implementation
- [docs/cicd_implementation_summary.md](docs/cicd_implementation_summary.md) - CI/CD implementation summary

The implementation includes:
- Continuous integration with comprehensive testing
- Release management with signing and provenance
- Deployment automation with environment promotion gates

## Repository Structure

The repository follows a standardized structure that aligns with industry best practices for decentralized applications:

- **contracts/** - Smart contract source code, deployment scripts, and tests
- **services/** - Off-chain services implemented in Rust
- **web-ui/** - WebAssembly-based frontend
- **infra/** - Infrastructure configurations for Docker, Kubernetes, and policies
- **docs/** - Comprehensive documentation covering all aspects of the system
- **tests/** - Various types of tests including unit, integration, e2e, performance, and chaos engineering

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Repository

This project is hosted at: https://github.com/attakdefand/MONDOL-DECENTRALIZED-EXCHANGE.git#   D E C E N T R A L I Z E D - A P P  
 