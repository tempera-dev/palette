# Palette Go SDK

Ergonomic OpenTelemetry tracing helpers for Palette (`package palette`).

Palette's default onboarding path is still zero-code OTLP export: point an
existing OpenTelemetry exporter at Palette with `PALETTE_*` env vars, with no
Palette SDK and no code edits. Use this Go SDK when you want an idiomatic helper
layer after that standards-first path.

```go
ctx := context.Background()
shutdown, _ := palette.Init(ctx, palette.ConfigFromEnv()) // reads PALETTE_* env
defer shutdown(ctx)

palette.Observe(ctx, "call_model", palette.KindLLMCall, func(ctx context.Context) error {
    palette.SetInput(ctx, "hello")
    palette.SetOutput(ctx, "world")
    return nil // span kind, palette.seq, release id, status, errors handled for you
})
```

Config falls back to env vars (`PALETTE_BASE_URL`, `PALETTE_TENANT_ID`, ...) with
local defaults (`http://127.0.0.1:8080`, `demo/demo/local`). See `example/main.go`
for an `agent.run -> agent.plan -> llm.call` trace.
