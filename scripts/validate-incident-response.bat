@echo off
echo Running Incident Response Tests Validation
echo ======================================

cd /d D:\DECENTRALIZED-APP

echo Building core crate...
cargo build -p core --quiet
if %ERRORLEVEL% NEQ 0 (
    echo Error: Failed to build core crate
    exit /b %ERRORLEVEL%
)

echo Running incident response functional tests...
cargo test --test incident_response_tests -- --nocapture
if %ERRORLEVEL% NEQ 0 (
    echo Error: Incident response functional tests failed
    exit /b %ERRORLEVEL%
)

echo Running observability integration tests...
cargo test --test observability_integration -- --nocapture
if %ERRORLEVEL% NEQ 0 (
    echo Error: Observability integration tests failed
    exit /b %ERRORLEVEL%
)

echo.
echo All incident response tests passed successfully!
echo Validation complete.