.PHONY: help build run test clean check fmt lint

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build the project
	cargo build

run: ## Run the project
	cargo run

test: ## Run tests
	cargo test

test-offline: ## Run tests in offline mode
	SQLX_OFFLINE=true cargo test

check: ## Check the project for errors
	cargo check

fmt: ## Format the code
	cargo fmt

lint: ## Run clippy lints
	cargo clippy

clean: ## Clean the project
	cargo clean

setup: ## Setup the project (install dependencies)
	cargo build

all: fmt lint test ## Run fmt, lint and test