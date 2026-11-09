# API Contract v1

## Implemented

- `GET /health` returns status and the daemon version.
- `GET /v1/capabilities` lists implemented API features.
- `GET /v1/hardware` reports logical CPU count, RAM and disk capacities. GPU and VRAM detection are pending.
- `GET /v1/budgets` and `PUT /v1/budgets` read and replace the saved settings. CPU percentage is a target. Runtime enforcement is pending.
- `POST /v1/knowledge/import` accepts `{"source":"notes.txt","text":"Local text"}` and returns the chunk count. The native caller reads its file and supplies UTF-8 text. The source label is a client-provided identifier. Reusing it atomically replaces its chunks. Text is limited to 128 KiB.
- `POST /v1/knowledge/search` accepts `{"query":"local"}` and returns up to ten BM25-ranked hits with source label, zero-based chunk ordinal and text. `abstained: true` indicates zero lexical matches. Evidence verification and answer generation are separate work.
- `POST /v1/decisions/admission` accepts positive `weights_bytes`, `kv_cache_bytes` and `runtime_bytes`. It evaluates their checked sum against the saved RAM budget and a live RAM snapshot, reserving the larger of 1 GiB or 10% of total RAM. The result and CPU thread suggestion are advisory.
- `POST /v1/agents/runs` accepts one to four `{"tool":"knowledge_search","query":"local"}` steps in a `steps` array. The synchronous response reports status, deduplicated citation references and redacted step summaries. It checks a two-second deadline between steps. An in-flight SQLite query can finish after that deadline. Clients must check status before using partial results.
- `GET /v1/agents/runs/{id}` retrieves a UTC timestamp and redacted summary using the run ID returned by POST. The database retains the latest 100 summaries by ID. Stored summaries omit queries, source labels, citations and document content.

Send requests to the numeric loopback address. The daemon requires a bearer token outside `/health`, rejects browser-origin requests, and returns `X-Request-Id` on API responses. API errors use `{"code":"…","error":"…"}`. Rotate the development credential by restarting with a fresh token. Production pairing and OS credential-vault integration are pending.

## Planned surface

`GET /health` readiness, build version; `GET /v1/capabilities` supported models, modalities, features; `GET /v1/hardware` sanitized inventory; `PUT /v1/budgets` validated limits; `GET/POST /v1/models` registry and import; `POST /v1/knowledge/import`, `/index`, `/search`; `POST /v1/chat/completions` streaming response with citations; `POST /v1/decisions/evaluate`; `POST /v1/agents/runs`, `GET /v1/agents/runs/{id}`, `POST /v1/agents/runs/{id}/approve`; `POST /v1/evals/runs`; `GET /v1/metrics`; `GET /v1/events` SSE.
Every request uses schema version, request ID, auth credential, cancellation semantics and typed errors. Local OpenAI-compatible chat subset is explicitly versioned; never claim universal API parity. API specs live in `contracts/openapi.yaml`, `contracts/events.schema.json`, `contracts/config.schema.json`.
