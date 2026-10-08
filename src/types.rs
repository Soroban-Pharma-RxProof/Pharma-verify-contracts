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
