#![no_std]

pub mod auth;
pub mod crypto;
pub mod events;
pub mod storage;
pub mod types;

#[cfg(test)]
mod test;

use soroban_sdk::{
    contract, contractimpl, Address, BytesN, Env, Symbol, Vec,
};
use crate::types::{
    Batch, BatchStatus, Error, PackagingSpec, Participant, ParticipantRole,
    RegulatorProposal, VerificationResult,
};

#[contract]
pub struct RxProofContract;

#[contractimpl]
impl RxProofContract {
    /// Initialize contract with an administrator and default regulator role
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if storage::get_admin(&env).is_some() {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();

        storage::set_admin(&env, &admin);
        storage::set_regulator(&env, &admin, true);
        storage::set_regulator_count(&env, 1);

        events::emit_regulator_added(&env, &admin);
        Ok(())
    }

    /// Add an authorized regulator (admin or existing regulator only)
    pub fn add_regulator(env: Env, caller: Address, new_regulator: Address) -> Result<(), Error> {
        auth::require_not_paused(&env)?;
        auth::require_regulator_or_admin(&env, &caller)?;

        if storage::is_regulator(&env, &new_regulator) {
            return Err(Error::RegulatorAlreadyExists);
        }

        storage::set_regulator(&env, &new_regulator, true);
        let count = storage::get_regulator_count(&env) + 1;
        storage::set_regulator_count(&env, count);

        events::emit_regulator_added(&env, &new_regulator);
        Ok(())
    }

    /// Propose removal of a regulator under multi-signature governance
    pub fn propose_remove_regulator(
        env: Env,
        caller: Address,
        regulator_to_remove: Address,
    ) -> Result<u64, Error> {
        auth::require_not_paused(&env)?;
        auth::require_regulator(&env, &caller)?;

        if !storage::is_regulator(&env, &regulator_to_remove) {
            return Err(Error::RegulatorNotFound);
        }

        let total_regulators = storage::get_regulator_count(&env);
        if total_regulators <= 1 {
            return Err(Error::CannotRemoveLastRegulator);
        }

        let proposal_id = storage::increment_proposal_counter(&env);
        let proposal = RegulatorProposal {
            proposal_id,
            proposer: caller.clone(),
            target_regulator: regulator_to_remove.clone(),
            approvals_count: 1,
            executed: false,
            created_at: env.ledger().timestamp(),
        };

        storage::set_proposal(&env, &proposal);
        storage::set_voted(&env, proposal_id, &caller);

        events::emit_regulator_removal_proposed(&env, proposal_id, &caller, &regulator_to_remove);
        events::emit_regulator_removal_voted(&env, proposal_id, &caller, 1);

        // Required threshold: strict majority ((N / 2) + 1)
        let threshold = (total_regulators / 2) + 1;
        if proposal.approvals_count >= threshold {
            let mut executed_proposal = proposal;
            executed_proposal.executed = true;
            storage::set_proposal(&env, &executed_proposal);
            storage::set_regulator(&env, &regulator_to_remove, false);
            storage::set_regulator_count(&env, total_regulators - 1);
            events::emit_regulator_removed(&env, &regulator_to_remove);
        }

        Ok(proposal_id)
    }

    /// Approve an active regulator removal proposal (multi-signature threshold)
    pub fn approve_remove_regulator(
        env: Env,
        caller: Address,
        proposal_id: u64,
    ) -> Result<(), Error> {
        auth::require_not_paused(&env)?;
        auth::require_regulator(&env, &caller)?;

        let mut proposal = storage::get_proposal(&env, proposal_id).ok_or(Error::ProposalNotFound)?;
        if proposal.executed {
            return Err(Error::ProposalAlreadyExecuted);
        }

        if storage::has_voted(&env, proposal_id, &caller) {
            return Err(Error::AlreadyVoted);
        }

        storage::set_voted(&env, proposal_id, &caller);
        proposal.approvals_count += 1;

        let total_regulators = storage::get_regulator_count(&env);
        let threshold = (total_regulators / 2) + 1;

        if proposal.approvals_count >= threshold {
            proposal.executed = true;
            storage::set_regulator(&env, &proposal.target_regulator, false);
            if total_regulators > 0 {
                storage::set_regulator_count(&env, total_regulators - 1);
            }
            events::emit_regulator_removed(&env, &proposal.target_regulator);
        }

        storage::set_proposal(&env, &proposal);
        events::emit_regulator_removal_voted(&env, proposal_id, &caller, proposal.approvals_count);

        Ok(())
    }

