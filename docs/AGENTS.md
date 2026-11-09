# MAMA Agents — Execution, Monitoring, Security
## Runtime
Typed DAG workflows with planner, supervisor, tool broker, executor, verifier, human-approval gate and checkpoint store. Roles: research agent, knowledge-index agent, coding assistant, scientific literature agent, legal document analyst (non-lawyer), medical literature assistant (non-clinician), agriculture analyst, evaluation agent. These are capability templates, not independent unconstrained autonomous processes.
## Tool policy
Tools declare schemas, capabilities, side effects, network/FS permissions, timeouts, quotas and audit classification. Read-only retrieval may run automatically; write/delete/network/browser/shell actions require scoped approvals. MCP adapters run with allowlists and secret isolation; untrusted retrieved instructions never grant authority.
## Agent budgets
Max steps, wall-clock, token count, RAM/VRAM, concurrency, tool invocations, output size and optional cloud spending. Fail closed when exceeded; pause/retry idempotent steps; human-readable trace and cancellation.
## Monitoring
Per run: state, owner, start/end, decision trace, tool calls, retries, approval history, evidence IDs, tokens, cost, latency, peak resource use, failures and verification outcome. Dashboard supports pause/resume/cancel/replay with sensitive fields redacted.
## Evaluation
Task completion with executable validators, tool-call correctness, policy violations, prompt-injection resilience, human intervention rate, recovery success, hallucination/grounding and p95 completion time. Use offline synthetic and realistic permissioned tasks.
