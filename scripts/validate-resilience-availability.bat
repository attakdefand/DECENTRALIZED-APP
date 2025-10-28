@echo off
REM Validate Resilience and Availability Implementation
REM This script validates the complete resilience and availability implementation

echo Validating Resilience and Availability Implementation

REM Run unit tests
echo Running unit tests...
cargo test --package core --test resilience_availability_integration
if %ERRORLEVEL% NEQ 0 (
    echo Unit tests failed!
    exit /b 1
)

REM Run simulation tests
echo Running simulation tests...
cargo run --package core --bin resilience_availability_simulation
if %ERRORLEVEL% NEQ 0 (
    echo Simulation tests failed!
    exit /b 1
)

REM Run integration tests
echo Running integration tests...
cargo test --test resilience_availability_integration
if %ERRORLEVEL% NEQ 0 (
    echo Integration tests failed!
    exit /b 1
)

echo All Resilience and Availability Validation Tests Passed!
echo Implementation is ready for production use.