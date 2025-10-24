# Package script for creating distributable packages for different platforms
# This script builds the application for multiple targets and creates packages

Write-Host "Creating distributable packages..." -ForegroundColor Green

# Create output directory
New-Item -ItemType Directory -Path "dist" -Force | Out-Null

# Get version from Cargo.toml
$versionLine = Get-Content Cargo.toml | Where-Object { $_ -match "^version = " } | Select-Object -First 1
$version = $versionLine -split '"' | Select-Object -Index 1
Write-Host "Building version $version" -ForegroundColor Green

# Build for Windows
Write-Host "Building for Windows (x86_64)..." -ForegroundColor Yellow
cargo build --release --bin dex-cli
Compress-Archive -Path "target\release\dex-cli.exe" -DestinationPath "dist\dex-windows-x86_64-v$version.zip" -Force

Write-Host "Packages created successfully:" -ForegroundColor Green
Write-Host "  - dist\dex-windows-x86_64-v$version.zip" -ForegroundColor White

Write-Host "To install, extract the package and run the installer." -ForegroundColor Yellow