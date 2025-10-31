#!/bin/bash

# Cross-platform installer for Decentralized Exchange Application
# Supports Linux and macOS

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)
        DETECTED_OS=Linux
        ;;
    Darwin*)
        DETECTED_OS=Mac
        ;;
    *)
        echo -e "${RED}Unsupported OS: ${OS}${NC}"
        exit 1
        ;;
esac

echo -e "${GREEN}Installing Decentralized Exchange Application on ${DETECTED_OS}${NC}"

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo -e "${YELLOW}Rust not found. Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Check if Cargo is available
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Cargo not found. Please install Rust manually and try again.${NC}"
    exit 1
fi

echo -e "${GREEN}Building Decentralized Exchange CLI...${NC}"

# Build the CLI
cargo build --release --bin dex-cli

# Install the binary
echo -e "${GREEN}Installing binary...${NC}"
sudo cp target/release/dex-cli /usr/local/bin/dex

# Create config directory
sudo mkdir -p /etc/dex
sudo chmod 755 /etc/dex

echo -e "${GREEN}Installation complete!${NC}"
echo -e "${YELLOW}You can now run the application with: dex start${NC}"
echo -e "${YELLOW}For more information, run: dex --help${NC}"