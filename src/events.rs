use soroban_sdk::{symbol_short, Address, BytesN, Env, Symbol};
use crate::types::ParticipantRole;

pub fn emit_batch_registered(
    env: &Env,
    batch_id: &BytesN<32>,
    manufacturer: &Address,
    merkle_root: &BytesN<32>,
    total_quantity: u32,
    expiry_timestamp: u64,
) {
    let topics = (symbol_short!("batch_reg"), batch_id.clone(), manufacturer.clone());
    env.events().publish(topics, (merkle_root.clone(), total_quantity, expiry_timestamp));
}

pub fn emit_custody_transferred(
    env: &Env,
    batch_id: &BytesN<32>,
    from: &Address,
    to: &Address,
) {
    let topics = (symbol_short!("custody"), batch_id.clone(), from.clone());
    env.events().publish(topics, to.clone());
}
