# MAMA Observe — Local Monitoring and Diagnostics
## Metrics
System CPU, RAM, swap, VRAM, SSD capacity/IO, temperature/battery where supported; model load, TTFT, decode tokens/s, queue depth, context/KV cache, MoE expert hit/miss and prefetch accuracy; RAG parse/index/search/rerank/grounding; decisions confidence/latency; agent state/tool errors/approvals; installer/update failures.
## Tracing
OpenTelemetry-compatible trace/span schema with request ID, parent/child spans for classification → retrieval → inference → verification → tool execution. Structured JSON logs with levels and retention. User content and paths redacted by default; local dashboards operate without cloud collector.
## Alerting and troubleshooting
Low-memory/low-disk warnings, repeated OOM, model mismatch, index corruption, stale pack, high latency, failed tool permission, unexpected network activity. Configurable in-app notifications; optional opt-in diagnostics export with preview.
## Privacy
Telemetry off-device transmission disabled by default. Never include raw medical/legal documents, prompts, API keys or user file paths in exported traces without explicit consent. Local audit retention and delete/export controls.
