@echo off
echo Running Telemetry Tests Validation
echo =================================

cd /d D:\DECENTRALIZED-APP

echo Building core crate...
cargo build -p core --quiet
if %ERRORLEVEL% NEQ 0 (
    echo Error: Failed to build core crate
    exit /b %ERRORLEVEL%
)

echo Running telemetry functional tests...
cargo test --test telemetry_tests -- --nocapture
if %ERRORLEVEL% NEQ 0 (
    echo Error: Telemetry functional tests failed
    exit /b %ERRORLEVEL%
)

echo Running observability integration tests...
cargo test --test observability_integration -- --nocapture
if %ERRORLEVEL% NEQ 0 (
    echo Error: Observability integration tests failed
    exit /b %ERRORLEVEL%
)

echo.
echo All telemetry tests passed successfully!
echo Validation complete.