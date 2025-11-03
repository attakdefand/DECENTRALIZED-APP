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
FROM gcr.io/distroless/cc-debian12:nonroot

WORKDIR /app
ENV RUST_LOG=info

COPY --from=builder /app/target/release/aa-bundler /app/bin/aa-bundler

EXPOSE 9095

USER nonroot
ENTRYPOINT ["/app/bin/aa-bundler"]
