# EMMY_CHANGELOG.md

This file is the single source of truth for all changes made during the
Stellar Wave Program appeal audit. Entries are append-only and dated.

---

## 2026-09-20

### PR: feat/escrow-example — Add escrow example contract

**Files added:**
- `examples/escrow/Cargo.toml`
- `examples/escrow/src/lib.rs` — Full escrow contract + 9 unit tests
- `examples/escrow/README.md` — Usage, use cases, deploy instructions

**File changed:** `Cargo.toml` — added `examples/escrow` to workspace members

**What changed:**
Added a real, working Soroban contract built from this template. The escrow
contract implements a three-party (depositor / recipient / arbiter) trustless
token escrow with approve, refund, and cancel paths.

Tests cover:
- Approve sends tokens to recipient
- Refund returns tokens to depositor
- Cancel (by depositor) returns tokens
- Double deposit panics
- Double approve panics
- Refund after approve panics
- Cancel after approve panics
- Zero-amount panics
- Read helpers return correct values

**Why:** A template-only repo with no real example contract shows minimal
ecosystem impact. The escrow demonstrates that this template is usable for
real work and adds a genuinely useful primitive to the Stellar/Soroban
ecosystem.

---

### PR: docs/template-readme — Updated README

**File changed:** `README.md`

**What changed:**
- Added clear statement of the repo's dual purpose (scaffold + working example)
- Added repository layout section including the new examples/ directory
- Added escrow example description with lifecycle and use cases
- Updated all CLI examples to include escrow contract
- Added reference to examples/escrow/README.md for full instructions

**Why:** The README did not reflect the new example contract or explain the
"template + real example" framing needed for the appeal.

---

## 2026-09-20 (follow-up)

### PR: docs/appeal-framing — Honest appeal framing in README

**File changed:** `README.md`

**What changed:**
Added a new "About this repo" section near the top (before "What this repo is")
that explicitly states:
- This is developer tooling (a Soroban contract template), not a product
- Its value is reducing time-to-compiling-contract for Stellar/Soroban devs
- The escrow example makes the template's utility concrete and verifiable
- The appeal claim is ecosystem tooling value, not a standalone application

**Why:** Confirmed decision from the project owner: the Stellar Wave Program
appeal should frame this repo honestly as "developer tooling + a real, tested
example", not disguise it as a full product. The README now states this
directly so evaluators understand the repo's nature without ambiguity.

---
