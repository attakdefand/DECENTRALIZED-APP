# Use Rust official image as build environment
FROM rust:1.75 as builder

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml .
COPY crates/core/Cargo.toml ./crates/core/
COPY crates/indexer/Cargo.toml ./crates/indexer/
COPY services/indexer-rs/Cargo.toml ./services/indexer-rs/

# Create dummy src files to satisfy Cargo
RUN mkdir -p crates/core/src && echo "fn main() {}" > crates/core/src/lib.rs
RUN mkdir -p crates/indexer/src && echo "fn main() {}" > crates/indexer/src/lib.rs
RUN mkdir -p services/indexer-rs/src && echo "fn main() {}" > services/indexer-rs/src/main.rs

# Build dependencies only (to cache them)
RUN cargo build --release -p indexer-service
RUN rm -f target/release/deps/indexer*

# Copy the source code
COPY crates/core/src ./crates/core/src
COPY crates/indexer/src ./crates/indexer/src
COPY services/indexer-rs/src ./services/indexer-rs/src

# Build the application
RUN cargo build --release -p indexer-service

# Runtime stage
FROM gcr.io/distroless/cc-debian12:nonroot

WORKDIR /app
ENV RUST_LOG=info

COPY --from=builder /app/target/release/indexer /app/bin/indexer

EXPOSE 8080

USER nonroot
ENTRYPOINT ["/app/bin/indexer"]
