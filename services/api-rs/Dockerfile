# Use Rust official image as build environment
FROM rust:1.75 as builder

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml .
COPY crates/core/Cargo.toml ./crates/core/
COPY services/api-rs/Cargo.toml ./services/api-rs/

# Create dummy src files to satisfy Cargo
RUN mkdir -p crates/core/src && echo "fn main() {}" > crates/core/src/lib.rs
RUN mkdir -p services/api-rs/src && echo "fn main() {}" > services/api-rs/src/main.rs

# Build dependencies only (to cache them)
RUN cargo build --release -p api-service
RUN rm -f target/release/deps/api*

# Copy the source code
COPY crates/core/src ./crates/core/src
COPY services/api-rs/src ./services/api-rs/src

# Build the application
RUN cargo build --release -p api-service

# Runtime stage
FROM gcr.io/distroless/cc-debian12

# Create non-root user
RUN addgroup --gid 10001 app && adduser --uid 10000 --gid 10001 app

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder --chown=10000:10001 /app/target/release/api /usr/local/bin/api

# Switch to non-root user
USER 10000:10001

# Expose port
EXPOSE 3000

# Run the application
ENTRYPOINT ["api"]