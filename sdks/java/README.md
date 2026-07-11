# Palette Java SDK

Ergonomic, OpenTelemetry-native agent observability for Palette. This is the
**Layer 2** (hand-written) SDK: span helpers built on the OpenTelemetry Java SDK
that emit Palette-conformant spans over OTLP/HTTP.

## Onboarding order

Palette's default onboarding remains the zero-code OTLP path: point an existing
OpenTelemetry/OpenInference exporter at Palette with environment variables and no
Palette SDK code. Use this Java SDK when a service wants in-process helpers for
span boundaries, Palette semantic-convention constants, and short-lived process
flush behavior while still exporting standard OTLP/HTTP.

## Quickstart (5 lines)

```java
import ai.palette.sdk.*;

Palette.init(PaletteConfig.fromEnv());                       // reads PALETTE_* env vars
String answer = Palette.observe("handle", SemConv.AGENT_RUN, () -> {
    Palette.setInput(query);                                // input.value
    String out = runAgent(query);
    Palette.setOutput(out);                                 // output.value
    return out;
});
Palette.flush();                                            // before a short-lived process exits
```

`PaletteConfig.fromEnv()` reads `PALETTE_BASE_URL` (default `http://127.0.0.1:8080`),
`PALETTE_TENANT_ID`/`PALETTE_PROJECT_ID` (default `demo`), `PALETTE_ENVIRONMENT_ID`
(default `local`), `PALETTE_API_KEY`, `PALETTE_SERVICE_NAME`, and `PALETTE_RELEASE_ID`.
Spans export to `{base}/v1/otlp/{tenant}/{project}/{environment}/v1/traces`.
Span kinds and attribute keys live in one place — `SemConv` — mirroring the
server's OTLP normalizer and the Python/TypeScript SDKs.
