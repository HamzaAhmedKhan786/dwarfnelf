# DwarfnElf product scope

## Classification

| Field | Classification |
|---|---|
| Primary domain | AI Infrastructure & Systems Engineering |
| Specialization | Adaptive LLM Inference & Edge AI |
| Secondary domain | Generative AI / Large Language Models |
| Supporting technology | Machine Learning Systems (MLSys) |
| Additional capabilities | RAG, AI Agents, Model Routing, Decision Intelligence |
| Deployment category | Local AI / On-Device AI / Offline AI |
| Product category | AI Developer Tools & Runtime Platform |

## Capability coverage

| Area | Covered by DwarfnElf |
|---|---|
| Product | README, PRD, functional requirements, roadmap, release gates |
| Architecture | System architecture, system design, repository structure, API contracts |
| Inference | Dense and supported MoE execution, hardware budgets, caching, model strategy |
| RAG | Domain packs, hybrid retrieval, reranking, graph/temporal/agentic research paths |
| Decision engine | Local typed decisions, confidence calibration, optional Jev integration |
| AI agents | Planning, tools, MCP, permissions, approvals, checkpoints, monitoring |
| Evaluation | Accuracy, retrieval, grounding, agent success, performance, hardware benchmarks |
| Observability | Metrics, traces, logs, diagnostics, resource monitoring |
| Security | Local authentication, privacy, sandboxing, provenance, supply-chain protections |
| Deployment | Desktop installer, independent daemon, CLI, updates, rollback |
| Engineering | Rust workspace, Python research dependencies, JSON schemas, YAML configuration |

## Named subsystem boundaries

| Subsystem | Responsibility |
|---|---|
| MAMA Core | Adaptive modular inference, model planning, resource-aware execution |
| MAMA Decisions | Typed structured decision routing and calibrated confidence |
| MAMA Knowledge | Local domain-specific RAG, provenance, citations, abstention |
| MAMA Agents | Bounded orchestration, tools, permissions, approvals, monitoring |
| MAMA Eval | Quality, safety, retrieval, grounding, agent, and performance evaluation |

## Technology mapping

| Component | Baseline technology | Responsibility |
|---|---|---|
| Backend service | Rust + Tokio | Local service, scheduling, process supervision |
| Inference engine | llama.cpp (C/C++) | Quantized local LLM execution |
| Experimental modular inference | C++/Rust model-specific kernels | MoE expert loading, prefetching, conditional execution |
| Hardware discovery | Rust + native OS APIs | CPU, RAM, GPU, storage, and available-resource inspection |
| API | Axum (Rust) | REST, streaming, health checks |
| Structured decisions | Rust interface + local classifier/LLM | Typed routing, scoring, and policy validation |
| RAG indexing | SQLite FTS5 + embedded vector search | Local hybrid retrieval |
| Embeddings | Bundled ONNX Runtime models | Local document embeddings |
| Agents | Rust workflow orchestrator | Tools, planning, permissions, approvals, execution |
| Frontend | React + TypeScript + Vite | Dashboard, chat, and knowledge management |
| Desktop packaging | Tauri | Windows, Linux, and macOS desktop application |
| Research/evaluation | Python, PyTorch, pytest, lm-evaluation-harness | Training experiments, evaluation, and benchmarks |
| Distribution | GitHub Releases + signed installers | Downloadable production builds |

## Implementation status

- Implemented baseline: authenticated Rust daemon API, CPU/RAM/disk inventory, SQLite budget persistence, advisory CPU admission, FTS5 lexical search, and read-only knowledge workflows with bounded redacted history.
- Research prototypes: Python RAG, orchestrator, and bounded ReAct scripts.
- Next milestone: checksum-validated GGUF registry and a pinned llama.cpp CPU worker.
- Planned: OS-backed pairing, ONNX embeddings and hybrid retrieval, desktop client, agent sandbox, installers, and signed updates.
- Experimental: dense-model modularization, query-conditioned execution, MoE SSD expert streaming, and reinforcement-learning-based execution policies.

The attached visual assets are treated as brand references. They do not define runtime behavior, security policy, or implementation authority.
