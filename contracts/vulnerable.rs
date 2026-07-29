#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct VulnerableContract;

#[contractimpl]
impl VulnerableContract {
    pub fn withdraw(env: Env, from: Address, amount: i128) {
        from.require_auth();
        let balance: i128 = env.storage().instance().get(&from).unwrap_or(0);
        let new_balance = balance - amount;
        env.storage().instance().set(&from, &new_balance);
    }
}