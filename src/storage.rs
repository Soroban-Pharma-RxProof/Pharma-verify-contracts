use soroban_sdk::{Address, BytesN, Env};
use crate::types::{Batch, DataKey, Participant, RegulatorProposal};

pub const INSTANCE_LIFETIME_THRESHOLD: u32 = 17_280;
pub const INSTANCE_BUMP_AMOUNT: u32 = 518_400;

pub const PERSISTENT_LIFETIME_THRESHOLD: u32 = 17_280;
pub const PERSISTENT_BUMP_AMOUNT: u32 = 518_400;

pub fn bump_instance(env: &Env) {
    env.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn bump_persistent(env: &Env, key: &DataKey) {
    env.storage().persistent().extend_ttl(key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
}
