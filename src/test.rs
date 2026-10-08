#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    vec, Address, Bytes, BytesN, Env, Symbol,
};
use crate::crypto::hash_sorted_pair;
use crate::types::{PackagingSpec, ParticipantRole, VerificationResult};

fn create_test_leaf(env: &Env, val: u8) -> BytesN<32> {
    let mut arr = [0u8; 32];
    arr[0] = val;
    let b = Bytes::from_array(env, &arr);
    env.crypto().sha256(&b).into()
}

#[test]
fn test_initialize_and_admin() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RxProofContract, ());
    let client = RxProofContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    assert_eq!(client.get_admin(), Some(admin.clone()));
    assert_eq!(client.is_regulator(&admin), true);
    assert_eq!(client.get_regulator_count(), 1);

    // Second initialization should fail
    let res = client.try_initialize(&admin);
    assert!(res.is_err());
}

#[test]
fn test_regulator_management_and_multisig_removal() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RxProofContract, ());
    let client = RxProofContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let reg2 = Address::generate(&env);
    let reg3 = Address::generate(&env);

    client.add_regulator(&admin, &reg2);
    client.add_regulator(&admin, &reg3);
    assert_eq!(client.get_regulator_count(), 3);
    assert_eq!(client.is_regulator(&reg2), true);
    assert_eq!(client.is_regulator(&reg3), true);

    // Cannot remove when only 1 regulator (tested on 3 here)
    // reg1 (admin) proposes removing reg2: total=3, threshold=(3/2)+1 = 2
    let proposal_id = client.propose_remove_regulator(&admin, &reg2);
    let proposal = client.get_proposal(&proposal_id).unwrap();
    assert_eq!(proposal.approvals_count, 1);
    assert_eq!(proposal.executed, false);

    // Voting again from same address should fail
    let duplicate_vote = client.try_approve_remove_regulator(&admin, &proposal_id);
    assert!(duplicate_vote.is_err());

    // reg3 votes to approve: threshold 2 reached -> executed!
    client.approve_remove_regulator(&reg3, &proposal_id);
    let proposal_after = client.get_proposal(&proposal_id).unwrap();
    assert_eq!(proposal_after.executed, true);
    assert_eq!(client.is_regulator(&reg2), false);
    assert_eq!(client.get_regulator_count(), 2);
}

#[test]
fn test_emergency_pause_and_unpause() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RxProofContract, ());
    let client = RxProofContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    assert_eq!(client.is_paused(), false);

    client.pause(&admin);
    assert_eq!(client.is_paused(), true);

    // Registering participant should fail while paused
    let mfg = Address::generate(&env);
    let meta = create_test_leaf(&env, 1);
    let res = client.try_register_participant(&admin, &mfg, &ParticipantRole::Manufacturer, &meta);
    assert!(res.is_err());

    // Unpause
    client.unpause(&admin);
    assert_eq!(client.is_paused(), false);

    client.register_participant(&admin, &mfg, &ParticipantRole::Manufacturer, &meta);
    assert!(client.get_participant(&mfg).is_some());
}

#[test]
fn test_participant_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RxProofContract, ());
    let client = RxProofContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let mfg = Address::generate(&env);
    let meta = create_test_leaf(&env, 1);

    client.register_participant(&admin, &mfg, &ParticipantRole::Manufacturer, &meta);
    let p = client.get_participant(&mfg).unwrap();
    assert_eq!(p.active, true);
    assert_eq!(p.role, ParticipantRole::Manufacturer);

    // Duplicate registration fails
    let dup = client.try_register_participant(&admin, &mfg, &ParticipantRole::Manufacturer, &meta);
    assert!(dup.is_err());

    // Deactivate participant
    client.set_participant_active(&admin, &mfg, &false);
    let p2 = client.get_participant(&mfg).unwrap();
    assert_eq!(p2.active, false);

    // Reactivate
    client.set_participant_active(&admin, &mfg, &true);
    let p3 = client.get_participant(&mfg).unwrap();
    assert_eq!(p3.active, true);
}

