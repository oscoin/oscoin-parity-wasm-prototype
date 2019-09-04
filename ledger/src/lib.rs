#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![feature(alloc_prelude)]
extern crate alloc;

use crate::pwasm::{String, H256, U256};

pub mod interface;
pub mod pwasm;
pub mod storage;

use interface::dispatch;
pub use interface::{Call, Ledger, ProjectId, Query, Update};
use storage::Storage;

pub fn call() {
    let ledger = Ledger_ {
        env: Storage::new(pwasm::Pwasm),
    };
    let call_result = Call::deserialize(pwasm_ethereum::input().as_slice());
    let call = match call_result {
        Ok(call) => call,
        Err(err) => {
            panic!("Failed to deserialize ledger call: {}", err);
        }
    };
    let response = dispatch(ledger, call);
    pwasm_ethereum::ret(&response);
}

/// Implements [Ledger] backed by [Storage].
pub struct Ledger_ {
    pub env: Storage,
}

lazy_static::lazy_static! {
    static ref COUNTER_ADDRESS: H256 =
        H256::from(
            [2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
        );
}

impl Ledger for Ledger_ {
    fn ping(&mut self) -> String {
        String::from("pong")
    }

    fn counter_inc(&mut self) {
        let data = self.env.read(COUNTER_ADDRESS.as_ref());
        let counter = U256::from(data.as_slice());
        self.env
            .write(COUNTER_ADDRESS.as_ref(), H256::from(counter + 1).as_ref());
    }

    fn counter_value(&mut self) -> u32 {
        let data = self.env.read(COUNTER_ADDRESS.as_ref());
        U256::from(data.as_slice()).low_u32()
    }

    fn register_project(&mut self, account: ProjectId, url: String) {
        self.env.write(account.as_ref(), url.as_ref())
    }

    fn get_project_url(&mut self, account: ProjectId) -> String {
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
        assert_eq!(counter, 10);
    }

    #[test]
    fn counter_default() {
        let mut ledger = new_ledger();
        let counter = ledger.counter_value();
        assert_eq!(counter, 0);
    }

    #[test]
    fn register_project() {
        let mut ledger = new_ledger();
        let account = [0u8; 20];
        let url = "https://example.com";
        ledger.register_project(account, url.into());
        let expected_url = ledger.get_project_url(account);
        assert_eq!(url, expected_url);
    }

    fn new_ledger() -> Ledger_ {
        Ledger_ {
            env: Storage::new(pwasm::TestEnv::new()),
        }
    }
}
