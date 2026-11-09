# Repository Tree (full product scope)
```text
dwarfnelf/
├── README.md  LICENSE-DECISION.md  CONTRIBUTING.md  CODE_OF_CONDUCT.md  SECURITY.md
├── requirements.txt  pyproject.toml  Cargo.toml  Cargo.lock  package.json
├── apps/
│   ├── desktop/             # Tauri + React/TypeScript, screens, onboarding, chat, models, knowledge, agents, metrics, settings
│   ├── cli/                 # Rust command line
│   └── daemon/              # Rust background service
├── crates/
│   ├── mama-api/            # Axum handlers, auth, SSE, capability discovery
│   ├── mama-hardware/       # probes, budget enforcement, admission
│   ├── mama-scheduler/      # queues, priorities, cancellation
│   ├── mama-inference/      # model backends, workers, offload, KV, MoE, adapters
│   ├── mama-model-registry/ # manifests, downloads, integrity, license checks
│   ├── mama-decisions/      # local structured classifier, optional Jev adapter
│   ├── mama-knowledge/      # parsing, chunking, indexing, hybrid/graph/temporal retrieval, citations
│   ├── mama-agents/         # planning, tools, MCP, approvals, sandbox, checkpoints
│   ├── mama-eval/           # benchmark orchestration and result schemas
│   ├── mama-observe/        # metrics, tracing, diagnostics
│   ├── mama-security/       # credentials, ACL, validation
│   ├── mama-storage/        # SQLite, vector, content store, migrations
│   └── mama-common/         # typed IDs, errors, config, event contracts
├── native/
│   ├── llama.cpp/           # pinned C++ inference source or reference
│   ├── onnxruntime/         # platform binary packaging manifests
│   └── kernels/             # conditional inference research
├── contracts/              # OpenAPI, JSON schemas, event contracts, SDK generation
├── configs/                # hardware, model, RAG, agents, observability, security
├── models/manifests/       # no committed weights
├── knowledge/packs/        # pack manifests, licenses, recipes, no copyrighted corpora
├── datasets/manifests/     # dataset source manifests, no large datasets
├── research/               # notebooks, training, MoE probes, calibration, ablations
├── benchmarks/             # workloads, harness adapters, reports, hardware profiles
├── tests/                  # unit, integration, contract, E2E, security, stress, fixtures
├── installers/             # Windows, Linux, macOS packaging and signing
├── scripts/                # development, reproducible build, model checks
├── migrations/             # metadata and index schema migrations
├── docs/                   # all design docs, ADRs, runbooks
├── .github/workflows/      # CI, supply chain, release, performance
└── .gitignore
```
This is the target layout. See [development status](DEVELOPMENT_STATUS.md) for executable components and verification evidence.
