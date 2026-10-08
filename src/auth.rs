use soroban_sdk::{Address, Env};
use crate::storage;
use crate::types::{Error, Participant, ParticipantRole};

pub fn require_admin(env: &Env, caller: &Address) -> Result<(), Error> {
    caller.require_auth();
    let admin = storage::get_admin(env).ok_or(Error::NotInitialized)?;
    if *caller != admin {
        return Err(Error::Unauthorized);
    }
    Ok(())
}

pub fn require_regulator(env: &Env, caller: &Address) -> Result<(), Error> {
    caller.require_auth();
    if !storage::is_regulator(env, caller) {
        return Err(Error::Unauthorized);
    }
    Ok(())
}
