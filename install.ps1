# Unified installer for Decentralized Exchange Application
# Detects the environment and runs the appropriate installation script

Write-Host "Decentralized Exchange Application Installer" -ForegroundColor Green

# Check if we're on Windows
if ($IsWindows -or $env:OS) {
    Write-Host "Detected Windows. Running Windows installer..." -ForegroundColor Green
    & "$PSScriptRoot\install\install.ps1"
} 
# Check if we're on Linux/macOS with PowerShell Core
elseif ($IsLinux -or $IsMacOS) {
    Write-Host "Detected Unix-like system. Running shell installer..." -ForegroundColor Green
    $scriptPath = Join-Path $PSScriptRoot "install/install.sh"
    bash $scriptPath
} 
else {
    Write-Host "Unsupported platform." -ForegroundColor Red
    exit 1
}

# Check if installation was successful
if ($LASTEXITCODE -eq 0) {
    Write-Host "`nInstallation completed successfully!" -ForegroundColor Green
    Write-Host "IMPORTANT: The application requires Rust nightly for edition2024 support." -ForegroundColor Yellow
    Write-Host "If you encounter any issues, please ensure you have the latest nightly Rust version installed:" -ForegroundColor Yellow
    Write-Host "  rustup install nightly" -ForegroundColor Yellow
    Write-Host "  rustup default nightly" -ForegroundColor Yellow
    Write-Host "`nYou can now run the application with: dex start" -ForegroundColor Green
    Write-Host "For more information, run: dex --help" -ForegroundColor Yellow
} else {
    Write-Host "`nInstallation failed. Please check the error messages above." -ForegroundColor Red
    Write-Host "The application requires Rust nightly for edition2024 support." -ForegroundColor Yellow
    Write-Host "Please install Rust nightly manually and try again:" -ForegroundColor Yellow
    Write-Host "  rustup install nightly" -ForegroundColor Yellow
    Write-Host "  rustup default nightly" -ForegroundColor Yellow
    Write-Host "`nFor detailed troubleshooting, see: TROUBLESHOOTING-RUST-EDITION2024.md" -ForegroundColor Yellow
    exit $LASTEXITCODE
}