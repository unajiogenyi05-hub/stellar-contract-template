#!/usr/bin/env bash
# invoke.sh — Invoke a function on a deployed Soroban contract
#
# Usage:
#   ./scripts/invoke.sh <contract-id> <function> [--network testnet|mainnet] [-- [args...]]
#
# Examples:
#   ./scripts/invoke.sh CXXX... hello --network testnet -- --to Dev
#   ./scripts/invoke.sh CXXX... increment --network testnet
#   ./scripts/invoke.sh CXXX... get --network testnet

set -euo pipefail

CONTRACT_ID="${1:-}"
FUNCTION_NAME="${2:-}"

if [[ -z "$CONTRACT_ID" || -z "$FUNCTION_NAME" ]]; then
    echo "Usage: $0 <contract-id> <function> [--network testnet|mainnet] [-- [args...]]"
    exit 1
fi

shift 2

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

if [[ -f "$ROOT_DIR/.env" ]]; then
    source "$ROOT_DIR/.env"
else
    echo "Error: .env file not found. Copy .env.example to .env and fill in your values."
    exit 1
fi

NETWORK="testnet"
REMAINING_ARGS=()

while [[ $# -gt 0 ]]; do
    case "$1" in
        --network)
            NETWORK="$2"
            shift 2
            ;;
        --)
            shift
            REMAINING_ARGS+=("$@")
            break
            ;;
        *)
            REMAINING_ARGS+=("$1")
            shift
            ;;
    esac
done

case "$NETWORK" in
    testnet)
        RPC_URL="${TESTNET_RPC_URL}"
        PASSPHRASE="${TESTNET_NETWORK_PASSPHRASE}"
        ;;
    mainnet)
        RPC_URL="${MAINNET_RPC_URL}"
        PASSPHRASE="${MAINNET_NETWORK_PASSPHRASE}"
        ;;
    *)
        echo "Unknown network: $NETWORK. Use 'testnet' or 'mainnet'."
        exit 1
        ;;
esac

echo "Invoking $FUNCTION_NAME on $CONTRACT_ID ($NETWORK)..."

stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source "$SOURCE_ACCOUNT" \
    --rpc-url "$RPC_URL" \
    --network-passphrase "$PASSPHRASE" \
    -- "$FUNCTION_NAME" "${REMAINING_ARGS[@]+"${REMAINING_ARGS[@]}"}"
