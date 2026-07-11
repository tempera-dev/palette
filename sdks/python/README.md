# Palette Python SDK

Ergonomic, OpenTelemetry-native agent observability for [Palette](https://github.com/palette/palette).

This is the **Layer 2** (hand-written, ergonomic) SDK: `@observe` decorators,
drop-in provider wrappers, and LangChain/LlamaIndex callbacks. The
**Layer 1** generated control-plane client (datasets, experiments, gates, evals,
usage, etc.) is published separately as `palette_client`, generated from the
Palette OpenAPI contract so it never drifts from the API.

## Install

```bash
pip install palette-sdk                 # core (OTLP/HTTP export)
pip install "palette-sdk[openai]"       # + OpenAI wrapper
pip install "palette-sdk[anthropic]"    # + Anthropic wrapper
pip install "palette-sdk[groq]"         # + Groq SDK for wrap_groq()
pip install "palette-sdk[mistral]"      # + OpenAI client for Mistral's compatible endpoint
pip install "palette-sdk[langchain]"    # + LangChain callback
pip install "palette-sdk[llamaindex]"   # + LlamaIndex callback
pip install "palette-sdk[grpc]"         # OTLP/gRPC export instead of HTTP
```

## Quickstart (5 lines)

```python
import palette

palette.init(tenant_id="acme", project_id="support-bot", environment_id="prod")

@palette.observe(kind=palette.SpanKind.AGENT_RUN)
def handle(query): ...
```

All `init()` arguments fall back to `PALETTE_*` env vars
(`PALETTE_BASE_URL`, `PALETTE_TENANT_ID`, `PALETTE_PROJECT_ID`,
`PALETTE_ENVIRONMENT_ID`, `PALETTE_API_KEY`), so `palette.init()` works with no args
when the environment is configured.

## Zero-code env bootstrap

For apps launched with OpenTelemetry Python auto-instrumentation, select the
Palette configurator and provide only environment variables. No application code
edits are required. Install the OpenTelemetry launcher and instrumentors
separately for the libraries your app uses.

```bash
export PALETTE_BASE_URL=http://127.0.0.1:8080
export PALETTE_TENANT_ID=demo
export PALETTE_PROJECT_ID=demo
export PALETTE_ENVIRONMENT_ID=local
export PALETTE_SERVICE_NAME=my-agent
export OTEL_PYTHON_CONFIGURATOR=palette

opentelemetry-instrument python app.py
```

If provider constructor auto-instrumentation is installed, enable it explicitly:

```bash
export PALETTE_AUTO_INSTRUMENT=openai,anthropic
```

The bootstrap module is safe to import; it initializes tracing only when the
`palette` OpenTelemetry configurator runs or `palette.bootstrap.bootstrap_from_env()`
is called directly.

## Drop-in provider wrappers

Auto-instrument installed provider clients:

```python
import palette

palette.init()
palette.instrument(providers=["openai", "anthropic"])

from openai import OpenAI

client = OpenAI()
# every client.chat.completions.create(...) is now an llm.call span
# with model + token counts
```

Or wrap one client explicitly:

```python
from openai import OpenAI
client = palette.wrap_openai(OpenAI())
# every client.chat.completions.create(...) is now an llm.call span
# with model + token counts

from anthropic import Anthropic
client = palette.wrap_anthropic(Anthropic())

from groq import Groq
client = palette.wrap_groq(Groq())

from openai import OpenAI
mistral_client = OpenAI(base_url="https://api.mistral.ai/v1")
client = palette.wrap_mistral(mistral_client)
```

## Framework callbacks

The framework adapters are part of the SDK's public surface (importable straight
from `palette`, matching the TypeScript SDK). Each one imports its framework
lazily, so `import palette` never requires LangChain or LlamaIndex to be installed.

```python
from palette import PaletteCallbackHandler
chain.invoke(inputs, config={"callbacks": [PaletteCallbackHandler()]})

from palette import PaletteLlamaIndexHandler
from llama_index.core import Settings
from llama_index.core.callbacks import CallbackManager
Settings.callback_manager = CallbackManager([PaletteLlamaIndexHandler()])
```

## Manual spans

```python
with palette.span("retrieve-policy", kind=palette.SpanKind.RETRIEVAL_QUERY) as s:
    palette.set_input("refund policy")
    palette.set_output(result)
```

## Semantic conventions

Span kinds and attribute keys live in one place — `palette.semconv` — and mirror
the server's OTLP normalizer (`crates/palette-otlp`). This keeps the SDK's emitted
spans in lockstep with what Palette ingests.

## Transport

By default the SDK exports over **OTLP/HTTP** to
`{base_url}/v1/otlp/{tenant}/{project}/{environment}/v1/traces`. Set
`protocol="grpc"` (and install the `grpc` extra) to export to the OTLP gRPC
endpoint instead.
