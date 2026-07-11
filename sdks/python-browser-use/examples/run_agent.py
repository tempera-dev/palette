"""Run a ``browser-use`` Agent with Palette instrumentation.

This emits an ``llm.call`` + ``tool.call`` span pair per browser step over
OTLP/gRPC to Palette (defaults to ``localhost:4317``; override with
``$PALETTE_OTLP_ENDPOINT``).

Prereqs::

    pip install palette-browser-use[browser-use]
    # plus an LLM provider key for browser-use, e.g. OPENAI_API_KEY

Run::

    python examples/run_agent.py
"""

from __future__ import annotations

import asyncio
import os

from palette_browser_use import instrument


async def main() -> None:
    # Imported here so the rest of the SDK (and its tests) never require
    # browser-use to be installed.
    from browser_use import Agent
    from browser_use.llm import ChatOpenAI  # provider import path may vary by version

    agent = Agent(
        task="Go to https://example.com and report the page heading.",
        llm=ChatOpenAI(model="gpt-4o"),
    )

    # Wire Palette instrumentation. `endpoint` is optional; defaults to
    # $PALETTE_OTLP_ENDPOINT or localhost:4317.
    inst = instrument(agent, endpoint=os.environ.get("PALETTE_OTLP_ENDPOINT"))

    try:
        # Splat the on_step_start / on_step_end hooks into the run.
        await agent.run(**inst.run_kwargs())
    finally:
        # Flush pending spans to Palette before exiting.
        inst.tracer.shutdown()


if __name__ == "__main__":
    asyncio.run(main())
