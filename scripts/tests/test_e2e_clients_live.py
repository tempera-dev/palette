#!/usr/bin/env python3
from __future__ import annotations

import os
import shutil
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts" / "e2e-clients-live.sh"


def write_executable(path: Path, body: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(body)
    path.chmod(0o755)


def seed_repo(root: Path, *, python_runner: str | None = None) -> Path:
    script = root / "scripts" / "e2e-clients-live.sh"
    script.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(SCRIPT, script)
    script.chmod(0o755)

    write_executable(
        root / "target" / "debug" / "beaterd",
        """#!/usr/bin/env bash
if [ -n "${BEATER_TEST_LOG:-}" ]; then
  printf 'beaterd:%s\\n' "$*" >> "$BEATER_TEST_LOG"
fi
exec /bin/sleep 600
""",
    )
    if python_runner is not None:
        write_executable(root / "sdks" / "conformance" / "python" / "run.sh", python_runner)
    return script


def seed_fake_bin(bin_dir: Path) -> None:
    write_executable(
        bin_dir / "cargo",
        """#!/usr/bin/env bash
if [ -n "${BEATER_TEST_LOG:-}" ]; then
  printf 'cargo:%s\\n' "$*" >> "$BEATER_TEST_LOG"
fi
exit 0
""",
    )
    write_executable(
        bin_dir / "curl",
        """#!/usr/bin/env bash
if [ -n "${BEATER_TEST_LOG:-}" ]; then
  printf 'curl:%s\\n' "$*" >> "$BEATER_TEST_LOG"
fi
printf 'ok'
""",
    )
    write_executable(
        bin_dir / "python3",
        """#!/usr/bin/env bash
if [ -n "${BEATER_TEST_LOG:-}" ]; then
  printf 'python3:%s\\n' "$*" >> "$BEATER_TEST_LOG"
fi
exit 0
""",
    )
    write_executable(
        bin_dir / "node",
        """#!/usr/bin/env bash
exit 1
""",
    )
    write_executable(
        bin_dir / "go",
        """#!/usr/bin/env bash
exit 1
""",
    )
    write_executable(
        bin_dir / "cmake",
        """#!/usr/bin/env bash
exit 1
""",
    )
    write_executable(
        bin_dir / "brew",
        """#!/usr/bin/env bash
exit 1
""",
    )
    write_executable(
        bin_dir / "pkg-config",
        """#!/usr/bin/env bash
exit 1
""",
    )
    write_executable(
        bin_dir / "mvn",
        """#!/usr/bin/env bash
exit 1
""",
    )


def run_conformance(
    *,
    python_runner: str | None = None,
    env: dict[str, str] | None = None,
) -> tuple[subprocess.CompletedProcess[str], str]:
    with tempfile.TemporaryDirectory() as temp:
        root = Path(temp)
        script = seed_repo(root, python_runner=python_runner)
        bin_dir = root / "bin"
        log = root / "calls.log"
        seed_fake_bin(bin_dir)

        merged_env = os.environ.copy()
        merged_env.update(
            {
                "PATH": f"{bin_dir}{os.pathsep}{merged_env['PATH']}",
                "BEATER_TEST_LOG": str(log),
            }
        )
        if env:
            merged_env.update(env)

        result = subprocess.run(
            ["bash", str(script)],
            cwd=root,
            env=merged_env,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        return result, log.read_text() if log.exists() else ""


def test_live_conformance_runs_present_python_program_and_skips_missing_programs() -> None:
    runner = """#!/usr/bin/env bash
{
  printf 'runner-base=%s tenant=%s project=%s\\n' \
    "$BEATER_BASE_URL" "$BEATER_TENANT" "$BEATER_PROJECT"
} >> "$BEATER_TEST_LOG"
"""

    result, log = run_conformance(python_runner=runner)

    assert result.returncode == 0, result.stderr
    assert "PASS: python" in result.stdout
    assert "SKIP typescript (no conformance program yet)" in result.stdout
    assert "SKIP: typescript rust go c java cpp" in result.stdout
    assert "FAIL: none" in result.stdout
    assert "cargo:build -q -p beaterd" in log
    assert "beaterd:--addr 127.0.0.1:18100" in log
    assert "curl:-fsS http://127.0.0.1:18100/health" in log
    assert "runner-base=http://127.0.0.1:18100 tenant=demo project=demo" in log


def test_live_conformance_fails_when_every_language_is_skipped() -> None:
    result, _log = run_conformance()

    assert result.returncode == 1
    assert "PASS: none" in result.stdout
    assert "ERROR: no language conformance ran (all skipped)" in result.stderr


def test_live_conformance_fails_when_required_language_does_not_pass() -> None:
    runner = """#!/usr/bin/env bash
exit 0
"""

    result, _log = run_conformance(
        python_runner=runner,
        env={"BEATER_CONFORMANCE_REQUIRE": "python,go"},
    )

    assert result.returncode == 1
    assert "PASS: python" in result.stdout
    assert "ERROR: required language 'go' did not PASS" in result.stderr


def main() -> None:
    for test in (
        test_live_conformance_runs_present_python_program_and_skips_missing_programs,
        test_live_conformance_fails_when_every_language_is_skipped,
        test_live_conformance_fails_when_required_language_does_not_pass,
    ):
        test()
    print("e2e-clients-live tests passed")


if __name__ == "__main__":
    main()
