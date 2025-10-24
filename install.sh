#!/bin/bash

# Unified installer for Decentralized Exchange Application
# Detects the OS and runs the appropriate installation script

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Decentralized Exchange Application Installer${NC}"

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)
        echo -e "${GREEN}Detected Linux. Running Linux installer...${NC}"
        bash "$(dirname "$0")/install/install.sh"
        ;;
    Darwin*)
        echo -e "${GREEN}Detected macOS. Running macOS installer...${NC}"
        bash "$(dirname "$0")/install/install.sh"
        ;;
    *)
        # Check if we're on Windows with WSL
        if grep -q Microsoft /proc/version; then
            echo -e "${GREEN}Detected Windows Subsystem for Linux. Running Linux installer...${NC}"
            bash "$(dirname "$0")/install/install.sh"
        else
            # Try to detect PowerShell
            if command -v powershell.exe &> /dev/null; then
                echo -e "${GREEN}Detected Windows. Running PowerShell installer...${NC}"
                powershell.exe -ExecutionPolicy Bypass -File "$(dirname "$0")\install\install.ps1"
            else
                echo -e "${RED}Unsupported OS: ${OS}${NC}"
                exit 1
            fi
        fi
        ;;
esac