#!/usr/bin/env python3
"""Bounded ReAct-style agent runtime with fail-closed tool policy."""
from dataclasses import dataclass, field
from typing import Callable, Any

@dataclass
class Tool:
    name: str; description: str; handler: Callable[[str], str]; read_only: bool = True

@dataclass
class AgentPolicy:
    max_steps: int = 6; allowed_tools: set[str] = field(default_factory=set); require_approval_for_writes: bool = True

class ReActAgent:
    def __init__(self, tools: list[Tool], policy: AgentPolicy | None = None):
        self.tools = {tool.name: tool for tool in tools}; self.policy = policy or AgentPolicy()

    def run(self, task: str, decide: Callable[[str, list[dict]], dict[str, Any]], approve: Callable[[Tool, str], bool] | None = None) -> dict:
        trace, observation = [], ""
        for step in range(1, self.policy.max_steps + 1):
            decision = decide(task, trace + ([{"type": "observation", "content": observation}] if observation else []))
            if decision.get("action") == "final": return {"status": "completed", "answer": decision.get("answer", ""), "trace": trace}
            name, argument = decision.get("tool", ""), decision.get("input", "")
            tool = self.tools.get(name)
            if not tool or name not in self.policy.allowed_tools:
                return {"status": "blocked", "reason": f"tool not allowed: {name}", "trace": trace}
            if not tool.read_only and self.policy.require_approval_for_writes and not (approve and approve(tool, argument)):
                return {"status": "approval_required", "tool": name, "trace": trace}
            observation = tool.handler(argument)
            trace.append({"step": step, "type": "action", "tool": name, "input": argument, "observation": observation})
        return {"status": "budget_exhausted", "trace": trace}
