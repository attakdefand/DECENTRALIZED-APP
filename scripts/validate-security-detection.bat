@echo off
echo Running Security Detection Tests Validation
echo ==========================================

cd /d D:\DECENTRALIZED-APP

echo Building core crate...
cargo build -p core --quiet
if %ERRORLEVEL% NEQ 0 (
    echo Error: Failed to build core crate
    exit /b %ERRORLEVEL%
)

echo Running security detection functional tests...
cargo test --test security_detection_tests -- --nocapture
if %ERRORLEVEL% NEQ 0 (
    echo Error: Security detection functional tests failed
    exit /b %ERRORLEVEL%
)

echo Running observability integration tests...
cargo test --test observability_integration -- --nocapture
if %ERRORLEVEL% NEQ 0 (
    echo Error: Observability integration tests failed
    exit /b %ERRORLEVEL%
)

echo.
echo All security detection tests passed successfully!
echo Validation complete.