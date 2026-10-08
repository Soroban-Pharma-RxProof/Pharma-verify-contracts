use soroban_sdk::{contracterror, contracttype, Address, BytesN, Symbol};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ParticipantRole {
    Manufacturer = 1,
    Distributor = 2,
    Pharmacy = 3,
    Regulator = 4,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Participant {
    pub address: Address,
    pub role: ParticipantRole,
    pub active: bool,
    pub metadata_hash: BytesN<32>,
    pub registered_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackagingSpec {
    pub total_strips: u32,
    pub units_per_strip: u32,
}

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum BatchStatus {
    Active = 1,
    Recalled = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Batch {
    pub batch_id: BytesN<32>,
    pub manufacturer: Address,
    pub current_custody: Address,
    pub merkle_root: BytesN<32>,
    pub total_quantity: u32,
    pub dispensed_packs: u32,
    pub packaging_spec: PackagingSpec,
    pub expiry_timestamp: u64,
    pub metadata_hash: BytesN<32>,
    pub status: BatchStatus,
    pub recall_reason: Option<Symbol>,
    pub created_at: u64,
}

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum VerificationResult {
    Authentic = 1,
    Expired = 2,
    Recalled = 3,
    Suspicious = 4,
    Invalid = 5,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegulatorProposal {
    pub proposal_id: u64,
    pub proposer: Address,
    pub target_regulator: Address,
    pub approvals_count: u32,
    pub executed: bool,
    pub created_at: u64,
}
