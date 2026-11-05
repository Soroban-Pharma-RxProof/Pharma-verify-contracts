# Contributing to Soroban Pharma RxProof Smart Contracts

Thank you for your interest in contributing to the **Soroban Pharma RxProof Smart Contracts** project. This repository secures pharmaceutical provenance on Stellar/Soroban (Protocol 22), providing cryptographic verification against counterfeit medicines and unauthorized supply chain diversion.

---

## Code of Conduct

We are committed to providing a welcoming and harassment-free experience for everyone. Please be respectful, constructive, and collaborative in all discussions and code reviews.

---

## Development Prerequisites

Before contributing, ensure you have installed:
- **Rust Toolchain**: Stable channel (`>= 1.80.0`, pinned via `rust-toolchain.toml` to `1.81.0`).
- **WASM Target**: `wasm32-unknown-unknown`
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- **Stellar CLI**: (`>= 22.0.0`)
  ```bash
  cargo install --locked stellar-cli --features opt
  ```
- **Linters**:
  ```bash
  rustup component add clippy rustfmt
  ```

---

## Local Development Workflow

### 1. Fork & Clone
```bash
git clone https://github.com/<your-username>/Pharma-verify-contracts.git
cd Pharma-verify-contracts
```

### 2. Running Unit & Integration Tests
```bash
cargo test
```
The test suite executes comprehensive scenarios:
- Contract initialization and admin assignment
- Multi-signature regulator removal consensus threshold
- Emergency circuit breaker (pause/unpause)
- Participant licensing lifecycle
- Batch registration with sorted-pair Merkle root
- Supply chain custody transitions (`Manufacturer -> Distributor -> Pharmacy`)
- Pack burn on dispense and blister strip bitmask tracking
- Public counterfeit report submission

### 3. Formatting & Linting
Ensure all code adheres strictly to rustfmt and clippy standards before committing:
```bash
# Check formatting
cargo fmt --all -- --check

# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets -- -D warnings
```

### 4. Building the WASM Bytecode
```bash
cargo build --target wasm32-unknown-unknown --release
```
Optimized WASM output will be generated at `target/wasm32-unknown-unknown/release/rxproof_contracts.wasm`.

---

## Architecture & Code Conventions

1. **`no_std` Compliance**:
   All contract logic in `src/` must remain `#![no_std]`. Unit tests (`src/test.rs`) run with the host test harness.
2. **Deterministic Cryptography**:
   All Merkle proof nodes must be generated using `hash_sorted_pair` ($H(\min(A,B) \parallel \max(A,B))$) to prevent proof malleability.
3. **State Archival & Rent**:
   Ensure all persistent storage access patterns bump TTL using `bump_persistent` and `bump_instance` defined in `src/storage.rs`.
4. **Structured Event Logging**:
   Every state mutation must emit a standardized contract event defined in `src/events.rs`.

---

## Submitting Pull Requests

1. Create a feature branch from `main`:
   ```bash
   git checkout -b feat/your-feature-name
   ```
2. Commit your changes following [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat(contracts): ...`
   - `fix(contracts): ...`
   - `test(contracts): ...`
   - `docs(contracts): ...`
3. Verify that `cargo test`, `cargo fmt`, and `cargo clippy` pass cleanly.
4. Open a Pull Request referencing any related issues. Fill out the PR checklist completely.
