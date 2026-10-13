#!/bin/bash
# Deploy StellarWatch contracts to Stellar testnet with verification
# Usage: ./scripts/deploy.sh <identity-name>

set -e

IDENTITY=${1:-deployer-account}
NETWORK=${NETWORK:-testnet}
RPC_URL=${RPC_URL:-https://soroban-testnet.stellar.org}
PASSPHRASE=${PASSPHRASE:-"Test SDF Network ; September 2015"}

SOURCE_ADDRESS=$(stellar keys address "$IDENTITY")

echo "Deploying StellarWatch contracts..."
echo "Identity: $IDENTITY"
echo "Address:  $SOURCE_ADDRESS"
echo "Network:  $NETWORK"
echo ""

# 1. Build all contracts
echo "[1/5] Building contracts with Rust 1.88 + wasm32v1-none..."
cargo +1.88 build --target wasm32v1-none --release

# 2. Deploy contract-registry
echo "[2/5] Deploying contract-registry..."
REGISTRY_ID=$(stellar contract deploy \
  --source-account "$IDENTITY" \
  --wasm target/wasm32v1-none/release/contract_registry.wasm \
  --rpc-url "$RPC_URL" \
  --network-passphrase "$PASSPHRASE" 2>&1 | tail -1)
echo "contract-registry: $REGISTRY_ID"

# 3. Deploy health-registry
echo "[3/5] Deploying health-registry..."
HEALTH_ID=$(stellar contract deploy \
  --source-account "$IDENTITY" \
  --wasm target/wasm32v1-none/release/health_registry.wasm \
  --rpc-url "$RPC_URL" \
  --network-passphrase "$PASSPHRASE" 2>&1 | tail -1)
echo "health-registry: $HEALTH_ID"

# 4. Deploy alert-rules
echo "[4/5] Deploying alert-rules..."
ALERT_ID=$(stellar contract deploy \
  --source-account "$IDENTITY" \
  --wasm target/wasm32v1-none/release/alert_rules.wasm \
  --rpc-url "$RPC_URL" \
  --network-passphrase "$PASSPHRASE" 2>&1 | tail -1)
echo "alert-rules: $ALERT_ID"

# 5. Verify each deployment by calling a read function
echo "[5/5] Verifying deployments..."

echo -n "  Verifying contract-registry... "
REGISTRY_OWNER=$(stellar contract invoke \
  --network "$NETWORK" \
  --source-account "$IDENTITY" \
  --id "$REGISTRY_ID" \
  -- get_owner 2>&1 | tail -1)
if [ "$REGISTRY_OWNER" = "\"$SOURCE_ADDRESS\"" ] || echo "$REGISTRY_OWNER" | grep -q "$SOURCE_ADDRESS"; then
  echo "OK"
else
  echo "FAILED"
  exit 1
fi

echo -n "  Verifying health-registry... "
HEALTH_OWNER=$(stellar contract invoke \
  --network "$NETWORK" \
  --source-account "$IDENTITY" \
  --id "$HEALTH_ID" \
  -- get_owner 2>&1 | tail -1)
if echo "$HEALTH_OWNER" | grep -q "$SOURCE_ADDRESS"; then
  echo "OK"
else
  echo "FAILED"
  exit 1
fi

echo -n "  Verifying alert-rules... "
ALERT_OWNER=$(stellar contract invoke \
  --network "$NETWORK" \
  --source-account "$IDENTITY" \
  --id "$ALERT_ID" \
  -- get_owner 2>&1 | tail -1)
if echo "$ALERT_OWNER" | grep -q "$SOURCE_ADDRESS"; then
  echo "OK"
else
  echo "FAILED"
  exit 1
fi

echo ""
echo "==================================================="
echo "Deployment complete. Contract IDs:"
echo "==================================================="
echo "CONTRACT_REGISTRY_ID=$REGISTRY_ID"
echo "HEALTH_REGISTRY_ID=$HEALTH_ID"
echo "ALERT_RULES_ID=$ALERT_ID"
echo "PUBLIC_KEY=$SOURCE_ADDRESS"
echo "==================================================="
