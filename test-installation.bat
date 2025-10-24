@echo off
echo === Testing Cross-Platform Installation System ===
echo.

echo 1. Testing CLI Help Command...
target\debug\dex-cli.exe --help
if %ERRORLEVEL% EQU 0 (
    echo    SUCCESS: CLI help command works
) else (
    echo    FAILED: CLI help command failed
    exit /b 1
)
echo.

echo 2. Testing CLI Init Command...
target\debug\dex-cli.exe init
if %ERRORLEVEL% EQU 0 (
    echo    SUCCESS: CLI init command works
) else (
    echo    FAILED: CLI init command failed
    exit /b 1
)
echo.

echo 3. Testing CLI Status Command...
target\debug\dex-cli.exe status
if %ERRORLEVEL% EQU 0 (
    echo    SUCCESS: CLI status command works
) else (
    echo    FAILED: CLI status command failed
    exit /b 1
)
echo.

echo === All Tests Passed! Installation System is Working ===