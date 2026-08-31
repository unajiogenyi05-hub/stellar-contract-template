.PHONY: all build test clean fmt lint deploy-testnet deploy-mainnet setup fund help

# ── Configuration ──────────────────────────────────────────────────────────────
include .env
export

CONTRACTS  := hello-world counter
WASM_DIR   := target/wasm32-unknown-unknown/release

# ── Default ────────────────────────────────────────────────────────────────────
all: build

# ── Setup ──────────────────────────────────────────────────────────────────────
setup:
	@echo "Installing Rust wasm32 target..."
	rustup target add wasm32-unknown-unknown
	@echo "Installing stellar-cli..."
	cargo install --locked stellar-cli --features opt
	@echo "Setup complete."

# ── Build ──────────────────────────────────────────────────────────────────────
build:
	@echo "Building all contracts..."
	stellar contract build
	@echo "Build complete. WASMs in $(WASM_DIR)/"

build-optimized:
	@echo "Building optimized contracts..."
	stellar contract build --profile release
	@for contract in $(CONTRACTS); do \
		stellar contract optimize \
			--wasm $(WASM_DIR)/$${contract//-/_}.wasm; \
	done
	@echo "Optimized build complete."

# ── Test ───────────────────────────────────────────────────────────────────────
test:
	@echo "Running all tests..."
	cargo test --workspace

test-verbose:
	cargo test --workspace -- --nocapture

# ── Lint / Format ──────────────────────────────────────────────────────────────
fmt:
	cargo fmt --all

lint:
	cargo fmt --all -- --check
	cargo clippy --workspace --target wasm32-unknown-unknown -- -D warnings

# ── Clean ──────────────────────────────────────────────────────────────────────
clean:
	cargo clean

# ── Deploy ─────────────────────────────────────────────────────────────────────
deploy-testnet:
	@echo "Deploying to testnet..."
	@$(MAKE) _deploy NETWORK=testnet RPC_URL=$(TESTNET_RPC_URL) PASSPHRASE="$(TESTNET_NETWORK_PASSPHRASE)"

deploy-mainnet:
	@echo "⚠  Deploying to MAINNET. Press Ctrl-C within 5s to abort."
	@sleep 5
	@$(MAKE) _deploy NETWORK=mainnet RPC_URL=$(MAINNET_RPC_URL) PASSPHRASE="$(MAINNET_NETWORK_PASSPHRASE)"

_deploy:
	@for contract in $(CONTRACTS); do \
		echo "Deploying $$contract to $(NETWORK)..."; \
		stellar contract deploy \
			--wasm $(WASM_DIR)/$${contract//-/_}.wasm \
			--source $(SOURCE_ACCOUNT) \
			--rpc-url $(RPC_URL) \
			--network-passphrase "$(PASSPHRASE)"; \
	done

# ── Fund testnet account ───────────────────────────────────────────────────────
fund:
	stellar keys fund $(SOURCE_ACCOUNT) --network testnet

# ── Help ───────────────────────────────────────────────────────────────────────
help:
	@echo ""
	@echo "  make setup            Install dependencies (Rust target + stellar-cli)"
	@echo "  make build            Build all contracts"
	@echo "  make build-optimized  Build and optimize WASM sizes"
	@echo "  make test             Run all unit tests"
	@echo "  make fmt              Format code"
	@echo "  make lint             Check formatting and clippy"
	@echo "  make deploy-testnet   Deploy contracts to testnet"
	@echo "  make deploy-mainnet   Deploy contracts to mainnet"
	@echo "  make fund             Fund SOURCE_ACCOUNT on testnet via friendbot"
	@echo "  make clean            Remove build artifacts"
	@echo ""
