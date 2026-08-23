SHELL := /bin/bash
.PHONY: help fmt run build-d build-r lint check test clean migrate-add migrate-run migrate-revert

help:
	@echo "  make fmt                                 - Format code"
	@echo "  make run                                 - Run project"
	@echo "  make build-d                             - Build project in debug mode"
	@echo "  make build-r                             - Build project in release mode"
	@echo "  make lint                                - Lint check by clippy"
	@echo "  make check                               - Check code without building"
	@echo "  make test                                - Run tests"
	@echo "  make clean                               - Clean compiled build artifacts and intermediate files"
	@echo "  make migrate-add name='migration_name'   - Add database Migration"
	@echo "  make migrate-run                         - Apply database migration"
	@echo "  make migrate-revert                      - Revert database migration"

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

migrate-add:
	sqlx migrate add --source ./apps/api/migrations $(name)

migrate-run:
	sqlx migrate run --source ./apps/api/migrations

migrate-revert:
	sqlx migrate revert --source ./apps/api/migrations