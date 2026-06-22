# Gate 2 Outside Runner Card

Use this card for the unaided Gate 2 run. It is intentionally shorter than the
full proof template.

## Before You Start

Use a machine with:

- Docker Desktop or another local Docker daemon
- Docker Compose v2
- `git`, `curl`, `python3` 3.9 or newer
- `ffmpeg` or `ffprobe`
- `shasum` or `sha256sum`
- local ports `8080`, `4317`, and `3000` free

Run from an empty parent directory that does not already contain `beater/`.
Do not set Beater, Docker Compose, or alternate port environment variables.

## Timed Command

Run this exact command from Bash, zsh, Git Bash, or WSL2:

```bash
bash -o pipefail -lc 'curl -fsSL https://raw.githubusercontent.com/jadenfix/beater/main/scripts/gate2-outside-local-preflight.sh | bash && t="$(date +%s)" && git clone https://github.com/jadenfix/beater.git && cd beater && BEATER_GATE2_CLONE_STARTED_EPOCH="$t" scripts/gate2-outside-run.sh'
```

The timer includes clone and image-pull time.

## Timed Browser Step

When the terminal prints `Open this quickstart trace-list URL first:`, open that
URL in a normal browser immediately. Do not wait for the script to finish.

Click the quickstart trace, then click the `llm.call` span. Confirm that the
prompt, completion, model, token breakdown, cost, and latency are visible.
Only then press Enter in the terminal. The terminal checkpoint must happen
before the 5-minute clone-to-click SLO expires.

## Post-SLO Evidence

After the timed click is recorded, leave the command running. It will run the
browser proof, generate the all-kind waterfall trace, record the browser video,
and save `docs/demos/gate2-outside-compose.log`.

Open the printed all-kind dashboard URL and confirm the waterfall shows:

```text
run -> turn -> step -> tool -> MCP
```

## Proof Handoff

After the command exits, use the printed
`scripts/generate-gate2-outside-proof.py --print-command` output. Replace every `...` field with real runner values, then run it.

Commit the evidence before validation:

```bash
git add docs/demos/gate2-outside-person-proof.md \
  docs/demos/gate2-compose-stopwatch.md \
  docs/demos/gate2-compose-browser-demo.webm \
  docs/demos/gate2-compose-browser-demo.md \
  docs/demos/gate2-outside-compose.log
git commit -m "add gate2 outside proof"
scripts/validate-gate2-outside-proof.sh
```

Gate 2 only closes if this proof comes from someone outside the project who
completed the run unaided using public repository instructions.
