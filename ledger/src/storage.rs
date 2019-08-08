//! Storage backend for [Ledger_] that is more powerful than the primitive Pwasm ethereum storage.
//!
//! The storage provided by the [pwasm] primitives is limited to 32 byte values. [Storage] provides
//! a storage that can store arbitrarily large values.
//!
//! ```
//! # use oscoin_ledger::storage::Storage;
//! let mut storage = Storage { env: oscoin_ledger::pwasm::TestEnv::new() };
//! let some_bytes = Vec::from(b"abcdef" as &[u8]);
//! storage.write("key".as_ref(), some_bytes.as_ref());
//! assert_eq!(some_bytes, storage.read("key".as_ref()));
//! ```
use crate::pwasm;
use crate::pwasm::{Vec, H256, U256};

pub struct Storage<E: pwasm::Env> {
    pub env: E,
}

/// Number of bytes that can be stored with the pwasm environment
const CHUNK_SIZE: usize = 32;

impl<E: pwasm::Env> Storage<E> {
    pub fn write(&mut self, key: &[u8], value: &[u8]) {
        let key_hash = U256::from(pwasm_std::keccak(key));
        let u256_len = U256::from(value.len());
        self.env
            .write(&H256::from(key_hash), &H256::from(u256_len).into());
        let chunks = value.chunks(CHUNK_SIZE);
        for (chunk, i) in chunks.zip(1..) {
            let fixed_chunk = padded_bytes_32(chunk);
            self.env.write(&H256::from(key_hash + i), &fixed_chunk);
        }
    }

    pub fn read(&mut self, key: &[u8]) -> Vec<u8> {
        let key_hash = pwasm_std::keccak(key);
        let mut data = Vec::new();
        let mut data_to_read = U256::from(self.env.read(&key_hash)).as_usize();
        let mut chunk_offset = U256::from(key_hash) + 1;
        while data_to_read > 0 {
            let chunk = self.env.read(&H256::from(chunk_offset));
            let len = core::cmp::min(data_to_read, CHUNK_SIZE);
            data.extend_from_slice(&chunk.as_ref()[0..len]);
            data_to_read = data_to_read.saturating_sub(CHUNK_SIZE);
            chunk_offset = chunk_offset + 1;
        }
        data
    }
}

/// Expands or shrinks a byte slice to fit into a 32 byte array.
///
/// If the slice has fewer than 32 bytes it is padded with zeroes to the right.
fn padded_bytes_32(data: &[u8]) -> [u8; 32] {
    let mut vec = Vec::from(data);
    vec.resize_with(32, Default::default);
    core::convert::TryInto::try_into(&vec[0..32]).expect("qed")
}
