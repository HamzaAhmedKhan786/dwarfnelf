# Full-Scope Implementation Plan
All workstreams are included in product scope from day one; sequence reflects engineering dependencies, **not postponed feature discovery**.
1. Governance, threat model, ADRs, API/schema contracts, reproducible build and CI.
2. Rust daemon/CLI, desktop pairing/discovery, hardware profiler and resource budget enforcement.
3. llama.cpp worker, model registry, import/download integrity, dense CPU/GPU execution and streaming.
4. SQLite/FTS+vector RAG, domain pack manifests, parsers, index lifecycle, all retrieval policies and citation verifier.
5. Typed local decisions, calibration, optional Jev provider; planner and model/RAG joint policy.
6. Agent supervisor, tool registry, MCP adapters, approvals, sandbox, checkpoint/replay.
7. MoE expert telemetry, hierarchical caches, SSD/RAM/VRAM placement, context prefetch and conditional execution experiments.
8. Multimodal and speech backends, quantization/adapters and compatible speculative decoding.
9. Evaluation/observability suites, domain-specific accuracy gates, hardware benchmark matrix, dashboard and report export.
10. Signed cross-platform installers, updater/rollback, backup/restore, security hardening and release certification.
Each workstream has unit, integration, performance, adversarial, and offline E2E tests. Experimental techniques can fail their hypotheses without making baseline local inference unusable. See `RELEASE_GATES.md`.
