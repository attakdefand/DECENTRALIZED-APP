# Use Rust official image as base
FROM rust:1.70 as builder

# Create app directory
WORKDIR /app

# Copy the manifest file
COPY Cargo.toml .
COPY crates/core/Cargo.toml crates/core/
COPY services/ipfs-rs/Cargo.toml services/ipfs-rs/

# Create dummy src files to satisfy Cargo
RUN mkdir -p crates/core/src && echo "fn main() {}" > crates/core/src/lib.rs
RUN mkdir -p services/ipfs-rs/src && echo "fn main() {}" > services/ipfs-rs/src/main.rs

# Build dependencies to cache them
RUN cargo build --release -p ipfs-service

# Copy the actual source code
COPY crates/core/src crates/core/src
COPY services/ipfs-rs/src services/ipfs-rs/src

# Build the application
RUN cargo build --release -p ipfs-service

# Production stage
FROM debian:bullseye-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/ipfs-monitor .

# Expose port
EXPOSE 9093

# Run the application
CMD ["./ipfs-monitor"]