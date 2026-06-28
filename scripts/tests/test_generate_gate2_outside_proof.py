#!/usr/bin/env python3
from __future__ import annotations

import os
import subprocess
import sys
import tempfile
from pathlib import Path


REPO = Path(__file__).resolve().parents[2]
SCRIPT = REPO / "scripts" / "generate-gate2-outside-proof.py"
DEMOS = REPO / "docs" / "demos"
COMMIT = "1234567890abcdef1234567890abcdef12345678"


def write_stopwatch_proof(path: Path) -> None:
    path.write_text(
        f"""# Gate 2 Compose Stopwatch Proof

- Timing start source: external-clone
- Clone started at: 2026-06-20T12:00:00Z
- Script started at: 2026-06-20T12:00:15Z
- Started: 2026-06-20T12:00:15Z
- Ended: 2026-06-20T12:02:00Z
- Time-to-first-trace: 45s
- Script-to-first-trace: 30s
- Time-to-quickstart-click: 72s
- Script-to-quickstart-click: 57s
- Quickstart click source: manual-outside-runner
- Manual quickstart confirmation: yes
- Manual confirmation source: browser-selected-llm-detail
- Manual confirmation code: AB743641
- Manual confirmation salt: `gate2-contract-test-salt`
- Total duration: 120s
- Script duration: 105s
- Git SHA: `{COMMIT}`
- Git branch: `main`
- Git origin: `https://github.com/jadenfix/beater.git`
- Git worktree clean: yes
- OS/arch: `Linux x86_64`
- Docker: `Docker version 29.2.0, build 0b9d198`
- Docker Compose: `Docker Compose version v5.0.2`
- Clean start: yes
- Outside-run wrapper: yes
- Compose logs artifact: `docs/demos/gate2-outside-compose.log`
- Terminal transcript artifact: `docs/demos/gate2-outside-terminal.log`
- Beater image reference: `ghcr.io/jadenfix/beater/beaterd:{COMMIT}`
- Dashboard image reference: `ghcr.io/jadenfix/beater/dashboard:{COMMIT}`
- Dashboard e2e image reference: `ghcr.io/jadenfix/beater/dashboard-e2e:{COMMIT}`
- OTEL Python image reference: `ghcr.io/jadenfix/beater/otel-python:{COMMIT}`
- Beater image digest: `ghcr.io/jadenfix/beater/beaterd@sha256:{'a' * 64}`
- Dashboard image digest: `ghcr.io/jadenfix/beater/dashboard@sha256:{'b' * 64}`
- Dashboard e2e image digest: `ghcr.io/jadenfix/beater/dashboard-e2e@sha256:{'c' * 64}`
- OTEL Python image digest: `ghcr.io/jadenfix/beater/otel-python@sha256:{'d' * 64}`
- Quickstart snippet: `examples/python/five_line_otel.py`
- API endpoint: `http://127.0.0.1:8080`
- Dashboard base: `http://127.0.0.1:3000`
- Quickstart release ID: `gate2-1234567890ab-1782000000-12345`
- Quickstart trace: `0123456789abcdef0123456789abcdef`
- Quickstart span: `0123456789abcdef`
- Quickstart dashboard: http://127.0.0.1:3000/?trace=0123456789abcdef0123456789abcdef
- All-kind nested trace: `fedcba9876543210fedcba9876543210`
- All-kind dashboard: http://127.0.0.1:3000/?trace=fedcba9876543210fedcba9876543210
- Browser recording: passed
- Browser recording artifact: `docs/demos/gate2-compose-browser-demo.webm`
- Browser recording notes: `docs/demos/gate2-compose-browser-demo.md`
- Browser recording SHA256: `{'1' * 64}`
- Redaction browser proof: passed
- Redaction trace: `aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa`
- Redaction span: `bbbbbbbbbbbbbbbb`
- Redaction dashboard: http://127.0.0.1:3000/?trace=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
- Redaction unmask reason: gate2-redaction-review

## Compose Images

```text
beater-stopwatch-beaterd-1 ghcr.io/jadenfix/beater/beaterd {COMMIT}
beater-stopwatch-dashboard-1 ghcr.io/jadenfix/beater/dashboard {COMMIT}
```

This is an outside-run stopwatch source artifact.
""",
        encoding="utf-8",
    )


def relative_to_repo(path: Path) -> str:
    return path.relative_to(REPO).as_posix()


