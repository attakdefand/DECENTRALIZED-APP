# Use Rust official image as build environment
FROM rust:1.70 as builder

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml .
COPY crates/core/Cargo.toml ./crates/core/
COPY crates/indexer/Cargo.toml ./crates/indexer/
COPY services/indexer-rs/Cargo.toml ./services/indexer-rs/

# Copy source code
COPY crates/core/src ./crates/core/src
COPY crates/indexer/src ./crates/indexer/src
COPY services/indexer-rs/src ./services/indexer-rs/src

# Build the application
RUN cargo build --release --manifest-path services/indexer-rs/Cargo.toml

# Use Debian slim image for runtime
FROM debian:bullseye-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/indexer ./indexer

# Expose port
EXPOSE 8080

# Run the binary
CMD ["./indexer"]