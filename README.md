# DwarfnElf
**Small when needed. Powerful when possible.**

DwarfnElf is a **production-targeted, local-first adaptive AI platform** powered by **MAMA — Model Adaptive Modular AI**. It is designed to run the most capable compatible model within user-defined CPU, RAM, VRAM, SSD, power and latency budgets. It combines hardware-aware inference, local RAG knowledge packs, structured decision routing, supervised agents, verification, evaluations and observability.

> **Project status:** The Rust daemon serves an authenticated loopback API for CPU/RAM/disk inspection, SQLite-backed budgets and BM25 text search. It also evaluates advisory CPU admission and runs bounded read-only knowledge workflows with redacted history. Local generation and desktop packaging remain on the roadmap.

To try the implemented backend, run `.\scripts\smoke-daemon.ps1` from the repository root in PowerShell 7.2 or later. The script builds the daemon, starts it on a temporary loopback port and checks the API with a disposable database. See [backend setup](#develop-the-backend) and the [test report](docs/TEST_REPORT.md).

## Brand and distribution
- Product / desktop: **DwarfnElf**; technical engine: **MAMA**.
- Source repository: [HamzaAhmedKhan786/dwarfnelf](https://github.com/HamzaAhmedKhan786/dwarfnelf).
- Rust binary / CLI: `dwarfnelf`; daemon: `dwarfnelfd`; desktop: `DwarfnElf Desktop`.
- The distribution plan includes a bundled desktop and daemon installer and a separate headless installer. The daemon already runs independently.
- Packaged releases are intended to bundle runtime dependencies. Model and dataset downloads require consent and license review.

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
The daemon uses Rust, Tokio, Axum and bundled SQLite with WAL and FTS5. Python supports research and evaluation. The planned stack adds a pinned llama.cpp inference worker, ONNX Runtime for local embeddings and classifiers, embedded vector search, and a Tauri desktop client with React and TypeScript. See [technology choices](docs/TECH_STACK.md) and [dependency management](docs/DEPENDENCIES.md).

## Core research limitation
Splitting a dense 70B checkpoint into seven 10B files **does not** create seven independent usable models. Native MoE conditional experts are architecture-specific; dense-model pruning, distillation or early exit require dedicated training/calibration. RAG improves evidence availability, not inherent model reasoning or arbitrary model-size feasibility. A global 80% correctness guarantee is not scientifically defensible; define domain/task-specific 80% gates with confidence intervals and separately measure retention vs a full-model baseline.

## Documentation
- [Product scope and technology map](docs/PRODUCT_SCOPE.md) · [AI domains and methodologies](docs/AI_DOMAINS.md)
- [Candidate datasets and models](docs/KNOWLEDGE_SOURCES.md)
- [Development status](docs/DEVELOPMENT_STATUS.md) · [Requirement traceability](docs/REQUIREMENT_TRACEABILITY.md)
- [Test report](docs/TEST_REPORT.md) · [Dependency blockers](docs/DEPENDENCY_BLOCKERS.md) · [Remaining work](docs/REMAINING_WORK.md) · [Release readiness](docs/RELEASE_READINESS.md)
- [Product requirements](docs/PRD.md) · [Requirements](docs/REQUIREMENTS.md) · [System architecture](docs/SYSTEM_ARCHITECTURE.md) · [System design](docs/SYSTEM_DESIGN.md)
- [RAG design](docs/RAG_DESIGN.md) · [Model and inference design](docs/MODEL_INFERENCE.md) · [Decisions and Jev](docs/DECISION_ENGINE.md) · [Agents](docs/AGENTS.md)
- [Evaluation](docs/EVALUATION.md) · [Observability](docs/OBSERVABILITY.md) · [Security](docs/SECURITY.md) · [Deployment](docs/DEPLOYMENT.md) · [API](docs/API_CONTRACT.md) · [Data governance](docs/DATA_GOVERNANCE.md)
- [Architecture decisions](docs/ADR/0001-monorepo-local-daemon.md) · [Release gates](docs/RELEASE_GATES.md) · [Roadmap](docs/IMPLEMENTATION_PLAN.md) · [Risks](docs/RISK_REGISTER.md)

## Develop the backend
Use Rust stable and a native C toolchain for bundled SQLite. On Windows, use PowerShell 7.2 or newer for the smoke script.

```powershell
cargo fmt --all --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
.\scripts\smoke-daemon.ps1
```

For a manual local session, run the following in PowerShell. Keep the SQLite file and its sidecars in a directory accessible only to your account.

```powershell
$stateDir = Join-Path $env:LOCALAPPDATA 'DwarfnElf'
New-Item -ItemType Directory -Path $stateDir -Force | Out-Null
$env:DWARFNELF_DB = Join-Path $stateDir 'state.sqlite'
$env:DWARFNELF_TOKEN = [Convert]::ToHexString([Security.Cryptography.RandomNumberGenerator]::GetBytes(32))
cargo run --locked --bin dwarfnelfd
```

The daemon defaults to `127.0.0.1:8765`. Native clients use the same token in the `Authorization: Bearer …` header. `GET /health` is public on loopback. After stopping the daemon, clear the session token with `Remove-Item Env:DWARFNELF_TOKEN`. For endpoints and current limits, see the [API contract](docs/API_CONTRACT.md).

If Windows reports error 4551 or a fresh build reports a missing procedural macro, check the documented [Code Integrity blocker](docs/DEPENDENCY_BLOCKERS.md).

## Monorepo layout
See `docs/REPOSITORY_STRUCTURE.md` for the target tree and component ownership. The workspace contains the running backend slice alongside scaffolding, sample configuration and research tools.

## Contribution and licensing
Product code license is **undecided** pending dependency/license audit; do not assume model or dataset redistribution rights. `LICENSE-DECISION.md`, `CONTRIBUTING.md`, `SECURITY.md`, and `CODE_OF_CONDUCT.md` define release governance. Never commit model weights, copyrighted datasets, secrets or private user documents.
