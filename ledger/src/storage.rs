use crate::pwasm;
use crate::pwasm::{Vec, H256, U256};

/// Number of bytes that can be stored with the pwasm environment
const CHUNK_SIZE: usize = 32;

/// Simple key-value store for serializable data that is backed by [pwasm::Env].
///
/// ```
/// # use oscoin_ledger::storage::Storage;
/// let mut test_env = oscoin_ledger::pwasm::TestEnv::new();
/// let mut storage = Storage::new(&mut test_env);
/// let vec = Vec::from(b"abcdef" as &[u8]);
/// storage.write(b"key", &vec);
/// assert_eq!(Some(vec), storage.read(b"key").unwrap());
/// ```
///
/// # Implementation
///
/// [Storage] is implemented on top of [pwasm::Env] which provides a key-value store for fixed 32
/// byte keys and values. To store a key-value pair  we first serialize the value to `Vec<u8>`.
/// Then we compute the 32 byte key (say `0x123`) by hashing the storage key. We then store the
/// length of the serialized data as the value at `0x123`. (This requires us to expand `u32` to
/// 32 bytes). Then we store the serialized data in the subsequent keys, that is `0x124`, `0x125`,
/// etc. Reading a value reverses this process.
///
/// This mechanism is similar to [what Solidity does][solidity-store].
///
/// [solidity-store]: https://medium.com/@hayeah/diving-into-the-ethereum-vm-the-hidden-costs-of-arrays-28e119f04a9b
///
pub struct Storage<'a> {
    env: &'a mut dyn pwasm::Env,
}

impl<'a> Storage<'a> {
    pub fn new(env: &mut dyn pwasm::Env) -> Storage {
        Storage { env }
    }

    pub fn read<T: serde::de::DeserializeOwned>(
        &mut self,
        key: &[u8],
    ) -> serde_cbor::Result<Option<T>> {
        let data = self.read_bytes(key);
        if data.is_empty() {
            Ok(None)
        } else {
            serde_cbor::from_slice(&data)
        }
    }

    pub fn write<T: serde::Serialize>(&mut self, key: &[u8], value: &T) {
        let data = serde_cbor::to_vec(value).expect("Serialization can never fail");
        self.write_bytes(key, &data)
    }

    fn write_bytes(&mut self, key: &[u8], value: &[u8]) {
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

    fn read_bytes(&mut self, key: &[u8]) -> Vec<u8> {
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
