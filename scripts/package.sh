#!/bin/bash

# Package script for creating distributable packages for different platforms
# This script builds the application for multiple targets and creates packages

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Creating distributable packages...${NC}"

# Create output directory
mkdir -p dist

# Get version from Cargo.toml
VERSION=$(grep "^version = " Cargo.toml | head -n1 | cut -d '"' -f 2)
echo -e "${GREEN}Building version ${VERSION}${NC}"

# Build for Linux
echo -e "${YELLOW}Building for Linux (x86_64)...${NC}"
cargo build --release --target x86_64-unknown-linux-gnu --bin dex-cli
tar -czf "dist/dex-linux-x86_64-v${VERSION}.tar.gz" -C target/x86_64-unknown-linux-gnu/release dex-cli

# Build for macOS
echo -e "${YELLOW}Building for macOS (x86_64)...${NC}"
cargo build --release --target x86_64-apple-darwin --bin dex-cli
tar -czf "dist/dex-macos-x86_64-v${VERSION}.tar.gz" -C target/x86_64-apple-darwin/release dex-cli

# Build for Windows
echo -e "${YELLOW}Building for Windows (x86_64)...${NC}"
cargo build --release --target x86_64-pc-windows-gnu --bin dex-cli
zip -j "dist/dex-windows-x86_64-v${VERSION}.zip" target/x86_64-pc-windows-gnu/release/dex-cli.exe

echo -e "${GREEN}Packages created successfully:${NC}"
echo -e "  - dist/dex-linux-x86_64-v${VERSION}.tar.gz"
echo -e "  - dist/dex-macos-x86_64-v${VERSION}.tar.gz"
echo -e "  - dist/dex-windows-x86_64-v${VERSION}.zip"

echo -e "${YELLOW}To install, extract the package for your platform and run the installer.${NC}"