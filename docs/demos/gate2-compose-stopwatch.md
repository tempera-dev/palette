# Gate 2 Compose Stopwatch Proof

- Started: 2026-06-20T07:05:29Z
- Ended: 2026-06-20T07:06:01Z
- Duration: 32s
- Limit: 300s
- Startup mode: prebuilt-image
- Compose project: beater-stopwatch-prebuilt
- Snippet: `examples/python/five_line_otel.py`
- OTLP endpoint: `http://127.0.0.1:14317`
- Trace: `27d7a29e0d2172cd88927fc29481aa5b`
- Dashboard: http://127.0.0.1:13000/?tenant=demo&project=demo&environment=local&trace=27d7a29e0d2172cd88927fc29481aa5b

This is an automated local stopwatch proof. The mandate still requires an
outside-person run to fully close Gate 2.

Regenerate:

```bash
BEATER_GATE2_WRITE_PROOF=1 scripts/gate2-compose-stopwatch.sh
```
