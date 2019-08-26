///! Client library for interacting with the oscoin ledger on a Parity Ethereum node.
///
/// # Getting Started
///
/// ```no_run
/// # use futures::Future;
/// let client = oscoin_client::Client::new_from_file().unwrap();
/// client.ping().wait().unwrap();
/// ```
use std::error;
use std::fmt;
use std::str::FromStr;

use futures::future::Future;
use web3::contract::tokens::{Detokenize, Tokenize};
use web3::contract::{Contract, Options};
use web3::transports::http::Http;
use web3::transports::EventLoopHandle;
use web3::types::TransactionReceipt;
pub use web3::types::{Address, U256};
use web3::Web3;

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

/// Provides access to the Oscoin Ledger contract through a node.
///
/// If a client is dropped the IO event loop is dropped, too and the client requests will error.
pub struct Client {
    _event_loop_handle: EventLoopHandle,
    web3: Web3<Http>,
    contract: Contract<Http>,
}

// Public methods
impl Client {
    /// Creates a new client calling the ledger at the given contract address.
    pub fn new(ledger_contract: Address) -> Client {
        let (event_loop_handle, http) = web3::transports::Http::new(LOCALHOST_NODE_URL)
            .expect("Node URL is hardcoded and valid");
        let web3 = web3::Web3::new(http);
        let contract_abi =
            ethabi::Contract::load(CONTRACT_ABI_JSON).expect("ABI is hardcoded and valid");
        let contract = Contract::new(web3.eth(), ledger_contract, contract_abi.clone());
        Client {
            _event_loop_handle: event_loop_handle,
            web3,
            contract,
        }
    }

    /// Creates a new client using the contract address stored in [CONTRACT_ADDRESS_FILE]. See
    /// [Client::new].
    pub fn new_from_file() -> Result<Client, ReadContractAddressError> {
        let contract_address = read_contract_address()?;
        Ok(Self::new(contract_address))
    }

    pub fn new_account(&self) -> CallFuture<Address> {
        self.web3.personal().new_account("")
    }

    pub fn ping(&self) -> QueryResult<String> {
        self.query("ping", ())
    }

    pub fn counter_value(&self) -> QueryResult<U256> {
        self.query("counter_value", ())
    }

    pub fn counter_inc(&self, sender: Address) -> SubmitResult {
        self.submit(sender, "counter_inc", ())
    }

    pub fn register_project(&self, sender: Address, account: Address, url: String) -> SubmitResult {
        self.submit(sender, "register_project", (account, url))
    }

    pub fn get_project_url(&self, account: Address) -> QueryResult<String> {
        self.query("get_project_url", (account,))
    }
}

// Private methods
impl Client {
    /// Queries the ledger contract by calling a method with the given parameters.
    fn query<'a, R: Detokenize + 'a>(
        &'a self,
        method: &'a str,
        params: impl Tokenize + 'a,
    ) -> QueryResult<'a, R> {
        let future = self
            .contract
            .query(method, params, None, Options::default(), None);
        QueryResult {
            future: Box::new(future),
        }
    }

    /// Submit a ledger transaction that calls the given method with the given parameters on the
    /// ledger contract.
    ///
    /// Note that an error is only visible as a zero status in the [TransactionReceipt].
    fn submit<'a>(
        &'a self,
        sender: Address,
        method: &'a str,
        params: impl Tokenize + 'a,
    ) -> SubmitResult<'a> {
        let future = self.unlock_account_(sender).and_then(move |()| {
            self.contract
                .call_with_confirmations(method, params, sender, Options::default(), 0)
        });
        SubmitResult {
            future: Box::new(future),
        }
    }

    /// Unlock the node account used by the client.
    /// Note the `_` at the end to differentiate from the same function
    /// imported from `web3`.
    ///
    /// TODO: Panics when the unlock RPC method responds with `false`. It should result in an
    /// error.
    fn unlock_account_(
        &self,
        address: Address,
    ) -> impl Future<Item = (), Error = web3::error::Error> {
        self.web3
            .personal()
            .unlock_account(address, "", None)
            .map(move |unlocked| {
                if !unlocked {
                    // TODO turn this into an error
                    panic!("Failed to unlock account {}", address)
                }
            })
    }
}

/// Returned by queries to the ledger contract.
///
/// The [Future] interfaces allows one to retrieve the result of the query.
pub struct QueryResult<'a, T> {
    future: Box<dyn Future<Item = T, Error = web3::contract::Error> + 'a>,
}

impl<'a, T> Future for QueryResult<'a, T> {
    type Item = T;
    type Error = web3::contract::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        self.future.poll()
    }
}

/// Returned by the client when submiting transactions to the ledger.
///
/// The [Future] interfaces allows one to retrieve the result of the query.
pub struct SubmitResult<'a> {
    future: Box<dyn Future<Item = TransactionReceipt, Error = web3::Error> + 'a>,
}

impl<'a> Future for SubmitResult<'a> {
    type Item = TransactionReceipt;
    type Error = web3::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        self.future.poll()
    }
}

/// [Future] for API call results with error [web3::error::Error]
pub type CallFuture<T> = web3::helpers::CallFuture<T, <Http as web3::Transport>::Out>;