#[test]
fn test_batch_registration_and_custody_transfer() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RxProofContract, ());
    let client = RxProofContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let mfg = Address::generate(&env);
    let dist = Address::generate(&env);
    let pharmacy = Address::generate(&env);
    let meta = create_test_leaf(&env, 1);

    client.register_participant(&admin, &mfg, &ParticipantRole::Manufacturer, &meta);
    client.register_participant(&admin, &dist, &ParticipantRole::Distributor, &meta);
    client.register_participant(&admin, &pharmacy, &ParticipantRole::Pharmacy, &meta);

    // Build Merkle tree for 2 packs: leaf0, leaf1
    let leaf0 = create_test_leaf(&env, 10);
    let leaf1 = create_test_leaf(&env, 11);
    let root = hash_sorted_pair(&env, &leaf0, &leaf1);

    let batch_id = create_test_leaf(&env, 99);
    let spec = PackagingSpec {
        total_strips: 2,
        units_per_strip: 10,
    };
    let expiry = env.ledger().timestamp() + 100_000;

    client.register_batch(&mfg, &batch_id, &root, &2, &spec, &expiry, &meta);

    let batch = client.get_batch(&batch_id).unwrap();
    assert_eq!(batch.manufacturer, mfg);
    assert_eq!(batch.current_custody, mfg);
    assert_eq!(batch.total_quantity, 2);
    assert_eq!(batch.dispensed_packs, 0);

    // Transfer custody: Mfg -> Dist
    client.transfer_custody(&mfg, &batch_id, &dist);
    assert_eq!(client.get_custody(&batch_id), Some(dist.clone()));

    // Transfer custody: Dist -> Pharmacy
    client.transfer_custody(&dist, &batch_id, &pharmacy);
    assert_eq!(client.get_custody(&batch_id), Some(pharmacy.clone()));
}

#[test]
fn test_verify_pack_verdicts() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RxProofContract, ());
    let client = RxProofContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let mfg = Address::generate(&env);
    let pharmacy = Address::generate(&env);
    let meta = create_test_leaf(&env, 1);

    client.register_participant(&admin, &mfg, &ParticipantRole::Manufacturer, &meta);
    client.register_participant(&admin, &pharmacy, &ParticipantRole::Pharmacy, &meta);

    // 2 leaves
    let leaf0 = create_test_leaf(&env, 20);
    let leaf1 = create_test_leaf(&env, 21);
    let root = hash_sorted_pair(&env, &leaf0, &leaf1);

    let batch_id = create_test_leaf(&env, 55);
    let spec = PackagingSpec {
        total_strips: 1,
        units_per_strip: 20,
    };
    let expiry = env.ledger().timestamp() + 50_000;

    client.register_batch(&mfg, &batch_id, &root, &2, &spec, &expiry, &meta);

    let proof_for_0 = vec![&env, leaf1.clone()];
    let proof_for_1 = vec![&env, leaf0.clone()];

    // 1. Authentic packs
    let verdict = client.verify_pack(&batch_id, &leaf0, &proof_for_0, &None);
    assert_eq!(verdict, VerificationResult::Authentic);
    let verdict1 = client.verify_pack(&batch_id, &leaf1, &proof_for_1, &None);
    assert_eq!(verdict1, VerificationResult::Authentic);

    // 2. Invalid proof
    let fake_leaf = create_test_leaf(&env, 99);
    let invalid_verdict = client.verify_pack(&batch_id, &fake_leaf, &proof_for_0, &None);
    assert_eq!(invalid_verdict, VerificationResult::Invalid);

    // 3. Recall batch
    let reason = Symbol::new(&env, "contamination");
    client.recall_batch(&mfg, &batch_id, &reason);

    let recalled_verdict = client.verify_pack(&batch_id, &leaf0, &proof_for_0, &None);
    assert_eq!(recalled_verdict, VerificationResult::Recalled);
}

#[test]
fn test_dispense_full_pack_and_cloning_detection() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RxProofContract, ());
    let client = RxProofContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let mfg = Address::generate(&env);
    let pharmacy = Address::generate(&env);
    let meta = create_test_leaf(&env, 1);

    client.register_participant(&admin, &mfg, &ParticipantRole::Manufacturer, &meta);
    client.register_participant(&admin, &pharmacy, &ParticipantRole::Pharmacy, &meta);

    let leaf0 = create_test_leaf(&env, 30);
    let leaf1 = create_test_leaf(&env, 31);
    let root = hash_sorted_pair(&env, &leaf0, &leaf1);

    let batch_id = create_test_leaf(&env, 77);
    let spec = PackagingSpec {
        total_strips: 1,
        units_per_strip: 10,
    };
    let expiry = env.ledger().timestamp() + 100_000;

    client.register_batch(&mfg, &batch_id, &root, &1, &spec, &expiry, &meta);
    client.transfer_custody(&mfg, &batch_id, &pharmacy);

    let proof_for_0 = vec![&env, leaf1.clone()];

    // Prior to dispense: Authentic
    assert_eq!(
        client.verify_pack(&batch_id, &leaf0, &proof_for_0, &None),
        VerificationResult::Authentic
    );

    // Dispense pack
    client.dispense_pack(&pharmacy, &batch_id, &leaf0, &proof_for_0, &None);
    assert_eq!(client.is_serial_dispensed(&batch_id, &leaf0), true);

    // After dispense: pack is burned! Verification yields Suspicious (cloned code warning)
    assert_eq!(
        client.verify_pack(&batch_id, &leaf0, &proof_for_0, &None),
        VerificationResult::Suspicious
    );

    // Second dispense attempt fails
    let replay_res = client.try_dispense_pack(&pharmacy, &batch_id, &leaf0, &proof_for_0, &None);
    assert!(replay_res.is_err());
}

