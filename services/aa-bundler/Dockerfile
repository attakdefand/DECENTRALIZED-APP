# Use Rust official image as build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy the manifest file
COPY Cargo.toml .
COPY crates/core/Cargo.toml ./crates/core/
COPY services/aa-bundler/Cargo.toml ./services/aa-bundler/

# Create dummy files to satisfy the workspace structure
RUN mkdir -p crates/core/src && echo "fn main() {}" > crates/core/src/lib.rs
RUN mkdir -p services/aa-bundler/src && echo "fn main() {}" > services/aa-bundler/src/main.rs

# Build dependencies only (to cache them)
RUN cargo build --release -p aa-bundler
RUN rm -f target/release/deps/aa_bundler*

# Copy the source code
COPY crates/core/src ./crates/core/src
COPY services/aa-bundler/src ./services/aa-bundler/src

# Build the application
RUN cargo build --release -p aa-bundler

# Runtime stage
FROM debian:bookworm-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create app user
RUN groupadd -r app && useradd -r -g app app

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/aa-bundler /usr/local/bin/aa-bundler

# Change ownership of the binary
RUN chown app:app /usr/local/bin/aa-bundler

# Switch to non-root user
USER app

# Expose metrics port
EXPOSE 9095

# Run the application
ENTRYPOINT ["aa-bundler"]