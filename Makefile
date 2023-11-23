# Makefile for Rust project

# Environment variables
RUSTC = rustc
PYTHON = python3

# Default target
all: build

# Clean the project
clean:
	@echo "Cleaning project..."
	@cargo clean

# Install dependencies
install:
	@echo "Installing dependencies..."
	@cargo build

# Build the project
build:
	@echo "Building project..."
	@cargo build

# Run the project
run:
	@echo "Running project..."
	@cargo run

# Test the project
test:
	@echo "Running tests..."
	@cargo test

# Package the project
package:
	@echo "Packaging project..."
	@cargo package

# Deploy the project
deploy:
	@echo "Deploying project..."
	@cargo deploy

# Lint the project
lint:
	@echo "Linting project..."
	@cargo lint

# Checkstyle the project
checkstyle:
	@echo "Running checkstyle..."
	@cargo checkstyle

# Backup the project
backup:
	@echo "Creating backup..."
	@tar -czvf backup.tar.gz .

# Initialize the project
init:
	@echo "Initializing project..."
	@cargo init

.PHONY: all clean install build run test package deploy lint checkstyle backup init