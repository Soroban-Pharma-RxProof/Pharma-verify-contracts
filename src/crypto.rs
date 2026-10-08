use soroban_sdk::{Bytes, BytesN, Env, Vec};
use crate::types::Error;

pub const MAX_PROOF_DEPTH: u32 = 32;

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
