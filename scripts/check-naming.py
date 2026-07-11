#!/usr/bin/env python3
"""Naming lint: fail if any legacy project name leaks back into the tree.

The project was renamed from its legacy codename to "Palette". This guard keeps
the rename from regressing: it denies the case-insensitive substrings for the
old product name and its sibling music-pun codename across every tracked file.

Patterns are assembled from fragments so this file does not match itself, and
the file is skipped explicitly as a second belt-and-braces measure. Binary
blobs are skipped. Add a narrowly-scoped entry to ``EXCEPTIONS`` only for an
unavoidable historical fixture; the goal is zero.
"""

from __future__ import annotations

import subprocess
import sys

# Assembled from fragments so the literal forbidden strings never appear here.
FORBIDDEN = ("beat" + "er", "beat" + "box")

# Explicit, narrowly-scoped exceptions (relative repo paths). Aim for zero.
EXCEPTIONS: set[str] = set()

# This linter itself is skipped (it names the forbidden fragments by design).
SELF = "scripts/check-naming.py"


def tracked_files() -> list[str]:
    out = subprocess.run(
        ["git", "ls-files", "-z"],
        check=True,
        capture_output=True,
        text=True,
    ).stdout
    return [p for p in out.split("\0") if p]


def main() -> int:
    violations: list[str] = []
    for path in tracked_files():
        if path == SELF or path in EXCEPTIONS:
            continue
        try:
            with open(path, "rb") as fh:
                raw = fh.read()
        except OSError:
            continue
        if b"\0" in raw:  # binary blob
            continue
        text = raw.decode("utf-8", errors="ignore").lower()
        hits = [tok for tok in FORBIDDEN if tok in text]
        if hits:
            for lineno, line in enumerate(text.splitlines(), start=1):
                if any(tok in line for tok in FORBIDDEN):
                    violations.append(f"{path}:{lineno}: {line.strip()[:160]}")

    if violations:
        sys.stderr.write(
            "check-naming: legacy name(s) found in tracked files "
            f"({', '.join(FORBIDDEN)}):\n"
        )
        for v in violations:
            sys.stderr.write(f"  {v}\n")
        sys.stderr.write(f"\n{len(violations)} violation(s).\n")
        return 1

    print("check-naming: OK (no legacy names in tracked files)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
