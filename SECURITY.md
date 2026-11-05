# Security Policy

## Responsible Disclosure

The **Soroban Pharma RxProof** consortium takes security and the integrity of pharmaceutical provenance with the highest seriousness. Counterfeit or tampered drug verification flaws directly impact patient safety and public health.

If you believe you have discovered a security vulnerability in our smart contracts, cryptographic proof engines, or access control mechanisms, please report it immediately.

**Please DO NOT open a public GitHub issue.**

### Contact
Send a detailed vulnerability report via email to:
- **Email:** `security@rxproof.pharma` or contact the core maintainers via private communication channels.

Please include:
1. Target contract version and commit SHA.
2. Proof of concept (PoC) code or reproducible test script.
3. Detailed impact assessment (e.g., unauthorized custody transfer, fake proof bypass, serial replay, denial-of-service).
4. Any suggested remediations.

We will acknowledge receipt within **24 hours** and provide regular status updates.

---

## Scope & Threat Model

### In Scope
- **Smart Contract Logic**: `src/lib.rs`, `src/auth.rs`, `src/storage.rs`, `src/types.rs`.
- **Cryptographic Algorithms**: `src/crypto.rs` (sorted-pair Merkle verification, depth cap bypass, second-preimage attacks).
- **Access Control & Multi-sig**: Unauthorized regulator onboarding, proposal threshold bypass, custody manipulation.
- **Supply Chain Invariants**: Serial cloning prevention, bitmask tampering, batch quantity over-dispense.

### Out of Scope
- Attacks requiring physical device compromise.
- Phishing or social engineering targeting individual private keys.
- Network-level outages or consensus halts of the public Stellar network.

---

## Security Guarantees & Assumptions

1. **Burn-on-Dispense Invariant**:
   Once a pack serial hash is dispensed, it is irrevocably marked `SerialDispensed`. Any subsequent verification request will return `VerificationResult::Suspicious`.
2. **Deterministic Merkle Proofs**:
   Leaf hashing and internal nodes enforce strict lexicographical sorting ($H(\min(A, B) \parallel \max(A, B))$) to eliminate proof malleability.
3. **Consensus Governance**:
   Regulator removal requires strict majority consensus ($\lfloor N / 2 \rfloor + 1$) among active oversight authorities, protecting against rogue single-key takeover.
