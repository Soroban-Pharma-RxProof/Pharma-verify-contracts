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

pub fn emit_batch_recalled(
    env: &Env,
    batch_id: &BytesN<32>,
    recalled_by: &Address,
    reason: Symbol,
) {
    let topics = (symbol_short!("recalled"), batch_id.clone(), recalled_by.clone());
    env.events().publish(topics, reason);
}

pub fn emit_pack_dispensed(
    env: &Env,
    batch_id: &BytesN<32>,
    serial_hash: &BytesN<32>,
    pharmacy: &Address,
) {
    let topics = (symbol_short!("dispensed"), batch_id.clone(), pharmacy.clone());
    env.events().publish(topics, serial_hash.clone());
}

pub fn emit_partial_dispensed(
    env: &Env,
    batch_id: &BytesN<32>,
    serial_hash: &BytesN<32>,
    strip_index: u32,
    pharmacy: &Address,
) {
    let topics = (symbol_short!("part_disp"), batch_id.clone(), pharmacy.clone());
    env.events().publish(topics, (serial_hash.clone(), strip_index));
}

pub fn emit_suspicious_reported(
    env: &Env,
    batch_id: &BytesN<32>,
    serial_hash: &BytesN<32>,
    reporter: &Address,
    reason: Symbol,
) {
    let topics = (symbol_short!("susp_rep"), batch_id.clone(), reporter.clone());
    env.events().publish(topics, (serial_hash.clone(), reason));
}
