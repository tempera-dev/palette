#!/usr/bin/env python3
"""Guard the live-replay RFC's architecture contract language."""

from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parent.parent
RFC = ROOT / "docs" / "engineering" / "rfc-live-replay.md"


REQUIRED = {
    "durable stream invariant": [
        "append-only, durable event log first",
        "persist event -> notify live subscribers -> client catches up from persisted log",
        "dropped notification",
        "backfill",
        "LiveReplayStore",
        "list_after",
        "beaterd` restart loses only in-memory fanout, not stream history",
    ],
    "sse resume contract": [
        "EventSource",
        "text/event-stream",
        "Last-Event-ID",
        "id: lr_trace_00000000042_abcd1234",
        "event: span.completed",
        "data: {\"event_id\"",
        "GET .../events?follow=true` emits SSE `id`, `event`, and `data`",
    ],
    "auth and privacy": [
        "tenant",
        "project",
        "same trace-read project scope as static trace reads",
        "re-check auth before initial backfill",
        "close or downgrade the stream if long-lived permissions become invalid",
        "never expose unmasked payloads without the existing audited unmask path",
        "redacted static trace reads and live events expose consistent payload visibility",
    ],
    "honest replay labels": [
        "provider, tool, memory, and clock cassettes",
        "deterministic_replay",
        "forked_replay",
        "simulation",
        "incomplete cassettes",
        "never deterministic",
        "Claiming deterministic replay when cassettes are missing or mismatched",
    ],
    "test coverage": [
        "Rust contract tests:",
        "API tests:",
        "Runtime smoke:",
        "Dashboard E2E:",
        "CLI E2E:",
        "reconnect resumes without duplicates",
        "reconnects do not lose or duplicate semantic events",
    ],
}


def main() -> int:
    text = RFC.read_text(encoding="utf-8")
    normalized_text = " ".join(text.split())
    failures = []

    for section, needles in REQUIRED.items():
        for needle in needles:
            normalized_needle = " ".join(needle.split())
            if normalized_needle not in normalized_text:
                failures.append(f"{section}: missing {needle!r}")

    if failures:
        print("Live replay RFC drift:", file=sys.stderr)
        for failure in failures:
            print(f"  - {failure}", file=sys.stderr)
        return 1

    print("Live replay RFC covers durable SSE, privacy, replay labels, and tests.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
