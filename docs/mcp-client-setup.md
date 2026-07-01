# Beater in Claude Code and Codex

Beater exposes the same MCP tool set over two transports:

- Local stdio: `beaterd mcp --stdio`
- Hosted streamable HTTP: `POST /mcp` with OAuth 2.1 bearer tokens

Use stdio for local Claude Code or Codex sessions running on the same machine as
`beaterd`. Use hosted HTTP when the MCP client connects to a deployed Beater API.

## Local stdio

Start from a local data directory. The command below is intentionally explicit so
the MCP process has the same storage every time the client launches it:

```sh
beaterd --data-dir ~/.local/share/beater mcp --stdio
```

`beaterd` defaults to `--auth-mode required`, so real tool calls (anything other
than `initialize`, `tools/list`, and `help`) need credentials. stdio has no HTTP
headers; the transport reads them from the environment instead:

| Variable | Sent as |
| --- | --- |
| `BEATER_API_KEY` | `x-beater-api-key` |
| `BEATER_MCP_TOKEN` (or `BEATER_API_TOKEN`) | `Authorization: Bearer …` |
| `BEATER_PROJECT_ID` | `x-beater-project-id` (strict-auth scope) |
| `BEATER_ENVIRONMENT_ID` | `x-beater-environment-id` (strict-auth scope) |

Bootstrap a key once with `beaterctl api-key-create --data-dir ~/.local/share/beater
--tenant-id <t> --project-id <p> --environment-id <e> --scopes trace-read`, then
export `BEATER_API_KEY` (plus the two scope ids) in the environment the MCP client
uses to launch `beaterd`. For anonymous local experimentation on a loopback
address, pass `--auth-mode local` before the `mcp` subcommand instead.

Smoke the local transport before wiring a client:

```sh
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' \
  '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"help","arguments":{}}}' \
  | beaterd --data-dir ~/.local/share/beater mcp --stdio
```

Expected result: one JSON-RPC response per input line. `tools/list` returns the
generated `/v1` operation tools plus `help`; `tools/call` on `help` returns local
tool usage text without calling a hosted service.

## Claude Code

Add Beater as a local stdio MCP server:

```sh
claude mcp add beater -- beaterd --data-dir ~/.local/share/beater mcp --stdio
```

Then restart or reload Claude Code's MCP servers and run a quick tool inventory
from the client. The first check should be `tools/list`; the second should be a
read-only `tools/call` for `help`.

## Codex

Add Beater to Codex's MCP configuration as a stdio server:

```toml
[mcp_servers.beater]
command = "beaterd"
args = ["--data-dir", "~/.local/share/beater", "mcp", "--stdio"]
```

Reload Codex's MCP configuration and verify the server with `tools/list`, then a
read-only `tools/call` for `help`.

## Hosted HTTP and OAuth

For hosted or remote clients, point the MCP client at:

```text
https://<beater-api-host>/mcp
```

An uncredentialed `POST /mcp` returns an RFC 9728 `WWW-Authenticate` challenge
pointing at the protected-resource metadata, which in turn names the
authorization server:

```text
https://<beater-api-host>/.well-known/oauth-protected-resource
https://<beater-api-host>/.well-known/oauth-authorization-server
```

The authorization flow uses:

```text
GET  /oauth/authorize
POST /oauth/token
POST /mcp
GET  /mcp
```

After the client has a bearer token, smoke the hosted transport:

```sh
curl -fsS -X POST "https://<beater-api-host>/mcp" \
  -H "authorization: Bearer $BEATER_MCP_TOKEN" \
  -H "content-type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}'

curl -fsS -X POST "https://<beater-api-host>/mcp" \
  -H "authorization: Bearer $BEATER_MCP_TOKEN" \
  -H "content-type: application/json" \
  -d '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"help","arguments":{}}}'
```

Keep hosted runtime secrets in the deployed Beater environment. Do not put API
keys, provider secrets, or OAuth client secrets into the MCP client config file.
