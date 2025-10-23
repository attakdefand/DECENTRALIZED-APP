# Build script for the WebAssembly web UI (PowerShell version)

Write-Host "Building WebAssembly web UI..." -ForegroundColor Green

# Check if wasm-pack is installed
if (!(Get-Command wasm-pack -ErrorAction SilentlyContinue)) {
    Write-Host "Installing wasm-pack..." -ForegroundColor Yellow
    curl https://rustwasm.github.io/wasm-pack/installer/init.ps1 -UseBasicParsing -OutFile init.ps1
    .\init.ps1
    Remove-Item init.ps1
}

# Build the WebAssembly package
Write-Host "Compiling Rust to WebAssembly..." -ForegroundColor Yellow
wasm-pack build --target web --out-dir pkg

Write-Host "Web UI build complete!" -ForegroundColor Green
Write-Host "To run the web UI, serve the index.html file with a local server." -ForegroundColor Cyan