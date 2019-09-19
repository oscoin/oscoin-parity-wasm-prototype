#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![feature(alloc_prelude)]
extern crate alloc;

use alloc::prelude::v1::*;
use alloc::vec;

use crate::pwasm::{Address, String};

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

    fn register_project(
        &mut self,
        name: String,
        description: String,
        img_url: String,
    ) -> ProjectId {
        let members = vec![self.env.sender().to_fixed_bytes()];
        let id = compute_project_id(self.env.sender(), self.env.block_number());
        self.storage().write(
            &id,
            &Project {
                name,
                description,
                img_url,
                members,
            },
        );
        id
    }

    fn get_project(&mut self, account: ProjectId) -> Option<Project> {
        self.storage().read::<Project>(&account).unwrap()
    }
}

/// Computes the ID of a project registered by `creator` in the given block through a cryptographic
/// hash.
///
/// *FIXME* Two project registration transactions from the same author in the same block will
/// result in the same project ID. In this case the first project is overwritten. We could prevent
/// this by including the account nonce in the computation but this is unavialble from the Pwasm
/// environment.
pub fn compute_project_id(creator: Address, block_number: u64) -> ProjectId {
    let mut data = Vec::from(creator.as_bytes());
    data.extend_from_slice(&block_number.to_be_bytes());
    let hash = pwasm_std::keccak(&data);
    let mut project_id: [u8; 20] = Default::default();
    project_id.copy_from_slice(&hash[0..20]);
    project_id
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

        let name = "monokol";
        let description = "Looking glass into the future.";
        let img_url = "https://monok.el/img/logo.svg";
        let project_id =
            ledger.register_project(name.to_owned(), description.to_owned(), img_url.to_owned());
        let project = ledger.get_project(project_id).unwrap();

        assert_eq!(project.name, name);
        assert_eq!(project.description, description);
        assert_eq!(project.img_url, img_url);
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
