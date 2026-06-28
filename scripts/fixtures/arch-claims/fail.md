# Test Architecture (fail fixture)

This document has a risky completion claim with NO matching ledger row.

## Untracked Component

The following table row claims "hyperdrive-module" is built, but that component
has no row in the status ledger — the linter must flag it.

| Component | Status |
|---|---|
| **hyperdrive-module** | [built] |

## Known Component

The following IS in the ledger so it should NOT be flagged:

| Component | Status |
|---|---|
| **beaterd server** | [built] |
