# DEX-OS Dockerfile
FROM rust:1.80 as builder

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml .
COPY Cargo.lock .

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/dex-os /usr/local/bin/dex-os

# Expose port
EXPOSE 3000

# Run the application
CMD ["dex-os"]