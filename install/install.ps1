# Cross-platform installer for Decentralized Exchange Application
# Supports Windows

# Colors for output
$RED = "Red"
$GREEN = "Green"
$YELLOW = "Yellow"

Write-Host "Installing Decentralized Exchange Application on Windows" -ForegroundColor $GREEN

# Check if Rust is installed
$rustc = Get-Command rustc -ErrorAction SilentlyContinue
if (-not $rustc) {
    Write-Host "Rust not found. Installing Rust..." -ForegroundColor $YELLOW
    # Download and install Rust
    $rustupUrl = "https://win.rustup.rs/x86_64"
    $rustupPath = "$env:TEMP\rustup-init.exe"
    Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath
    Start-Process -FilePath $rustupPath -ArgumentList "-y" -Wait
    Remove-Item $rustupPath
    
    # Refresh environment variables
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
}

# Check if Cargo is available
$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargo) {
    Write-Host "Cargo not found. Please install Rust manually and try again." -ForegroundColor $RED
    exit 1
}

Write-Host "Building Decentralized Exchange CLI..." -ForegroundColor $GREEN

# Build the CLI
cargo build --release --bin dex-cli

# Install the binary
Write-Host "Installing binary..." -ForegroundColor $GREEN
$binaryPath = "target\release\dex-cli.exe"
if (Test-Path $binaryPath) {
    # Copy to a directory in PATH
    $destination = "$env:USERPROFILE\.dex\bin"
    New-Item -ItemType Directory -Path $destination -Force | Out-Null
    Copy-Item $binaryPath "$destination\dex.exe"
    
    # Add to PATH if not already there
    $path = [Environment]::GetEnvironmentVariable("Path", "User")
    if (-not $path.Contains($destination)) {
        [Environment]::SetEnvironmentVariable("Path", "$path;$destination", "User")
        Write-Host "Added $destination to PATH. Please restart your terminal or log out and back in for changes to take effect." -ForegroundColor $YELLOW
    }
    
    # Create config directory
    New-Item -ItemType Directory -Path "$env:ProgramData\dex" -Force | Out-Null
}

Write-Host "Installation complete!" -ForegroundColor $GREEN
Write-Host "You can now run the application with: dex start" -ForegroundColor $YELLOW
Write-Host "For more information, run: dex --help" -ForegroundColor $YELLOW