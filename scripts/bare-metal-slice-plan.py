#!/usr/bin/env python3
"""Create deterministic lane slices for subagent-style execution."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Partition bare-metal targets across lane workers")
    parser.add_argument(
        "--policy",
        default="docs/engineering/bare-metal-lane-policy.json",
        help="Policy path",
    )
    parser.add_argument(
        "--shards",
        type=int,
        default=1,
        help="Total number of shards (>=1)",
    )
    parser.add_argument(
        "--shard-id",
        type=int,
        default=0,
        help="Current shard id (0-based)",
    )
    return parser.parse_args()


def load_targets(path: Path) -> list[str]:
    data = json.loads(path.read_text(encoding="utf-8"))
    return [str(item.get("name", "")).strip() for item in data.get("targets", []) if str(item.get("name", "")).strip()]


def main() -> None:
    args = parse_args()
    if args.shards < 1:
        raise SystemExit("--shards must be >= 1")
    if not (0 <= args.shard_id < args.shards):
        raise SystemExit("--shard-id must be in [0, shards-1]")

    policy_path = Path(args.policy)
    if not policy_path.exists():
        raise SystemExit(f"policy not found: {policy_path}")

    targets = load_targets(policy_path)
    if not targets:
        raise SystemExit("no targets found")

    selected = [target for idx, target in enumerate(targets) if idx % args.shards == args.shard_id]

    print(f"# shard: {args.shard_id}/{args.shards}")
    for target in selected:
        print(target)


if __name__ == "__main__":
    main()

