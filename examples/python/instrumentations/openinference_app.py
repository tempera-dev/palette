"""OpenInference -> Palette fixture app (R11.2).

Emits a small agent trace using the **OpenInference** semantic-convention
attributes (the convention used by Arize Phoenix and friends) over stock
OpenTelemetry OTLP/gRPC. Palette ingests OpenInference attributes natively, so no
Palette SDK is required -- this is the zero-SDK onboarding path.

Run a local paletted (`docker compose up`) and then:

    pip install opentelemetry-sdk opentelemetry-exporter-otlp-proto-grpc
    python examples/python/instrumentations/openinference_app.py

Open the dashboard and click the trace: you should see an agent.run -> llm.call
waterfall with model, tokens, cost, and redacted-capable input/output.
"""

import os

from opentelemetry import trace
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor


def build_tracer() -> trace.Tracer:
    provider = TracerProvider()
    provider.add_span_processor(
        BatchSpanProcessor(
            OTLPSpanExporter(
                endpoint=os.getenv("OTEL_EXPORTER_OTLP_ENDPOINT", "http://127.0.0.1:4317"),
                insecure=True,
                headers=(
                    ("x-palette-tenant-id", os.getenv("PALETTE_TENANT_ID", "demo")),
                    ("x-palette-project-id", os.getenv("PALETTE_PROJECT_ID", "demo")),
                    ("x-palette-environment-id", os.getenv("PALETTE_ENVIRONMENT_ID", "local")),
                ),
            )
        )
    )
    trace.set_tracer_provider(provider)
    return trace.get_tracer("palette.example.openinference")


def main() -> None:
    tracer = build_tracer()
    release = os.getenv("PALETTE_RELEASE_ID", "openinference-example")

    # OpenInference uses `openinference.span.kind` to mark agent/LLM/tool spans.
    with tracer.start_as_current_span(
        "handle_refund",
        attributes={
            "openinference.span.kind": "AGENT",
            "palette.span.kind": "agent.run",
            "palette.release_id": release,
            "input.value": "late delivery refund after 31 days",
        },
    ):
        with tracer.start_as_current_span(
            "call_model",
            attributes={
                "openinference.span.kind": "LLM",
                "palette.span.kind": "llm.call",
                "llm.provider": "openai",
                "llm.model_name": "gpt-4o-mini",
                "llm.token_count.prompt": 42,
                "llm.token_count.completion": 18,
                "llm.cost.amount_micros": 1200,
                "llm.cost.currency": "USD",
                "input.value": "look up refund policy and decide",
                "output.value": "Escalate: order is outside the standard refund window.",
                "palette.release_id": release,
            },
        ):
            pass

    provider = trace.get_tracer_provider()
    provider.force_flush()  # type: ignore[attr-defined]
    provider.shutdown()  # type: ignore[attr-defined]
    print("OpenInference trace flushed -- open the dashboard to inspect it")


if __name__ == "__main__":
    main()
