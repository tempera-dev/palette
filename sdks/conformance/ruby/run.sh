#!/usr/bin/env bash
# Live conformance for the generated Ruby control-plane client: install the
# generated gem from source and run a program that round-trips against beaterd.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
client="$(cd "$here/../../clients/ruby" && pwd)"

: "${BEATER_BASE_URL:?BEATER_BASE_URL must be set (live beaterd)}"

# Install the generated client's runtime deps and build/install it as a local
# gem into a throwaway GEM_HOME so nothing touches the system gem set.
export GEM_HOME="/tmp/beater-conf-ruby"
export GEM_PATH="$GEM_HOME"
export PATH="$GEM_HOME/bin:$PATH"
mkdir -p "$GEM_HOME"

(
  cd "$client"
  gem build beater_client.gemspec -o /tmp/beater-conf-ruby.gem >/dev/null
  gem install --local /tmp/beater-conf-ruby.gem >/dev/null
)

ruby -I"$client/lib" "$here/conformance.rb"
