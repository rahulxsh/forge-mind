# Forge Mind

A Rust-based RAG project for learning and building with **Rust, Axum, and Tokio**.

## Current Goal

Build the project incrementally while learning Rust backend and async development through practical implementation.

## Current Status

* Cargo workspace
* Axum API
* Tokio runtime
* Configuration
* `/health` endpoint
* Tracing/logging
* Docker setup
* Docker environment configuration

## Run

```bash
cargo run -p api
```

Docker:

```bash
docker build -t forge-mind-api .
docker run --env-file .env -p 3030:3030 forge-mind-api
```

Current API:

```text
GET /health
```