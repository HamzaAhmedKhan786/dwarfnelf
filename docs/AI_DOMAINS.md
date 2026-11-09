# AI domains in DwarfnElf

| Domain | How DwarfnElf uses it |
|---|---|
| Deep Learning | Transformer inference, quantization, supported MoE routing, KV-cache management, adapters, and model evaluation. |
| Machine Learning | Lightweight classifiers for domain/query routing, calibration, hardware capability prediction, and model selection. |
| Natural Language Processing | Tokenization, query understanding, embeddings, lexical/semantic retrieval, reranking, chunking, and citation matching. |
| Generative AI | Local text generation, structured JSON outputs, reasoning, summarization, code assistance, and grounded responses. |
| Reinforcement Learning | Future research for adaptive execution policies; not required for the baseline and never a prerequisite for safe routing. |
| Systems Engineering | CPU/GPU scheduling, memory admission, cache placement, SSD offload, process isolation, cancellation, and observability. |

## Baseline methodology

The baseline is layered: deterministic policy boundaries first, a typed orchestrator second, retrieval and model adapters third, and agents last. Agents use bounded ReAct (reason → act → observe) with allowlisted tools, explicit approvals for writes/network actions, step/token/time budgets, checkpoints, and an audit trace. Retrieved text is data, never authority.

## RAG methods

`python scripts/rag_build.py <documents> <index.json> --method lexical` is the lowest-dependency baseline. `tfidf` is the default offline semantic approximation. `dense` uses `sentence-transformers`; `hybrid-dense` combines lexical and dense scores. All methods preserve source and chunk identifiers so a later SQLite/ANN backend can retain citations and provenance.
