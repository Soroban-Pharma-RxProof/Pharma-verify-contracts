
use soroban_sdk::{Bytes, BytesN, Env, Vec};
use crate::types::Error;

pub const MAX_PROOF_DEPTH: u32 = 32;

/// Compute SHA-256 hash of two 32-byte hashes sorted lexicographically.
/// This prevents proof malleability and second-preimage attacks.
pub fn hash_sorted_pair(env: &Env, a: &BytesN<32>, b: &BytesN<32>) -> BytesN<32> {
    let a_arr = a.to_array();
    let b_arr = b.to_array();

    let (first, second) = if a_arr <= b_arr {
        (a_arr, b_arr)
    } else {
        (b_arr, a_arr)
    };

    let mut combined = [0u8; 64];
    let mut i = 0;
    while i < 32 {
        combined[i] = first[i];
        combined[i + 32] = second[i];
        i += 1;
    }

    let bytes = Bytes::from_array(env, &combined);
    env.crypto().sha256(&bytes).into()
}

/// Verify a Merkle inclusion proof against a known root with sorted-pair hashing
/// and depth cap enforcement.
pub fn verify_merkle_proof(
    env: &Env,
    leaf: &BytesN<32>,
    proof: &Vec<BytesN<32>>,
    root: &BytesN<32>,
) -> Result<bool, Error> {
    if proof.len() > MAX_PROOF_DEPTH {
        return Err(Error::ProofDepthExceeded);
    }

    let mut current = leaf.clone();
    for sibling in proof.iter() {
        current = hash_sorted_pair(env, &current, &sibling);
    }

    Ok(current == *root)
}

