# Contributing to stellar-contract-template

Thanks for your interest in contributing! Here's how to get started.

---

## Development setup

1. Fork the repo and clone it:

   ```bash
   git clone https://github.com/unajiogenyi05-hub/stellar-contract-template
   cd stellar-contract-template
   ```

2. Install dependencies:

   ```bash
   make setup
   ```

3. Copy and configure environment:

   ```bash
   cp .env.example .env
   # Set SOURCE_ACCOUNT to a testnet secret key
   ```

4. Verify everything works:

   ```bash
   make build
   make test
   make lint
   ```

---

## Workflow

- **Branch** off `main` for every change: `git checkout -b feat/my-feature`
- **Commit** small, focused changes with clear messages
- **Test** your changes: `make test`
- **Lint** before pushing: `make lint`
- **Open a PR** against `main` with a description of what changed and why

---

## Guidelines

- Keep contracts minimal and well-commented — this is a template others learn from
- Every contract must have unit tests
- Don't break existing examples without a good reason
- Scripts must work on Linux and macOS
- CI must pass before a PR is merged

---

## Reporting issues

Use the issue templates when filing a bug or requesting a feature. Include:

- Steps to reproduce (for bugs)
- Expected vs actual behavior
- Your OS, Rust version, and stellar-cli version

---

## Questions

Open a Discussion on GitHub or file an issue with the `question` label.
