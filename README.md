# DwarfnElf
**Small when needed. Powerful when possible.**

DwarfnElf is a **production-targeted, local-first adaptive AI platform** powered by **MAMA — Model Adaptive Modular AI**. It is designed to run the most capable compatible model within user-defined CPU, RAM, VRAM, SSD, power and latency budgets. It combines hardware-aware inference, local RAG knowledge packs, structured decision routing, supervised agents, verification, evaluations and observability.

> **Project status:** The repository is under active implementation. A first Rust daemon vertical slice now builds locally and exposes health, capability, and budget endpoints. Inference, installers, RAG, agents, and evaluation remain unimplemented; no accuracy or performance claims are made.

## Brand and distribution
- Product / desktop: **DwarfnElf**; technical engine: **MAMA**.
- Suggested GitHub repository: **`dwarfnelf`** (check availability, domains and trademarks before release).
- Rust binary / CLI: `dwarfnelf`; daemon: `dwarfnelfd`; desktop: `DwarfnElf Desktop`.
- One monorepo, one default installer bundling desktop and daemon, plus optional headless daemon-only installer. Separate processes communicate over authenticated loopback or local IPC. UI closing does not necessarily stop the daemon.
- End users do **not** install Python, pip, Rust, Docker, vector DBs or Node. Models and domain datasets are separately downloaded/imported with explicit consent and verified licensing.

## Core product capabilities (all in product scope)
1. Hardware inventory, benchmark-based capability profiling, thermal/battery awareness and strict configurable budgets.
2. Dense quantized inference, architecture-supported MoE expert streaming, adapter selection, CPU/GPU/disk placement, KV-cache management, quantization, batching, prefetching, speculative decoding where supported and experimental query-conditioned execution.
3. MAMA Decisions: typed local decisions, calibration, deterministic guardrails, provider interface; optional Jev remote adapter only if licensing, availability and privacy permit.
4. Domain knowledge packs: science, computer science, medical, agriculture, country- and time-scoped law, and arbitrary user folders. Hybrid/graph/temporal/hierarchical/multimodal retrieval with citations, trust ranking, freshness and abstention.
5. MAMA Agents: bounded planning, tool registry, approvals, sandbox, workflow checkpoints, budgets, human-in-the-loop, MCP-compatible connectors with explicit permissions.
6. MAMA Eval: accuracy, groundedness, citation fidelity, retrieval metrics, agent task success, regression and performance measurements across hardware tiers.
7. MAMA Observe: metrics, tracing, event logs, model/expert/cache traces, redaction, local dashboard and opt-in diagnostic export.
8. Secure offline-first packaging, self-updates, model provenance, signed artifacts, SBOM, migrations, backup and recovery.

## Installation experience (target behavior; not yet implemented)
1. Download signed DwarfnElf installer from GitHub Releases, verify signature, install.
2. Desktop launches daemon or securely discovers an already running compatible daemon.
3. Hardware wizard scans CPU/GPU, total/available memory, storage, battery and OS. User sets explicit budgets; OS safety reserves are always retained.
4. Compatibility planner proposes models and performance estimates with uncertainty. User approves model downloads or selects existing local weights.
5. User selects domain pack(s), downloads licensed public corpora or points to a folder; indexes are created locally, with resumable progress and provenance.
6. User chooses evidence policy (strict / primary / supporting), agent permissions and execution strategy, then chats or uses local API.
7. Dashboard shows quality indicators, citations, memory, TTFT, tokens/s, cache behavior and agent/tool traces.

## Proposed CLI (daemon CLI is not implemented yet)
```powershell
dwarfnelf status
dwarfnelf hardware inspect
dwarfnelf budget set --ram-gb 6 --ssd-gb 20 --cpu-percent 50
dwarfnelf models discover
dwarfnelf models import --path 'C:\Models\model.gguf'
dwarfnelf knowledge import --domain legal --jurisdiction DE --path 'C:\Knowledge\GermanLaw'
dwarfnelf knowledge index --domain legal
dwarfnelf chat --domain legal --evidence strict
dwarfnelf eval run --suite core --profile cpu-8gb
dwarfnelf agents runs list
dwarfnelf diagnostics export --redact
```

## Engineering baseline
Rust stable + Tokio/Axum for daemon, resource policy and agents; C++ llama.cpp for initial local inference; ONNX Runtime for local embedding/classifier/reranker models; SQLite WAL + FTS5 plus benchmark-selected embedded vector index; Tauri + React/TypeScript for desktop; Python for **research/evaluations only**. All native dependencies are bundled. No dependency on Jev cloud service for core offline operation. See `docs/TECH_STACK.md` and `docs/DEPENDENCIES.md`.

## Core research limitation
Splitting a dense 70B checkpoint into seven 10B files **does not** create seven independent usable models. Native MoE conditional experts are architecture-specific; dense-model pruning, distillation or early exit require dedicated training/calibration. RAG improves evidence availability, not inherent model reasoning or arbitrary model-size feasibility. A global 80% correctness guarantee is not scientifically defensible; define domain/task-specific 80% gates with confidence intervals and separately measure retention vs a full-model baseline.

## Documentation
- [Product scope and technology map](docs/PRODUCT_SCOPE.md) · [AI domains and methodologies](docs/AI_DOMAINS.md)
- [Product requirements](docs/PRD.md) · [Requirements](docs/REQUIREMENTS.md) · [System architecture](docs/SYSTEM_ARCHITECTURE.md) · [System design](docs/SYSTEM_DESIGN.md)
- [RAG design](docs/RAG_DESIGN.md) · [Model and inference design](docs/MODEL_INFERENCE.md) · [Decisions and Jev](docs/DECISION_ENGINE.md) · [Agents](docs/AGENTS.md)
- [Evaluation](docs/EVALUATION.md) · [Observability](docs/OBSERVABILITY.md) · [Security](docs/SECURITY.md) · [Deployment](docs/DEPLOYMENT.md) · [API](docs/API_CONTRACT.md) · [Data governance](docs/DATA_GOVERNANCE.md)
- [Architecture decisions](docs/ADR/0001-monorepo-local-daemon.md) · [Release gates](docs/RELEASE_GATES.md) · [Roadmap](docs/IMPLEMENTATION_PLAN.md) · [Risks](docs/RISK_REGISTER.md)

## Build the first vertical slice
Requires Rust stable. From the repository root:

```powershell
cargo fmt --all
cargo test --workspace
.\scripts\smoke-daemon.ps1
```

The daemon binds to `127.0.0.1:8765` and currently implements `/health`, `/v1/capabilities`, and `/v1/budgets`. The smoke test is intentionally local and does not download models or contact external services.

## Monorepo layout
See `docs/REPOSITORY_STRUCTURE.md` for the complete tree and component ownership. This blueprint includes scaffolding placeholders, sample schemas/configuration and research requirements; it is **not** an end-to-end implemented product.

## Contribution and licensing
Product code license is **undecided** pending dependency/license audit; do not assume model or dataset redistribution rights. `LICENSE-DECISION.md`, `CONTRIBUTING.md`, `SECURITY.md`, and `CODE_OF_CONDUCT.md` define release governance. Never commit model weights, copyrighted datasets, secrets or private user documents.