    /// Emergency pause (halts state mutations during security incidents)
    pub fn pause(env: Env, caller: Address) -> Result<(), Error> {
        auth::require_regulator_or_admin(&env, &caller)?;
        storage::set_paused(&env, true);
        events::emit_contract_paused(&env, &caller);
        Ok(())
    }

    /// Emergency unpause
    pub fn unpause(env: Env, caller: Address) -> Result<(), Error> {
        auth::require_regulator_or_admin(&env, &caller)?;
        storage::set_paused(&env, false);
        events::emit_contract_unpaused(&env, &caller);
        Ok(())
    }

    /// Register a supply chain participant (regulator only)
    pub fn register_participant(
        env: Env,
        caller: Address,
        participant: Address,
        role: ParticipantRole,
        metadata_hash: BytesN<32>,
    ) -> Result<(), Error> {
        auth::require_not_paused(&env)?;
        auth::require_regulator(&env, &caller)?;

        if storage::has_participant(&env, &participant) {
            return Err(Error::ParticipantAlreadyExists);
        }

        let record = Participant {
            address: participant.clone(),
            role,
            active: true,
            metadata_hash: metadata_hash.clone(),
            registered_at: env.ledger().timestamp(),
        };

        storage::set_participant(&env, &record);
        events::emit_participant_registered(&env, &participant, role, &metadata_hash);
        Ok(())
    }

    /// Suspend or activate a participant (regulator only)
    pub fn set_participant_active(
        env: Env,
        caller: Address,
        participant: Address,
        active: bool,
    ) -> Result<(), Error> {
        auth::require_not_paused(&env)?;
        auth::require_regulator(&env, &caller)?;

        let mut record = storage::get_participant(&env, &participant)
            .ok_or(Error::ParticipantNotFound)?;
        record.active = active;

        storage::set_participant(&env, &record);
        events::emit_participant_status_changed(&env, &participant, active);
        Ok(())
    }

    /// Register a new medicine batch with Merkle root and packaging specifications
    pub fn register_batch(
        env: Env,
        caller: Address,
        batch_id: BytesN<32>,
        merkle_root: BytesN<32>,
        total_quantity: u32,
        packaging_spec: PackagingSpec,
        expiry_timestamp: u64,
        metadata_hash: BytesN<32>,
    ) -> Result<(), Error> {
        auth::require_not_paused(&env)?;
        auth::require_active_role(&env, &caller, ParticipantRole::Manufacturer)?;

        if storage::has_batch(&env, &batch_id) {
            return Err(Error::BatchAlreadyExists);
        }
        if total_quantity == 0 {
            return Err(Error::InvalidParameters);
        }
        if expiry_timestamp <= env.ledger().timestamp() {
            return Err(Error::BatchExpired);
        }

        let safe_spec = PackagingSpec {
            total_strips: if packaging_spec.total_strips == 0 { 1 } else { packaging_spec.total_strips },
            units_per_strip: packaging_spec.units_per_strip,
        };

        let batch = Batch {
            batch_id: batch_id.clone(),
            manufacturer: caller.clone(),
            current_custody: caller.clone(),
            merkle_root: merkle_root.clone(),
            total_quantity,
            dispensed_packs: 0,
            packaging_spec: safe_spec,
            expiry_timestamp,
            metadata_hash,
            status: BatchStatus::Active,
            recall_reason: Option::None,
            created_at: env.ledger().timestamp(),
        };

        storage::set_batch(&env, &batch);
        events::emit_batch_registered(
            &env,
            &batch_id,
            &caller,
            &merkle_root,
            total_quantity,
            expiry_timestamp,
        );

        Ok(())
    }

