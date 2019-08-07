///! Client library for talking
///
/// # Getting Started
///
/// ```
/// let client = Client::new_from_file();
/// client.ping().wait().unwrap();
/// ```
use std::error;
use std::fmt;
use std::str::FromStr;

use web3::contract::{Contract, Options};
use web3::transports::http::Http;
use web3::transports::EventLoopHandle;
use web3::types::Address;
use web3::Transport;

/// URL pointing to a parity ethereum node running on localhost.
///
/// This is the URL used by the client. It is currently not possible to change it.
const LOCALHOST_NODE_URL: &str = "http://localhost:8545";

const CONTRACT_ABI_JSON: &[u8] = include_bytes!("../../target/json/Ledger.json");

/// File Path to load and store the ledger contract address to. Is `./.oscoin_ledger_address`.
pub const CONTRACT_ADDRESS_FILE: &str = "./.oscoin_ledger_address";

/// Error returned when reading the contract address from a file fails.
#[derive(Debug)]
pub enum ReadContractAddressError {
    HexError(rustc_hex::FromHexError),
    IoError(std::io::Error),
}

impl fmt::Display for ReadContractAddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HexError(hex_error) => {
                write!(f, "failed to decode address from hex: {}", hex_error)
            }
            Self::IoError(io_error) => write!(
                f,
                "failed read address from file {}: {}",
                CONTRACT_ADDRESS_FILE, io_error
            ),
        }
    }
}

impl error::Error for ReadContractAddressError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::HexError(hex_error) => Some(hex_error),
            Self::IoError(io_error) => Some(io_error),
        }
    }
}

impl From<rustc_hex::FromHexError> for ReadContractAddressError {
    fn from(hex_error: rustc_hex::FromHexError) -> ReadContractAddressError {
        ReadContractAddressError::HexError(hex_error)
    }
}

impl From<std::io::Error> for ReadContractAddressError {
    fn from(io_error: std::io::Error) -> ReadContractAddressError {
        ReadContractAddressError::IoError(io_error)
    }
}

pub fn read_contract_address() -> Result<Address, ReadContractAddressError> {
    let contract_address_hex = std::fs::read_to_string(CONTRACT_ADDRESS_FILE)?;
    Address::from_str(&contract_address_hex).map_err(ReadContractAddressError::HexError)
}

pub type QueryResult<T> = web3::contract::QueryResult<T, <Http as Transport>::Out>;

/// If a client is dropped the IO event loop is dropped, too and the client requests will error.
pub struct Client {
    _event_loop_handle: EventLoopHandle,
    contract: Contract<Http>,
}

impl Client {
    /// Creates a new client calling the ledger at the given contract address.
    pub fn new(contract_address: Address) -> Client {
        let (event_loop_handle, http) = web3::transports::Http::new(LOCALHOST_NODE_URL)
            .expect("Node URL is hardcoded and valid");
        let web3 = web3::Web3::new(http);
        let contract_abi =
            ethabi::Contract::load(CONTRACT_ABI_JSON).expect("ABI is hardcoded and valid");
        let contract = Contract::new(web3.eth(), contract_address, contract_abi);
        Client {
            _event_loop_handle: event_loop_handle,
            contract,
        }
    }

    /// Creates a new client using the contract address stored in [CONTRACT_ADDRESS_FILE].
    pub fn new_from_file() -> Result<Client, ReadContractAddressError> {
        let contract_address = read_contract_address()?;
        Ok(Self::new(contract_address))
    }

    pub fn ping(&self) -> QueryResult<String> {
        self.contract
            .query("ping", (), None, Options::default(), None)
    }
}
