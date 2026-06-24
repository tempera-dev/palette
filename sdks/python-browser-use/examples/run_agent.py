"""Run a ``browser-use`` Agent with Beater instrumentation.

This emits an ``llm.call`` + ``tool.call`` span pair per browser step over
OTLP/gRPC to Beater (defaults to ``localhost:4317``; override with
``$BEATER_OTLP_ENDPOINT``).

Prereqs::

    pip install beater-browser-use[browser-use]
    # plus an LLM provider key for browser-use, e.g. OPENAI_API_KEY

Run::

    python examples/run_agent.py
"""

from __future__ import annotations

import asyncio
import os

from beater_browser_use import instrument


async def main() -> None:
    # Imported here so the rest of the SDK (and its tests) never require
    # browser-use to be installed.
    from browser_use import Agent
    from browser_use.llm import ChatOpenAI  # provider import path may vary by version

    agent = Agent(
        task="Go to https://example.com and report the page heading.",
        llm=ChatOpenAI(model="gpt-4o"),
    )

    # Wire Beater instrumentation. `endpoint` is optional; defaults to
    # $BEATER_OTLP_ENDPOINT or localhost:4317.
    inst = instrument(agent, endpoint=os.environ.get("BEATER_OTLP_ENDPOINT"))

    try:
        # Splat the on_step_start / on_step_end hooks into the run.
        await agent.run(**inst.run_kwargs())
    finally:
        # Flush pending spans to Beater before exiting.
        inst.tracer.shutdown()


if __name__ == "__main__":
    asyncio.run(main())