    /// Transfer batch custody along the supply chain
    pub fn transfer_custody(
        env: Env,
        caller: Address,
        batch_id: BytesN<32>,
        to: Address,
    ) -> Result<(), Error> {
        auth::require_not_paused(&env)?;
        caller.require_auth();

        let mut batch = storage::get_batch(&env, &batch_id).ok_or(Error::BatchNotFound)?;
        if batch.status != BatchStatus::Active {
            return Err(Error::BatchRecalled);
        }
        if batch.current_custody != caller {
            return Err(Error::NotCurrentCustodian);
        }

        let recipient = storage::get_participant(&env, &to).ok_or(Error::ParticipantNotFound)?;
        if !recipient.active {
            return Err(Error::ParticipantInactive);
        }

        let sender = storage::get_participant(&env, &caller).ok_or(Error::ParticipantNotFound)?;
        match sender.role {
            ParticipantRole::Manufacturer => {
                if recipient.role != ParticipantRole::Distributor && recipient.role != ParticipantRole::Pharmacy {
                    return Err(Error::Unauthorized);
                }
            }
            ParticipantRole::Distributor => {
                if recipient.role != ParticipantRole::Distributor && recipient.role != ParticipantRole::Pharmacy {
                    return Err(Error::Unauthorized);
                }
            }
            ParticipantRole::Pharmacy => {
                if recipient.role != ParticipantRole::Pharmacy {
                    return Err(Error::Unauthorized);
                }
            }
            ParticipantRole::Regulator => {
                return Err(Error::Unauthorized);
            }
        }

        batch.current_custody = to.clone();
        storage::set_batch(&env, &batch);

        events::emit_custody_transferred(&env, &batch_id, &caller, &to);
        Ok(())
    }

    /// Issue a batch recall (authorized manufacturer or regulators)
    pub fn recall_batch(
        env: Env,
        caller: Address,
        batch_id: BytesN<32>,
        reason: Symbol,
    ) -> Result<(), Error> {
        caller.require_auth();

        let mut batch = storage::get_batch(&env, &batch_id).ok_or(Error::BatchNotFound)?;
        let is_mfg = batch.manufacturer == caller;
        let is_reg = storage::is_regulator(&env, &caller);
        let is_admin = storage::get_admin(&env).map(|a| a == caller).unwrap_or(false);

        if !is_mfg && !is_reg && !is_admin {
            return Err(Error::Unauthorized);
        }

        batch.status = BatchStatus::Recalled;
        batch.recall_reason = Option::Some(reason.clone());
        storage::set_batch(&env, &batch);

        events::emit_batch_recalled(&env, &batch_id, &caller, reason);
        Ok(())
    }

    /// Verify a pack's authenticity via Merkle proof and check expiry, recalls, and clone status
    pub fn verify_pack(
        env: Env,
        batch_id: BytesN<32>,
        serial_hash: BytesN<32>,
        merkle_proof: Vec<BytesN<32>>,
        strip_index: Option<u32>,
    ) -> VerificationResult {
        let batch = match storage::get_batch(&env, &batch_id) {
            Some(b) => b,
            None => return VerificationResult::Invalid,
        };

        let proof_res = crypto::verify_merkle_proof(
            &env,
            &serial_hash,
            &merkle_proof,
            &batch.merkle_root,
        );
        match proof_res {
            Ok(true) => {}
            _ => return VerificationResult::Invalid,
        }

        if batch.status == BatchStatus::Recalled {
            return VerificationResult::Recalled;
        }

        if env.ledger().timestamp() >= batch.expiry_timestamp {
            return VerificationResult::Expired;
        }

        if storage::is_suspicious_reported(&env, &batch_id, &serial_hash) {
            return VerificationResult::Suspicious;
        }

        if storage::is_serial_dispensed(&env, &batch_id, &serial_hash) {
            return VerificationResult::Suspicious;
        }

        if let Option::Some(s_idx) = strip_index {
            if s_idx >= batch.packaging_spec.total_strips {
                return VerificationResult::Invalid;
            }
            let mask = storage::get_dispensed_strips(&env, &batch_id, &serial_hash);
            if (mask & (1 << s_idx)) != 0 {
                return VerificationResult::Suspicious;
            }
        }

        VerificationResult::Authentic
    }

