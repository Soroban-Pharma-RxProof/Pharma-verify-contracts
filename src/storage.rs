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

pub fn get_admin(env: &Env) -> Option<Address> {
    bump_instance(env);
    env.storage().instance().get(&DataKey::Admin)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
    bump_instance(env);
}

pub fn is_paused(env: &Env) -> bool {
    bump_instance(env);
    env.storage().instance().get(&DataKey::IsPaused).unwrap_or(false)
}

pub fn set_paused(env: &Env, paused: bool) {
    env.storage().instance().set(&DataKey::IsPaused, &paused);
    bump_instance(env);
}

pub fn is_regulator(env: &Env, address: &Address) -> bool {
    bump_instance(env);
    env.storage().instance().get(&DataKey::Regulator(address.clone())).unwrap_or(false)
}

pub fn set_regulator(env: &Env, address: &Address, active: bool) {
    env.storage().instance().set(&DataKey::Regulator(address.clone()), &active);
    bump_instance(env);
}

pub fn get_regulator_count(env: &Env) -> u32 {
    bump_instance(env);
    env.storage().instance().get(&DataKey::RegulatorCount).unwrap_or(0)
}

pub fn set_regulator_count(env: &Env, count: u32) {
    env.storage().instance().set(&DataKey::RegulatorCount, &count);
    bump_instance(env);
}

pub fn get_proposal_counter(env: &Env) -> u64 {
    bump_instance(env);
    env.storage().instance().get(&DataKey::ProposalCounter).unwrap_or(0)
}

pub fn increment_proposal_counter(env: &Env) -> u64 {
    let next = get_proposal_counter(env) + 1;
    env.storage().instance().set(&DataKey::ProposalCounter, &next);
    bump_instance(env);
    next
}

pub fn get_proposal(env: &Env, proposal_id: u64) -> Option<RegulatorProposal> {
    let key = DataKey::RegulatorProposal(proposal_id);
    if let Some(proposal) = env.storage().persistent().get(&key) {
        bump_persistent(env, &key);
        Some(proposal)
    } else {
        None
    }
}

pub fn set_proposal(env: &Env, proposal: &RegulatorProposal) {
    let key = DataKey::RegulatorProposal(proposal.proposal_id);
    env.storage().persistent().set(&key, proposal);
    bump_persistent(env, &key);
}

pub fn has_voted(env: &Env, proposal_id: u64, voter: &Address) -> bool {
    let key = DataKey::ProposalVoted(proposal_id, voter.clone());
    env.storage().persistent().get(&key).unwrap_or(false)
}

pub fn set_voted(env: &Env, proposal_id: u64, voter: &Address) {
    let key = DataKey::ProposalVoted(proposal_id, voter.clone());
    env.storage().persistent().set(&key, &true);
    bump_persistent(env, &key);
}

pub fn has_participant(env: &Env, address: &Address) -> bool {
    let key = DataKey::Participant(address.clone());
    env.storage().persistent().has(&key)
}

pub fn get_participant(env: &Env, address: &Address) -> Option<Participant> {
    let key = DataKey::Participant(address.clone());
    if let Some(participant) = env.storage().persistent().get(&key) {
        bump_persistent(env, &key);
        Some(participant)
    } else {
        None
    }
}

pub fn set_participant(env: &Env, participant: &Participant) {
    let key = DataKey::Participant(participant.address.clone());
    env.storage().persistent().set(&key, participant);
    bump_persistent(env, &key);
}

pub fn has_batch(env: &Env, batch_id: &BytesN<32>) -> bool {
    let key = DataKey::Batch(batch_id.clone());
    env.storage().persistent().has(&key)
}

pub fn get_batch(env: &Env, batch_id: &BytesN<32>) -> Option<Batch> {
    let key = DataKey::Batch(batch_id.clone());
    if let Some(batch) = env.storage().persistent().get(&key) {
        bump_persistent(env, &key);
        Some(batch)
    } else {
        None
    }
}

pub fn set_batch(env: &Env, batch: &Batch) {
    let key = DataKey::Batch(batch.batch_id.clone());
    env.storage().persistent().set(&key, batch);
    bump_persistent(env, &key);
}
