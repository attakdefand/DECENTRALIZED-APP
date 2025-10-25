# Smart Contract Math Safety Patterns Implementation

This directory contains the implementation of smart contract math safety patterns including overflow/underflow protection, precision handling, and value conservation.

## Overview

The math safety patterns implementation ensures:
- Proper overflow/underflow protection using Solidity 0.8+ built-in checks
- Precise fixed-point arithmetic for financial calculations
- Value conservation in AMM and lending operations
- Safe fee calculations with proper rounding
- Differential testing against reference models
- Property-based testing for mathematical invariants

## Prerequisites

Before running the math safety tests, you need to install the following tools:

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

The math safety patterns implementation is built into the smart contracts themselves:
- [MathSafetyDemo.sol](src/core/MathSafetyDemo.sol): Implements comprehensive math safety patterns
- [SafeToken.sol](src/core/SafeToken.sol): Implements basic math safety patterns

## Running Tests

### Running the Math Safety Tests

To run the complete math safety tests:

```powershell
.\scripts\run-math-safety-tests.ps1
```

This script will:
1. Run unit tests for all math safety patterns
2. Run fuzz tests to validate invariants
3. Run invariant tests to ensure value conservation
4. Run edge case tests for boundary conditions
5. Run differential tests against reference models
6. Produce detailed reports

### Running Individual Tests

To run unit tests:

```bash
cd contracts
forge test --match-contract MathSafetyTest --match-test test* -vvv
```

To run fuzz tests:

```bash
cd contracts
forge test --match-contract MathSafetyTest --match-test testProperty* -vvv
```

To run invariant tests:

```bash
cd contracts
forge test --match-contract MathSafetyTest --match-test *Invariant* -vvv
```

To run edge case tests:

```bash
cd contracts
forge test --match-contract MathSafetyTest --match-test *EdgeCase* -vvv
```

To run differential tests:

```bash
cd contracts
forge test --match-contract MathSafetyTest --match-test *Reference* -vvv
```

## Documentation

- [MATH-SAFETY.md](docs/contracts/MATH-SAFETY.md): Detailed documentation of the math safety patterns implementation
- [MathSafetyDemo.sol](src/core/MathSafetyDemo.sol): Contract implementing advanced math safety patterns
- [MathSafetyTest.sol](test/core/MathSafetyTest.sol): Comprehensive test suite for all math safety patterns

## CI/CD Integration

The math safety tests are integrated into GitHub Actions through the workflow file:
- [.github/workflows/contract-math-safety.yml](.github/workflows/contract-math-safety.yml)

This workflow:
1. Runs all math safety tests on every push/PR to contract files
2. Generates test reports
3. Uploads results as artifacts

## Test Results

Test results and logs are stored in the `math-safety-results` directory after running the tests:

- Unit test logs
- Fuzz test logs
- Invariant test logs
- Edge case test logs
- Differential test logs
- Summary markdown report

## Key Math Safety Patterns Implemented

### Overflow/Underflow Protection

All contracts use Solidity 0.8+ built-in overflow/underflow protection:
- Automatic checks for arithmetic operations
- Manual bounds checking where needed
- SafeMath library for explicit clarity

### Precision Handling

Financial calculations use proper precision handling:
- Fixed-point arithmetic with 18 decimal places
- Basis points for percentage calculations
- Proper rounding in fee calculations

### Value Conservation

Value conservation is maintained in all operations:
- AMM constant product formula (x * y = k)
- Lending conservation (total supplied >= total borrowed)
- Regular invariant checking

### Arithmetic Safety

All arithmetic operations are performed safely:
- Fee calculations with bounds checking
- Interest calculations with time-based rates
- Slippage protection for user safety

## Troubleshooting

### Common Issues

1. **Foundry not found**: Ensure Foundry is installed and available in your PATH
2. **Test failures**: Review test output and contract logic
3. **Overflow errors**: Ensure using Solidity 0.8+ or SafeMath
4. **Precision issues**: Check fixed-point arithmetic implementation

### Getting Help

If you encounter issues:
1. Check the detailed logs in the math-safety-results directory
2. Verify all prerequisites are installed
3. Ensure dependencies are properly installed
4. Consult the documentation in [MATH-SAFETY.md](docs/contracts/MATH-SAFETY.md)

## Contributing

To contribute to the math safety patterns implementation:

1. Follow the existing patterns in the contracts
2. Add new test cases to [MathSafetyTest.sol](test/core/MathSafetyTest.sol)
3. Update documentation as needed
4. Ensure all tests pass before submitting changes