    /// Dispense a pack or blister strip at an authorized pharmacy, burning the code
    pub fn dispense_pack(
        env: Env,
        caller: Address,
        batch_id: BytesN<32>,
        serial_hash: BytesN<32>,
        merkle_proof: Vec<BytesN<32>>,
        strip_index: Option<u32>,
    ) -> Result<(), Error> {
        auth::require_not_paused(&env)?;
        auth::require_active_role(&env, &caller, ParticipantRole::Pharmacy)?;

        let mut batch = storage::get_batch(&env, &batch_id).ok_or(Error::BatchNotFound)?;
        if batch.current_custody != caller {
            return Err(Error::NotCurrentCustodian);
        }
        if batch.status != BatchStatus::Active {
            return Err(Error::BatchRecalled);
        }
        if env.ledger().timestamp() >= batch.expiry_timestamp {
            return Err(Error::BatchExpired);
        }

        let is_valid = crypto::verify_merkle_proof(
            &env,
            &serial_hash,
            &merkle_proof,
            &batch.merkle_root,
        )?;
        if !is_valid {
            return Err(Error::InvalidMerkleProof);
        }

        if batch.dispensed_packs >= batch.total_quantity {
            return Err(Error::BatchQuantityExceeded);
        }

        match strip_index {
            Option::None => {
                if storage::is_serial_dispensed(&env, &batch_id, &serial_hash) {
                    return Err(Error::SerialAlreadyDispensed);
                }
                storage::set_serial_dispensed(&env, &batch_id, &serial_hash);
                batch.dispensed_packs += 1;
                storage::set_batch(&env, &batch);

                events::emit_pack_dispensed(&env, &batch_id, &serial_hash, &caller);
            }
            Option::Some(s_idx) => {
                if s_idx >= batch.packaging_spec.total_strips {
                    return Err(Error::InvalidStripIndex);
                }
                if storage::is_serial_dispensed(&env, &batch_id, &serial_hash) {
                    return Err(Error::SerialAlreadyDispensed);
                }

                let mut mask = storage::get_dispensed_strips(&env, &batch_id, &serial_hash);
                let bit = 1 << s_idx;
                if (mask & bit) != 0 {
                    return Err(Error::StripAlreadyDispensed);
                }

                mask |= bit;
                storage::set_dispensed_strips(&env, &batch_id, &serial_hash, mask);
                events::emit_partial_dispensed(&env, &batch_id, &serial_hash, s_idx, &caller);

                // If all blister strips in this pack are now dispensed, mark the whole pack burned
                let full_mask = (1 << batch.packaging_spec.total_strips) - 1;
                if mask == full_mask {
                    storage::set_serial_dispensed(&env, &batch_id, &serial_hash);
                    batch.dispensed_packs += 1;
                    storage::set_batch(&env, &batch);
                    events::emit_pack_dispensed(&env, &batch_id, &serial_hash, &caller);
                }
            }
        }

        Ok(())
    }

    /// Report a suspicious or cloned pack (open to public, patients, and inspectors)
    pub fn report_suspicious(
        env: Env,
        caller: Address,
        batch_id: BytesN<32>,
        serial_hash: BytesN<32>,
        reason: Symbol,
    ) -> Result<(), Error> {
        caller.require_auth();
        storage::set_suspicious_reported(&env, &batch_id, &serial_hash);
        events::emit_suspicious_reported(&env, &batch_id, &serial_hash, &caller, reason);
        Ok(())
    }

    // ---------------- Read-only Getters ----------------

    pub fn get_admin(env: Env) -> Option<Address> {
        storage::get_admin(&env)
    }

    pub fn is_paused(env: Env) -> bool {
        storage::is_paused(&env)
    }

    pub fn is_regulator(env: Env, regulator: Address) -> bool {
        storage::is_regulator(&env, &regulator)
    }

    pub fn get_regulator_count(env: Env) -> u32 {
        storage::get_regulator_count(&env)
    }

    pub fn get_proposal(env: Env, proposal_id: u64) -> Option<RegulatorProposal> {
        storage::get_proposal(&env, proposal_id)
    }

    pub fn get_participant(env: Env, participant: Address) -> Option<Participant> {
        storage::get_participant(&env, &participant)
    }

    pub fn get_batch(env: Env, batch_id: BytesN<32>) -> Option<Batch> {
        storage::get_batch(&env, &batch_id)
    }

    pub fn get_custody(env: Env, batch_id: BytesN<32>) -> Option<Address> {
        storage::get_batch(&env, &batch_id).map(|b| b.current_custody)
    }

    pub fn is_serial_dispensed(env: Env, batch_id: BytesN<32>, serial_hash: BytesN<32>) -> bool {
        storage::is_serial_dispensed(&env, &batch_id, &serial_hash)
    }

    pub fn get_dispensed_strips(env: Env, batch_id: BytesN<32>, serial_hash: BytesN<32>) -> u32 {
        storage::get_dispensed_strips(&env, &batch_id, &serial_hash)
    }

    pub fn is_suspicious_reported(env: Env, batch_id: BytesN<32>, serial_hash: BytesN<32>) -> bool {
        storage::is_suspicious_reported(&env, &batch_id, &serial_hash)
    }
}

