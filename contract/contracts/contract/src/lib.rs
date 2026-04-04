#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env};

#[contract]
pub struct DelegationContract;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Delegate(Address),       // delegator -> representative
    DelegatorCount(Address), // representative -> number of delegators
}

#[contractimpl]
impl DelegationContract {
    pub fn delegate(env: Env, delegator: Address, representative: Address) {
        delegator.require_auth();

        if delegator == representative {
            panic!("cannot delegate to self");
        }

        let old_rep = env
            .storage()
            .persistent()
            .get::<DataKey, Address>(&DataKey::Delegate(delegator.clone()));

        // Remove old delegation count if delegator already delegated before
        if let Some(prev_rep) = old_rep {
            if prev_rep == representative {
                panic!("already delegated to this representative");
            }

            let prev_count = env
                .storage()
                .persistent()
                .get::<DataKey, u32>(&DataKey::DelegatorCount(prev_rep.clone()))
                .unwrap_or(0);

            if prev_count > 0 {
                env.storage()
                    .persistent()
                    .set(&DataKey::DelegatorCount(prev_rep), &(prev_count - 1));
            }
        }

        // Save new delegation
        env.storage()
            .persistent()
            .set(&DataKey::Delegate(delegator.clone()), &representative);

        // Increase representative count
        let new_count = env
            .storage()
            .persistent()
            .get::<DataKey, u32>(&DataKey::DelegatorCount(representative.clone()))
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&DataKey::DelegatorCount(representative.clone()), &(new_count + 1));

        // Emit event
        env.events().publish(
            (symbol_short!("delegate"),),
            (delegator, representative),
        );
    }

    pub fn revoke(env: Env, delegator: Address) {
        delegator.require_auth();

        let rep = env
            .storage()
            .persistent()
            .get::<DataKey, Address>(&DataKey::Delegate(delegator.clone()))
            .unwrap_or_else(|| panic!("no active delegation found"));

        env.storage()
            .persistent()
            .remove(&DataKey::Delegate(delegator.clone()));

        let count = env
            .storage()
            .persistent()
            .get::<DataKey, u32>(&DataKey::DelegatorCount(rep.clone()))
            .unwrap_or(0);

        if count > 0 {
            env.storage()
                .persistent()
                .set(&DataKey::DelegatorCount(rep.clone()), &(count - 1));
        }

        env.events()
            .publish((symbol_short!("revoke"),), (delegator, rep));
    }

    pub fn get_delegate(env: Env, delegator: Address) -> Option<Address> {
        env.storage()
            .persistent()
            .get::<DataKey, Address>(&DataKey::Delegate(delegator))
    }

    pub fn get_delegator_count(env: Env, representative: Address) -> u32 {
        env.storage()
            .persistent()
            .get::<DataKey, u32>(&DataKey::DelegatorCount(representative))
            .unwrap_or(0)
    }

    pub fn has_delegated(env: Env, delegator: Address) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Delegate(delegator))
    }
}