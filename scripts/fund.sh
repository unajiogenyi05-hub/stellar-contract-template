#!/usr/bin/env bash
# fund.sh — Fund a Stellar account on testnet via Friendbot
#
# Usage:
#   ./scripts/fund.sh [account]
#
# If no account is given, SOURCE_ACCOUNT from .env is used.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

if [[ -f "$ROOT_DIR/.env" ]]; then
    source "$ROOT_DIR/.env"
else
    echo "Error: .env file not found. Copy .env.example to .env and fill in your values."
    exit 1
fi

ACCOUNT="${1:-${SOURCE_ACCOUNT:-}}"

if [[ -z "$ACCOUNT" ]]; then
    echo "Error: No account specified and SOURCE_ACCOUNT is not set in .env"
    echo "Usage: $0 [account]"
    exit 1
fi

echo "Funding account on testnet via Friendbot..."
echo "  Account: $ACCOUNT"

stellar keys fund "$ACCOUNT" --network testnet

echo ""
echo "✅ Account funded successfully on testnet."
