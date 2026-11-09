# MAMA Decisions — Structured Routing and Jev

The current offline decision surface evaluates advisory CPU admission from a typed estimate, the saved budget and a live RAM snapshot. General model routing and confidence calibration remain planned.

Jev/System One is an external TypeSafe AI technology discussed in https://typesafe.ai/blog/introducing-system-one-models-and-jev. Do not assume self-hostable weights, offline availability, or redistribution rights. Jev provider is optional and user-enabled only; core product works offline with bundled local classifiers or constrained local model decisions.
## Typed decision schema
`DecisionRequest {request_id, state_summary, candidate_actions, constraints, evidence_refs, schema_version}` → `DecisionResult {selected_action, probabilities?, calibrated_confidence?, abstain, reasons, provider_id, latency_ms}`. Probability claims require validation and calibration. A deterministic policy validator enforces memory, security and tool constraints after model suggestions.
## Decision domains
Query/domain classification; retrieval policy; model/backend choice; memory profile; agent tool permissions; verification escalation; fallback/abstention. Never treat probabilistic confidence as proof of medical/legal accuracy.
## Provider requirements
Local ONNX classifier, local LLM JSON-schema adapter, optional remote Jev adapter with opt-in, timeouts, redaction and explicit privacy warning. Compare providers using classification F1, expected calibration error, Brier score, abstention risk/coverage, latency and RAM.
