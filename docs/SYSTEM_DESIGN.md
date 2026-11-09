# System Design
## Hardware budget admission
Collect total and *currently available* resources; reserve OS safety margin and other processes. Admission computes weights + runtime buffers + KV cache + embeddings + retrieval index + agent/tool overhead + transfer buffers. Respect user maxima; use process/job-object/cgroup enforcement where supported; distinguish advisory CPU-percent targets. Disk quota includes downloaded models, indices, temp files and checkpoints. Track high-water marks and preempt under pressure.
## Scheduling
Jobs have priority, memory estimate, cancellation token, deadline, modality, privacy policy and model affinity. Separate queues for interactive inference, indexing, evaluation and background downloads. Avoid starvation; cap concurrency by measured memory; preserve interactive latency; use cooperative cancellation and checkpoints.
## Adaptive planning
Inputs: user query, conversation state, domain, evidence policy, available hardware, user budget, model manifest, measured performance profiles. Outputs: typed plan `{retrievers, model_id, quantization, context, offload, cache_policy, tool_policy, fallback}`. Policy is deterministic at safety boundaries; ML router only recommends. Record selection rationale and confidence.
## Cache and execution
Hierarchical caches: KV, embedding, document, model weights, MoE experts. LRU/ARC-like policies selected by benchmarks; expert prefetch must not override native expert routing. Cache keys include model hash, tokenizer, quantization and version. SSD offload uses bounded I/O and validates hashes. Sparse MoE support only for tested architectures. Dense modularization remains experimental and is never represented as lossless arbitrary splitting.
## RAG indexing lifecycle
Discover → permission check → checksum → parse → normalize → deduplicate → metadata → chunk → embed → lexical/vector index → validate → atomic publish → incremental update → purge. Use transactional manifests and resumable jobs. No silent network access.
## Agents
Planner creates bounded DAG; supervisor authorizes tool call; sandbox executes; verifier checks output; checkpoint saves state; user approves high-risk actions. Never allow retrieved text to override system policies.
## Quality
Decision confidence is calibrated on held-out data, not raw LLM self-confidence. Source grounding uses passage entailment and citation verification with error rates measured. If insufficient evidence, abstain or request clarification.
