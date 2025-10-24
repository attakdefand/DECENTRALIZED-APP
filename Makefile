# Makefile for DECENTRALIZED-APP

# Variables
VERSION ?= 0.1.0

# Colors
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[1;33m
NC := \033[0m # No Color

# Default target
.PHONY: help
help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

.PHONY: build
build: ## Build all components
	@echo "$(GREEN)Building all components...$(NC)"
	cargo build --workspace

.PHONY: test
test: ## Run all tests
	@echo "$(GREEN)Running all tests...$(NC)"
	cargo test --workspace

.PHONY: clean
clean: ## Clean build artifacts
	@echo "$(GREEN)Cleaning build artifacts...$(NC)"
	cargo clean

.PHONY: fmt
fmt: ## Format code
	@echo "$(GREEN)Formatting code...$(NC)"
	cargo fmt --all

.PHONY: clippy
clippy: ## Run clippy linter
	@echo "$(GREEN)Running clippy...$(NC)"
	cargo clippy --workspace -- -D warnings

.PHONY: check
check: fmt clippy ## Check code formatting and linting

.PHONY: web-ui-build
web-ui-build: ## Build WebAssembly UI
	@echo "$(GREEN)Building WebAssembly UI...$(NC)"
	cd web-ui && wasm-pack build --target web

.PHONY: web-ui-dev
web-ui-dev: ## Build WebAssembly UI for development
	@echo "$(GREEN)Building WebAssembly UI for development...$(NC)"
	cd web-ui && wasm-pack build --target web --dev

.PHONY: web-ui-test
web-ui-test: ## Test WebAssembly UI
	@echo "$(GREEN)Testing WebAssembly UI...$(NC)"
	cd web-ui && wasm-pack test --headless --firefox

.PHONY: docker-up
docker-up: ## Start all services with Docker Compose
	@echo "$(GREEN)Starting services with Docker Compose...$(NC)"
	docker-compose up

.PHONY: docker-down
docker-down: ## Stop all services with Docker Compose
	@echo "$(GREEN)Stopping services with Docker Compose...$(NC)"
	docker-compose down

.PHONY: release
release: ## Create a new release (requires VERSION env var)
	@echo "$(GREEN)Creating release $(VERSION)...$(NC)"
	./scripts/create-release.sh v$(VERSION)

.PHONY: docs
docs: ## Build documentation
	@echo "$(GREEN)Building documentation...$(NC)"
	cargo doc --workspace --no-deps

.PHONY: serve-docs
serve-docs: docs ## Serve documentation locally
	@echo "$(GREEN)Serving documentation...$(NC)"
	cargo doc --workspace --no-deps --open

.PHONY: install-tools
install-tools: ## Install development tools
	@echo "$(GREEN)Installing development tools...$(NC)"
	rustup component add rustfmt clippy
	cargo install wasm-pack
	cargo install basic-http-server