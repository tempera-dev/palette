#!/usr/bin/env python3
"""Fail closed on Rust advisories, with one exact reachability exception."""

from __future__ import annotations

import hashlib
import json
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ALLOWED_ADVISORY = "RUSTSEC-2026-0253"
ALLOWED_LRU_VERSION = "0.16.4"
ALLOWED_LRU_CHECKSUM = "7f66e8d5d03f609abc3a39e6f08e4164ebf1447a732906d39eb9b99b7919ef39"
ALLOWED_TANTIVY_VERSION = "0.26.1"
ALLOWED_READER_SHA256 = "87f4e20e311f757757af74e4cc68770d87e293be9be1d751600738dcbc389c20"


def run_json(*args: str) -> dict[str, object]:
    completed = subprocess.run(
        args,
        cwd=ROOT,
        check=True,
        text=True,
        stdout=subprocess.PIPE,
    )
    return json.loads(completed.stdout)


def one_package(metadata: dict[str, object], name: str, version: str) -> dict[str, object]:
    packages = metadata.get("packages")
    if not isinstance(packages, list):
        raise SystemExit("cargo metadata did not contain a package list")
    matches = [
        package
        for package in packages
        if isinstance(package, dict)
        and package.get("name") == name
        and package.get("version") == version
    ]
    if len(matches) != 1:
        raise SystemExit(f"expected exactly one {name} {version}, found {len(matches)}")
    return matches[0]


def main() -> None:
    audit = run_json("cargo", "audit", "--format", "json")
    vulnerabilities = audit.get("vulnerabilities")
    if not isinstance(vulnerabilities, dict) or vulnerabilities.get("count") != 0:
        raise SystemExit("cargo audit reported a vulnerability")

    warnings = audit.get("warnings")
    if not isinstance(warnings, dict) or set(warnings) != {"unsound"}:
        raise SystemExit(f"unexpected cargo-audit warning classes: {warnings!r}")
    unsound = warnings.get("unsound")
    if not isinstance(unsound, list) or len(unsound) != 1 or not isinstance(unsound[0], dict):
        raise SystemExit(f"expected one exact unsoundness warning, found {unsound!r}")

    warning = unsound[0]
    advisory = warning.get("advisory")
    package = warning.get("package")
    if not isinstance(advisory, dict) or advisory.get("id") != ALLOWED_ADVISORY:
        raise SystemExit(f"unexpected advisory: {advisory!r}")
    if (
        not isinstance(package, dict)
        or package.get("name") != "lru"
        or package.get("version") != ALLOWED_LRU_VERSION
        or package.get("checksum") != ALLOWED_LRU_CHECKSUM
    ):
        raise SystemExit(f"the bounded lru package changed: {package!r}")

    metadata = run_json("cargo", "metadata", "--locked", "--format-version", "1")
    lru = one_package(metadata, "lru", ALLOWED_LRU_VERSION)
    tantivy = one_package(metadata, "tantivy", ALLOWED_TANTIVY_VERSION)
    resolve = metadata.get("resolve")
    if not isinstance(resolve, dict) or not isinstance(resolve.get("nodes"), list):
        raise SystemExit("cargo metadata did not contain a resolved dependency graph")

    parents: list[str] = []
    for node in resolve["nodes"]:
        if not isinstance(node, dict) or not isinstance(node.get("deps"), list):
            continue
        if any(isinstance(dep, dict) and dep.get("pkg") == lru.get("id") for dep in node["deps"]):
            node_id = node.get("id")
            if isinstance(node_id, str):
                parents.append(node_id)
    if parents != [tantivy.get("id")]:
        raise SystemExit(f"lru dependency parents changed: {parents!r}")

    manifest_path = tantivy.get("manifest_path")
    if not isinstance(manifest_path, str):
        raise SystemExit("tantivy manifest path missing from cargo metadata")
    tantivy_root = Path(manifest_path).resolve().parent
    reader = tantivy_root / "src" / "store" / "reader.rs"
    reader_bytes = reader.read_bytes()
    reader_digest = hashlib.sha256(reader_bytes).hexdigest()
    if reader_digest != ALLOWED_READER_SHA256:
        raise SystemExit(
            "tantivy's audited LruCache call surface changed; "
            f"expected {ALLOWED_READER_SHA256}, got {reader_digest}"
        )

    lru_users = sorted(
        path.relative_to(tantivy_root).as_posix()
        for path in (tantivy_root / "src").rglob("*.rs")
        if "LruCache" in path.read_text(encoding="utf-8")
    )
    if lru_users != ["src/store/reader.rs"]:
        raise SystemExit(f"tantivy's LruCache use sites changed: {lru_users!r}")

    reader_text = reader_bytes.decode("utf-8")
    block_cache = reader_text.split("impl BlockCache {", 1)[1].split("#[derive(Debug, Default)]", 1)[0]
    if "LruCache::pop" in block_cache or ".pop(" in block_cache:
        raise SystemExit("the affected LruCache::pop path became reachable")

    print(
        "PASS: zero Rust vulnerabilities; the sole warning "
        f"{ALLOWED_ADVISORY} is confined to tantivy {ALLOWED_TANTIVY_VERSION}, "
        "whose exact audited BlockCache source never calls LruCache::pop"
    )


if __name__ == "__main__":
    main()
