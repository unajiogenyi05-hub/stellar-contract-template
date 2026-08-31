#!/usr/bin/env bash
# deploy.sh — Deploy a single Soroban contract to a Stellar network
#
# Usage:
#   ./scripts/deploy.sh <contract-name> [network]
#
# Examples:
#   ./scripts/deploy.sh hello-world testnet
#   ./scripts/deploy.sh counter mainnet

set -euo pipefail

CONTRACT_NAME="${1:-}"
NETWORK="${2:-testnet}"

if [[ -z "$CONTRACT_NAME" ]]; then
    echo "Usage: $0 <contract-name> [network]"
    echo "  contract-name: hello-world | counter"
    echo "  network:       testnet | mainnet  (default: testnet)"
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

if [[ -f "$ROOT_DIR/.env" ]]; then
    source "$ROOT_DIR/.env"
else
    echo "Error: .env file not found. Copy .env.example to .env and fill in your values."
    exit 1
fi

case "$NETWORK" in
    testnet)
        RPC_URL="${TESTNET_RPC_URL}"
        PASSPHRASE="${TESTNET_NETWORK_PASSPHRASE}"
        ;;
    mainnet)
        RPC_URL="${MAINNET_RPC_URL}"
        PASSPHRASE="${MAINNET_NETWORK_PASSPHRASE}"
        echo "⚠  Deploying to MAINNET. Press Ctrl-C within 5s to abort."
        sleep 5
        ;;
    *)
        echo "Unknown network: $NETWORK. Use 'testnet' or 'mainnet'."
        exit 1
        ;;
esac

WASM_NAME="${CONTRACT_NAME//-/_}"
WASM_PATH="$ROOT_DIR/target/wasm32-unknown-unknown/release/${WASM_NAME}.wasm"

if [[ ! -f "$WASM_PATH" ]]; then
    echo "WASM not found at $WASM_PATH"
    echo "Run 'make build' first."
    exit 1
fi

echo "Deploying $CONTRACT_NAME to $NETWORK..."
echo "  WASM: $WASM_PATH"
echo "  RPC:  $RPC_URL"

CONTRACT_ID=$(stellar contract deploy \
    --wasm "$WASM_PATH" \
    --source "$SOURCE_ACCOUNT" \
    --rpc-url "$RPC_URL" \
    --network-passphrase "$PASSPHRASE")

echo ""
echo "✅ Deployed $CONTRACT_NAME"
echo "   Contract ID: $CONTRACT_ID"
echo "   Network:     $NETWORK"

DEPLOYMENTS_FILE="$ROOT_DIR/deployments.json"
if command -v jq &>/dev/null; then
    if [[ ! -f "$DEPLOYMENTS_FILE" ]]; then
        echo "{}" > "$DEPLOYMENTS_FILE"
    fi
    TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    jq --arg net "$NETWORK" \
       --arg contract "$CONTRACT_NAME" \
       --arg id "$CONTRACT_ID" \
       --arg ts "$TIMESTAMP" \
       '.[$net][$contract] = {"contract_id": $id, "deployed_at": $ts}' \
       "$DEPLOYMENTS_FILE" > "$DEPLOYMENTS_FILE.tmp" && mv "$DEPLOYMENTS_FILE.tmp" "$DEPLOYMENTS_FILE"
    echo "   Saved to: deployments.json"
fi
