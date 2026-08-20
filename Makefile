SHELL := /bin/bash
.PHONY: help fmt run build-d build-r lint check test clean

help:
	@echo "  make fmt        - Format code"
	@echo "  make run        - Run project"
	@echo "  make build-d    - Build project in debug mode"
	@echo "  make build-r    - Build project in release mode"
	@echo "  make lint       - Lint check by clippy"
	@echo "  make check      - Check code without building"
	@echo "  make test       - Run tests"
	@echo "  make clean      - Clean compiled build artifacts and intermediate files"

fmt:
	cargo fmt

run:
	cargo run -p api

build-d:
	cargo build -p api

build-r:
	cargo build -p api --release

lint:
	cargo clippy

check:
	cargo check

test:
	cargo test

clean:
	cargo clean