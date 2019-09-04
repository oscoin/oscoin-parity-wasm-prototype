#![no_std]

#[no_mangle]
pub fn call() {
    oscoin_ledger::call();
}

#[no_mangle]
pub fn deploy() {}
