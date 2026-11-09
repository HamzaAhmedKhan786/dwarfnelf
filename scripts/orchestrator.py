#!/usr/bin/env python3
"""Budget-aware local orchestration baseline.

This module plans; it does not pretend to be a language model. A production
model adapter can consume the typed plan without changing safety boundaries.
"""
from dataclasses import dataclass, asdict
import argparse, json, re

@dataclass
class ResourceBudget:
    ram_gb: float = 4.0; vram_gb: float = 0.0; cpu_percent: int = 100; latency_ms: int = 5000

@dataclass
class Plan:
    domain: str; model_role: str; retrieval: str; agent_mode: str; evidence: str; abstain_if_unsupported: bool; reasons: list[str]

class Orchestrator:
    def plan(self, query: str, budget: ResourceBudget, evidence: str = "primary") -> Plan:
        text = query.lower(); reasons = []
        domain = "code" if re.search(r"\b(code|python|rust|bug|function|api)\b", text) else "general"
        if re.search(r"\b(legal|law|contract|regulation)\b", text): domain = "legal"
        if re.search(r"\b(medical|clinical|symptom|drug)\b", text): domain = "medical"
        retrieval = "hybrid" if budget.ram_gb >= 2 else "lexical"
        model_role = "dense_baseline" if budget.ram_gb >= 4 else "small_router"
        agent_mode = "react_bounded" if re.search(r"\b(search|inspect|calculate|compare|retrieve)\b", text) else "answer_only"
        reasons += [f"domain classified as {domain}", f"retrieval={retrieval} under {budget.ram_gb:g}GB RAM"]
        if domain in {"legal", "medical"}: evidence = "strict"; reasons.append("high-risk domain requires strict evidence")
        return Plan(domain, model_role, retrieval, agent_mode, evidence, True, reasons)

def main():
    p = argparse.ArgumentParser(); p.add_argument("query"); p.add_argument("--ram-gb", type=float, default=4); p.add_argument("--json", action="store_true")
    a = p.parse_args(); result = asdict(Orchestrator().plan(a.query, ResourceBudget(ram_gb=a.ram_gb)))
    print(json.dumps(result, indent=2) if a.json else result)
if __name__ == "__main__": main()
