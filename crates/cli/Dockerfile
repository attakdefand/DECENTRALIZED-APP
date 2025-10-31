# Use Rust official image as build environment
FROM rust:1.75 as builder

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml .
COPY crates/core/Cargo.toml ./crates/core/
COPY crates/cli/Cargo.toml ./crates/cli/

# Create dummy src files to satisfy Cargo
RUN mkdir -p crates/core/src && echo "fn main() {}" > crates/core/src/lib.rs
RUN mkdir -p crates/cli/src && echo "fn main() {}" > crates/cli/src/main.rs

# Build dependencies only (to cache them)
RUN cargo build --release -p dex-cli
RUN rm -f target/release/deps/dex_cli*

# Copy the source code
COPY crates/core/src ./crates/core/src
COPY crates/cli/src ./crates/cli/src

# Build the application
RUN cargo build --release -p dex-cli

# Runtime stage
FROM gcr.io/distroless/cc-debian12

# Create non-root user
RUN addgroup --gid 10001 app && adduser --uid 10000 --gid 10001 app

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder --chown=10000:10001 /app/target/release/dex-cli /usr/local/bin/dex-cli

# Switch to non-root user
USER 10000:10001

# Run the application
ENTRYPOINT ["dex-cli"]