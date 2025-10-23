# Development server script for the WebAssembly web UI

Write-Host "Starting development server for WebAssembly web UI..." -ForegroundColor Green

# Check if miniserve is installed
if (!(Get-Command miniserve -ErrorAction SilentlyContinue)) {
    Write-Host "Installing miniserve..." -ForegroundColor Yellow
    cargo install miniserve
}

# Build the WebAssembly package first
Write-Host "Building WebAssembly package..." -ForegroundColor Yellow
wasm-pack build --target web --out-dir pkg

# Start the development server
Write-Host "Starting development server on http://localhost:8080" -ForegroundColor Cyan
miniserve . --port 8080