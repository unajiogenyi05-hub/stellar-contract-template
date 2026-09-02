# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅ Yes    |

This template targets **`soroban-sdk = "27.0.6"`**. Security fixes will be backported to the latest minor version only.

> **Important:** This repository contains **boilerplate template code**. It has not been formally audited. You must conduct your own security review before deploying any contract derived from this template to mainnet.

---

## Reporting a Vulnerability

Please **do not** open a public GitHub issue for security vulnerabilities.

Use GitHub's built-in private vulnerability reporting:
1. Go to the [Security tab](https://github.com/unajiogenyi05-hub/stellar-contract-template/security)
2. Click **"Report a vulnerability"**
3. Fill in the details

Alternatively, email the maintainer directly via the contact listed on the GitHub profile.

### Response SLA

| Stage | Target |
|-------|--------|
| Acknowledgement | Within 48 hours |
| Initial assessment | Within 5 business days |
| Patch / mitigation | Within 14 days for critical issues |

---

## Scope

### In scope

The following Soroban-specific risk areas are in scope for this template:

- **Missing `require_auth()` / authorization bypass** — functions that should require caller authentication but do not
- **Storage key collisions** — `DataKey` enum variants that could unintentionally overwrite each other in instance or persistent storage
- **Integer overflow in token arithmetic** — unchecked arithmetic on `i128` amounts that could cause overflow or underflow
- **TTL / ledger expiry edge cases** — storage entries that expire before they should, causing contract state loss
- **Re-entrancy patterns** — cross-contract call sequences that could be exploited in the Soroban execution model
- **Unsafe use of `#[allow(deprecated)]` or `#[allow(clippy::...)]`** — suppressed lints that hide real issues

### Out of scope

- Issues that only affect the Stellar testnet and have no mainnet impact
- Publicly known Horizon API data exposure (all data queried is already public on-chain)
- Issues in `soroban-sdk` itself — please report those to the [Stellar security team](https://stellar.org/security)
- Social engineering or phishing attacks

---

## Security Best Practices for Template Users

When using this template as the basis for a production contract:

1. **Run `cargo audit`** before deploying — check for known CVEs in dependencies
2. **Enable all Clippy lints** — do not suppress warnings without understanding them
3. **Test with `mock_all_auths()`** carefully — ensure you also test auth failure paths
4. **Conduct a formal audit** for any contract handling real token value
5. **Use `extend_ttl` appropriately** — persistent storage entries expire; ensure long-lived state is extended

---

## Acknowledgements

We appreciate responsible disclosure from the security community. Reporters of valid vulnerabilities will be credited in the release notes (unless they prefer to remain anonymous).
