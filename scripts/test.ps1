# Test script for the decentralized application

Write-Host "Running tests for decentralized application..." -ForegroundColor Green

# Run unit tests for all crates
cargo test --workspace

# Test services
cargo test --manifest-path services/indexer-rs/Cargo.toml
cargo test --manifest-path services/api-rs/Cargo.toml
cargo test --manifest-path services/keepers-rs/Cargo.toml

if ($LASTEXITCODE -eq 0) {
    Write-Host "All tests passed!" -ForegroundColor Green
} else {
    Write-Host "Some tests failed!" -ForegroundColor Red
    exit $LASTEXITCODE
}

# Run integration tests
cargo test --test integration_test

if ($LASTEXITCODE -eq 0) {
    Write-Host "Integration tests passed!" -ForegroundColor Green
} else {
    Write-Host "Integration tests failed!" -ForegroundColor Red
    exit $LASTEXITCODE
}