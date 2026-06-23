import re


REMOTE_URL = "https://github.com/jadenfix/beater.git"
REMOTE_MAIN_REF = "refs/heads/main"
RAW_PREFLIGHT_PATH = "scripts/gate2-outside-local-preflight.sh"
RAW_PREFLIGHT_URL_PREFIX = "https://raw.githubusercontent.com/jadenfix/beater"
PUBLIC_SHA_RESOLUTION_COMMAND = (
    f'sha_line="$(git ls-remote --exit-code {REMOTE_URL} {REMOTE_MAIN_REF})" && '
    'sha="${sha_line%%[[:space:]]*}" && test -n "$sha"'
)
RAW_PUBLIC_PREFLIGHT_COMMAND = (
    'preflight="$(mktemp "${TMPDIR:-/tmp}/beater-gate2-preflight.XXXXXX")" && '
    f'curl -fsSL "{RAW_PREFLIGHT_URL_PREFIX}/$sha/{RAW_PREFLIGHT_PATH}" '
    '-o "$preflight" && bash "$preflight"'
)
CLONE_VERIFICATION_COMMAND = (
    f"git clone {REMOTE_URL} && cd beater && "
    'test "$(git rev-parse HEAD)" = "$sha" && '
    'BEATER_GATE2_CLONE_STARTED_EPOCH="$t" scripts/gate2-outside-run.sh'
)
OUTSIDE_RUNNER_COMMAND = (
    f"bash -o pipefail -lc '{PUBLIC_SHA_RESOLUTION_COMMAND} && "
    f'{RAW_PUBLIC_PREFLIGHT_COMMAND} && t="$(date +%s)" && '
    f"{CLONE_VERIFICATION_COMMAND}'"
)
OUTSIDE_RUN_ATTESTATION = (
    "I attest that I am not a Beater project maintainer, I received no "
    "step-by-step help beyond public repository instructions, I used a fresh "
    "clone, and I completed the Gate 2 flow unaided."
)
DIAGNOSTIC_ATTESTATION = (
    "Diagnostic maintainer full-run used a browser click to read the manual confirmation code; "
    "this is not outside-person evidence and cannot close Gate 2."
)
IMMUTABLE_LOG_URL = re.compile(
    r"https://github\.com/jadenfix/beater/actions/runs/[0-9]+(?:/job/[0-9]+)?"
)


def raw_public_preflight_command_for_sha(expected_commit):
    return (
        'preflight="$(mktemp "${TMPDIR:-/tmp}/beater-gate2-preflight.XXXXXX")" && '
        f'curl -fsSL "{RAW_PREFLIGHT_URL_PREFIX}/{expected_commit}/{RAW_PREFLIGHT_PATH}" '
        '-o "$preflight" && bash "$preflight"'
    )


def is_immutable_log_url(value):
    return bool(IMMUTABLE_LOG_URL.fullmatch(value))


UNRESOLVED_REQUIRED_VALUES = {
    "...",
    "\u2026",
    "unknown",
    "not requested",
    "not reported",
    "tbd",
    "todo",
}
EMBEDDED_PLACEHOLDER = re.compile(
    "(^|[\\s:;,/()_-])(\\.\\.\\.|\\u2026|tbd|todo)($|[\\s:;,/()_-])", re.I
)

LLM_OBSERVATION_FRAGMENTS = [
    "llm.call",
    "prompt",
    "completion",
    "model",
    "token breakdown",
    "cost",
    "latency",
    "confirmation code",
]
WATERFALL_OBSERVATION_FRAGMENTS = ["run", "turn", "step", "tool", "MCP"]

NEGATED_OBSERVATION = re.compile(
    r"\b(?:did\s+not|didn't|could\s+not|couldn't|cannot|can't|failed\s+to\s+see|"
    r"not\s+visible|not\s+shown|not\s+showing|missing|without)\b"
)
VISIBLE_OBSERVATION = re.compile(
    r"\b(?:saw|seen|visible|read|confirmed|verified|opened|clicked|showed|displayed|inspected)\b"
)


def clean_value(value):
    return value.strip().strip("`").strip()


def markdown_field_values(source_text, name):
    return [
        clean_value(match)
        for match in re.findall(
            r"^- " + re.escape(name) + r":[ \t]*(.*)$",
            source_text,
            re.MULTILINE,
        )
    ]


def contains_placeholder_fragment(value):
    return bool(EMBEDDED_PLACEHOLDER.search(value))


def is_unresolved_marker(value):
    cleaned = clean_value(value)
    return not cleaned or cleaned.lower() in UNRESOLVED_REQUIRED_VALUES


def is_unresolved_argument(value, *, allow_none=False):
    cleaned = clean_value(value)
    return (
        is_unresolved_marker(cleaned)
        or (cleaned.lower() == "none" and not allow_none)
        or contains_placeholder_fragment(cleaned)
    )


def observation_errors(field_name, value, required_fragments):
    normalized = value.lower()
    errors = []
    if NEGATED_OBSERVATION.search(normalized):
        errors.append(f"{field_name} must be a positive observation, not negated evidence")
    if not VISIBLE_OBSERVATION.search(normalized):
        errors.append(f"{field_name} must describe a positive visible observation")
    missing = [
        fragment for fragment in required_fragments if fragment.lower() not in normalized
    ]
    if missing:
        errors.append(f"{field_name} must mention: " + ", ".join(missing))
    return errors
