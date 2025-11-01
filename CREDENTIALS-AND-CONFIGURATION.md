# Complete Credentials and Configuration Guide

This document provides a comprehensive guide to all credentials and configuration required for the Decentralized Exchange (DEX) application.

## 1. Git/GPG Configuration

### GPG Keys
The project uses two GPG keys for commit signing:

1. **Primary Key**:
   - Key ID: `EAA3BD9BD5DDCE9A0EF9ED9FD7FE186C98EF1E30`
   - Email: `corodostudio@gmail.com`
   - Purpose: GitHub commit verification

2. **Secondary Key**:
   - Key ID: `AFA6C2850A909E0B`
   - Email: `attakdefand@gmail.com`
   - Fingerprint: `1D8031F5380552449BF7C167AFA6C2850A909E0B`
   - Purpose: Alternative signing key

### Git User Configuration
- Username: `AttakDefand`
- Email: `corodostudio@gmail.com`

## 2. Development Environment Prerequisites

### Rust Toolchain
- **Version**: Nightly (required for edition2024 support)
- **Components**: rustfmt, clippy
- **Configuration File**: `rust-toolchain.toml`

### Node.js
- **Purpose**: WASM tooling and development utilities
- **Version**: 16+ recommended

### Docker
- **Purpose**: Containerized services and local development
- **Components**: Docker Engine, Docker Compose

### Ethereum Development Network
- **Tools**: Hardhat/Ganache
- **Purpose**: Local blockchain for testing and development

## 3. Infrastructure Services

### PostgreSQL Database
- **Database Name**: `dex_os`
- **Username**: `dex_user`
- **Password**: Use a strong, randomly generated password
- **Port**: 5432

### Redis
- **Purpose**: Caching layer
- **Port**: 6379

### IPFS Node
- **Purpose**: Decentralized storage
- **Ports**: 
  - Swarm: 4001
  - Gateway: 8080
  - API: 5001

### Prometheus
- **Purpose**: Metrics collection
- **Port**: 9090

### Grafana
- **Purpose**: Visualization dashboard
- **Port**: 3000

## 4. Blockchain Networks

### Supported Chains
1. **EVM-compatible chains** (Ethereum Virtual Machine)
2. **Solana** (SVM - Solana Virtual Machine)
3. **Cosmos** (CosmWasm contracts)
4. **IBC-enabled chains** (Inter-Blockchain Communication)

## 5. API Services

### Connection URLs
- **Ethereum RPC URL**: `http://localhost:8545`
- **Database URL**: `postgresql://dex_user:${DATABASE_PASSWORD}@database:5432/dex_os` (use environment variables)

## 6. Security and Authentication

### Security Features
1. **Service Contract Allowlist**: Access control mechanism
2. **Authentication at Edge**: API security layer
3. **Rate Limiting**: Abuse prevention
4. **Contract Validation**: Smart contract security

## 7. Deployment Configuration

### Orchestration
- **Kubernetes**: Production deployment
- **Docker Compose**: Local development

### Port Mappings
| Service | Port | Description |
|---------|------|-------------|
| API Service | 3000 | RESTful API interface |
| Indexer Service | 3001 | Blockchain event indexing |
| Local Node | 8545 | Ethereum development node |
| Database | 5432 | PostgreSQL database |
| Redis | 6379 | Caching service |
| IPFS Swarm | 4001 | Peer-to-peer communication |
| IPFS Gateway | 8080 | HTTP gateway |
| IPFS API | 5001 | Management API |
| Prometheus | 9090 | Metrics collection |
| Grafana | 3000 | Visualization dashboard |

## Setup Instructions

### Automated Setup
Run the provided setup scripts:

**Windows (PowerShell)**:
```powershell
.\setup-environment.ps1
```

**Linux/macOS (Bash)**:
```bash
chmod +x setup-environment.sh
./setup-environment.sh
```

### Manual Setup

#### 1. Install Required Tools
```bash
# Install Rust nightly
rustup install nightly
rustup default nightly

# Install Node.js (varies by OS)
# Ubuntu/Debian:
curl -fsSL https://deb.nodesource.com/setup_16.x | sudo -E bash -
sudo apt-get install -y nodejs

# macOS:
brew install node

# Install Docker (varies by OS)
# Ubuntu/Debian:
sudo apt install docker.io docker-compose

# macOS:
brew install docker --cask
```

#### 2. Configure Git with GPG Signing
```bash
# Set Git user configuration
git config --global user.name "AttakDefand"
git config --global user.email "corodostudio@gmail.com"

# Configure GPG signing
git config --global commit.gpgsign true

# Windows:
git config --global gpg.program "C:\Program Files (x86)\GnuPG\bin\gpg.exe"

# Linux/macOS:
git config --global gpg.program gpg
```

#### 3. Import GPG Keys
```bash
# If you have access to the private keys
gpg --import private-key.asc
```

#### 4. Environment Variables
Create a `.env` file in the project root with the following content:
```env
# Database Configuration
DATABASE_URL=postgresql://dex_user:${DATABASE_PASSWORD}@localhost:5432/dex_os
DATABASE_NAME=dex_os
DATABASE_USER=dex_user
DATABASE_PASSWORD= # Set to a strong, randomly generated password

# Redis Configuration
REDIS_URL=redis://localhost:6379

# Ethereum RPC Configuration
ETHEREUM_RPC_URL=http://localhost:8545

# IPFS Configuration
IPFS_API_URL=http://localhost:5001
IPFS_GATEWAY_URL=http://localhost:8080

# API Service Configuration
API_PORT=3000
API_HOST=localhost

# Indexer Service Configuration
INDEXER_PORT=3001
INDEXER_HOST=localhost

# Security Configuration
SERVICE_CONTRACT_ALLOWLIST_ENABLED=true
RATE_LIMITING_ENABLED=true
CONTRACT_VALIDATION_ENABLED=true

# Logging Configuration
RUST_LOG=info
LOG_LEVEL=info

# Development Configuration
NODE_ENV=development
DEBUG=false
```

## Verification

After setup, verify all components are correctly configured:

```bash
# Check Rust installation
rustc --version

# Check Node.js installation
node --version

# Check Docker installation
docker --version

# Check GPG installation
gpg --version

# Check Git configuration
git config --global --list
```

## Troubleshooting

### Common Issues

1. **GPG Signing Failures**:
   - Ensure the GPG key is imported
   - Verify the GPG program path in Git config
   - Check that the email in Git config matches the GPG key email

2. **Docker Permission Issues**:
   - Add your user to the docker group: `sudo usermod -aG docker $USER`
   - Log out and back in

3. **Port Conflicts**:
   - Stop services using conflicting ports
   - Modify port mappings in docker-compose.yml if needed

4. **Database Connection Issues**:
   - Verify the database service is running
   - Check the DATABASE_URL in your environment

This comprehensive setup ensures all required credentials and configurations are properly established for developing, testing, and deploying the decentralized exchange application.