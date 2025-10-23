# Use Rust official image as build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy the manifest file
COPY Cargo.toml .
COPY crates/core/Cargo.toml ./crates/core/
COPY services/mev-monitor/Cargo.toml ./services/mev-monitor/

# Create dummy files to satisfy the workspace structure
RUN mkdir -p crates/core/src && echo "fn main() {}" > crates/core/src/lib.rs
RUN mkdir -p services/mev-monitor/src && echo "fn main() {}" > services/mev-monitor/src/main.rs

# Build dependencies only (to cache them)
RUN cargo build --release -p mev-monitor
RUN rm -f target/release/deps/mev_monitor*

# Copy the source code
COPY crates/core/src ./crates/core/src
COPY services/mev-monitor/src ./services/mev-monitor/src

# Build the application
RUN cargo build --release -p mev-monitor

# Runtime stage
FROM debian:bookworm-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create app user
RUN groupadd -r app && useradd -r -g app app

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/mev-monitor /usr/local/bin/mev-monitor

# Change ownership of the binary
RUN chown app:app /usr/local/bin/mev-monitor

# Switch to non-root user
USER app

# Expose metrics port
EXPOSE 9094

# Run the application
ENTRYPOINT ["mev-monitor"]