## Description
Provide a concise explanation of the changes introduced by this pull request.

## Related Issues
Closes #(issue) / Relates to #(issue)

## Type of Change
- [ ] Bug fix (non-breaking change fixing an issue)
- [ ] New feature (non-breaking enhancement to contract logic or SDK)
- [ ] Breaking change (modifies storage layout, public interface, or error codes)
- [ ] Cryptographic or security improvement
- [ ] Documentation update

## Soroban Contract Verification Checklist
- [ ] All code strictly complies with `#![no_std]` in `src/`
- [ ] State mutations correctly invoke TTL bumps (`bump_persistent` / `bump_instance`)
- [ ] Sorted-pair Merkle invariants are preserved
- [ ] Events are properly emitted for state mutations
- [ ] `cargo test` passes locally without errors
- [ ] `cargo build --target wasm32-unknown-unknown --release` produces valid WASM
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes without warnings
