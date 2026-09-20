# Escrow — Example Contract

This is a working Soroban contract built from this template. It demonstrates
how to build a real, useful contract using the scaffold provided.

## What it does

A trustless token escrow between three parties:

- **Depositor** — locks tokens into the contract
- **Recipient** — receives tokens if the arbiter approves
- **Arbiter**   — decides to approve (release to recipient) or refund (return
  to depositor)

The depositor can also cancel at any time before the arbiter acts.

## Contract lifecycle

```
deposit() → Pending
    │
    ├── arbiter.approve() → Approved  (tokens → recipient)
    ├── arbiter.refund()  → Refunded  (tokens → depositor)
    └── depositor.cancel() → Cancelled (tokens → depositor)
```

## Use cases

- **Freelance payment** — client deposits, arbiter (e.g. a DAO committee)
  approves when work is delivered
- **OTC trade** — party A deposits Token X; a separate OTC contract handles
  Token Y; arbiter confirms both sides before releasing
- **Grant disbursement** — foundation deposits grant funds; arbiter (grants
  committee) approves milestone delivery

## Build and test

```bash
# From the repo root
cargo test -p escrow

# Build WASM
stellar contract build
```

## Deploy to testnet

```bash
# Fund an account
stellar keys generate my-key --network testnet --fund
export DEPLOYER=$(stellar keys address my-key)

# Deploy
export ESCROW_ID=$(stellar contract deploy \
  --wasm target/wasm32v1-none/release/escrow.wasm \
  --source my-key \
  --network testnet)
echo "Escrow contract: $ESCROW_ID"

# Deposit (replace addresses and token ID)
stellar contract invoke \
  --id "$ESCROW_ID" \
  --source my-key \
  --network testnet \
  -- deposit \
  --depositor "$DEPOSITOR_ADDR" \
  --recipient "$RECIPIENT_ADDR" \
  --arbiter "$ARBITER_ADDR" \
  --token_address "$TOKEN_ID" \
  --amount 100000

# Approve (arbiter)
stellar contract invoke \
  --id "$ESCROW_ID" \
  --source arbiter-key \
  --network testnet \
  -- approve
```

## How this was built from the template

1. Created `examples/escrow/` directory
2. Added `Cargo.toml` following the template's pattern (see root README)
3. Registered the package in the workspace `Cargo.toml`
4. Wrote the contract in `src/lib.rs`
5. Ran `cargo test -p escrow` — all tests pass
