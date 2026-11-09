# MAMA Knowledge — Full RAG Design and Selection Rules
## Domain packs
Science (open papers, methods); CS (versioned official docs/code); medical (guidelines and open-access publications, dates/authority); agriculture (regional extension, crop/climate); law (country, court, jurisdiction, enactment/effective/repeal dates); custom user corpora. Every pack manifest: identifier, source URLs, license, redistribution permissions, language, region, revision, checksum, schema, sensitivity, ingestion recipe, benchmark set. Public access is not blanket redistribution permission.
## Retrieval decision matrix
| Method | Select when | Risks / gate |
|---|---|---|
| SQLite FTS5 / BM25-like lexical | Exact statute numbers, API names, citations, rare terms | Synonym misses; test exact-match recall |
| Dense vector | Paraphrases and conceptual queries | Embedding mismatch; evaluate recall by domain/language |
| Hybrid (RRF or calibrated fusion) | Default for mixed natural-language + exact terms | Tune weights only on validation set |
| Metadata filtering | Jurisdiction, version, date, language, source authority | Missing metadata may drop valid evidence |
| Parent-child / hierarchical | Long sections, legal clauses, scientific papers | Parent context can dilute relevant passage |
| Contextual chunking | Tables, definitions, section-dependent references | Index bloat; evaluate fidelity |
| Multi-query / rewrite | Ambiguous vocabulary, multiple paraphrases | Extra latency and query drift; cap branches |
| Reranking (cross-encoder) | Initial candidate pool noisy | Compute overhead; hardware-adaptive enablement |
| Context compression | Small context windows | Loss of qualifiers; verify against originals |
| Graph-assisted | Explicit entities and relations | Graph build/maintenance cost; not universal default |
| Temporal retrieval | Regulations/guidelines change over time | Must filter effective date and distinguish historical law |
| Multimodal retrieval | Scanned figures, diagrams, audio/video | Requires bundled OCR/vision/audio models; provenance |
| Agentic iterative retrieval | Multi-hop research requiring multiple searches | Strict step/time/tool budgets and traceability |
| SQL/structured retrieval | Tables, numeric datasets, precise filters | Query sandbox, schema validation, no arbitrary writes |
## Routing algorithm
1. Classify intent, domain, jurisdiction, temporal scope, modality and risk.
2. Enforce ACL, provenance and date filters before ranking.
3. Run lexical+dense hybrid by default when models fit budgets; lexical-only fallback on 8 GB RAM.
4. Add parent expansion for context-sensitive documents; rerank if expected gain exceeds latency budget.
5. Use graph/SQL/agentic methods only for structurally appropriate multi-hop questions.
6. Deduplicate and diversify; preserve section offsets; check authority, recency, contradictions and injection.
7. Build grounded prompt with token budget, quote-level source IDs, explicit abstention.
8. Verify cited spans; return answer, citations, uncertainty, retrieval trace and conflicts.
## Evidence modes
Strict: no unsupported factual assertions; abstain when insufficient. Primary: retrieved evidence leads, model interpretation marked. Supporting: model may answer with clear separation of retrieved and unsourced knowledge. For legal/medical, defaults favor authoritative dated evidence and warnings about limits.
## Evaluation
Retrieval Recall@5/10, Precision@k, MRR, nDCG, answer exact match/F1 where appropriate, citation precision/recall, faithfulness, answer relevance, hallucination/abstention, temporal/jurisdiction accuracy, latency and memory. Benchmark every domain, language, evidence mode and retrieval route; include adversarial documents.
## Local packaging
Bundled SQLite and vector library; ONNX embedding/reranker binaries shipped. Embedding model weights may be bundled in larger installers or separately imported/downloaded with consent. User imports files by path; incremental updates; encrypted corpus option; secure deletion semantics documented.