#[test]
fn test_partial_blister_dispense() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RxProofContract, ());
    let client = RxProofContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let mfg = Address::generate(&env);
    let pharmacy = Address::generate(&env);
    let meta = create_test_leaf(&env, 1);

    client.register_participant(&admin, &mfg, &ParticipantRole::Manufacturer, &meta);
    client.register_participant(&admin, &pharmacy, &ParticipantRole::Pharmacy, &meta);

    let leaf0 = create_test_leaf(&env, 40);
    let leaf1 = create_test_leaf(&env, 41);
    let root = hash_sorted_pair(&env, &leaf0, &leaf1);

    let batch_id = create_test_leaf(&env, 88);
    let spec = PackagingSpec {
        total_strips: 2, // 2 blister strips
        units_per_strip: 10,
    };
    let expiry = env.ledger().timestamp() + 100_000;

    client.register_batch(&mfg, &batch_id, &root, &2, &spec, &expiry, &meta);
    client.transfer_custody(&mfg, &batch_id, &pharmacy);

    let proof_for_0 = vec![&env, leaf1.clone()];

    // Dispense Strip 0
    client.dispense_pack(&pharmacy, &batch_id, &leaf0, &proof_for_0, &Some(0));
    assert_eq!(client.get_dispensed_strips(&batch_id, &leaf0), 1); // bit 0 = 1
    assert_eq!(client.is_serial_dispensed(&batch_id, &leaf0), false); // pack not fully burned yet

    // Verifying Strip 0 gives Suspicious (already dispensed)
    assert_eq!(
        client.verify_pack(&batch_id, &leaf0, &proof_for_0, &Some(0)),
        VerificationResult::Suspicious
    );
    // Verifying Strip 1 gives Authentic (still remaining)
    assert_eq!(
        client.verify_pack(&batch_id, &leaf0, &proof_for_0, &Some(1)),
        VerificationResult::Authentic
    );

    // Re-dispensing Strip 0 fails
    let dup_res = client.try_dispense_pack(&pharmacy, &batch_id, &leaf0, &proof_for_0, &Some(0));
    assert!(dup_res.is_err());

    // Dispense Strip 1: completes pack! Full pack is now burned
    client.dispense_pack(&pharmacy, &batch_id, &leaf0, &proof_for_0, &Some(1));
    assert_eq!(client.get_dispensed_strips(&batch_id, &leaf0), 3); // bits 0 and 1 = 3
    assert_eq!(client.is_serial_dispensed(&batch_id, &leaf0), true);

    let b = client.get_batch(&batch_id).unwrap();
    assert_eq!(b.dispensed_packs, 1);
}

#[test]
fn test_suspicious_report() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RxProofContract, ());
    let client = RxProofContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let mfg = Address::generate(&env);
    let meta = create_test_leaf(&env, 1);
    client.register_participant(&admin, &mfg, &ParticipantRole::Manufacturer, &meta);

    let leaf0 = create_test_leaf(&env, 50);
    let root = leaf0.clone();
    let batch_id = create_test_leaf(&env, 91);
    let spec = PackagingSpec {
        total_strips: 1,
        units_per_strip: 10,
    };
    let expiry = env.ledger().timestamp() + 100_000;

    client.register_batch(&mfg, &batch_id, &root, &5, &spec, &expiry, &meta);

    let reporter = Address::generate(&env);
    let reason = Symbol::new(&env, "counterfeit_packaging");

    client.report_suspicious(&reporter, &batch_id, &leaf0, &reason);
    assert_eq!(client.is_suspicious_reported(&batch_id, &leaf0), true);

    let empty_proof = vec![&env];
    assert_eq!(
        client.verify_pack(&batch_id, &leaf0, &empty_proof, &None),
        VerificationResult::Suspicious
    );
}

