#!/usr/bin/env python3
"""Live MCP conformance driver used by scripts/e2e-mcp-live.sh.

Drives a running beaterd's `POST /mcp` endpoint with real JSON-RPC 2.0 —
initialize (version negotiation), tools/list, tools/call — and asserts parity
with the direct REST surface. Two modes:

  MODE=anon      beaterd runs --auth-mode local; asserts an ingested trace is
                 visible through MCP and that tools/call listTraces returns
                 byte-identical JSON to GET /v1/traces/... .
  MODE=required  beaterd runs --auth-mode required; asserts an uncredentialed
                 tools/call is rejected with HTTP 401 semantics (and an RFC 9728
                 WWW-Authenticate challenge), then succeeds with the API key.

Environment: BEATER_MCP_URL, BEATER_REST_URL, MODE, and in required mode
BEATER_API_KEY / BEATER_PROJECT_ID / BEATER_ENVIRONMENT_ID. Uses only the
Python standard library.
"""
from __future__ import annotations

import json
import os
import sys
import urllib.error
import urllib.request

MCP_URL = os.environ["BEATER_MCP_URL"]
REST_URL = os.environ["BEATER_REST_URL"]
MODE = os.environ.get("MODE", "anon")

_next_id = 0


def fail(message: str) -> None:
    print(f"e2e-mcp-live FAILED: {message}", file=sys.stderr)
    raise SystemExit(1)


def http(url: str, body: bytes | None = None, headers: dict[str, str] | None = None):
    request = urllib.request.Request(url, data=body, headers=headers or {})
    if body is not None:
        request.add_header("content-type", "application/json")
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            return response.status, dict(response.headers), response.read()
    except urllib.error.HTTPError as err:
        return err.code, dict(err.headers), err.read()


def rpc(method: str, params: dict, headers: dict[str, str] | None = None):
    global _next_id
    _next_id += 1
    payload = {"jsonrpc": "2.0", "id": _next_id, "method": method, "params": params}
    status, response_headers, body = http(
        MCP_URL, json.dumps(payload).encode(), headers
    )
    if status != 200:
        fail(f"{method} returned HTTP {status}: {body[:400]!r}")
    message = json.loads(body)
    if message.get("jsonrpc") != "2.0" or message.get("id") != _next_id:
        fail(f"{method} returned malformed JSON-RPC: {message}")
    return message, response_headers


def auth_headers() -> dict[str, str]:
    return {
        "x-beater-api-key": os.environ["BEATER_API_KEY"],
        "x-beater-project-id": os.environ["BEATER_PROJECT_ID"],
        "x-beater-environment-id": os.environ["BEATER_ENVIRONMENT_ID"],
    }


def check_probe() -> None:
    status, _, body = http(MCP_URL)
    if status != 200:
        fail(f"GET /mcp probe returned {status}")
    probe = json.loads(body)
    if probe.get("serverInfo", {}).get("name") != "beater-mcp":
        fail(f"GET /mcp probe missing serverInfo: {probe}")
    print("ok: GET /mcp capability probe")


def check_initialize() -> None:
    # Ask for an older supported revision; the server must echo it back
    # (standard MCP version negotiation), and advertise the tools capability.
    message, _ = rpc("initialize", {"protocolVersion": "2025-03-26"})
    result = message.get("result", {})
    if result.get("protocolVersion") != "2025-03-26":
        fail(f"initialize did not negotiate requested version: {result}")
    if "tools" not in result.get("capabilities", {}):
        fail(f"initialize missing tools capability: {result}")
    print("ok: initialize + protocol version negotiation")


def check_tools_list() -> list[dict]:
    message, _ = rpc("tools/list", {})
    tools = message.get("result", {}).get("tools")
    if not isinstance(tools, list) or len(tools) < 50:
        fail(f"tools/list returned too few tools: {len(tools or [])}")
    names = {tool["name"] for tool in tools}
    for required in ("listTraces", "getTrace", "searchSpans", "help"):
        if required not in names:
            fail(f"tools/list missing {required}")
    for tool in tools:
        if tool.get("inputSchema", {}).get("type") != "object":
            fail(f"tool {tool.get('name')} has malformed inputSchema")
    print(f"ok: tools/list ({len(tools)} tools, catalog well-formed)")
    return tools


