#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![allow(non_snake_case)]
#![feature(proc_macro_hygiene)]

use pwasm_abi_derive::eth_abi;
// Implements the dispatch and dispatch_ctor methods
use pwasm_abi::eth::EndpointInterface;

use crate::pwasm::{Address, String, H256, U256};

pub mod pwasm;
pub mod storage;

use storage::Storage;

pub fn call() {
    let mut endpoint = LedgerEndpoint::new(Ledger_ {
        env: Storage { env: pwasm::Pwasm },
    });
    pwasm_ethereum::ret(&endpoint.dispatch(&pwasm_ethereum::input()));
}

#[eth_abi(LedgerEndpoint, LedgerClient)]
pub trait Ledger {
    #[constant]
    fn ping(&mut self) -> String;

    fn counter_inc(&mut self);

    #[constant]
    fn counter_value(&mut self) -> U256;

    fn register_project(&mut self, account: Address, url: String);

    #[constant]
    fn get_project_url(&mut self, account: Address) -> String;
}

/// Implements [Ledger] backed by [Storage].
pub struct Ledger_<E: pwasm::Env> {
    pub env: Storage<E>,
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
        let data = self.env.read(COUNTER_ADDRESS.as_ref());
        let counter = U256::from(data.as_slice());
        self.env
            .write(COUNTER_ADDRESS.as_ref(), H256::from(counter + 1).as_ref());
    }

    fn counter_value(&mut self) -> U256 {
        let data = self.env.read(COUNTER_ADDRESS.as_ref());
        U256::from(data.as_slice())
    }

    fn register_project(&mut self, account: Address, url: String) {
        self.env.write(account.as_ref(), url.as_ref())
    }

    fn get_project_url(&mut self, account: Address) -> String {
        String::from_utf8(self.env.read(account.as_ref())).unwrap()
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

    #[test]
    fn register_project() {
        let mut ledger = new_ledger();
        let account = Address::zero();
        let url = "https://example.com";
        ledger.register_project(account, url.into());
        let expected_url = ledger.get_project_url(account);
        assert_eq!(url, expected_url);
    }

    fn new_ledger() -> Ledger_<pwasm::TestEnv> {
        Ledger_ {
            env: Storage {
                env: pwasm::TestEnv::new(),
            },
        }
    }
}
