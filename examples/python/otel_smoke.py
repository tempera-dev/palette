import os

from opentelemetry import trace
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk.resources import Resource
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor
from opentelemetry.trace import Status, StatusCode


endpoint = os.getenv("OTEL_EXPORTER_OTLP_ENDPOINT", "http://127.0.0.1:4317")
headers = (
    ("x-beater-tenant-id", os.getenv("BEATER_TENANT_ID", "demo")),
    ("x-beater-project-id", os.getenv("BEATER_PROJECT_ID", "demo")),
    ("x-beater-environment-id", os.getenv("BEATER_ENVIRONMENT_ID", "local")),
)

provider = TracerProvider(resource=Resource.create({"service.name": "beater-otel-python-smoke"}))
provider.add_span_processor(
    BatchSpanProcessor(OTLPSpanExporter(endpoint=endpoint, insecure=True, headers=headers))
)
trace.set_tracer_provider(provider)
tracer = trace.get_tracer("beater.examples.python")

with tracer.start_as_current_span(
    "refund-agent-run",
    attributes={
        "openinference.span.kind": "agent.run",
        "beater.seq": 1,
        "agent.release_id": "compose-demo",
    },
) as run:
    run.set_status(Status(StatusCode.OK))
    with tracer.start_as_current_span(
        "plan-refund-resolution",
        attributes={"openinference.span.kind": "agent.step", "beater.seq": 2},
    ) as step:
        step.set_status(Status(StatusCode.OK))
    with tracer.start_as_current_span(
        "call-policy-model",
        attributes={
            "openinference.span.kind": "llm.call",
            "beater.seq": 3,
            "llm.provider": "openai",
            "llm.model_name": "gpt-demo",
            "llm.token_count.prompt": 18,
            "llm.token_count.completion": 11,
            "llm.token_count.reasoning": 4,
            "llm.cost.amount_micros": 2500,
            "llm.cost.currency": "USD",
            "input.value": "Can this order be refunded after 31 days?",
            "output.value": "Escalate because the order is outside the standard window.",
        },
    ) as llm:
        llm.set_status(Status(StatusCode.OK))
    with tracer.start_as_current_span(
        "lookup-order-tool",
        attributes={
            "openinference.span.kind": "tool.call",
            "beater.seq": 4,
            "input.value": '{"order_id":"ord_123"}',
            "output.value": '{"status":"delivered","age_days":31}',
        },
    ) as tool:
        tool.set_status(Status(StatusCode.OK))

provider.force_flush()
provider.shutdown()
print("sent stock OpenTelemetry Python trace to", endpoint)
