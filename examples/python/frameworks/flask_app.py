"""Flask + OTLP -> Palette example app (R11.4).

A minimal Flask service whose request handler emits an agent trace to paletted
over stock OpenTelemetry OTLP/gRPC. Demonstrates the Python framework adoption
path through standards (no Palette SDK).

Run a local paletted (`docker compose up`) and then:

    pip install flask opentelemetry-sdk opentelemetry-exporter-otlp-proto-grpc
    flask --app examples/python/frameworks/flask_app run --port 8001
    curl -X POST localhost:8001/agent -d '{"prompt":"refund please"}' -H 'content-type: application/json'
"""

import os

from flask import Flask, jsonify, request
from opentelemetry import trace
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor

_provider = TracerProvider()
_provider.add_span_processor(
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
trace.set_tracer_provider(_provider)
_tracer = trace.get_tracer("palette.example.flask")

app = Flask(__name__)


@app.post("/agent")
def run_agent():
    body = request.get_json(silent=True) or {}
    prompt = body.get("prompt", "")
    release = os.getenv("PALETTE_RELEASE_ID", "flask-example")
    with _tracer.start_as_current_span(
        "handle_request",
        attributes={"palette.span.kind": "agent.run", "palette.release_id": release, "input.value": prompt},
    ):
        with _tracer.start_as_current_span(
            "call_model",
            attributes={
                "palette.span.kind": "llm.call",
                "llm.provider": "openai",
                "llm.model_name": "gpt-4o-mini",
                "palette.release_id": release,
                "input.value": prompt,
                "output.value": "ok",
            },
        ):
            decision = "escalate"
    return jsonify({"decision": decision})
