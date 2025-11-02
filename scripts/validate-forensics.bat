@echo off
echo Running Forensics Tests Validation
echo ================================

cd /d D:\DECENTRALIZED-APP

echo Building core crate...
cargo build -p core --quiet
if %ERRORLEVEL% NEQ 0 (
    echo Error: Failed to build core crate
    exit /b %ERRORLEVEL%
)

echo Running forensics functional tests...
cargo test --test forensics_evidence_tests -- --nocapture
if %ERRORLEVEL% NEQ 0 (
    echo Error: Forensics functional tests failed
    exit /b %ERRORLEVEL%
)

echo Running observability integration tests...
cargo test --test observability_integration -- --nocapture
if %ERRORLEVEL% NEQ 0 (
    echo Error: Observability integration tests failed
    exit /b %ERRORLEVEL%
)

echo.
echo All forensics tests passed successfully!
echo Validation complete.