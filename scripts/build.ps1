# Build script for the decentralized application

Write-Host "Building decentralized application..." -ForegroundColor Green

# Build all crates in the workspace
cargo build --release

# Build all services
Write-Host "Building off-chain services..." -ForegroundColor Yellow
cargo build --release --manifest-path services/indexer-rs/Cargo.toml
cargo build --release --manifest-path services/api-rs/Cargo.toml
cargo build --release --manifest-path services/keepers-rs/Cargo.toml

if ($LASTEXITCODE -eq 0) {
    Write-Host "Build successful!" -ForegroundColor Green
} else {
    Write-Host "Build failed!" -ForegroundColor Red
    exit $LASTEXITCODE
}

# Show build artifacts
Write-Host "Build artifacts are located in target/release/" -ForegroundColor Yellow