#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![feature(alloc_prelude)]
extern crate alloc;

use alloc::prelude::v1::*;
use alloc::vec;

use crate::pwasm::String;

pub mod interface;
pub mod pwasm;
pub mod storage;

use interface::dispatch;
pub use interface::{Call, Ledger, Project, ProjectId, Query, Update};
use storage::Storage;

pub fn call() {
    let ledger = Ledger_::new(pwasm::Pwasm);
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
    env: Box<dyn pwasm::Env>,
}

impl Ledger_ {
    pub fn new(env: impl pwasm::Env + 'static) -> Ledger_ {
        Ledger_ { env: Box::new(env) }
    }

    fn storage(&mut self) -> Storage {
        Storage::new(self.env.as_mut())
    }
}

const COUNTER_KEY: &[u8] = b"counter";

impl Ledger for Ledger_ {
    fn ping(&mut self) -> String {
        String::from("pong")
    }

    fn counter_inc(&mut self) {
        let val: u32 = self.storage().read(COUNTER_KEY).unwrap().unwrap_or(0);
        self.storage().write(COUNTER_KEY, &(val + 1));
    }

    fn counter_value(&mut self) -> u32 {
        self.storage().read(COUNTER_KEY).unwrap().unwrap_or(0)
    }

    fn register_project(&mut self, account: ProjectId, url: String) {
        let members = vec![self.env.sender().to_fixed_bytes()];
        self.storage().write(&account, &Project { url, members });
    }

    fn get_project(&mut self, account: ProjectId) -> Option<Project> {
        self.storage().read::<Project>(&account).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pwasm::Address;

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
        assert_eq!(project.url, url);
        assert_eq!(project.members, vec![test_sender().to_fixed_bytes()]);
    }

    fn new_ledger() -> Ledger_ {
        let mut test_env = pwasm::TestEnv::new();
        test_env.sender = test_sender();
        Ledger_::new(test_env)
    }

    fn test_sender() -> Address {
        Address::from_low_u64_le(123_456_789)
    }
}
