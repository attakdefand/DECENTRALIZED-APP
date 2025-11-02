# Group A - Smart Contracts Testing

This directory contains all documentation and resources related to Group A testing for the DECENTRALIZED-APP project.

## Overview

Group A focuses on the foundational layer of testing for the decentralized application, ensuring the core smart contract logic and mathematical safety. This is the most critical testing group as it ensures the security and correctness of the underlying protocol.

## Directory Structure

```
Group-A/
├── 1. group_a_testing_plan.md          # Detailed testing plan for all Group A domains
├── 2. existing_tests_mapping.md        # Mapping of existing tests to Group A domains
├── 3. test_coverage_matrix.md          # Coverage analysis showing what's tested and what's missing
├── 4. test_execution_plan.md           # How to execute Group A tests and CI/CD integration
├── 5. test_development_guide.md        # Guide for developing new Group A tests
├── 6. group_a_summary.md               # Comprehensive summary of Group A testing
└── README.md                           # This file
```

## Testing Domains

1. **Logic & Math Testing** - CEI patterns, reentrancy guards, access control, mathematical safety
2. **Upgradeability Testing** - Storage layout, proxy mechanisms, upgrade processes
3. **AMM/DEX Testing** - Constant product formulas, liquidity operations, fee distribution
4. **Orderbook Testing** - Matching engine, price-time priority, order types
5. **Lending/Perps Testing** - Supply/borrow operations, liquidation mechanisms, risk management
6. **Oracle Testing** - Price feeds, staleness detection, outlier rejection
7. **MEV & Fairness Testing** - MEV detection/prevention, fair ordering mechanisms
8. **Account Abstraction Testing** - UserOps validation, paymaster functionality
9. **Tx/Mempool Testing** - Privacy, replay protection, transaction routing
10. **Cross-chain/Bridges Testing** - Proof verification, challenge windows, replay guard

## Tools and Technologies

- **Foundry** - Ethereum testing framework for unit, fuzz, and invariant tests
- **Slither** - Static analysis tool for vulnerability detection
- **Echidna** - Property-based fuzz testing tool

## Execution Scripts

- **PowerShell**: `scripts/run-group-a-tests.ps1`
- **Bash**: `scripts/run-group-a-tests.sh`

## CI/CD Integration

The Group A tests are integrated into the CI/CD pipeline with the following requirements:
- All unit, fuzz, and invariant tests must pass
- Slither static analysis must show no critical vulnerabilities
- Gas consumption must stay within defined thresholds

## Test Development

Follow the guidelines in [test_development_guide.md](5. test_development_guide.md) when creating new tests for Group A domains.

## Related Files

- Existing test files in `contracts/test/core/`
- Configuration in `contracts/foundry.toml`
- Documentation in `contracts/test/INVARIANTS.md`