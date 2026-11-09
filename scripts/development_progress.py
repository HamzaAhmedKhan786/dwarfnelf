"""Print verified completion from the repository's weighted criteria."""

import json
from collections import defaultdict
from pathlib import Path


def main():
    path = Path(__file__).resolve().parents[1] / "docs" / "development-progress.json"
    criteria = json.loads(path.read_text(encoding="utf-8"))["criteria"]
    streams = defaultdict(lambda: [0, 0])
    mvp_earned = mvp_weight = 0
    for item in criteria:
        weight = item["weight"]
        earned = weight if item["status"] == "verified" else 0
        if earned and not item.get("evidence"):
            raise ValueError(f"{item['id']} is missing evidence")
        totals = streams[item["stream"]]
        totals[0] += earned
        totals[1] += weight
        if item["mvp"]:
            mvp_earned += earned
            mvp_weight += weight

    total_earned = sum(earned for earned, _ in streams.values())
    total_weight = sum(weight for _, weight in streams.values())
    if total_weight != 100 or not mvp_weight:
        raise ValueError("Expected 100 total points and a nonempty MVP scope")
    for name, (earned, weight) in streams.items():
        print(f"{name}: {earned}/{weight} ({earned / weight:.1%})")
    print(f"Overall: {total_earned}/{total_weight} ({total_earned / total_weight:.1%})")
    print(f"MVP: {mvp_earned}/{mvp_weight} ({mvp_earned / mvp_weight:.1%})")


if __name__ == "__main__":
    main()
