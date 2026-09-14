#!/bin/bash
# Deploy StellarWatch contracts to Stellar testnet
# Usage: ./scripts/deploy.sh <identity-name>

set -e

IDENTITY=${1:-deployer-account}
NETWORK=${NETWORK:-testnet}
RPC_URL=${RPC_URL:-https://soroban-testnet.stellar.org}
PASSPHRASE=${PASSPHRASE:-"Test SDF Network ; September 2015"}

echo "Deploying StellarWatch contracts..."
echo "Identity: $IDENTITY"
echo "Network: $NETWORK"
echo ""

# 1. Build all contracts
echo "[1/5] Building contracts..."
cargo build --target wasm32-unknown-unknown --release

# 2. Deploy contract-registry
echo "[2/5] Deploying contract-registry..."
REGISTRY_ID=$(stellar contract deploy \
  --source-account "$IDENTITY" \
  --wasm target/wasm32-unknown-unknown/release/contract_registry.wasm \
  --rpc-url "$RPC_URL" \
  --network-passphrase "$PASSPHRASE" 2>&1 | tail -1)
echo "contract-registry: $REGISTRY_ID"

# 3. Deploy health-registry
echo "[3/5] Deploying health-registry..."
HEALTH_ID=$(stellar contract deploy \
  --source-account "$IDENTITY" \
  --wasm target/wasm32-unknown-unknown/release/health_registry.wasm \
  --rpc-url "$RPC_URL" \
  --network-passphrase "$PASSPHRASE" 2>&1 | tail -1)
echo "health-registry: $HEALTH_ID"

# 4. Deploy alert-rules
echo "[4/5] Deploying alert-rules..."
ALERT_ID=$(stellar contract deploy \
  --source-account "$IDENTITY" \
  --wasm target/wasm32-unknown-unknown/release/alert_rules.wasm \
  --rpc-url "$RPC_URL" \
  --network-passphrase "$PASSPHRASE" 2>&1 | tail -1)
echo "alert-rules: $ALERT_ID"

# 5. Print summary
echo "[5/5] Done."
echo ""
echo "==================================================="
echo "Contract IDs"
echo "==================================================="
echo "REGISTRY_ID=$REGISTRY_ID"
echo "HEALTH_ID=$HEALTH_ID"
echo "ALERT_ID=$ALERT_ID"
echo "==================================================="
echo ""
echo "Save these IDs. You will need them for the app configuration."
