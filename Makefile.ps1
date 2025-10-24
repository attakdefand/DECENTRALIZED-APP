# PowerShell script equivalent to Makefile for DECENTRALIZED-APP

param(
    [Parameter(Position=0)]
    [string]$Command = "help"
)

function Show-Help {
    Write-Host "Available commands:" -ForegroundColor Cyan
    Write-Host "  build          Build all components" -ForegroundColor Green
    Write-Host "  test           Run all tests" -ForegroundColor Green
    Write-Host "  clean          Clean build artifacts" -ForegroundColor Green
    Write-Host "  fmt            Format code" -ForegroundColor Green
    Write-Host "  clippy         Run clippy linter" -ForegroundColor Green
    Write-Host "  check          Check code formatting and linting" -ForegroundColor Green
    Write-Host "  web-ui-build   Build WebAssembly UI" -ForegroundColor Green
    Write-Host "  web-ui-dev     Build WebAssembly UI for development" -ForegroundColor Green
    Write-Host "  web-ui-test    Test WebAssembly UI" -ForegroundColor Green
    Write-Host "  docker-up      Start all services with Docker Compose" -ForegroundColor Green
    Write-Host "  docker-down    Stop all services with Docker Compose" -ForegroundColor Green
    Write-Host "  release        Create a new release (requires -Version parameter)" -ForegroundColor Green
    Write-Host "  docs           Build documentation" -ForegroundColor Green
    Write-Host "  serve-docs     Serve documentation locally" -ForegroundColor Green
    Write-Host "  install-tools  Install development tools" -ForegroundColor Green
}

function Invoke-Build {
    Write-Host "Building all components..." -ForegroundColor Green
    cargo build --workspace
}

function Invoke-Test {
    Write-Host "Running all tests..." -ForegroundColor Green
    cargo test --workspace
}

function Invoke-Clean {
    Write-Host "Cleaning build artifacts..." -ForegroundColor Green
    cargo clean
}

function Invoke-Fmt {
    Write-Host "Formatting code..." -ForegroundColor Green
    cargo fmt --all
}

function Invoke-Clippy {
    Write-Host "Running clippy..." -ForegroundColor Green
    cargo clippy --workspace -- -D warnings
}

function Invoke-Check {
    Invoke-Fmt
    Invoke-Clippy
}

function Invoke-WebUIBuild {
    Write-Host "Building WebAssembly UI..." -ForegroundColor Green
    Set-Location web-ui
    wasm-pack build --target web
    Set-Location ..
}

function Invoke-WebUIDev {
    Write-Host "Building WebAssembly UI for development..." -ForegroundColor Green
    Set-Location web-ui
    wasm-pack build --target web --dev
    Set-Location ..
}

function Invoke-WebUITest {
    Write-Host "Testing WebAssembly UI..." -ForegroundColor Green
    Set-Location web-ui
    wasm-pack test --headless --firefox
    Set-Location ..
}

function Invoke-DockerUp {
    Write-Host "Starting services with Docker Compose..." -ForegroundColor Green
    docker-compose up
}

function Invoke-DockerDown {
    Write-Host "Stopping services with Docker Compose..." -ForegroundColor Green
    docker-compose down
}

function Invoke-Release {
    param([string]$Version)
    
    if (-not $Version) {
        Write-Error "Version parameter is required for release command"
        Write-Host "Usage: .\Makefile.ps1 release -Version 1.2.3" -ForegroundColor Yellow
        return
    }
    
    Write-Host "Creating release v$Version..." -ForegroundColor Green
    .\scripts\create-release.ps1 -Version "v$Version"
}

function Invoke-Docs {
    Write-Host "Building documentation..." -ForegroundColor Green
    cargo doc --workspace --no-deps
}

function Invoke-ServeDocs {
    Write-Host "Serving documentation..." -ForegroundColor Green
    cargo doc --workspace --no-deps --open
}

function Invoke-InstallTools {
    Write-Host "Installing development tools..." -ForegroundColor Green
    rustup component add rustfmt clippy
    cargo install wasm-pack
    cargo install basic-http-server
}

# Main command router
switch ($Command) {
    "help" { Show-Help }
    "build" { Invoke-Build }
    "test" { Invoke-Test }
    "clean" { Invoke-Clean }
    "fmt" { Invoke-Fmt }
    "clippy" { Invoke-Clippy }
    "check" { Invoke-Check }
    "web-ui-build" { Invoke-WebUIBuild }
    "web-ui-dev" { Invoke-WebUIDev }
    "web-ui-test" { Invoke-WebUITest }
    "docker-up" { Invoke-DockerUp }
    "docker-down" { Invoke-DockerDown }
    "release" { 
        $versionParam = $PSBoundParameters["Version"]
        Invoke-Release -Version $versionParam
    }
    "docs" { Invoke-Docs }
    "serve-docs" { Invoke-ServeDocs }
    "install-tools" { Invoke-InstallTools }
    default { 
        Write-Host "Unknown command: $Command" -ForegroundColor Red
        Show-Help 
    }
}