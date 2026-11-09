# MAMA Core — Model Strategy and Research
## Model classes
Tiny router: Qwen2.5-0.5B-Instruct or task-specific ONNX classifier; general CPU baseline: Qwen2.5-1.5B-Instruct GGUF; additional dense profiles: compatible 3B/7B/8B GGUF; MoE research: Qwen1.5-MoE-A2.7B-Chat and Qwen3-30B-A3B where supported. Verify actual checkpoint, tokenizer, model license, quantization and backend compatibility before publishing manifests. These are candidates, not bundled assets or guaranteed 8 GB configurations.
## Backends
C++ llama.cpp as stable reference engine; model-specific MoE streaming backend for expert placement research; ONNX Runtime for classification/embeddings; experimental conditional-depth/adapter backend only with model-compatible training/calibration. Capability negotiation rejects unsupported models.
## User budgets
CPU-only 8/16/32 GB; NVIDIA 8/16/24 GB VRAM with RAM 16–64 GB; later driver/vendor coverage is an engineering compatibility task within full scope. Profiles include weights, KV cache, runtime buffers, model loading overhead, RAG and agent footprint. SSD offload trades capacity for severe possible latency costs.
## Research hypotheses
H1: query-aware expert prefetch beats routing-history-only caching on identical MoE model/hardware. H2: joint model+retrieval+memory planning improves quality/latency Pareto frontier. H3: task-aware quantization/adapter policies improve selected benchmarks. H4: supported conditional computation can reduce FLOPs without unacceptable quality loss. Each requires a preregistered baseline, held-out workloads and failure report.
## Evaluation control
Same model revision, tokenizer, quantization, prompt, generation settings, dataset and hardware; repeat cold/warm runs; record p50/p95 TTFT, decode t/s, peak RAM/VRAM, SSD reads, energy if supported, accuracy, expert cache hit and quality degradation. Avoid claiming '70B on 8GB' implies useful speed.
