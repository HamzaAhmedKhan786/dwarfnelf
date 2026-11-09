# Production Technology Stack
- Rust stable: Tokio, Axum, serde, tracing, clap, sqlx/rusqlite (select one after benchmark), native OS APIs; Cargo.lock committed.
- C++: llama.cpp pinned git commit as native inference dependency; CMake and signed platform binaries. Rust FFI or isolated worker with versioned IPC.
- ONNX Runtime: bundled inference for classifier/embeddings/rerank; license and platform support audited.
- Storage: SQLite WAL + FTS5; choose USearch or equivalent embedded ANN after benchmarks; optional graph relation tables, not mandatory graph server.
- Desktop: Tauri 2 + React + TypeScript + Vite, lockfile and platform packaging.
- Python research: Python 3.11/3.12, PyTorch/Transformers/PEFT/Accelerate, lm-evaluation-harness, datasets, pytest. Not a runtime end-user requirement.
- Observability: tracing + OpenTelemetry-compatible exporters, local structured logs; network exporters disabled by default.
- Security: OS credential vault, signed updates, sandboxed tools, SBOM.
- External Jev: optional opt-in provider only; no offline dependence or unverified self-host claim.
