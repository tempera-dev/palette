# Palette Rust SDK

Ergonomic, OpenTelemetry-native agent observability for [Palette](https://github.com/palette/palette).

This is the **Layer 2** (hand-written, ergonomic) SDK — a thin, idiomatic
wrapper over the OpenTelemetry API, mirroring the Python (`sdks/python`) and
TypeScript (`sdks/typescript`) SDKs. It exposes `init`, `observe`, `span`,
`set_input`/`set_output`, and a shared `semconv` module kept in lockstep with the
server normalizer in `crates/palette-otlp`.

## Quickstart (5 lines)

```rust
use palette::{span_kind, PaletteConfig};

palette::init(PaletteConfig::from_env());                       // OTLP/HTTP -> paletted
let out = palette::observe("handle", span_kind::AGENT_RUN, || {
    palette::set_input("user query");                         // input.value
    palette::set_output("done");                              // output.value
    "done"
});
palette::shutdown();                                           // flush before exit
```

All `PaletteConfig` fields fall back to `PALETTE_*` env vars
(`PALETTE_BASE_URL` default `http://127.0.0.1:8080`, `PALETTE_TENANT_ID`/
`PALETTE_PROJECT_ID`/`PALETTE_ENVIRONMENT_ID` default `demo`/`demo`/`local`,
`PALETTE_API_KEY`, `PALETTE_SERVICE_NAME`, `PALETTE_RELEASE_ID`), so
`PaletteConfig::from_env()` works with no explicit args.

## API

- `init(config)` — wire up an OTLP/HTTP (protobuf) exporter pointed at
  `{base}/v1/otlp/{tenant}/{project}/{environment}/v1/traces` and install it as
  the global tracer.
- `observe(name, kind, f)` / `observe_async(name, kind, fut).await` — run a
  closure/future inside a Palette span, setting `openinference.span.kind`,
  `palette.seq`, and `agent.release_id`, then mark status OK.
- `span(name, kind)` — open a span as an RAII guard; call `set_input`/
  `set_output`/`set_attribute`/`ok`/`error` on it; the span ends on drop.
- `set_input(v)` / `set_output(v)` / `set_attribute(k, v)` — operate on the
  current span.
- `flush()` / `shutdown()` — flush pending spans before a short-lived program
  exits.
- `span_kind` / `attr` — the 11 span kinds and canonical attribute keys.

## Example

```sh
PALETTE_TENANT_ID=demo PALETTE_PROJECT_ID=demo PALETTE_ENVIRONMENT_ID=local \
    cargo run --example quickstart
```

Emits an `agent.run -> agent.plan -> llm.call` trace.
