"""Five-line Palette quickstart: instrument an agent and emit a trace.

Run a local paletted (e.g. `docker compose up`), then:

    PALETTE_TENANT_ID=demo PALETTE_PROJECT_ID=demo PALETTE_ENVIRONMENT_ID=local \
        python examples/quickstart.py

Then open the dashboard and click the trace.
"""

import palette
from palette import SpanKind


@palette.observe(kind=SpanKind.AGENT_RUN)
def handle_refund(query: str) -> str:
    plan = make_plan(query)
    return call_model(plan)


@palette.observe(kind=SpanKind.AGENT_PLAN)
def make_plan(query: str) -> str:
    return f"look up policy for: {query}"


@palette.observe(kind=SpanKind.LLM_CALL)
def call_model(plan: str) -> str:
    # In a real app: palette.wrap_openai(OpenAI()).chat.completions.create(...)
    palette.set_output("Escalate: order is outside the standard refund window.")
    return "escalate"


if __name__ == "__main__":
    palette.init(service_name="palette-python-quickstart", release_id="quickstart")
    result = handle_refund("late delivery refund after 31 days")
    print(f"agent result: {result}")
    palette.flush()
    print("trace flushed -- open the dashboard to inspect it")
