# Complete Requirements Catalogue
IDs are stable and testable. Current implementation and verification evidence are recorded in [requirement traceability](REQUIREMENT_TRACEABILITY.md).
## Hardware & budgets
HW-01 Detect CPU ISA/cores, OS, available/total RAM, GPU vendor/driver/VRAM, storage volumes, free disk, power/thermal data where accessible.
HW-02 Support CPU-only 8/16/32 GB RAM; GPU profiles 8/16/24 GB VRAM with RAM 16–64 GB; do not imply all models fit all profiles.
HW-03 User configures max RAM/VRAM/SSD cache, CPU threads/utilization target, context length, battery policy; validate against OS safety reserves and current free resources.
HW-04 Re-profile when resources change; cancel/replan under pressure; do not advertise soft CPU targets as hard OS-enforced caps.
## Models & inference
INF-01 Registry with hashes, formats, architectures, quantization, license, tokenizer, modalities, compatible backends.
INF-02 Support dense GGUF inference, model-specific native MoE, CPU/GPU offload, bounded KV caches, context windows, streaming and cancellation.
INF-03 Modular inference research track: expert-level telemetry, placement, prefetch, activation prediction, conditional depth only when model-compatible, fallback to native routing.
INF-04 Compare exact model/precision/prompt against llama.cpp and MoE reference baselines; reject unproven speed claims.
INF-05 Support adapter manifests and safe compatibility validation; multimodal and speech backends through capability negotiation.
## Knowledge & RAG
RAG-01 Local file import and consented download; PDFs, TXT, MD, HTML, DOCX, JSONL, CSV, code, images/audio when supported; parsing failures visible.
RAG-02 Provenance, source version, licensing, jurisdiction/effective dates, checksum, ACL, dedup, incremental indexing and deletion propagation.
RAG-03 Dense, lexical, hybrid, metadata, parent-child, hierarchical, contextual, graph-assisted, temporal, multi-query, reranking, multimodal and agentic retrieval via explicit policies.
RAG-04 Primary/secondary/strict evidence policies; cite exact passages; distinguish source assertions from generated interpretation.
RAG-05 Country-specific legal packs and dated medical guidance; conflicting sources surfaced, not silently merged.
## Decisions and agents
DEC-01 Typed decisions with schema, confidence calibration, abstain, explanation metadata and fallback; offline provider mandatory.
DEC-02 Optional external Jev integration only with permission and verified availability; no assumption of self-hostable weights.
AGT-01 Bounded planners, task graph, checkpoint/restart, MCP adapters, permissioned tools, human approvals, sandbox, time/token/cost budgets.
AGT-02 Audit every tool request, result, approval and failure with secrets redacted.
## UX/API/operations
UX-01 Bundled desktop installer plus daemon-only option; independent daemon lifecycle and auto discovery.
API-01 Versioned authenticated loopback API, streaming events, capability discovery, OpenAI-compatible subset documented explicitly.
OBS-01 Local metrics/traces/logs for runtime, retrieval, decisions and agents; opt-in diagnostic export.
EVAL-01 Task-specific >=80% target gates where feasible; full-model quality retention tracked separately; confidence intervals and held-out tests.
SEC-01 Signed binaries, SBOM, dependency/license audit, update integrity, local credential protection, prompt-injection boundaries, safe paths, encryption at rest options.
OPS-01 Backups, migrations, rollback, disaster recovery, crash-safe jobs, compatibility matrix and release certification.
