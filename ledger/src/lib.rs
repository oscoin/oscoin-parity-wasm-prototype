#![cfg_attr(not(test), no_std)]
#![allow(non_snake_case)]
#![feature(proc_macro_hygiene)]

use pwasm_abi_derive::eth_abi;
// Implements the dispatch and dispatch_ctor methods
use pwasm_abi::eth::EndpointInterface;

use crate::pwasm::{String, H256, U256};

mod pwasm;

#[no_mangle]
pub fn call() {
    let mut endpoint = LedgerEndpoint::new(Ledger_ { env: pwasm::Pwasm });
    pwasm_ethereum::ret(&endpoint.dispatch(&pwasm_ethereum::input()));
}

#[no_mangle]
pub fn deploy() {}

#[eth_abi(LedgerEndpoint, LedgerClient)]
pub trait Ledger {
    #[constant]
    fn ping(&mut self) -> String;

    fn counter_inc(&mut self);

    #[constant]
    fn counter_value(&mut self) -> U256;
}

pub struct Ledger_<E: pwasm::Env> {
    env: E,
}

lazy_static::lazy_static! {
    static ref COUNTER_ADDRESS: H256 =
        H256::from(
            [2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
        );
}

impl<E: pwasm::Env> Ledger for Ledger_<E> {
    fn ping(&mut self) -> String {
        String::from("pong")
    }

    fn counter_inc(&mut self) {
        let data = self.env.read(&COUNTER_ADDRESS);
        let counter = U256::from(data);
        self.env.write(&COUNTER_ADDRESS, &(counter + 1).into());
    }

    fn counter_value(&mut self) -> U256 {
        let data = self.env.read(&COUNTER_ADDRESS);
        U256::from(data)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn counter_inc() {
        let mut ledger = new_ledger();
        for _ in 0..10 {
            ledger.counter_inc()
        }
        let counter = ledger.counter_value();
        assert_eq!(counter, U256::from(10));
    }

    #[test]
    fn counter_default() {
        let mut ledger = new_ledger();
        let counter = ledger.counter_value();
        assert_eq!(counter, U256::from(0));
    }

    fn new_ledger() -> Ledger_<pwasm::TestEnv> {
        Ledger_ {
            env: pwasm::TestEnv::new(),
        }
    }
}
