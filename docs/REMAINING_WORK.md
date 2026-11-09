# Remaining work

| Priority | Deliverable | Rough implementation range |
|---|---|---|
| P0 | Register a checksum-verified GGUF with provenance and backend compatibility, then integrate a pinned CPU worker with streaming and cancellation | 1–3 engineering weeks |
| P0 | OS credential vault and pairing, worker lifecycle, live resource checks, and a Windows clean-build verification lane | 2–4 engineering weeks |
| P1 | Pack provenance and filters, safe parsers, embeddings, hybrid search, evidence evaluation and CLI | 3–7 engineering weeks |
| P1 | Desktop onboarding and chat, scoped tool approvals, checkpoints and MCP sandboxing | 4–8 engineering weeks |
| P2 | Quality and pressure benchmarks, observability, installers, SBOM, signing, backup/restore and rollback | 4–10 engineering weeks |
| Research | MoE expert placement, speculative decoding, adapters and additional modalities | Scope after CPU baselines |

Ranges assume one engineer familiar with the repo, an available native Windows verifier, reviewed dependencies and access to a small approved GGUF and legal evaluation fixtures. They are planning estimates, with overlap and verification effort varying by platform.
