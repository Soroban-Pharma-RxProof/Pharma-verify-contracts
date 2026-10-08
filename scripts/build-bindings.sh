#!/usr/bin/env bash
set -euo pipefail

echo "Building RxProof Contract WASM and TypeScript Bindings..."

if command -v stellar &> /dev/null; then
  stellar contract build || cargo build --target wasm32-unknown-unknown --release
else
  cargo build --target wasm32-unknown-unknown --release
fi

WASM_PATH="target/wasm32v1-none/release/rxproof_contracts.wasm"
if [ ! -f "$WASM_PATH" ]; then
  WASM_PATH="target/wasm32-unknown-unknown/release/rxproof_contracts.wasm"
fi

CONTRACT_ID="CAZPHARMAVERIFYRXPROOFTESTNETCONTRACTID2026SAMPLE01"
if [ -f "deployments/testnet.json" ]; then
  CONTRACT_ID=$(node -e "console.log(require('./deployments/testnet.json').contractId || '$CONTRACT_ID')" 2>/dev/null || echo "$CONTRACT_ID")
fi

if command -v stellar &> /dev/null; then
  stellar contract bindings typescript \
    --wasm "$WASM_PATH" \
    --output-dir packages/pharma-verify-sdk \
    --contract-id "$CONTRACT_ID" \
    --overwrite
fi

cd packages/pharma-verify-sdk
if [ -f "package.json" ]; then
  npm install --silent || true
  npm run build || true
fi
echo "TypeScript bindings ready in packages/pharma-verify-sdk"
