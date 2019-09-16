//! Access to Parity Wasm primitives
//!
//! [pwasm::Env] defines a trait for all methods the ledger runtime needs to provide. When the ledger runs
//! as a Parity Wasm Smart Contract thhe functionality is provided by `Pwasm` using the
//! `pwasm_ethereum` crate.
//!
//! For testing purposes a [pwasm::TestEnv] implementation is provided.
//!
//! Oscoin Ledger code must not use `pwasm_ethereum` directly. It must only use functionality
//! exposed by [pwasm::Env].

#[doc(inline)]
pub use pwasm_abi::types::*;

/// Primitives allowing access to the smart contract environment.
pub trait Env {
    fn write(&mut self, key: &H256, value: &[u8; 32]);
    fn read(&self, key: &H256) -> [u8; 32];
    fn sender(&self) -> Address;
    fn block_number(&self) -> u64;
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

    fn sender(&self) -> Address {
        pwasm_ethereum::sender()
    }

    fn block_number(&self) -> u64 {
        pwasm_ethereum::block_number()
    }
}

#[cfg(any(feature = "std", test))]
pub use test_env::*;

#[cfg(any(feature = "std", test))]
mod test_env {
    use super::*;

    use std::collections::HashMap;

    /// Implements [Env] using a [HashMap].
    ///
    /// Create an empty [TestEnv] with
    /// ```
    /// # use oscoin_ledger::pwasm::*;
    /// let testEnv = TestEnv::default();
    /// ```
    #[derive(Default)]
    pub struct TestEnv {
        state: HashMap<H256, [u8; 32]>,
        pub sender: Address,
        pub block_number: u64,
    }

    impl TestEnv {
        pub fn new() -> TestEnv {
            Default::default()
        }
    }

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

        fn sender(&self) -> Address {
            self.sender
        }

        fn block_number(&self) -> u64 {
            self.block_number
        }
    }
}
