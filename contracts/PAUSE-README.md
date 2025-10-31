# Smart Contract Pause/Circuit Breaker Implementation

This directory contains the implementation of smart contract pause/circuit breaker patterns including pausable functions, rate caps, and emergency controls.

## Overview

The pause/circuit breaker patterns implementation ensures:
- Proper pause/unpause functionality with role-based access control
- Rate limiting for functions and function groups
- Emergency controls that safely halt risky functions
- Comprehensive scenario testing and chaos engineering capabilities
- Fast Mean Time to Recovery (MTTR) for pause operations
- Complete paused-state coverage

## Prerequisites

Before running the pause/circuit breaker tests, you need to install the following tools:

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

The pause/circuit breaker patterns implementation consists of:
- [CircuitBreaker.sol](src/core/CircuitBreaker.sol): Core circuit breaker implementation with pause, rate limiting, and emergency controls
- [EnhancedVault.sol](src/core/EnhancedVault.sol): Vault contract integrated with circuit breaker
- [CircuitBreakerTest.sol](test/core/CircuitBreakerTest.sol): Comprehensive test suite for all pause/circuit breaker patterns

## Running Tests

### Running the Pause/Circuit Breaker Tests

To run the complete pause/circuit breaker tests:

```powershell
.\scripts\run-pause-tests.ps1
```

This script will:
1. Run unit tests for all pause/circuit breaker patterns
2. Run integration tests for circuit breaker integration
3. Run scenario tests for complex pause situations
4. Run property tests to validate system properties
5. Run edge case tests for boundary conditions
6. Run chaos tests for resilience validation
7. Run emergency control tests for critical situations
8. Produce detailed reports

### Running Individual Tests

To run unit tests:

```bash
cd contracts
forge test --match-contract CircuitBreakerTest --match-test test* -vvv
```

To run integration tests:

```bash
cd contracts
forge test --match-contract CircuitBreakerTest --match-test *Integration* -vvv
```

To run scenario tests:

```bash
cd contracts
forge test --match-contract CircuitBreakerTest --match-test *Scenario* -vvv
```

To run property tests:

```bash
cd contracts
forge test --match-contract CircuitBreakerTest --match-test testProperty* -vvv
```

To run edge case tests:

```bash
cd contracts
forge test --match-contract CircuitBreakerTest --match-test *EdgeCase* -vvv
```

To run chaos tests:

```bash
cd contracts
forge test --match-contract CircuitBreakerTest --match-test *Chaos* -vvv
```

To run emergency control tests:

```bash
cd contracts
forge test --match-contract CircuitBreakerTest --match-test *Emergency* -vvv
```

## Documentation

- [PAUSE-CIRCUIT-BREAKER.md](docs/contracts/PAUSE-CIRCUIT-BREAKER.md): Detailed documentation of the pause/circuit breaker patterns implementation
- [CircuitBreaker.sol](src/core/CircuitBreaker.sol): Core circuit breaker contract implementation
- [EnhancedVault.sol](src/core/EnhancedVault.sol): Vault contract with circuit breaker integration
- [CircuitBreakerTest.sol](test/core/CircuitBreakerTest.sol): Comprehensive test suite for all pause/circuit breaker patterns

## CI/CD Integration

The pause/circuit breaker tests are integrated into GitHub Actions through the workflow file:
- [.github/workflows/contract-pause.yml](.github/workflows/contract-pause.yml)

This workflow:
1. Runs all pause/circuit breaker tests on every push/PR to contract files
2. Generates test reports
3. Uploads results as artifacts

## Test Results

Test results and logs are stored in the `pause-results` directory after running the tests:

- Unit test logs
- Integration test logs
- Scenario test logs
- Property test logs
- Edge case test logs
- Chaos test logs
- Emergency control test logs
- Summary markdown report

## Key Pause/Circuit Breaker Patterns Implemented

### Pausable Functions

The implementation provides granular control over function pausing:
- Individual function pause/unpause with function selectors
- Function group pause/unpause for related functions
- Role-based access control for pausing and unpausing
- Event emission for all pause operations

### Rate Caps

Rate limiting prevents abuse of functions:
- Per-function rate limits with configurable windows
- Per-function group rate limits
- Per-user tracking of rate limits
- Automatic rate limit reset after time windows

### Emergency Controls

Emergency controls provide immediate halt capabilities:
- Global emergency pause for all protected functions
- 24-hour automatic expiration of emergency pauses
- Separate roles for activating and deactivating emergency pause
- Complete state tracking of emergency pause conditions

### Integration Patterns

Contracts integrate with the circuit breaker:
- Function selector mapping for pause control
- Modifier-based enforcement of pause controls
- Group-based controls for collective function management
- Continuous emergency state checking

## Troubleshooting

### Common Issues

1. **Foundry not found**: Ensure Foundry is installed and available in your PATH
2. **Test failures**: Review test output and contract logic
3. **Authorization failures**: Check role assignments and permissions
4. **Rate limit violations**: Verify rate limit configurations

### Getting Help

If you encounter issues:
1. Check the detailed logs in the pause-results directory
2. Verify all prerequisites are installed
3. Ensure dependencies are properly installed
4. Consult the documentation in [PAUSE-CIRCUIT-BREAKER.md](docs/contracts/PAUSE-CIRCUIT-BREAKER.md)

## Contributing

To contribute to the pause/circuit breaker patterns implementation:

1. Follow the existing patterns in the contracts
2. Add new test cases to [CircuitBreakerTest.sol](test/core/CircuitBreakerTest.sol)
3. Update documentation as needed
4. Ensure all tests pass before submitting changes