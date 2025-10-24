# Use Rust official image as build environment
FROM rust:1.70 as builder

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml .
COPY crates/core/Cargo.toml ./crates/core/
COPY crates/keeper/Cargo.toml ./crates/keeper/
COPY services/keepers-rs/Cargo.toml ./services/keepers-rs/

# Copy source code
COPY crates/core/src ./crates/core/src
COPY crates/keeper/src ./crates/keeper/src
COPY services/keepers-rs/src ./services/keepers-rs/src

# Build the application
RUN cargo build --release --manifest-path services/keepers-rs/Cargo.toml

# Use Debian slim image for runtime
FROM debian:bullseye-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/keepers ./keepers

# Run the binary
CMD ["./keepers"]