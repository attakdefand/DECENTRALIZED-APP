@echo off
REM Validate Traffic Protection Implementation
REM This script validates the complete traffic protection implementation

echo Validating Traffic Protection Implementation

REM Run traffic protection tests
echo Running traffic protection tests...
cargo test --package core --test traffic_protection_tests
if %ERRORLEVEL% NEQ 0 (
    echo Traffic protection tests failed!
    exit /b 1
)

REM Run resilience and availability integration tests
echo Running resilience and availability integration tests...
cargo test --package core --test resilience_availability_integration
if %ERRORLEVEL% NEQ 0 (
    echo Resilience and availability integration tests failed!
    exit /b 1
)

echo All Traffic Protection Validation Tests Passed!
echo Implementation is ready for production use.