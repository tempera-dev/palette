#!/usr/bin/env bash
# Live conformance for the generated Kotlin control-plane client: a Gradle
# composite build pulls in the generated client (../../clients/kotlin) and runs
# a program that round-trips against beaterd.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

: "${BEATER_BASE_URL:?BEATER_BASE_URL must be set (live beaterd)}"

gradle -p "$here" --no-daemon --quiet --console=plain run