def check_unknown_tool() -> None:
    message, _ = rpc("tools/call", {"name": "definitelyNotATool", "arguments": {}})
    if "error" not in message:
        fail(f"unknown tool must be a JSON-RPC error: {message}")
    print("ok: unknown tool rejected")


def list_traces_arguments() -> dict:
    return {
        "tenant_id": os.environ.get("BEATER_TENANT_ID", "demo"),
        "project_id": os.environ.get("BEATER_PROJECT_ID", "demo"),
        "environment_id": os.environ.get("BEATER_ENVIRONMENT_ID", "local"),
    }


def check_anon_parity() -> None:
    arguments = list_traces_arguments()
    message, _ = rpc("tools/call", {"name": "listTraces", "arguments": arguments})
    result = message.get("result", {})
    if result.get("isError") is not False:
        fail(f"listTraces tools/call errored: {result}")
    structured = result.get("structuredContent")
    items = structured.get("items") if isinstance(structured, dict) else None
    if not isinstance(items, list) or not items:
        fail(f"expected the ingested trace in MCP listTraces, got: {structured}")

    rest_url = (
        f"{REST_URL}/v1/traces/{arguments['tenant_id']}"
        f"?project_id={arguments['project_id']}"
        f"&environment_id={arguments['environment_id']}"
    )
    status, _, body = http(rest_url)
    if status != 200:
        fail(f"REST listTraces returned {status}")
    if json.loads(body) != structured:
        fail("MCP structuredContent != REST JSON for listTraces")
    print(f"ok: MCP listTraces sees {len(items)} trace(s), byte-parity with REST")

    # Follow the trace through getTrace so a multi-step agentic flow
    # (list -> drill in) is proven, not just a single lookup.
    trace_id = items[0].get("trace_id")
    if not trace_id:
        fail(f"trace summary missing trace_id: {items[0]}")
    message, _ = rpc(
        "tools/call",
        {
            "name": "getTrace",
            "arguments": {"tenant_id": arguments["tenant_id"], "trace_id": trace_id},
        },
    )
    result = message.get("result", {})
    if result.get("isError") is not False:
        fail(f"getTrace tools/call errored: {result}")
    print(f"ok: MCP getTrace round-trips trace {trace_id}")

    help_message, _ = rpc(
        "tools/call", {"name": "help", "arguments": {"tool": "listTraces"}}
    )
    tool_doc = help_message["result"]["structuredContent"]["tool"]
    if tool_doc.get("name") != "listTraces" or tool_doc.get("method") != "GET":
        fail(f"help tool returned wrong doc: {tool_doc}")
    print("ok: help tool describes operations")


def check_required_auth() -> None:
    arguments = list_traces_arguments()
    call = {"name": "listTraces", "arguments": arguments}

    message, headers = rpc("tools/call", call)
    result = message.get("result", {})
    if result.get("isError") is not True:
        fail(f"uncredentialed call must fail under required auth: {result}")
    if result.get("_meta", {}).get("httpStatus") != 401:
        fail(f"uncredentialed call must surface HTTP 401: {result}")
    challenge = headers.get("WWW-Authenticate") or headers.get("www-authenticate")
    if not challenge or "resource_metadata" not in challenge:
        fail(f"missing RFC 9728 WWW-Authenticate challenge, got: {challenge}")
    print("ok: uncredentialed tools/call rejected with 401 + OAuth challenge")

    message, _ = rpc("tools/call", call, headers=auth_headers())
    result = message.get("result", {})
    if result.get("isError") is not False:
        fail(f"credentialed call must pass auth: {result}")
    if result.get("_meta", {}).get("httpStatus") != 200:
        fail(f"credentialed call must return 200: {result}")
    if not isinstance(result.get("structuredContent", {}).get("items"), list):
        fail(f"credentialed listTraces missing trace page: {result}")
    print("ok: API-key credentialed tools/call authorized end-to-end")


def main() -> None:
    check_probe()
    check_initialize()
    check_tools_list()
    check_unknown_tool()
    if MODE == "anon":
        check_anon_parity()
    elif MODE == "required":
        check_required_auth()
    else:
        fail(f"unknown MODE {MODE!r}")
    print(f"e2e-mcp-live: {MODE} mode passed")


if __name__ == "__main__":
    main()
