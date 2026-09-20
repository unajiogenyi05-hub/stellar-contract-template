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

### PR: fix/ci-lint-format — Fix Clippy and rustfmt CI failures

**File changed:** `examples/escrow/src/lib.rs`

**Clippy warnings fixed:**
1. `clippy::needless-borrows-for-generic-args` — Removed `&` before
   `env.current_contract_address()` in the `deposit()` function's `to`
   argument. The `token::Client::transfer` `to` parameter accepts
   `impl Into<MuxedAddress>`, not `&Address`, so the borrow was genuinely
   redundant and clippy's suggestion was correct.
2. `deprecated: Events::publish` (×4, all event-emit call sites) — Added
   `#![allow(deprecated)]` at file level with an explanatory comment.
   The proper fix is to migrate to the `#[contractevent]` macro (soroban-sdk
   v23+ recommendation), but that macro adds event structs to the contract
   spec (ABI), which constitutes an interface change. That migration is left
   for a dedicated PR so the change is explicitly reviewed and attributed.
   Using `#[allow(deprecated)]` here is intentional, not a silence-by-default.

**rustfmt diffs fixed (8 diffs, all in `examples/escrow/src/lib.rs`):**
- Method chain formatting: `env.storage().instance().set(...)` and
  `.get(...).expect(...)` chains that exceeded the line-length threshold
  reformatted to break after each `.`
- Two `deposit()` call sites in tests that rustfmt wanted on a single line
  (within the column limit) collapsed from multi-line to one-liner

**Why:** CI was failing on Format and Clippy jobs while Test and Build were
passing. This is a pure lint/formatting fix with zero behavior change.

---
