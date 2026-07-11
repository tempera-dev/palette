"""Unit tests for the browser-step span mapping.

These tests require ONLY ``opentelemetry-sdk`` (+ ``pytest``). ``browser-use`` is
never imported; instead we feed fake, duck-typed agent/history objects into the
mapping and assert the emitted spans carry the correct ``browser.*`` attributes.
Spans are captured with an ``InMemorySpanExporter`` so no live Palette or browser
is needed.
"""

from __future__ import annotations

import asyncio
import json
import pathlib

import pytest
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import SimpleSpanProcessor
from opentelemetry.sdk.trace.export.in_memory_span_exporter import (
    InMemorySpanExporter,
)
from opentelemetry.trace import StatusCode

from palette_browser_use import (
    PaletteBrowserUseTracer,
    StepRecord,
    instrument,
    make_hooks,
)
from palette_browser_use.semconv import Attr, Browser, SpanKind

FIXTURE = (
    pathlib.Path(__file__).resolve().parent.parent / "fixtures" / "recorded_run.json"
)


# --------------------------------------------------------------------------- #
# Harness: in-memory tracer.
# --------------------------------------------------------------------------- #


@pytest.fixture
def exporter():
    return InMemorySpanExporter()


@pytest.fixture
def tracer(exporter):
    provider = TracerProvider()
    provider.add_span_processor(SimpleSpanProcessor(exporter))
    t = PaletteBrowserUseTracer(tracer_provider=provider)
    yield t
    exporter.clear()


def _by_name(spans, name):
    return [s for s in spans if s.name == name]


# --------------------------------------------------------------------------- #
# Fake browser-use Agent / history (duck-typed).
# --------------------------------------------------------------------------- #


class _FakeBrain:
    def __init__(self, thinking, next_goal):
        self.thinking = thinking
        self.next_goal = next_goal


class _FakeAction:
    """Mimics a browser-use model_actions() entry with grounding fields."""

    def __init__(self, action, selector=None, url=None, title=None,
                 selector_existed=True, matched_element=True, error=None):
        self.action = action
        self.selector = selector
        self.url = url
        self.title = title
        self.selector_existed = selector_existed
        self.matched_element = matched_element
        self.error = error


class _FakeHistory:
    def __init__(self, thoughts, actions, outputs):
        self._thoughts = thoughts
        self._actions = actions
        self._outputs = outputs

    def model_thoughts(self):
        return self._thoughts

    def model_actions(self):
        return self._actions

    def model_outputs(self):
        return self._outputs


class _FakeSession:
    def __init__(self, engine="chromium", url=None):
        self.engine = engine
        self.get_current_page_url = url  # non-callable: defensive path


class _FakeLLM:
    def __init__(self, model):
        self.model = model


class _FakeAgent:
    def __init__(self, history, llm=None, session=None, n_steps=None):
        self.history = history
        self.llm = llm
        self.browser_session = session
        self.n_steps = n_steps


# --------------------------------------------------------------------------- #
# Tests: StepRecord -> spans.
# --------------------------------------------------------------------------- #


def test_record_emits_llm_and_tool_span_pair(tracer, exporter):
    rec = StepRecord(
        seq=0,
        model="gpt-4o",
        reasoning="open the page",
        action="goto",
        url="https://example.com",
        status="ok",
    )
    tracer.record_step(rec)

    spans = exporter.get_finished_spans()
    assert {s.name for s in spans} == {"llm.call", "tool.call"}

    llm = _by_name(spans, "llm.call")[0]
    assert llm.attributes[Attr.SPAN_KIND] == SpanKind.LLM_CALL
    assert llm.attributes[Attr.LLM_MODEL] == "gpt-4o"
    assert llm.attributes[Browser.REASONING] == "open the page"
    assert llm.attributes[Browser.STEP_SEQ] == 0

    tool = _by_name(spans, "tool.call")[0]
    assert tool.attributes[Attr.SPAN_KIND] == SpanKind.TOOL_CALL
    assert tool.attributes[Browser.ACTION] == "goto"
    assert tool.attributes[Browser.URL] == "https://example.com"
    assert tool.attributes[Browser.STEP_STATUS] == "ok"
    assert tool.attributes[Browser.STEP_SEQ] == 0


