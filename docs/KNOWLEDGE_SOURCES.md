# Candidate knowledge sources and models

Import is opt-in. Review source terms and item-level licenses before fetching or indexing. Pin a snapshot or revision, record the source URL, checksum, attribution, language, and publication date, and keep downloaded corpora and weights outside Git. The current Rust endpoint indexes small text submissions. Bulk ingestion and source-specific adapters remain planned.

| Domain | Public source | Intended use |
|---|---|---|
| General knowledge | [Wikimedia Dumps](https://dumps.wikimedia.org/) | Offline reference packs with attribution and snapshot metadata |
| Scientific research | [arXiv bulk data](https://info.arxiv.org/help/bulk_data.html) | Paper and abstract retrieval with version and rights metadata |
| Medical research | [PubMed data](https://pubmed.ncbi.nlm.nih.gov/download/) | Bibliographic and abstract retrieval keyed by PMID |
| Biomedical full text | [PMC Open Access Subset](https://pmc.ncbi.nlm.nih.gov/tools/openftlist/) | License-filtered full text keyed by PMCID |
| German law | [Gesetze im Internet](https://www.gesetze-im-internet.de/) | German federal legislation with jurisdiction, version and effective dates where verified |
| European law | [EUR-Lex](https://eur-lex.europa.eu/) | EU legal documents with language, identifier and consolidation metadata |
| Computer science | [Python documentation](https://docs.python.org/3/download.html) | Version-specific developer knowledge |
| Agriculture | [FAOSTAT](https://www.fao.org/faostat/en/#data) | Statistical series with geography, dates and units |
| General ML | [Hugging Face Datasets](https://huggingface.co/datasets) | Ingestion experiments with per-dataset license and revision review |

## Model candidates

- CPU generation starts with a small quantization of [Qwen2.5-1.5B-Instruct-GGUF](https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF) via [llama.cpp](https://github.com/ggml-org/llama.cpp). Record the exact GGUF filename, SHA-256 and worker revision before testing. Evaluate 3B, 7B and 8B variants and compatible MoE checkpoints separately.
- Use [all-MiniLM-L6-v2](https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2) for English embedding experiments and evaluate [multilingual-e5-small](https://huggingface.co/intfloat/multilingual-e5-small) separately on German and multilingual corpora. Use the model card's query and passage formatting and keep index model revisions separate.
- [ONNX Runtime](https://onnxruntime.ai/docs/) is the target for local embeddings, reranking and lightweight classifiers. Check export compatibility, tokenizer and pooling behavior, and parity before enabling a model.
