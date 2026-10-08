# Soroban Pharma RxProof - TypeScript Bindings Generator (PowerShell)
$ErrorActionPreference = "Stop"

Write-Host "Building RxProof Contract WASM and TypeScript Bindings..." -ForegroundColor Cyan

# 1. Build WASM
try {
    stellar contract build
} catch {
    cargo build --target wasm32-unknown-unknown --release
}

$WasmPath = "target/wasm32v1-none/release/rxproof_contracts.wasm"
if (-not (Test-Path $WasmPath)) {
    $WasmPath = "target/wasm32-unknown-unknown/release/rxproof_contracts.wasm"
}

if (-not (Test-Path $WasmPath)) {
    Write-Error "Error: WASM not found at $WasmPath"
    exit 1
}

$ContractId = "CAZPHARMAVERIFYRXPROOFTESTNETCONTRACTID2026SAMPLE01"
if (Test-Path "deployments/testnet.json") {
    $DeployData = Get-Content "deployments/testnet.json" | ConvertFrom-Json
    if ($DeployData.contractId) {
        $ContractId = $DeployData.contractId
    }
}

# 2. Generate TypeScript bindings
Write-Host "Generating bindings with stellar CLI for contract $ContractId..." -ForegroundColor Yellow
stellar contract bindings typescript `
    --wasm $WasmPath `
    --output-dir packages/pharma-verify-sdk `
    --contract-id $ContractId `
    --overwrite

Write-Host "Bindings generated successfully into packages/pharma-verify-sdk" -ForegroundColor Green
