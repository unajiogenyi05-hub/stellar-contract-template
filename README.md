# stellar-contract-template

A production-ready template for building and deploying
[Soroban](https://soroban.stellar.org) smart contracts on the Stellar network.

[![CI](https://github.com/unajiogenyi05-hub/stellar-contract-template/actions/workflows/ci.yml/badge.svg)](https://github.com/unajiogenyi05-hub/stellar-contract-template/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## About this repo

This is a **Soroban smart contract template** — developer tooling, not a
product or end-user application. Its value is in reducing the time it takes
a Stellar/Soroban developer to go from zero to a compiling, tested, CI-passing
contract project.

To make the template's practical utility concrete, the repo also ships a
**real, fully-tested example contract**: `examples/escrow/` is a trustless
token escrow built entirely from this scaffold. It demonstrates a non-trivial
use case (three-party arbitrated escrow), passes all 9 unit tests, and can be
deployed to testnet with the included scripts.

If you are evaluating this repository: the claim is not that it is a
standalone application. The claim is that it is useful developer tooling for
the Soroban ecosystem, backed by a real, working example of what it produces.

---

## What this repo is

This repository serves two purposes:

1. **A scaffold** — a minimal, CI-ready starting point for any new Soroban
   contract project, with Rust workspace config, Makefile, dev container,
   deploy scripts, and GitHub Actions already wired up.

2. **A working example** — the `examples/escrow/` directory contains a real,
   fully-tested escrow contract built *from* this template. It shows what a
   non-trivial contract looks like in practice.

---

## Repository layout

```
stellar-contract-template/
├── contracts/
│   ├── hello-world/         # Starter: greeting contract
│   └── counter/             # Starter: stateful counter
├── examples/
│   └── escrow/              # Real example: trustless token escrow
│       ├── Cargo.toml
│       ├── src/lib.rs       # Contract + 9 unit tests
│       └── README.md        # How it was built from this template
├── scripts/
│   ├── deploy.sh            # Deploy a single contract
│   ├── invoke.sh            # Call a contract function
│   └── fund.sh              # Fund via Friendbot (testnet)
├── .devcontainer/           # VS Code / Codespaces dev environment
├── .github/workflows/ci.yml # CI: fmt + clippy + test + build
├── .env.example
├── Cargo.toml               # Workspace root
├── Makefile
├── CHANGELOG.md
├── SECURITY.md
├── CONTRIBUTING.md
└── EMMY_CHANGELOG.md
```

---

## The escrow example

`examples/escrow/` is a trustless token escrow with three roles:

- **Depositor** — locks tokens into the contract via `deposit()`
- **Recipient** — receives tokens when the arbiter calls `approve()`
- **Arbiter**   — decides to release (`approve`) or return (`refund`)
- **Depositor** can also `cancel()` before the arbiter acts

Full test suite covers: approve, refund, cancel, double-deposit, double-approve,
refund after approve, cancel after approve, zero-amount, and read helpers.

See [`examples/escrow/README.md`](examples/escrow/README.md) for deploy
instructions and use-case examples.

---

## Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Rust | stable | [rustup.rs](https://rustup.rs) |
| wasm32v1-none target | — | `rustup target add wasm32v1-none` |
| stellar-cli | latest | `curl -sSfL https://install.stellar.org \| sh` |

---

## Quick start

```bash
# 1. Clone
git clone https://github.com/unajiogenyi05-hub/stellar-contract-template
cd stellar-contract-template

# 2. Configure
cp .env.example .env
# Edit .env — set SOURCE_ACCOUNT to your Stellar secret key

# 3. Build everything (template contracts + escrow example)
make build

# 4. Test everything
make test

# 5. Deploy to testnet
make fund              # fund your account via Friendbot (testnet only)
make deploy-testnet
```

---

## Open in Codespaces

Click **Code → Codespaces → Create codespace on main** and the dev container
will automatically install all dependencies.

---

## Adding a new contract

1. Create the contract directory:

   ```bash
   mkdir -p contracts/my-contract/src
   ```

2. Add `contracts/my-contract/Cargo.toml`:

   ```toml
   [package]
   name = "my-contract"
   version = "0.1.0"
   edition = "2021"

   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   soroban-sdk = { workspace = true }

   [dev-dependencies]
   soroban-sdk = { workspace = true, features = ["testutils"] }
   ```

3. Write your contract in `contracts/my-contract/src/lib.rs`.

4. Register it in the workspace `Cargo.toml` members list.

5. Add the contract name to the `CONTRACTS` list in `Makefile`.

See [`examples/escrow/`](examples/escrow/) for a complete worked example of
this process.

---

## Available commands

```bash
make setup            # install Rust target + stellar-cli
make build            # build all contracts to WASM
make build-optimized  # build + optimize WASM sizes
make test             # run all unit tests
make fmt              # format code
make lint             # fmt check + clippy
make deploy-testnet   # deploy all contracts to testnet
make deploy-mainnet   # deploy all contracts to mainnet (5s warning)
make fund             # fund SOURCE_ACCOUNT on testnet
make clean            # remove build artifacts
```

---

## Deploying a single contract

```bash
./scripts/deploy.sh hello-world testnet
./scripts/deploy.sh escrow testnet
```

## Invoking a contract function

```bash
# escrow: deposit
./scripts/invoke.sh <ESCROW_ID> deposit --network testnet -- \
  --depositor <DEPOSITOR_ADDR> \
  --recipient <RECIPIENT_ADDR> \
  --arbiter <ARBITER_ADDR> \
  --token_address <TOKEN_ID> \
  --amount 100000

# counter: increment
./scripts/invoke.sh <CONTRACT_ID> increment --network testnet
```

---

## Network details

| Network | RPC URL |
|---------|---------|
| Testnet | https://soroban-testnet.stellar.org |
| Mainnet | https://soroban-mainnet.stellar.org |

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[MIT](LICENSE)
