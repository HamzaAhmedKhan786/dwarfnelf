# Product Requirements Document — DwarfnElf / MAMA
## Vision and users
Offline-first local AI for ordinary users (CPU-only 8–32 GB RAM), power users (8/16/24 GB VRAM), researchers, regulated-domain professionals and developers. User controls resource allocation and domain knowledge. One installer; daemon-only headless mode.
## Jobs to be done
- Run locally compatible AI models with explicit memory/CPU/storage budgets and safe degradation.
- Add verifiable domain knowledge without a hosted vector DB or dependency installer.
- Execute constrained agents and workflows with human approval and traceability.
- Inspect performance and answer quality, not marketing claims.
## Required workflows
First-run onboarding; hardware detection; resource budget wizard; model registry/download/import; license acceptance; knowledge-pack discovery/download/import; local indexing; multi-domain chat; evidence policy; agents/tools/approvals; benchmark comparison; model update; offline export; diagnostics; uninstall/data retention controls.
## Functional acceptance criteria
- First-run works offline after binaries and chosen models/data are present; no Python/Node/Docker required.
- UI discovers/authenticates daemon and detects version mismatch; daemon operates without UI.
- Budget violations are prevented where enforceable, otherwise detected and safely handled with transparent warnings.
- Every sourced claim includes inspectable document identity, version, section/page and evidence snippet; unsupported answers can abstain.
- Agent actions outside read-only scope require explicit scoped approval; audit log supports replay.
- Every performance claim backed by repeatable same-hardware baseline and saved experiment metadata.
## Nonfunctional targets
Startup and inference latency measured per hardware tier; no fabricated universal speed SLA. No silent network access; crash-safe indexing; recoverable jobs; explicit disk quotas; secure loopback API; signed releases; WCAG-minded UI; telemetry off by default.
## Explicit non-goals
Guaranteed 80% correctness on arbitrary prompts; automatically transforming any dense model into expert modules without retraining; universal cross-platform GPU support without backend validation; distributing restricted domain datasets without rights.
## Product KPIs
First-run completion; model compatibility; peak RSS/VRAM; p50/p95 TTFT; decode tokens/s; retrieval Recall@k, nDCG@k; citation precision; domain QA accuracy; agent task success; crash rate; user-controlled storage footprint.
