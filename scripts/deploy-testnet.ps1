# Soroban Pharma RxProof - Testnet Deployment Script (PowerShell)
$ErrorActionPreference = "Stop"

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "Soroban Pharma RxProof: Deploy to Stellar Testnet" -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

$Network = "testnet"
$RpcUrl = "https://soroban-testnet.stellar.org"
$NetworkPassphrase = "Test SDF Network ; September 2015"

# 1. Identity generation & funding via Friendbot
Write-Host "[1/5] Ensuring deployer identity exists on testnet..." -ForegroundColor Yellow
try {
    stellar keys generate deployer --network $Network --fund
} catch {
    Write-Host "Deployer key exists or friendbot funded: $($_.Exception.Message)"
}
$DeployerAddr = (stellar keys address deployer).Trim()
Write-Host "Deployer Address: $DeployerAddr" -ForegroundColor Green

# 2. Build WASM contract
Write-Host "[2/5] Building contract WASM..." -ForegroundColor Yellow
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
    Write-Error "Error: Contract WASM not found at $WasmPath"
    exit 1
}
Write-Host "WASM ready at: $WasmPath" -ForegroundColor Green

# 3. Deploy contract
Write-Host "[3/5] Deploying contract to testnet..." -ForegroundColor Yellow
$ContractId = (stellar contract deploy `
    --wasm $WasmPath `
    --source deployer `
    --network $Network `
    --alias rxproof).Trim()

Write-Host "Contract deployed: $ContractId" -ForegroundColor Green

# 4. Initialize contract
Write-Host "[4/5] Initializing RxProof contract with admin..." -ForegroundColor Yellow
stellar contract invoke `
    --id rxproof `
    --source deployer `
    --network $Network `
    -- initialize `
    --admin $DeployerAddr

# 5. Save deployment metadata
Write-Host "[5/5] Writing deployment details to deployments/testnet.json..." -ForegroundColor Yellow
if (-not (Test-Path "deployments")) {
    New-Item -ItemType Directory -Path "deployments" | Out-Null
}

$DeploymentData = @{
    network = $Network
    rpcUrl = $RpcUrl
    networkPassphrase = $NetworkPassphrase
    contractId = $ContractId
    contractAlias = "rxproof"
    admin = $DeployerAddr
    wasmPath = $WasmPath
    deployedAt = (Get-Date -AsUTC -Format "yyyy-MM-ddTHH:mm:ssZ")
}

$DeploymentData | ConvertTo-Json -Depth 4 | Set-Content "deployments/testnet.json"
Write-Host "Deployment complete! Saved to deployments/testnet.json" -ForegroundColor Green
