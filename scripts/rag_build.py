#!/usr/bin/env python3
"""Build a small, inspectable local RAG index.

The index is intentionally JSONL/JSON so it can be used before SQLite and an
ANN backend are introduced. Methods: lexical, tfidf, dense, and hybrid.
Dense retrieval uses sentence-transformers when installed and otherwise fails
with an actionable message; hybrid always remains available offline.
"""
from __future__ import annotations

import argparse, hashlib, json, math, re
from collections import Counter, defaultdict
from pathlib import Path

TEXT_EXTENSIONS = {".txt", ".md", ".rst", ".csv", ".json", ".yaml", ".yml", ".py", ".rs"}
TOKEN_RE = re.compile(r"[\w'-]+", re.UNICODE)

def tokens(text: str) -> list[str]:
    return [x.lower() for x in TOKEN_RE.findall(text)]

def chunk(text: str, size: int, overlap: int) -> list[str]:
    words = text.split()
    if size <= 0 or overlap < 0 or overlap >= size:
        raise ValueError("chunk_size must be positive and overlap must be smaller than chunk_size")
    out = []
    step = size - overlap
    for start in range(0, len(words), step):
        part = " ".join(words[start:start + size]).strip()
        if part:
            out.append(part)
        if start + size >= len(words):
            break
    return out

def read_documents(root: Path, size: int, overlap: int) -> list[dict]:
    records = []
    for path in sorted(p for p in root.rglob("*") if p.is_file() and p.suffix.lower() in TEXT_EXTENSIONS):
        raw = path.read_text(encoding="utf-8", errors="replace")
        digest = hashlib.sha256(raw.encode()).hexdigest()
        for number, text in enumerate(chunk(raw, size, overlap)):
            records.append({"id": f"{digest[:16]}:{number}", "source": str(path), "chunk": number, "text": text})
    return records

def lexical_scores(records: list[dict], query: str) -> list[float]:
    q = Counter(tokens(query)); scores = []
    for record in records:
        doc = Counter(tokens(record["text"]))
        scores.append(sum((1 + math.log(1 + doc[t])) * weight for t, weight in q.items() if t in doc))
    return scores

def tfidf_scores(records: list[dict], query: str) -> list[float]:
    q = Counter(tokens(query)); df = Counter()
    docs = [Counter(tokens(r["text"])) for r in records]
    for doc in docs:
        df.update(doc.keys())
    n = max(len(docs), 1)
    def weight(term: str, count: int) -> float:
        return (1 + math.log(count)) * math.log((n + 1) / (df[term] + 1))
    qv = {t: weight(t, c) for t, c in q.items()}
    return [sum(qv.get(t, 0) * weight(t, c) for t, c in doc.items()) for doc in docs]

def dense_scores(records: list[dict], query: str, model_name: str) -> list[float]:
    try:
        from sentence_transformers import SentenceTransformer
    except ImportError as exc:
        raise RuntimeError("dense retrieval requires sentence-transformers; use --method hybrid or lexical for offline baseline") from exc
    model = SentenceTransformer(model_name)
    vectors = model.encode([query] + [r["text"] for r in records], normalize_embeddings=True)
    return [float(vectors[0] @ vector) for vector in vectors[1:]]

def rank(records: list[dict], query: str, method: str, model: str) -> list[dict]:
    if not records: return []
    lexical = lexical_scores(records, query)
    if method == "lexical": scores = lexical
    elif method == "tfidf": scores = tfidf_scores(records, query)
    elif method == "dense": scores = dense_scores(records, query, model)
    else:
        semantic = dense_scores(records, query, model) if method == "hybrid-dense" else tfidf_scores(records, query)
        def normalize(xs):
            top = max(xs) if xs else 0
            return [x / top if top else 0 for x in xs]
        scores = [0.5 * a + 0.5 * b for a, b in zip(normalize(lexical), normalize(semantic))]
    return [dict(r, score=round(score, 6)) for score, r in sorted(zip(scores, records), reverse=True, key=lambda x: x[0])]

def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input", type=Path); parser.add_argument("output", type=Path)
    parser.add_argument("--chunk-size", type=int, default=350); parser.add_argument("--overlap", type=int, default=50)
    parser.add_argument("--method", choices=["lexical", "tfidf", "dense", "hybrid-dense"], default="tfidf")
    parser.add_argument("--query", help="also print top matches for a query")
    parser.add_argument("--model", default="sentence-transformers/all-MiniLM-L6-v2")
    args = parser.parse_args(); records = read_documents(args.input, args.chunk_size, args.overlap)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps({"version": 1, "method": args.method, "records": records}, indent=2), encoding="utf-8")
    print(f"indexed {len(records)} chunks from {args.input} -> {args.output}")
    if args.query:
        for row in rank(records, args.query, args.method, args.model)[:5]: print(json.dumps(row, ensure_ascii=False))

if __name__ == "__main__": main()
