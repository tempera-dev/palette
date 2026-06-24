"""Canonical ``browser.*`` semantic-convention attribute keys.

These mirror the Rust source of truth in
``crates/beater-browser/src/semconv.rs``. Both the Rust capture layer and this
external instrumentation SDK emit these exact keys so the OTLP ingest mapping
(``beater-otlp``) can normalize any source identically. Keep them in lockstep
with the Rust crate.
"""

from __future__ import annotations


class SpanKind:
    """OpenInference ``openinference.span.kind`` values accepted by the normalizer.

    Browser steps project onto the existing agent-observability kinds: the LLM
    decision is an ``llm.call`` and the browser action is a ``tool.call``.
    """

    LLM_CALL = "llm.call"
    TOOL_CALL = "tool.call"


class Browser:
    """Canonical ``browser.*`` attribute keys (mirror of ``semconv.rs``)."""

    #: Browser engine that executed the step (``chromium``, ``firefox``, ...).
    ENGINE = "browser.engine"
    #: Action verb for the step (``goto``, ``click``, ``type``, ...).
    ACTION = "browser.action"
    #: Target selector for the action, when it targets an element.
    SELECTOR = "browser.selector"
    #: URL the page was on when the step executed.
    URL = "browser.url"
    #: Page title observed for the step.
    TITLE = "browser.title"
    #: Whether the action's selector resolved to an element in the DOM.
    SELECTOR_EXISTED = "browser.selector_existed"
    #: Whether the resolved element was the intended target (grounding success).
    MATCHED_ELEMENT = "browser.matched_element"
    #: Monotonic step sequence number within the agent run.
    STEP_SEQ = "browser.step_seq"
    #: Step status string (``ok`` or ``error``).
    STEP_STATUS = "browser.step_status"
    #: Artifact id of the stored DOM snapshot for the step.
    DOM_ARTIFACT = "browser.dom_artifact_id"
    #: Artifact id of the stored screenshot for the step.
    SCREENSHOT_ARTIFACT = "browser.screenshot_artifact_id"
    #: The agent's reasoning text for the decision (from ``model_thoughts``).
    REASONING = "browser.reasoning"


class Attr:
    """Non-``browser.*`` attribute keys this SDK also emits."""

    SPAN_KIND = "openinference.span.kind"
    #: Model name on the ``llm.call`` span.
    LLM_MODEL = "llm.model"


#: ``ok`` / ``error`` step-status string, matching the Rust ``StepStatus`` enum.
STATUS_OK = "ok"
STATUS_ERROR = "error"
