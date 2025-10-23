# Verification script for CI/CD components

Write-Host "Verifying CI/CD components..." -ForegroundColor Green

# Check workflow files
if (Test-Path ".github/workflows/ci.yml") {
    Write-Host "  ✓ CI workflow exists" -ForegroundColor Green
} else {
    Write-Host "  ✗ CI workflow missing" -ForegroundColor Red
    exit 1
}

if (Test-Path ".github/workflows/release.yml") {
    Write-Host "  ✓ Release workflow exists" -ForegroundColor Green
} else {
    Write-Host "  ✗ Release workflow missing" -ForegroundColor Red
    exit 1
}

if (Test-Path ".github/workflows/deploy.yml") {
    Write-Host "  ✓ Deploy workflow exists" -ForegroundColor Green
} else {
    Write-Host "  ✗ Deploy workflow missing" -ForegroundColor Red
    exit 1
}

# Check documentation
if (Test-Path "docs/cicd_controls_implementation.md") {
    Write-Host "  ✓ CI/CD implementation documentation exists" -ForegroundColor Green
} else {
    Write-Host "  ✗ CI/CD implementation documentation missing" -ForegroundColor Red
    exit 1
}

# Check CICD controls document
if (Test-Path "dapp_cicd_controls.md") {
    $content = Get-Content "dapp_cicd_controls.md" -Raw
    if ($content -match "\[x\] Build matrix") {
        Write-Host "  ✓ CICD controls document updated" -ForegroundColor Green
    } else {
        Write-Host "  ✗ CICD controls document not updated" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "  ✗ CICD controls document missing" -ForegroundColor Red
    exit 1
}

Write-Host "All CI/CD components verified successfully!" -ForegroundColor Green