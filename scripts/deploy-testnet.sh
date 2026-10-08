#!/usr/bin/env bash
set -euo pipefail

echo "=========================================================="
echo "Soroban Pharma RxProof: Deploy to Stellar Testnet (Bash)"
echo "=========================================================="

NETWORK="testnet"
RPC_URL="https://soroban-testnet.stellar.org"
NETWORK_PASSPHRASE="Test SDF Network ; September 2015"

echo "[1/5] Ensuring deployer identity exists on testnet..."
stellar keys generate deployer --network "$NETWORK" --fund || true
DEPLOYER_ADDR=$(stellar keys address deployer | tr -d '\r\n')
echo "Deployer Address: $DEPLOYER_ADDR"

echo "[2/5] Building contract WASM..."
if command -v stellar &> /dev/null; then
  stellar contract build || cargo build --target wasm32-unknown-unknown --release
else
  cargo build --target wasm32-unknown-unknown --release
fi

WASM_PATH="target/wasm32v1-none/release/rxproof_contracts.wasm"
if [ ! -f "$WASM_PATH" ]; then
  WASM_PATH="target/wasm32-unknown-unknown/release/rxproof_contracts.wasm"
fi

echo "WASM ready at: $WASM_PATH"

echo "[3/5] Deploying contract to testnet..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$WASM_PATH" \
  --source deployer \
  --network "$NETWORK" \
  --alias rxproof | tr -d '\r\n')

echo "Contract deployed: $CONTRACT_ID"

echo "[4/5] Initializing RxProof contract with admin..."
stellar contract invoke \
  --id rxproof \
  --source deployer \
  --network "$NETWORK" \
  -- \
  initialize \
  --admin "$DEPLOYER_ADDR"

echo "[5/5] Writing deployment details to deployments/testnet.json..."
mkdir -p deployments
cat <<EOF > deployments/testnet.json
{
  "network": "$NETWORK",
  "rpcUrl": "$RPC_URL",
  "networkPassphrase": "$NETWORK_PASSPHRASE",
  "contractId": "$CONTRACT_ID",
  "contractAlias": "rxproof",
  "admin": "$DEPLOYER_ADDR",
  "wasmPath": "$WASM_PATH",
  "deployedAt": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
}
EOF

echo "Deployment complete! Saved to deployments/testnet.json"
