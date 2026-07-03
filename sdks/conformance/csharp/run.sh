#!/usr/bin/env bash
# Live conformance for the generated C#/.NET control-plane client: build the
# conformance program (which project-references the generated client) and run it
# against beaterd.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

: "${BEATER_BASE_URL:?BEATER_BASE_URL must be set (live beaterd)}"

dotnet run -c Release --project "$here/Conformance.csproj"
