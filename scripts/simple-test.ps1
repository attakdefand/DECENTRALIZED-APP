# Simple test script for the installation system

Write-Host "Testing installation system..." -ForegroundColor Green

# Test CLI build
Write-Host "Testing CLI build..." -ForegroundColor Yellow
cargo build --bin dex-cli
if ($LASTEXITCODE -eq 0) {
    Write-Host "CLI builds successfully" -ForegroundColor Green
} else {
    Write-Host "CLI build failed" -ForegroundColor Red
    exit 1
}

# Test CLI execution
Write-Host "Testing CLI execution..." -ForegroundColor Yellow
cargo run --bin dex-cli -- --help > $null
if ($LASTEXITCODE -eq 0) {
    Write-Host "CLI executes successfully" -ForegroundColor Green
} else {
    Write-Host "CLI execution failed" -ForegroundColor Red
    exit 1
}

Write-Host "All tests passed!" -ForegroundColor Green