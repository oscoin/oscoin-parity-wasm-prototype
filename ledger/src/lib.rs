#![no_std]
#![allow(non_snake_case)]
#![feature(proc_macro_hygiene)]

use pwasm_abi::types::{String, H256, U256};
use pwasm_abi_derive::eth_abi;
// Implements the dispatch and dispatch_ctor methods
use pwasm_abi::eth::EndpointInterface;

#[no_mangle]
pub fn call() {
    let mut endpoint = LedgerEndpoint::new(Ledger_ {});
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

pub struct Ledger_ {}

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
        let data = pwasm_ethereum::read(&COUNTER_ADDRESS);
        let counter = U256::from(data);
        pwasm_ethereum::write(&COUNTER_ADDRESS, &(counter + 1).into());
    }

    fn counter_value(&mut self) -> U256 {
        let data = pwasm_ethereum::read(&COUNTER_ADDRESS);
        U256::from(data)
    }
}
