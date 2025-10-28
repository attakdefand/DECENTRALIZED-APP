@echo off
echo Running Graceful Degradation Tests Validation
echo ============================================

cd /d D:\DECENTRALIZED-APP

echo Building core crate...
cargo build -p core --quiet
if %ERRORLEVEL% NEQ 0 (
    echo Error: Failed to build core crate
    exit /b %ERRORLEVEL%
)

echo Running graceful degradation functional tests...
cargo test --test graceful_degradation_tests -- --nocapture
if %ERRORLEVEL% NEQ 0 (
    echo Error: Graceful degradation functional tests failed
    exit /b %ERRORLEVEL%
)

echo Running graceful degradation security validation tests...
cargo test --test graceful_degradation_security_validation -- --nocapture
if %ERRORLEVEL% NEQ 0 (
    echo Error: Graceful degradation security validation tests failed
    exit /b %ERRORLEVEL%
)

echo.
echo All graceful degradation tests passed successfully!
echo Validation complete.