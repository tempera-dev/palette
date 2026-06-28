#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
DOC = ROOT / "docs/apple-container.md"
CONTAINER_RUNTIME = ROOT / "scripts/container-runtime.sh"
BUILD_IMAGE = ROOT / "scripts/build-image.sh"
RUN_BEATERD = ROOT / "scripts/run-beaterd.sh"
CONTAINER_IMAGES_WORKFLOW = ROOT / ".github/workflows/container-images.yml"


REQUIRED = {
    DOC: [
        "BEATER_CONTAINER_RUNTIME=container",
        "scripts/container-runtime.sh",
        "scripts/build-image.sh beaterd:local",
        "scripts/run-beaterd.sh beaterd:local",
        "Docker publishes the port to `127.0.0.1:8080`",
        "Apple `container` gives each",
        "container its own IP",
        "Docker buildx, multi-platform",
        "Apple `container` is **not** exercised in CI",
        "Virtualization.framework",
        "self-hosted bare-metal Apple-silicon runner",
        "scripts/check-apple-container-docs.py",
    ],
    CONTAINER_RUNTIME: [
        "BEATER_CONTAINER_RUNTIME=docker|container",
        "command -v docker",
        "command -v container",
        "container system status",
        "container system start",
        "docker info",
        '"$CRT" build -t "$tag" -f Dockerfile "$@" .',
        '"$CRT" run -d --name "$name" -p "${port}:${port}" "$@" "$tag"',
        'echo "127.0.0.1:${port}"',
        'container inspect "$name"',
    ],
    BUILD_IMAGE: [
        "source scripts/container-runtime.sh",
        'TAG="${1:-beaterd:local}"',
        'crt_build "$TAG"',
    ],
    RUN_BEATERD: [
        "source scripts/container-runtime.sh",
        'TAG="${1:-beaterd:local}"',
        'NAME="beaterd-local"',
        "crt_stop",
        "crt_run",
        "crt_address",
        'curl -fsS "http://$addr/health"',
        "crt_logs",
    ],
    CONTAINER_IMAGES_WORKFLOW: [
        "docker/setup-buildx-action",
        "platform: linux/amd64",
        "platform: linux/arm64",
    ],
}


def fail(message: str) -> None:
    print(f"Apple container docs contract failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    for path, needles in REQUIRED.items():
        if not path.is_file():
            fail(f"missing file: {path.relative_to(ROOT)}")
        text = path.read_text()
        if not text.strip():
            fail(f"empty file: {path.relative_to(ROOT)}")
        for needle in needles:
            if needle not in text:
                fail(f"{path.relative_to(ROOT)} must mention {needle!r}")
    print("Apple container docs match the portable runtime scripts.")


if __name__ == "__main__":
    main()
