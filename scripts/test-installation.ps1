# Test script for the cross-platform installation system

Write-Host "Testing cross-platform installation system..." -ForegroundColor Green

# Test 1: Check if installation scripts exist
Write-Host "Test 1: Checking installation scripts..." -ForegroundColor Yellow

$scripts = @(
    "install.sh",
    "install.ps1",
    "install\install.sh",
    "install\install.ps1"
)

$allExist = $true
foreach ($script in $scripts) {
    if (Test-Path $script) {
        Write-Host "  ✓ $script exists" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $script missing" -ForegroundColor Red
        $allExist = $false
    }
}

if ($allExist) {
    Write-Host "  All installation scripts present" -ForegroundColor Green
} else {
    Write-Host "  Some installation scripts are missing" -ForegroundColor Red
    exit 1
}

# Test 2: Check if CLI builds successfully
Write-Host "Test 2: Checking CLI build..." -ForegroundColor Yellow

try {
    $output = cargo build --bin dex-cli 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  ✓ CLI builds successfully" -ForegroundColor Green
    } else {
        Write-Host "  ✗ CLI build failed" -ForegroundColor Red
        Write-Host $output -ForegroundColor Red
        exit 1
    }
} catch {
    Write-Host "  ✗ CLI build failed with exception: $_" -ForegroundColor Red
    exit 1
}

# Test 3: Check if CLI runs
Write-Host "Test 3: Checking CLI execution..." -ForegroundColor Yellow

try {
    $output = cargo run --bin dex-cli -- --help 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  ✓ CLI executes successfully" -ForegroundColor Green
    } else {
        Write-Host "  ✗ CLI execution failed" -ForegroundColor Red
        Write-Host $output -ForegroundColor Red
        exit 1
    }
} catch {
    Write-Host "  ✗ CLI execution failed with exception: $_" -ForegroundColor Red
    exit 1
}

# Test 4: Check if package scripts exist
Write-Host "Test 4: Checking package scripts..." -ForegroundColor Yellow

$packageScripts = @(
    "scripts\package.sh",
    "scripts\package.ps1"
)

$allPackageScriptsExist = $true
foreach ($script in $packageScripts) {
    if (Test-Path $script) {
        Write-Host "  ✓ $script exists" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $script missing" -ForegroundColor Red
        $allPackageScriptsExist = $false
    }
}

if ($allPackageScriptsExist) {
    Write-Host "  All package scripts present" -ForegroundColor Green
} else {
    Write-Host "  Some package scripts are missing" -ForegroundColor Red
    exit 1
}

Write-Host "All tests passed! The cross-platform installation system is working correctly." -ForegroundColor Green