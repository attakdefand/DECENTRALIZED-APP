# Smart Contract Event Schema Implementation

This directory contains the implementation of smart contract event schemas with proper indexing, auditability, and analytics readiness.

## Overview

The event schema implementation ensures:
- Proper event emission with consistent naming and structured data
- Indexed topics for efficient querying and analytics
- Comprehensive coverage of all critical state changes
- Auditability and transparency for all contract operations
- Analytics readiness with structured event data
- Event snapshot comparison capabilities
- ABI/event schema linting for consistency
- High event coverage percentage across all contract functions

## Prerequisites

Before running the event schema tests, you need to install the following tools:

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

The event schema implementation consists of:
- [EventSchema.sol](src/core/EventSchema.sol): Core event schema implementation with standardized events
- [EventSchemaToken.sol](src/core/EventSchemaToken.sol): ERC20 token contract integrated with event schema
- [EventSchemaTest.sol](test/core/EventSchemaTest.sol): Comprehensive test suite for all event schema patterns

## Running Tests

### Running the Event Schema Tests

To run the complete event schema tests:

```powershell
.\scripts\run-event-tests.ps1
```

This script will:
1. Run unit tests for all event schema patterns
2. Run integration tests for event schema integration
3. Run property tests to validate event properties
4. Run edge case tests for boundary conditions
5. Run snapshot comparison tests for consistency validation
6. Run schema linting tests for consistency verification
7. Run coverage tests to ensure comprehensive event coverage
8. Produce detailed reports

### Running Individual Tests

To run unit tests:

```bash
cd contracts
forge test --match-contract EventSchemaTest --match-test test* -vvv
```

To run integration tests:

```bash
cd contracts
forge test --match-contract EventSchemaTest --match-test *Integration* -vvv
```

To run property tests:

```bash
cd contracts
forge test --match-contract EventSchemaTest --match-test testProperty* -vvv
```

To run edge case tests:

```bash
cd contracts
forge test --match-contract EventSchemaTest --match-test *EdgeCase* -vvv
```

To run snapshot comparison tests:

```bash
cd contracts
forge test --match-contract EventSchemaTest --match-test *Snapshot* -vvv
```

To run schema linting tests:

```bash
cd contracts
forge test --match-contract EventSchemaTest --match-test *Lint* -vvv
```

To run coverage tests:

```bash
cd contracts
forge test --match-contract EventSchemaTest --match-test *Coverage* -vvv
```

## Documentation

- [EVENT-SCHEMA-README.md](EVENT-SCHEMA-README.md): This setup and usage guide
- [EVENT-SCHEMA.md](docs/contracts/EVENT-SCHEMA.md): Detailed documentation of the event schema implementation
- [EventSchema.sol](src/core/EventSchema.sol): Core event schema contract implementation
- [EventSchemaToken.sol](src/core/EventSchemaToken.sol): ERC20 token contract with event schema integration
- [EventSchemaTest.sol](test/core/EventSchemaTest.sol): Comprehensive test suite for all event schema patterns

## CI/CD Integration

The event schema tests are integrated into GitHub Actions through the workflow file:
- [.github/workflows/contract-event-schema.yml](.github/workflows/contract-event-schema.yml)

This workflow:
1. Runs all event schema tests on every push/PR to contract files
2. Generates test reports
3. Uploads results as artifacts

## Test Results

Test results and logs are stored in the `event-results` directory after running the tests:

- Unit test logs
- Integration test logs
- Property test logs
- Edge case test logs
- Snapshot comparison test logs
- Schema linting test logs
- Coverage test logs
- Summary markdown report

## Key Event Schema Patterns Implemented

### Standardized Event Schemas

The implementation provides standardized event schemas with:
- Consistent naming conventions for all events
- Indexed parameters for efficient querying
- Comprehensive coverage of critical state changes
- Structured data for analysis and reconstruction

### Indexability and Auditability

Events are designed for:
- Efficient indexing with critical parameters indexed
- Complete audit trails for all state changes
- Timestamps for temporal analysis
- Actor tracking for accountability

### Analytics Readiness

Events are structured for:
- Analytics platforms and business intelligence tools
- Performance metrics collection
- User action tracking
- Security event monitoring

### Event Snapshot Comparison

Testing includes:
- Event snapshot comparison for consistency validation
- Regression testing to ensure event emission consistency
- Coverage analysis to verify all events are properly emitted

### ABI/Event Schema Linting

Testing includes:
- ABI/event schema linting for consistency verification
- Signature validation to verify event signatures are consistent
- Naming convention enforcement
- Indexing consistency verification

### Event Coverage

Testing ensures:
- Comprehensive event coverage across all contract functions
- Coverage metrics to measure percentage of functions with event emission
- Path coverage to ensure all execution paths emit appropriate events
- Edge case coverage to verify events are emitted for edge cases

## Troubleshooting

### Common Issues

1. **Foundry not found**: Ensure Foundry is installed and available in your PATH
2. **Test failures**: Review test output and contract logic
3. **Event emission issues**: Check event definitions and emission logic
4. **Indexing problems**: Verify indexing strategy matches query patterns

### Getting Help

If you encounter issues:
1. Check the detailed logs in the event-results directory
2. Verify all prerequisites are installed
3. Ensure dependencies are properly installed
4. Consult the documentation in [EVENT-SCHEMA.md](docs/contracts/EVENT-SCHEMA.md)

## Contributing

To contribute to the event schema implementation:

1. Follow the existing patterns in the contracts
2. Add new test cases to [EventSchemaTest.sol](test/core/EventSchemaTest.sol)
3. Update documentation as needed
4. Ensure all tests pass before submitting changes