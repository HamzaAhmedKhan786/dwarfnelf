# MAMA Eval — End-to-End Evaluation Plan
## Principle
The requested >=80% correctness is a **task-specific target**, not a guarantee. Predefine task rubric, dataset, sample size, baseline, confidence interval and pass/fail gate. Track >=80% retention of full-model score separately; never conflate with >=80% absolute accuracy.
## Datasets
MMLU-Pro subsets (reasoning), GSM8K (math), IFEval (instructions), MBPP/HumanEval (sandboxed coding tests), multilingual and legal/medical/agriculture licensed domain QA, synthetic and human-annotated query-routing sets, BEIR-style retrieval corpora, RAGBench-style grounding tests where licensing allows, multi-hop QA, adversarial injection, long-context and tool-use benchmarks. Maintain train/validation/test splits and prevent contamination. Never auto-download restricted datasets.
## Test matrix
Hardware: CPU 8/16/32 GB; GPU 8/16/24 GB. Run baseline llama.cpp, native MoE, MAMA without RAG, MAMA RAG modes, MAMA conditional planner, and agent workflows. Vary cold/warm caches, context lengths, domain packs, memory budgets and concurrency. Include unsupported/infeasible configurations with expected graceful rejection.
## Metrics
Classification macro-F1, routing regret, Brier/ECE, answer EM/F1 and expert human rubric, retrieval Recall@k/nDCG, citation precision and faithfulness, abstention coverage, agent success and unsafe action rate, p50/p95 TTFT, tokens/s, end-to-end latency, peak RSS/VRAM, disk IOPS, joules/token when available, cache hit rate and crash/recovery rate.
## Quality gates
A release must pass domain-specific acceptance thresholds (target 80% where meaningful), no severe privacy/security regressions, deterministic resource admission, no unauthorized agent actions, evidence citation fidelity and reproducible benchmark reports. Statistical significance and 95% confidence intervals required for claimed improvements. Performance claims compare identical model/precision/workload/hardware and disclose failure cases.
## Reproducibility
Pin model hash, datasets/license, software version, driver, CPU ISA, quantization, seed, prompts, sampling, environment and warm/cold protocol. Export JSONL results and HTML report; redacted by default.
