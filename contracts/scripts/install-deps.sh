#!/bin/bash

# Install Foundry
curl -L https://foundry.paradigm.xyz | bash
source ~/.bashrc

# Install dependencies
forge install --no-commit openzeppelin/openzeppelin-contracts
forge install --no-commit foundry-rs/forge-std

echo "Dependencies installed successfully!"