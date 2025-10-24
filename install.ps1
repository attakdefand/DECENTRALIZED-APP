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