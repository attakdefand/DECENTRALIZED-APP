# Verification script for the installation system

Write-Host "=== Verifying Cross-Platform Installation System ===" -ForegroundColor Cyan
Write-Host ""

# Check if required files exist
Write-Host "1. Checking required files..." -ForegroundColor Yellow

$requiredFiles = @(
    "crates\cli\src\main.rs",
    "crates\cli\Cargo.toml",
    "install.sh",
    "install.ps1",
    "install\install.sh",
    "install\install.ps1",
    "scripts\package.sh",
    "scripts\package.ps1"
)

$allFilesExist = $true
foreach ($file in $requiredFiles) {
    if (Test-Path $file) {
        Write-Host "  ✓ $file" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $file (MISSING)" -ForegroundColor Red
        $allFilesExist = $false
    }
}

if ($allFilesExist) {
    Write-Host "  All required files present" -ForegroundColor Green
} else {
    Write-Host "  Some required files are missing" -ForegroundColor Red
    exit 1
}

Write-Host ""

# Check if CLI binary exists
Write-Host "2. Checking CLI binary..." -ForegroundColor Yellow

if (Test-Path "target\debug\dex-cli.exe") {
    Write-Host "  ✓ CLI binary exists" -ForegroundColor Green
} else {
    Write-Host "  ✗ CLI binary missing" -ForegroundColor Red
    exit 1
}

Write-Host ""

# Test CLI help command
Write-Host "3. Testing CLI help command..." -ForegroundColor Yellow

$output = .\target\debug\dex-cli.exe --help 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✓ CLI help command works" -ForegroundColor Green
    Write-Host "  Output:" -ForegroundColor Gray
    Write-Host "  $($output -join "`n  ")" -ForegroundColor Gray
} else {
    Write-Host "  ✗ CLI help command failed" -ForegroundColor Red
    Write-Host "  Error: $output" -ForegroundColor Red
    exit 1
}

Write-Host ""

# Test CLI init command
Write-Host "4. Testing CLI init command..." -ForegroundColor Yellow

$output = .\target\debug\dex-cli.exe init 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✓ CLI init command works" -ForegroundColor Green
} else {
    Write-Host "  ✗ CLI init command failed" -ForegroundColor Red
    Write-Host "  Error: $output" -ForegroundColor Red
    exit 1
}

Write-Host ""

# Test CLI status command
Write-Host "5. Testing CLI status command..." -ForegroundColor Yellow

$output = .\target\debug\dex-cli.exe status 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✓ CLI status command works" -ForegroundColor Green
} else {
    Write-Host "  ✗ CLI status command failed" -ForegroundColor Red
    Write-Host "  Error: $output" -ForegroundColor Red
    exit 1
}

Write-Host ""

Write-Host "=== Verification Complete ===" -ForegroundColor Cyan
Write-Host "✓ Cross-platform installation system is working correctly!" -ForegroundColor Green