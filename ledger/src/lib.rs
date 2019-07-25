#![no_std]
#![allow(non_snake_case)]
#![feature(proc_macro_hygiene)]

use pwasm_abi::types::String;
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
}

pub struct Ledger_;

impl Ledger for Ledger_ {
    fn ping(&mut self) -> String {
        String::from("pong")
    }
}