def test_tool_span_is_child_of_llm_span(tracer, exporter):
    tracer.record_step(StepRecord(seq=0, action="click", selector="#x"))
    spans = exporter.get_finished_spans()
    llm = _by_name(spans, "llm.call")[0]
    tool = _by_name(spans, "tool.call")[0]
    # Same trace, tool parented by llm.
    assert tool.context.trace_id == llm.context.trace_id
    assert tool.parent is not None
    assert tool.parent.span_id == llm.context.span_id


def test_grounding_success_attributes(tracer, exporter):
    rec = StepRecord(
        seq=1,
        action="click",
        selector="#submit",
        url="https://example.com/confirmed",
        status="ok",
        selector_existed=True,
        matched_element=True,
    )
    tracer.record_step(rec)
    tool = _by_name(exporter.get_finished_spans(), "tool.call")[0]
    assert tool.attributes[Browser.SELECTOR] == "#submit"
    assert tool.attributes[Browser.SELECTOR_EXISTED] is True
    assert tool.attributes[Browser.MATCHED_ELEMENT] is True
    assert tool.attributes[Browser.STEP_STATUS] == "ok"
    assert tool.status.status_code != StatusCode.ERROR


def test_grounding_miss_sets_error(tracer, exporter):
    rec = StepRecord(
        seq=2,
        action="type",
        selector="#promo-code",
        status="error",
        selector_existed=False,
        matched_element=False,
        error="selector did not resolve",
    )
    tracer.record_step(rec)
    tool = _by_name(exporter.get_finished_spans(), "tool.call")[0]
    assert tool.attributes[Browser.SELECTOR_EXISTED] is False
    assert tool.attributes[Browser.MATCHED_ELEMENT] is False
    assert tool.attributes[Browser.STEP_STATUS] == "error"
    assert tool.status.status_code == StatusCode.ERROR
    events = {e.name for e in tool.events}
    assert "browser.error" in events


# --------------------------------------------------------------------------- #
# Tests: from_agent (duck-typed live agent).
# --------------------------------------------------------------------------- #


def test_from_agent_maps_history(tracer, exporter):
    agent = _FakeAgent(
        history=_FakeHistory(
            thoughts=[_FakeBrain(thinking="click subscribe", next_goal="confirm")],
            actions=[
                _FakeAction(
                    action="click",
                    selector="#submit",
                    url="https://example.com/confirmed",
                    title="Confirmed",
                )
            ],
            outputs=[{"action": "click"}],
        ),
        llm=_FakeLLM("gpt-4o-mini"),
        session=_FakeSession(engine="chromium"),
        n_steps=3,
    )
    rec = tracer.record_agent_step(agent, seq=3)
    assert rec.action == "click"
    assert rec.selector == "#submit"
    assert rec.model == "gpt-4o-mini"
    assert rec.engine == "chromium"
    assert "click subscribe" in rec.reasoning

    tool = _by_name(exporter.get_finished_spans(), "tool.call")[0]
    assert tool.attributes[Browser.ACTION] == "click"
    assert tool.attributes[Browser.SELECTOR] == "#submit"
    assert tool.attributes[Browser.URL] == "https://example.com/confirmed"
    assert tool.attributes[Browser.ENGINE] == "chromium"
    assert tool.attributes[Browser.STEP_SEQ] == 3


def test_from_agent_grounding_miss(tracer, exporter):
    agent = _FakeAgent(
        history=_FakeHistory(
            thoughts=["look for promo field"],
            actions=[
                _FakeAction(
                    action="type",
                    selector="#promo-code",
                    selector_existed=False,
                    matched_element=False,
                    error="not found",
                )
            ],
            outputs=[{}],
        ),
    )
    tracer.record_agent_step(agent, seq=1)
    tool = _by_name(exporter.get_finished_spans(), "tool.call")[0]
    assert tool.attributes[Browser.STEP_STATUS] == "error"
    assert tool.attributes[Browser.SELECTOR_EXISTED] is False
    assert tool.status.status_code == StatusCode.ERROR


