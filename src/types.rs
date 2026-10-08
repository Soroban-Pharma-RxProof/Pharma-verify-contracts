
use soroban_sdk::{contracterror, contracttype, Address, BytesN, Symbol};

/// Role assigned to authorized supply chain participants
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ParticipantRole {
    Manufacturer = 1,
    Distributor = 2,
    Pharmacy = 3,
    Regulator = 4,
}

/// Participant registration record on-chain
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Participant {
    pub address: Address,
    pub role: ParticipantRole,
    pub active: bool,
    pub metadata_hash: BytesN<32>,
    pub registered_at: u64,
}

/// Packaging specifications for blister packs or bulk packages
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackagingSpec {
    /// Total blister strips per pack (default 1 for bottle/box)
    pub total_strips: u32,
    /// Number of individual dosage units per strip
    pub units_per_strip: u32,
}

/// Life-cycle status of a manufactured batch
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum BatchStatus {
    Active = 1,
    Recalled = 2,
}

/// Batch details stored in persistent storage
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

/// Verification verdict for a pack query
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

/// Multi-signature proposal record for removing a regulator
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

/// Contract storage keys
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    IsPaused,
    Regulator(Address),
    RegulatorCount,
    ProposalCounter,
    RegulatorProposal(u64),
    ProposalVoted(u64, Address),
    Participant(Address),
    Batch(BytesN<32>),
    SerialDispensed(BytesN<32>, BytesN<32>),
    DispensedStrips(BytesN<32>, BytesN<32>),
    SuspiciousReported(BytesN<32>, BytesN<32>),
}

/// Contract error definitions
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    ContractPaused = 4,
    ParticipantNotFound = 5,
    ParticipantInactive = 6,
    ParticipantAlreadyExists = 7,
    InvalidRole = 8,
    BatchAlreadyExists = 9,
    BatchNotFound = 10,
    BatchExpired = 11,
    BatchRecalled = 12,
    InvalidMerkleProof = 13,
    ProofDepthExceeded = 14,
    SerialAlreadyDispensed = 15,
    InvalidStripIndex = 16,
    StripAlreadyDispensed = 17,
    BatchQuantityExceeded = 18,
    NotCurrentCustodian = 19,
    RegulatorAlreadyExists = 20,
    RegulatorNotFound = 21,
    ProposalNotFound = 22,
    AlreadyVoted = 23,
    ProposalAlreadyExecuted = 24,
    CannotRemoveLastRegulator = 25,
    InvalidParameters = 26,
}

