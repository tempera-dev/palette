"""Beater instrumentation SDK for the ``browser-use`` agent framework.

Hooks a ``browser-use`` ``Agent`` and emits canonical ``browser.*`` browser-step
spans (an ``llm.call`` decision span + a ``tool.call`` action span per step) over
OTLP/gRPC into Beater.

Quickstart
----------
>>> from browser_use import Agent
>>> from beater_browser_use import instrument
>>>
>>> agent = Agent(task="...", llm=...)
>>> inst = instrument(agent, endpoint="localhost:4317")
>>> await agent.run(**inst.run_kwargs())
>>> inst.tracer.shutdown()
"""

from __future__ import annotations

from . import semconv
from .instrumentation import (
    BeaterBrowserUseTracer,
    StepRecord,
    instrument,
    make_hooks,
)
from .semconv import Attr, Browser, SpanKind

__all__ = [
    "BeaterBrowserUseTracer",
    "StepRecord",
    "instrument",
    "make_hooks",
    "semconv",
    "Attr",
    "Browser",
    "SpanKind",
]

__version__ = "0.1.0"
