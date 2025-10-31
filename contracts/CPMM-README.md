# Constant Product Market Maker (CPMM) Implementation

This directory contains the implementation of a Constant Product Market Maker (CPMM) with comprehensive safety checks, fee routing, and conservation invariants.

## Overview

The CPMM implementation ensures:
- Proper constant product formula (x * y = k) implementation
- Safe fee calculations with protocol fee routing
- Value conservation in all operations
- Comprehensive testing with property-based and differential testing
- Slippage protection for user safety
- Liquidity provision and removal with optimal amount calculations

## Prerequisites

Before running the CPMM tests, you need to install the following tools:

1. **Foundry**: A fast, portable, and modular toolkit for Ethereum application development
2. **Git**: For cloning dependencies

### Installing Foundry

To install Foundry, run the following command:

```bash
curl -L https://foundry.paradigm.xyz | bash
```

Then restart your terminal and run:

```bash
foundryup
```

## Configuration

The CPMM implementation consists of:
- [CPMM.sol](src/core/CPMM.sol): Core CPMM implementation with constant product formula
- [CPMMTest.sol](test/core/CPMMTest.sol): Comprehensive test suite for all CPMM patterns

## Running Tests

### Running the CPMM Tests

To run the complete CPMM tests:

```powershell
.\scripts\run-cpmm-tests.ps1
```

This script will:
1. Run unit tests for all CPMM patterns
2. Run fuzz tests to validate invariants
3. Run invariant tests to ensure value conservation
4. Run edge case tests for boundary conditions
5. Run differential tests against reference models
6. Produce detailed reports

### Running Individual Tests

To run unit tests:

```bash
cd contracts
forge test --match-contract CPMMTest --match-test test* -vvv
```

To run fuzz tests:

```bash
cd contracts
forge test --match-contract CPMMTest --match-test testProperty* -vvv
```

To run invariant tests:

```bash
cd contracts
forge test --match-contract CPMMTest --match-test *Invariant* -vvv
```

To run edge case tests:

```bash
cd contracts
forge test --match-contract CPMMTest --match-test *EdgeCase* -vvv
```

To run differential tests:

```bash
cd contracts
forge test --match-contract CPMMTest --match-test *Reference* -vvv
```

## Documentation

- [CPMM-README.md](CPMM-README.md): This setup and usage guide
- [CPMM.md](docs/contracts/CPMM.md): Detailed documentation of the CPMM implementation
- [CPMM.sol](src/core/CPMM.sol): Core CPMM contract implementation
- [CPMMTest.sol](test/core/CPMMTest.sol): Comprehensive test suite for all CPMM patterns

## CI/CD Integration

The CPMM tests are integrated into GitHub Actions through the workflow file:
- [.github/workflows/contract-cpmm.yml](.github/workflows/contract-cpmm.yml)

This workflow:
1. Runs all CPMM tests on every push/PR to contract files
2. Generates test reports
3. Uploads results as artifacts

## Test Results

Test results and logs are stored in the `cpmm-results` directory after running the tests:

- Unit test logs
- Fuzz test logs
- Invariant test logs
- Edge case test logs
- Differential test logs
- Summary markdown report

## Key CPMM Patterns Implemented

### Constant Product Formula

The implementation provides the constant product formula (x * y = k):
- Proper reserve updates according to the formula
- Fee integration to maintain the invariant
- Precision handling with fixed-point arithmetic
- Overflow protection with Solidity 0.8+

### Fee Routing

Comprehensive fee routing with:
- Configurable swap fees for each pool
- Protocol fees taken from swap fees
- Precise fee calculation with bounds checking
- Automatic fee distribution to recipients

### Slippage Bounds

Slippage protection ensures user safety:
- Minimum output amounts for swaps
- Maximum input amounts for desired outputs
- Configurable slippage limits
- Protection against sandwich attacks and MEV

### Liquidity Operations

Liquidity provision and removal with:
- Optimal deposit amounts based on current pool ratios
- Liquidity token minting and burning
- Proportional withdrawal of tokens
- Minimum liquidity locking to prevent rounding issues

## Troubleshooting

### Common Issues

1. **Foundry not found**: Ensure Foundry is installed and available in your PATH
2. **Test failures**: Review test output and contract logic
3. **Invariant violations**: Check constant product formula implementation
4. **Fee mismatches**: Verify fee calculation and routing logic

### Getting Help

If you encounter issues:
1. Check the detailed logs in the cpmm-results directory
2. Verify all prerequisites are installed
3. Ensure dependencies are properly installed
4. Consult the documentation in [CPMM.md](docs/contracts/CPMM.md)

## Contributing

To contribute to the CPMM implementation:

1. Follow the existing patterns in the contracts
2. Add new test cases to [CPMMTest.sol](test/core/CPMMTest.sol)
3. Update documentation as needed
4. Ensure all tests pass before submitting changes