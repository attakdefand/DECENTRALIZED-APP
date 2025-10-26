# Use Rust official image as build environment
FROM rust:1.75 as builder

# Install wasm-pack
RUN cargo install wasm-pack

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml .
COPY crates/core/Cargo.toml ./crates/core/
COPY web-ui/Cargo.toml ./web-ui/

# Create dummy src files to satisfy Cargo
RUN mkdir -p crates/core/src && echo "fn main() {}" > crates/core/src/lib.rs
RUN mkdir -p web-ui/src && echo "fn main() {}" > web-ui/src/lib.rs

# Build dependencies only (to cache them)
RUN cd web-ui && wasm-pack build --target web --release
RUN rm -f target/release/deps/web_ui*

# Copy the source code
COPY crates/core/src ./crates/core/src
COPY web-ui/src ./web-ui/src

# Build the application
RUN cd web-ui && wasm-pack build --target web --release

# Runtime stage
FROM nginx:alpine

# Copy the built files from builder stage
COPY --from=builder /app/web-ui/pkg /usr/share/nginx/html/pkg
COPY --from=builder /app/web-ui/index.html /usr/share/nginx/html/
COPY --from=builder /app/web-ui/styles.css /usr/share/nginx/html/

# Copy any additional assets
COPY --from=builder /app/web-ui/src/assets /usr/share/nginx/html/assets

# Expose port
EXPOSE 80

# Use default nginx entrypoint