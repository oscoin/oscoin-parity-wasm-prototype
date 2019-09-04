#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![feature(alloc_prelude)]
extern crate alloc;

use crate::pwasm::String;

pub mod interface;
pub mod pwasm;
pub mod storage;

use interface::dispatch;
pub use interface::{Call, Ledger, Project, ProjectId, Query, Update};
use storage::Storage;

pub fn call() {
    let ledger = Ledger_ {
        storage: Storage::new(pwasm::Pwasm),
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
    pub storage: Storage,
}

const COUNTER_KEY: &[u8] = b"counter";

impl Ledger for Ledger_ {
    fn ping(&mut self) -> String {
        String::from("pong")
    }

    fn counter_inc(&mut self) {
        let val: u32 = self.storage.read(COUNTER_KEY).unwrap().unwrap_or(0);
        self.storage.write(COUNTER_KEY, &(val + 1));
    }

    fn counter_value(&mut self) -> u32 {
        self.storage.read(COUNTER_KEY).unwrap().unwrap_or(0)
    }

    fn register_project(&mut self, account: ProjectId, url: String) {
        self.storage.write(&account, &Project { url });
    }

    fn get_project(&mut self, account: ProjectId) -> Option<Project> {
        self.storage.read::<Project>(&account).unwrap()
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
        let project = ledger.get_project(account).unwrap();
        assert_eq!(url, project.url);
    }

    fn new_ledger() -> Ledger_ {
        Ledger_ {
            storage: Storage::new(pwasm::TestEnv::new()),
        }
    }
}
