# Use Rust official image as build environment
FROM rust:1.70 as builder

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml .
COPY crates/core/Cargo.toml ./crates/core/
COPY services/api-rs/Cargo.toml ./services/api-rs/

# Copy source code
COPY crates/core/src ./crates/core/src
COPY services/api-rs/src ./services/api-rs/src

# Build the application
RUN cargo build --release --manifest-path services/api-rs/Cargo.toml

# Use Debian slim image for runtime
FROM debian:bullseye-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/api ./api

# Expose port
EXPOSE 3000

# Run the binary
CMD ["./api"]