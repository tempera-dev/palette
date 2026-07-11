"""Palette Python SDK -- ergonomic agent observability.

Quickstart::

    import palette

    palette.init(tenant_id="acme", project_id="support-bot", environment_id="prod")

    @palette.observe(kind=palette.SpanKind.AGENT_RUN)
    def handle(query): ...

Drop-in provider instrumentation::

    from openai import OpenAI
    client = palette.wrap_openai(OpenAI())
    from groq import Groq
    client = palette.wrap_groq(Groq())

Auto-instrument installed providers::

    palette.instrument(providers=["openai", "anthropic"])

This is the hand-written ergonomic (Layer 2) SDK. The generated control-plane
client (Layer 1, for datasets/experiments/gates/etc.) lives in the
``palette_client`` package generated from the OpenAPI contract.
"""

from __future__ import annotations

from .auto import InstrumentationResult, instrument
from .config import PaletteConfig
from .integrations import PaletteCallbackHandler, PaletteLlamaIndexHandler
from .observe import observe, set_input, set_output, span
from .providers.anthropic import wrap_anthropic
from .providers.openai import wrap_openai
from .providers.openai_compatible import wrap_groq, wrap_mistral, wrap_openai_compatible
from .semconv import Attr, SpanKind
from .tracing import flush, get_config, init, shutdown

__all__ = [
    "init",
    "observe",
    "span",
    "set_input",
    "set_output",
    "instrument",
    "wrap_openai",
    "wrap_anthropic",
    "wrap_openai_compatible",
    "wrap_groq",
    "wrap_mistral",
    "PaletteCallbackHandler",
    "PaletteLlamaIndexHandler",
    "flush",
    "shutdown",
    "get_config",
    "InstrumentationResult",
    "PaletteConfig",
    "SpanKind",
    "Attr",
]

__version__ = "0.1.0"
