@echo off
REM Run Resilience and Availability Tests Script
REM This script runs all resilience and availability tests for the decentralized application

echo Running Resilience and Availability Tests

REM Run unit tests
echo Running unit tests...
cargo test --package core --test resilience_availability_integration
if %ERRORLEVEL% NEQ 0 (
    echo Resilience and Availability Tests Failed!
    exit /b 1
)

REM Run simulation tests
echo Running simulation tests...
cargo run --package core --bin resilience_availability_simulation
if %ERRORLEVEL% NEQ 0 (
    echo Resilience and Availability Tests Failed!
    exit /b 1
)

REM Run integration tests
echo Running integration tests...
cargo test --test resilience_availability_simulation
if %ERRORLEVEL% NEQ 0 (
    echo Resilience and Availability Tests Failed!
    exit /b 1
)

echo All Resilience and Availability Tests Passed!