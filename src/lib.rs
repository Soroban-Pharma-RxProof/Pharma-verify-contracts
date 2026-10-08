#![no_std]

pub mod auth;
pub mod crypto;
pub mod events;
pub mod storage;
pub mod types;

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Symbol, Vec};
use crate::types::{
    Batch, BatchStatus, Error, PackagingSpec, Participant, ParticipantRole,
    RegulatorProposal, VerificationResult,
};

#[contract]
pub struct RxProofContract;
