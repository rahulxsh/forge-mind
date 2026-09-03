# Forge Mind

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) ![Axum](https://img.shields.io/badge/Axum-000000?style=for-the-badge&logo=rust&logoColor=white) ![Tokio](https://img.shields.io/badge/Tokio-000000?style=for-the-badge&logo=rust&logoColor=white) ![PostgreSQL](https://img.shields.io/badge/PostgreSQL-4169E1?style=for-the-badge&logo=postgresql&logoColor=white) ![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white) ![Qdrant](https://img.shields.io/badge/Qdrant-DC244C?style=for-the-badge&logo=qdrant&logoColor=white)![Status](https://img.shields.io/badge/Status-Alpha-orange?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

![GitHub Stars](https://img.shields.io/github/stars/rahulxsh/forge-mind?style=for-the-badge)
![GitHub Forks](https://img.shields.io/github/forks/rahulxsh/forge-mind?style=for-the-badge)
![GitHub Issues](https://img.shields.io/github/issues/rahulxsh/forge-mind?style=for-the-badge)

**Forge Mind** is an open-source, Rust-based RAG project focused on learning and building a production-oriented document processing and retrieval system with **Rust, Axum, Tokio, PostgreSQL, Datalab/Chandra-2, embeddings, and Qdrant**.

> 🚧 **Status: Active Development**

The project is built incrementally, with each part implemented and understood before moving to the next stage.

---

## Ingestion Architecture Overview

![Forge Mind Architecture](./architecture.png)

### Document Processing Flow

```text
┌──────────────┐
│   Document   │
│   Upload     │
└──────┬───────┘
       │
       ▼
┌──────────────────┐
│    Axum API      │
│                  │
│  HTTP Request    │
│  Validation      │
│  Serialization   │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│    AppState      │
│                  │
│  mpsc Sender     │
│  DB Repository   │
│  Shared Services │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Bounded mpsc    │
│     Channel      │
└────────┬─────────┘
         │
         ▼
┌──────────────────────────────┐
│       Worker Pool            │
│                              │
│  Worker 1  Worker 2  ... W10 │
└──────────────┬───────────────┘
               │
               ▼
          process_job()
               │
       ┌───────┴────────┐
       │                │
       ▼                ▼
  Extraction        Chunking
  Datalab /         Markdown
  Chandra-2         → Chunks
       │                │
       └───────┬────────┘
               │
               ▼
          Embeddings
               │
               ▼
            Qdrant
               │
               ▼
       Semantic Search
````

---

## Current Goal

Build the project incrementally while learning and applying Rust backend and asynchronous programming concepts through practical implementation.

The long-term goal is to build a complete RAG pipeline:

```text
Document
   ↓
Upload
   ↓
Background Processing
   ↓
Document Extraction
   ↓
Chunking
   ↓
Embeddings
   ↓
Vector Database
   ↓
Retrieval
   ↓
LLM
   ↓
RAG Response
```

---

## Current Status

### Backend Foundation

* [x] Cargo workspace
* [x] Axum API
* [x] Tokio runtime
* [x] Application configuration
* [x] Environment configuration
* [x] `/health` endpoint
* [x] Tracing / logging
* [x] Docker setup

### Async Worker System

* [x] Tokio `async/await`
* [x] `tokio::spawn`
* [x] Bounded `mpsc` channel
* [x] Backpressure
* [x] Worker pool
* [x] 10 concurrent workers
* [x] `Arc<Mutex<mpsc::Receiver<Job>>>`
* [x] `Send` / `Sync`
* [x] `CancellationToken`
* [x] Graceful shutdown
* [x] `JoinSet` / worker coordination

### Database

* [x] PostgreSQL
* [x] SQLx
* [x] Database migrations
* [x] Document model
* [x] Document status lifecycle
* [x] Repository pattern
* [x] CRUD operations
* [x] Integration tests

### Document Upload

* [x] Multipart upload
* [x] File validation
* [x] Extension validation
* [x] Local file storage
* [x] UUID-based storage filenames
* [x] Document metadata persistence
* [x] Upload integration tests

### Document Processing

* [x] Job creation
* [x] Background processing
* [x] Document status updates
* [x] Worker document lookup
* [x] `Queued → Processing → Processed`
* [x] Failed processing state
* [x] `process_job()` orchestration boundary
* [x] Datalab extraction crate
* [x] Datalab / Chandra-2 integration
* [x] Markdown extraction

### Next

* [ ] Chunking crate
* [ ] Chunk representation
* [ ] Markdown chunking
* [ ] Embedding service
* [ ] Qdrant integration
* [ ] Vector storage
* [ ] Retrieval
* [ ] Semantic search
* [ ] RAG generation

---

## Worker Architecture

The API does not perform document processing directly.

Instead, the API creates a job and the worker pool processes it asynchronously.

```text
                     Application
                          |
             +------------+------------+
             |                         |
             v                         v
         Axum Server              Worker Pool
             |                    (10 Workers)
             |                         |
             v                    +----+----+
         AppState                 |    |    |
             |                    v    v    v
             |                   W1   W2   W3 ... W10
             |
             v
       mpsc::Sender<Job>
             |
             v
       Bounded Channel
             |
             +----------------------+
                                    |
                              Workers pull jobs
                                    |
                                    v
                             process_job()
                                    |
                        +-----------+-----------+
                        |                       |
                        v                       v
                   Extraction                Chunking
                        |                       |
                    Datalab                  Chunks
                  / Chandra-2                   |
                        |                       v
                        v                   Embeddings
                    Markdown                    |
                                                v
                                             Qdrant
```

### Worker lifecycle

```text
Job received
     ↓
Queued
     ↓
Processing
     ↓
Document extraction
     ↓
Chunking
     ↓
Embedding
     ↓
Vector storage
     ↓
Processed
```

If processing fails:

```text
Processing
     ↓
Error
     ↓
Failed
```

### Graceful Shutdown

Workers listen for a `CancellationToken`.

```text
CancellationToken
       |
       v
Worker shutdown
       |
       v
JoinSet
       |
       v
Clean application shutdown
```

---

## Current API

| Method | Endpoint                | Description        |
| ------ | ----------------------- | ------------------ |
| `GET`  | `/health`               | Health check       |
| `POST` | `/api/v1/documents`     | Upload a document  |
| `GET`  | `/api/v1/documents`     | Get documents      |
| `GET`  | `/api/v1/documents/:id` | Get document by ID |

### Health Check

```bash
curl http://localhost:3030/health
```

---

## Run

### Local

```bash
cargo run -p api
```

### Tests

```bash
cargo test
```

or:

```bash
make test
```

---

## Docker

Build the image:

```bash
docker build -t forge-mind-api .
```

Run:

```bash
docker run --env-file .env -p 3030:3030 forge-mind-api
```

---

## Configuration

Create a `.env` file using the project's environment template.

Example:

```env
DATABASE_URL=postgres://user:password@localhost/forge_mind
HOST=0.0.0.0
PORT=3030

DATALAB_API_KEY=your_api_key
```

Secrets should never be committed to the repository.

---

## Technology Stack

| Technology              | Purpose                     |
| ----------------------- | --------------------------- |
| **Rust**                | Backend language            |
| **Axum**                | HTTP framework              |
| **Tokio**               | Async runtime               |
| **PostgreSQL**          | Document metadata and state |
| **SQLx**                | Database access             |
| **Datalab / Chandra-2** | Document extraction         |
| **Qdrant**              | Vector database             |
| **Docker**              | Containerization            |
| **Tracing**             | Observability               |

---

## Workspace Structure

```text
forge-mind/
│
├── apps/
│   └── api/
│       ├── migrations/
│       │
│       └── src/
│           ├── handlers/
│           ├── jobs/
│           ├── models/
│           ├── repositories/
│           ├── routes/
│           ├── services/
│           ├── storage/
│           ├── workers/
│           ├── error.rs
│           ├── response.rs
│           ├── lib.rs
│           └── main.rs
│
├── crates/
│   ├── config/
│   ├── database/
│   └── extraction/
│
├── tests/
│   └── fixtures/
│
├── Dockerfile
├── Makefile
├── Cargo.toml
└── README.md
```

---

## Workspace Crates

### Config

Centralizes application configuration and environment variables.

### Database

Contains reusable database-related functionality.

### Extraction

Provides the document extraction boundary.

The API/worker does not need to know the implementation details of the external extraction service.

```text
process_job()
      ↓
DatalabExtractor
      ↓
Datalab API
      ↓
Chandra-2
      ↓
Markdown
```

The extractor exposes a simple interface:

```rust
let markdown = extractor.extract(path).await?;
```

The same pattern will eventually be used for:

```text
crates/
├── extraction/
├── chunking/
├── embeddings/
└── vector_store/
```

---

## Design Principles

### 1. Asynchronous Processing

Document processing happens in background workers rather than blocking the HTTP request.

```text
HTTP Request
     ↓
Store Document
     ↓
Create Job
     ↓
Return Response
     ↓
   Worker
     ↓
Process Document
```

### 2. Bounded Queues

The worker queue is bounded to provide backpressure when the system receives more work than the workers can process.

### 3. Separation of Responsibilities

```text
Axum
  ↓
Application Layer
  ↓
Workers
  ↓
Domain / Infrastructure Crates
  ↓
External Services
```

### 4. External Services Behind Abstractions

External systems such as document extraction, embedding providers, and vector databases should be accessed through dedicated crates/interfaces.

This keeps the application decoupled from individual providers.

---

## Roadmap

```text
[x] Rust workspace
[x] Axum API
[x] Tokio
[x] PostgreSQL
[x] SQLx
[x] Background workers
[x] Worker pool
[x] Graceful shutdown
[x] File upload
[x] Local storage
[x] Document lifecycle
[x] Datalab / Chandra-2 extraction

[ ] Chunking
[ ] Embeddings
[ ] Qdrant
[ ] Retrieval
[ ] Semantic Search
[ ] RAG Generation
[ ] Production Deployment
```

---

## Open Source

Forge Mind is an open-source project.

Contributions, issues, feature requests, architectural discussions, and improvements are welcome.

If you find a bug or have an idea:

* Open an issue
* Start a discussion
* Submit a pull request

---

## Contributing

1. Fork the repository.
2. Create a feature branch.
3. Make your changes.
4. Add or update tests where appropriate.
5. Run:

```bash
make test
```

6. Open a pull request.

---

## Project Status

Forge Mind is currently under active development.

The architecture and APIs may change as the RAG pipeline evolves.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

Copyright (c) 2026 Forge Mind contributors