
use soroban_sdk::{Address, Env};
use crate::storage;
use crate::types::{Error, Participant, ParticipantRole};

/// Require caller authentication and verify caller is the configured admin
pub fn require_admin(env: &Env, caller: &Address) -> Result<(), Error> {
    caller.require_auth();
    let admin = storage::get_admin(env).ok_or(Error::NotInitialized)?;
    if *caller != admin {
        return Err(Error::Unauthorized);
    }
    Ok(())
}

/// Require caller authentication and verify caller is an active regulator
pub fn require_regulator(env: &Env, caller: &Address) -> Result<(), Error> {
    caller.require_auth();
    if !storage::is_regulator(env, caller) {
        return Err(Error::Unauthorized);
    }
    Ok(())
}

/// Require caller authentication and verify caller is either an active regulator or admin
pub fn require_regulator_or_admin(env: &Env, caller: &Address) -> Result<(), Error> {
    caller.require_auth();
    let is_admin = storage::get_admin(env).map(|a| a == *caller).unwrap_or(false);
    let is_reg = storage::is_regulator(env, caller);
    if !is_admin && !is_reg {
        return Err(Error::Unauthorized);
    }
    Ok(())
}

/// Require caller authentication, verify registered participant status and role
pub fn require_active_role(
    env: &Env,
    caller: &Address,
    expected_role: ParticipantRole,
) -> Result<Participant, Error> {
    caller.require_auth();
    let participant = storage::get_participant(env, caller).ok_or(Error::ParticipantNotFound)?;
    if !participant.active {
        return Err(Error::ParticipantInactive);
    }
    if participant.role != expected_role {
        return Err(Error::Unauthorized);
    }
    Ok(participant)
}

/// Require that contract execution is not currently halted by emergency pause
pub fn require_not_paused(env: &Env) -> Result<(), Error> {
    if storage::is_paused(env) {
        return Err(Error::ContractPaused);
    }
    Ok(())
}