def test_action_dict_verb_extraction(tracer, exporter):
    agent = _FakeAgent(
        history=_FakeHistory(
            thoughts=[],
            actions=[{"go_to_url": {"url": "https://example.com"}}],
            outputs=[],
        ),
    )
    rec = tracer.record_agent_step(agent, seq=0)
    assert rec.action == "go_to_url"


# --------------------------------------------------------------------------- #
# Tests: async hooks.
# --------------------------------------------------------------------------- #


def test_make_hooks_on_step_end_records(tracer, exporter):
    on_start, on_end = make_hooks(tracer=tracer)
    agent = _FakeAgent(
        history=_FakeHistory(
            thoughts=["go"],
            actions=[_FakeAction(action="goto", url="https://example.com")],
            outputs=[{}],
        ),
    )

    async def drive():
        await on_start(agent)
        await on_end(agent)

    asyncio.run(drive())
    spans = exporter.get_finished_spans()
    assert _by_name(spans, "tool.call")
    assert _by_name(spans, "llm.call")


def test_instrument_returns_unpackable_hooks(tracer):
    agent = _FakeAgent(history=_FakeHistory([], [], []))
    inst = instrument(agent, tracer=tracer)
    on_start, on_end = inst  # unpacking contract
    assert asyncio.iscoroutinefunction(on_start)
    assert asyncio.iscoroutinefunction(on_end)
    assert set(inst.run_kwargs()) == {"on_step_start", "on_step_end"}


# --------------------------------------------------------------------------- #
# Tests: fixture replay (the artifact reused by the Rust integration test).
# --------------------------------------------------------------------------- #


def test_fixture_is_valid_and_representative():
    data = json.loads(FIXTURE.read_text())
    steps = data["browser_steps"]
    assert 2 <= len(steps) <= 3

    statuses = [s["outcome"]["status"] for s in steps]
    assert "ok" in statuses and "error" in statuses  # success + grounding miss

    # The successful path ends at a confirmation page.
    last = steps[-1]
    assert last["outcome"]["status"] == "ok"
    assert "confirm" in last["outcome"]["observation"]["url"]

    # The grounding miss has selector_existed == False.
    miss = next(s for s in steps if s["outcome"]["status"] == "error")
    assert miss["outcome"]["grounding"]["selector_existed"] is False
    assert miss["outcome"]["grounding"]["matched_element"] is False

    for s in steps:
        outcome = s["outcome"]
        assert set(outcome["grounding"]) == {
            "selector",
            "selector_existed",
            "matched_element",
        }
        assert "url" in outcome["observation"]
        assert "dom_html" in outcome["observation"]


def test_fixture_replays_into_spans(tracer, exporter):
    data = json.loads(FIXTURE.read_text())
    for step in data["browser_steps"]:
        rec = StepRecord.from_outcome(step, seq=step["seq"])
        tracer.record_step(rec)

    spans = exporter.get_finished_spans()
    tool_spans = sorted(
        _by_name(spans, "tool.call"), key=lambda s: s.attributes[Browser.STEP_SEQ]
    )
    assert len(tool_spans) == len(data["browser_steps"])

    # Step 0: goto, ok.
    assert tool_spans[0].attributes[Browser.ACTION] == "goto"
    assert tool_spans[0].attributes[Browser.STEP_STATUS] == "ok"

    # Step 1: grounding miss.
    assert tool_spans[1].attributes[Browser.STEP_STATUS] == "error"
    assert tool_spans[1].attributes[Browser.SELECTOR] == "#promo-code"
    assert tool_spans[1].attributes[Browser.SELECTOR_EXISTED] is False

    # Step 2: click submit, lands on confirmation page.
    assert tool_spans[2].attributes[Browser.ACTION] == "click"
    assert tool_spans[2].attributes[Browser.STEP_STATUS] == "ok"
    assert "confirm" in tool_spans[2].attributes[Browser.URL]

    # llm.call spans carry reasoning + model.
    llm_spans = _by_name(spans, "llm.call")
    assert all(s.attributes[Attr.LLM_MODEL] == "gpt-4o" for s in llm_spans)
    assert all(Browser.REASONING in s.attributes for s in llm_spans)
