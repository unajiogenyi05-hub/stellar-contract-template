# stellar-contract-template

A production-ready template for building and deploying [Soroban](https://soroban.stellar.org) smart contracts on the Stellar network.

[![CI](https://github.com/unajiogenyi05-hub/stellar-contract-template/actions/workflows/ci.yml/badge.svg)](https://github.com/unajiogenyi05-hub/stellar-contract-template/actions/workflows/ci.yml)

---

## What's included

| Path | Description |
|------|-------------|
| `contracts/hello-world/` | Simple greeting contract — good starting point |
| `contracts/counter/` | Stateful counter with increment / reset |
| `scripts/` | Deploy, invoke, and fund helper scripts |
| `.github/workflows/ci.yml` | CI: fmt, clippy, test, build on every push/PR |
| `.devcontainer/` | One-click dev environment for VS Code / Codespaces |
| `Makefile` | Common commands (`build`, `test`, `deploy-testnet`, …) |

---

## Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Rust | stable | [rustup.rs](https://rustup.rs) |
| wasm32 target | — | `rustup target add wasm32v1-none` |
| stellar-cli | latest | `cargo install --locked stellar-cli --features opt` |

---

## Quick start

```bash
# 1. Clone
git clone https://github.com/unajiogenyi05-hub/stellar-contract-template
cd stellar-contract-template

# 2. Configure
cp .env.example .env
# Edit .env — set SOURCE_ACCOUNT to your Stellar secret key

# 3. Build
make build

# 4. Test
make test

# 5. Deploy to testnet
make fund              # fund your account via Friendbot (testnet only)
make deploy-testnet
```

That's it. Contract IDs are saved to `deployments.json`.

---

## Open in Codespaces

Click **Code → Codespaces → Create codespace on main** and the dev container will automatically install all dependencies. Skip to step 2 above.

---

## Project structure

```
stellar-contract-template/
├── contracts/
│   ├── hello-world/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── counter/
│       ├── Cargo.toml
│       └── src/lib.rs
├── scripts/
│   ├── deploy.sh       # deploy a single contract
│   ├── invoke.sh       # call a contract function
│   └── fund.sh         # fund an account via Friendbot
├── .devcontainer/
│   ├── devcontainer.json
│   └── setup.sh
├── .github/
│   └── workflows/
│       └── ci.yml
├── .env.example
├── .gitignore
├── Cargo.toml          # workspace root
└── Makefile
```

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

3. Add `contracts/my-contract/src/lib.rs` with your contract logic.

4. Register it in the workspace `Cargo.toml`:

   ```toml
   members = [
       "contracts/hello-world",
       "contracts/counter",
       "contracts/my-contract",   # ← add this
   ]
   ```

5. Add the contract name to the `CONTRACTS` list in `Makefile`.

---

## Deploying a single contract

```bash
./scripts/deploy.sh hello-world testnet
./scripts/deploy.sh counter testnet
```

## Invoking a contract function

```bash
# hello-world: pass --to as an argument after the separator (--)
./scripts/invoke.sh <CONTRACT_ID> hello --network testnet -- --to Dev

# counter: increment
./scripts/invoke.sh <CONTRACT_ID> increment --network testnet

# counter: get current value
./scripts/invoke.sh <CONTRACT_ID> get --network testnet
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
