#![no_std]

use soroban_sdk::{contract, contractimpl, token, Address, Env, IntoVal};

#[contract]
pub struct AtomicSwapContract;

#[contractimpl]
impl AtomicSwapContract {