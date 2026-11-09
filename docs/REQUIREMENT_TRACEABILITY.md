# Requirement traceability

Stable IDs come from the [requirements catalogue](REQUIREMENTS.md). The [criterion source](development-progress.json) records testable slices and weights. A verified slice can cover part of a larger requirement.

| Requirement IDs | Status and evidence |
|---|---|
| HW-01, HW-02 | Partial. CPU/RAM/disk snapshot covered by hardware test and smoke. GPU profiles and power/thermal probes pending. |
| HW-03, HW-04 | Partial. Saved budget, overflow, RAM reserve and missing probe tests. Hard enforcement and pressure cancellation pending. |
| INF-01, INF-02 | Planned. Registry and pinned CPU backend pending. |
| INF-03, INF-04, INF-05 | Experimental or planned. MoE and adapter tracks documented. No parity or acceleration evidence. |
| RAG-01, RAG-02 | Partial. Bounded text submission and transactional replacement. Bulk and file parsers, checksums, pack metadata and ACLs pending. |
| RAG-03, RAG-04 | Partial. BM25 results with source and chunk references and empty-hit abstention. Dense, hybrid and evidence verification pending. |
| RAG-05 | Planned. Dated and jurisdiction-aware verified packs pending. |
| DEC-01 | Partial. Typed CPU admission with explicit outcomes. Offline domain/model routing and calibration pending. |
| DEC-02 | Planned. Optional provider availability and permissions unverified. |
| AGT-01 | Partial. Read-only allowlist and bounded steps through the daemon. MCP, approvals, hard cancellation and checkpoints pending. |
| AGT-02 | Partial. Latest 100 redacted run summaries persisted. Replay and approval audit pending. |
| UX-01 | Partial. Independent daemon runs. Rust CLI, desktop pairing and installers pending. |
| API-01 | Partial. Authenticated v1 requests and capability discovery verified. Streaming and chat parity pending. |
| OBS-01 | Partial. JSON request logs and limited redacted run summaries. Metrics, retention and export pending. |
| EVAL-01 | Planned. Functional tests exist. Domain benchmark results and confidence intervals pending. |
| SEC-01 | Partial. Loopback, bearer and origin checks verified. OS credential storage, signing, audits and sandboxing pending. |
| OPS-01 | Partial. SQLite migration, persistence and future schema rejection tested. Backup/restore, recovery and release certification pending. |
