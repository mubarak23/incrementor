#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, log, Env, Symbol};

const COUNTER: Symbol = symbol_short!("Counter");

#[contract]
pub struct IncrementorContract;

#[contractimpl]
impl IncrementorContract {
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count +=1;

        log!(&env, "count: {}", count);

        env.storage().instance().set(&COUNTER, &count);

        env.storage().instance().extend_ttl(100, 100);

        count
    }
    pub fn get_current_value(env: Env) -> u32 {
        let count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count 
    }

    pub fn decrement(env: Env) -> u32 {
         let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
         count -=1;

         log!(&env, "count: {}", count);

         env.storage().instance().set(&COUNTER, &count);

         env.storage().instance().extend_ttl(100, 100);

        count
    }

    pub fn reset(env: Env)-> u32 {
         let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
         count = 0;

         log!(&env, "count: {}", count);

         env.storage().instance().set(&COUNTER, &count);

         env.storage().instance().extend_ttl(100, 100);

        count

    }
}


mod test;
