//! Access to Parity Wasm primitives
//!
//! [Env] defines a trait for all methods the ledger runtime needs to provide. When the ledger runs
//! as a Parity Wasm Smart Contract thhe functionality is provided by `Pwasm` using the
//! `pwasm_ethereum` crate.
//!
//! For testing purposes a [TestEnv] implementation is provided.
//!
//! Oscoin Ledger code must not use `pwasm_ethereum` directly. It must only use functionality
//! exposed by [Env].

#[doc(inline)]
pub use pwasm_abi::types::*;

/// Primitives allowing access to the smart contract environment.
pub trait Env {
    fn write(&mut self, key: &H256, value: &[u8; 32]);
    fn read(&self, key: &H256) -> [u8; 32];
}

/// Implements [Env] for the Parity Wasm Smart Contract environment using the `pwasm_ethereum` crate.
pub struct Pwasm;

impl Env for Pwasm {
    fn write(&mut self, key: &H256, value: &[u8; 32]) {
        pwasm_ethereum::write(key, value)
    }

    fn read(&self, key: &H256) -> [u8; 32] {
        pwasm_ethereum::read(key)
    }
}

#[cfg(any(feature = "std", test))]
use std::collections::HashMap;

/// Implements [Env] using a [HashMap].
///
/// Create an empty [TestEnv] with
/// ```
/// # use oscoin_ledger::pwasm::*;
/// let testEnv = TestEnv::default();
/// ```
#[cfg(any(feature = "std", test))]
#[derive(Default)]
pub struct TestEnv {
    state: HashMap<H256, [u8; 32]>,
}

#[cfg(any(feature = "std", test))]
impl TestEnv {
    pub fn new() -> TestEnv {
        Default::default()
    }
}

#[cfg(any(feature = "std", test))]
impl Env for TestEnv {
    fn write(&mut self, key: &H256, value: &[u8; 32]) {
        self.state.insert(*key, *value);
    }

    fn read(&self, key: &H256) -> [u8; 32] {
        match self.state.get(key) {
            Some(value) => *value,
            None => Default::default(),
        }
    }
}
