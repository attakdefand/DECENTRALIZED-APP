#!/bin/bash

# Build script for the WebAssembly web UI

set -e

echo "Building WebAssembly web UI..."

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null
then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WebAssembly package
echo "Compiling Rust to WebAssembly..."
wasm-pack build --target web --out-dir pkg

echo "Web UI build complete!"
echo "To run the web UI, serve the index.html file with a local server."