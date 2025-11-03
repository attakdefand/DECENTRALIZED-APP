# Use Rust official image as build environment
FROM rust:1.75 as builder

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml .
COPY crates/core/Cargo.toml ./crates/core/
COPY crates/keeper/Cargo.toml ./crates/keeper/
COPY services/keepers-rs/Cargo.toml ./services/keepers-rs/

# Create dummy src files to satisfy Cargo
RUN mkdir -p crates/core/src && echo "fn main() {}" > crates/core/src/lib.rs
RUN mkdir -p crates/keeper/src && echo "fn main() {}" > crates/keeper/src/lib.rs
RUN mkdir -p services/keepers-rs/src && echo "fn main() {}" > services/keepers-rs/src/main.rs

# Build dependencies only (to cache them)
RUN cargo build --release -p keepers-service
RUN rm -f target/release/deps/keepers*

# Copy the source code
COPY crates/core/src ./crates/core/src
COPY crates/keeper/src ./crates/keeper/src
COPY services/keepers-rs/src ./services/keepers-rs/src

# Build the application
RUN cargo build --release -p keepers-service

# Runtime stage
FROM gcr.io/distroless/cc-debian12:nonroot

WORKDIR /app
ENV RUST_LOG=info

COPY --from=builder /app/target/release/keepers /app/bin/keepers

USER nonroot
ENTRYPOINT ["/app/bin/keepers"]
