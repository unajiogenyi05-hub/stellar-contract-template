# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-09-02

### Added
- `contracts/hello-world/` — Simple greeting contract boilerplate demonstrating basic Soroban contract structure
- `contracts/counter/` — Stateful counter contract with `increment`, `get`, and `reset` functions, instance storage, TTL extension, and full unit tests
- `.github/workflows/ci.yml` — GitHub Actions CI pipeline with four jobs: `fmt` (rustfmt), `clippy` (wasm32v1-none target), `test` (cargo test), and `build` (WASM artifact upload)
- `scripts/deploy.sh` — Helper script for deploying a contract to testnet or mainnet
- `scripts/fund.sh` — Helper script for funding an account via Friendbot on testnet
- `scripts/invoke.sh` — Helper script for invoking a deployed contract function
- `.devcontainer/` — Dev container configuration for zero-setup onboarding via VS Code or GitHub Codespaces
- `.env.example` — Environment variable template for network and keypair configuration
- `Makefile` — Common commands: `build`, `test`, `fmt`, `lint`, `deploy-testnet`, `deploy-mainnet`, `fund`, `clean`
- `CONTRIBUTING.md` — Contribution guide covering prerequisites, adding new contracts, and PR process
- `LICENSE` — MIT license

[Unreleased]: https://github.com/unajiogenyi05-hub/stellar-contract-template/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/unajiogenyi05-hub/stellar-contract-template/releases/tag/v0.1.0
