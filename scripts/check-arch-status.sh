#!/usr/bin/env bash
# Drift guard for docs/architecture-status.md.
#
# Thin wrapper around `cargo xtask check-arch-status` so the architecture-status
# ledger gets the same shell-script entry point as the other contract/doc checks
# under scripts/ (check-contract-sync.sh, check-openapi-drift.sh, ...). The real
# logic lives in crates/xtask/src/main.rs (`check_arch_status`); see the header
# of docs/architecture-status.md for the maintainer workflow.
set -euo pipefail

cd "$(dirname "$0")/.."
exec cargo run -q -p xtask -- check-arch-status