def run_generator(
    stopwatch_path: Path,
    output_path: Path,
    terminal_transcript: str,
    compose_logs: str,
    *,
    runner_name: str | None = "Jane Outside Runner",
    attest: bool = True,
) -> subprocess.CompletedProcess[str]:
    command = [
        sys.executable,
        str(SCRIPT),
        "--stopwatch-proof",
        str(stopwatch_path),
        "--output",
        str(output_path),
    ]
    if runner_name is not None:
        command.extend(["--runner-name", runner_name])
    command.extend(
        [
            "--relationship",
            "external evaluator; no Beater project role",
            "--prior-exposure",
            "none",
            "--machine-os",
            "Ubuntu 24.04 x86_64",
            "--browser",
            "Chrome stable",
            "--network-notes",
            "home network; no VPN",
            "--llm-observation",
            (
                "clicked llm.call and saw prompt, completion, model, token breakdown, "
                "cost, latency, and confirmation code"
            ),
            "--waterfall-observation",
            "opened all-kind trace and saw run -> turn -> step -> tool -> MCP nesting",
            "--terminal-output-excerpt",
            "Gate 2 compose stopwatch passed; Browser recording: passed",
            "--terminal-transcript-saved",
            terminal_transcript,
            "--compose-logs-saved",
            compose_logs,
            "--preflight-status",
            "passed",
            "--failure-notes",
            "none",
            "--runner-notes",
            "No extra runner notes.",
            "--no-validate",
        ]
    )
    if attest:
        command.append("--attest-outside-run")
    env = dict(os.environ)
    env["PYTHONDONTWRITEBYTECODE"] = "1"
    return subprocess.run(
        command,
        cwd=REPO,
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )


def with_inputs(action) -> None:
    with tempfile.TemporaryDirectory() as temp:
        temp_root = Path(temp)
        stopwatch_path = temp_root / "gate2-compose-stopwatch.md"
        output_path = temp_root / "gate2-outside-person-proof.md"
        write_stopwatch_proof(stopwatch_path)
        with tempfile.TemporaryDirectory(
            dir=DEMOS, prefix=".gate2-proof-generator-test-"
        ) as artifacts:
            artifacts_root = Path(artifacts)
            transcript = artifacts_root / "terminal.log"
            compose_logs = artifacts_root / "compose.log"
            transcript.write_text("Manual outside-run checkpoint passed\n", encoding="utf-8")
            compose_logs.write_text("compose logs saved\n", encoding="utf-8")
            action(
                stopwatch_path,
                output_path,
                relative_to_repo(transcript),
                relative_to_repo(compose_logs),
            )


def test_generates_completed_outside_person_proof() -> None:
    def run(
        stopwatch_path: Path,
        output_path: Path,
        terminal_transcript: str,
        compose_logs: str,
    ) -> None:
        result = run_generator(
            stopwatch_path,
            output_path,
            terminal_transcript,
            compose_logs,
        )

        assert result.returncode == 0, result.stderr
        assert "Wrote Gate 2 outside-person proof:" in result.stdout
        proof = output_path.read_text(encoding="utf-8")
        assert "Status: completed." in proof
        assert "- Outside-run wrapper: yes" in proof
        assert "- Timing start source: external-clone" in proof
        assert "- Quickstart click source: manual-outside-runner" in proof
        assert "- Manual quickstart confirmation: yes" in proof
        assert "- Manual confirmation source: browser-selected-llm-detail" in proof
        assert "- Manual confirmation code: AB743641" in proof
        assert "- Outside-run terminal transcript: " + terminal_transcript in proof
        assert "- `docker compose` logs saved: " + compose_logs in proof
        assert "I attest that I am not a Beater project maintainer" in proof
        assert "Runner llm.call observation: clicked llm.call and saw prompt" in proof
        assert "Runner waterfall observation: opened all-kind trace and saw run" in proof

    with_inputs(run)


def test_missing_runner_name_fails_without_writing_proof() -> None:
    def run(
        stopwatch_path: Path,
        output_path: Path,
        terminal_transcript: str,
        compose_logs: str,
    ) -> None:
        result = run_generator(
            stopwatch_path,
            output_path,
            terminal_transcript,
            compose_logs,
            runner_name=None,
        )

        assert result.returncode != 0
        assert "--runner-name must be provided with a concrete value" in result.stderr
        assert not output_path.exists()

    with_inputs(run)


if __name__ == "__main__":
    for test in [
        test_generates_completed_outside_person_proof,
        test_missing_runner_name_fails_without_writing_proof,
    ]:
        test()
    print("generate-gate2-outside-proof tests passed")
