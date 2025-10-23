@echo off
echo Verifying CI/CD components...

echo Checking workflow files...
if exist ".github\workflows\ci.yml" (
    echo   ✓ CI workflow exists
) else (
    echo   ✗ CI workflow missing
    exit /b 1
)

if exist ".github\workflows\release.yml" (
    echo   ✓ Release workflow exists
) else (
    echo   ✗ Release workflow missing
    exit /b 1
)

if exist ".github\workflows\deploy.yml" (
    echo   ✓ Deploy workflow exists
) else (
    echo   ✗ Deploy workflow missing
    exit /b 1
)

echo Checking documentation...
if exist "docs\cicd_controls_implementation.md" (
    echo   ✓ CI/CD implementation documentation exists
) else (
    echo   ✗ CI/CD implementation documentation missing
    exit /b 1
)

if exist "dapp_cicd_controls.md" (
    echo   ✓ CICD controls document exists
) else (
    echo   ✗ CICD controls document missing
    exit /b 1
)

echo All CI/CD components verified successfully!