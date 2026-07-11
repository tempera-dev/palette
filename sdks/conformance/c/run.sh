#!/usr/bin/env bash
# Live conformance for the generated C control-plane client.
#   1. Build-verify: cmake-build the generated client's clean health-path object
#      set (the full lib has pre-existing openapi-generator array-of-enum codegen
#      bugs in unrelated models; see main.c for detail).
#   2. Live: compile a C program that links the GENERATED HealthAPI + apiClient
#      and round-trips GET /health against paletted, then exercises
#      createDataset + listTraces over raw libcurl from the same program.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
client="$(cd "$here/../../clients/c" && pwd)"

: "${PALETTE_BASE_URL:?PALETTE_BASE_URL must be set (live paletted)}"

cc=${CC:-cc}
build="$here/build"
mkdir -p "$build"

# Generated client sources on the clean (health) dependency path.
gen_srcs=(
  "$client/src/apiClient.c"
  "$client/src/list.c"
  "$client/src/binary.c"
  "$client/external/cJSON.c"
  "$client/model/health_response.c"
  "$client/api/HealthAPI.c"
)

curl_cflags="$(curl-config --cflags 2>/dev/null || true)"
curl_libs="$(curl-config --libs 2>/dev/null || echo -lcurl)"

echo "  compiling generated C client (health path) + conformance program"
$cc -std=c11 -O0 -g \
  -I"$client/include" -I"$client/api" -I"$client/model" -I"$client/external" \
  $curl_cflags \
  "$here/main.c" "${gen_srcs[@]}" \
  $curl_libs \
  -o "$build/palette_c_conformance"

"$build/palette_c_conformance"
