use std::convert::From;
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

use ethereum_types::U64;
use futures::future::Future;
use web3::transports::http::Http;
use web3::transports::EventLoopHandle;
use web3::types::TransactionReceipt;
pub use web3::types::{Address, H256, U256};
use web3::Web3;

use oscoin_ledger::{
    compute_project_id, Call as LedgerCall, ProjectId, Query as LedgerQuery, Update as LedgerUpdate,
};

/// URL pointing to a parity ethereum node running on localhost.
///
/// This is the URL used by the client. It is currently not possible to change it.
const LOCALHOST_NODE_URL: &str = "http://localhost:8545";

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

#[derive(Debug)]
pub enum Error {
    /// Returns the ID of the failed transaction.
    /// Transaction failure is signaled by the `status` field in
    /// `TransactionReceipt.`
    TransactionFailure(H256),
    Web3(web3::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TransactionFailure(hash) => write!(
                f,
                "Transaction execution failure. Transaction ID is: {}",
                hash
            ),
            Self::Web3(web3_error) => fmt::Display::fmt(&web3_error, f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::TransactionFailure(_) => None,
            Self::Web3(web3_error) => Some(web3_error),
        }
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
    ledger_address: Address,
}

// Public methods
impl Client {
    /// Creates a new client calling the ledger at the given contract address.
    pub fn new(ledger_address: Address) -> Client {
        let (event_loop_handle, http) = web3::transports::Http::new(LOCALHOST_NODE_URL)
            .expect("Node URL is hardcoded and valid");
        let web3 = web3::Web3::new(http);
        Client {
            _event_loop_handle: event_loop_handle,
            web3,
            ledger_address,
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
        self.query(LedgerQuery::Ping)
    }

    pub fn counter_value(&self) -> QueryResult<u32> {
        self.query(LedgerQuery::CounterValue)
    }

    pub fn counter_inc(&self, sender: Address) -> SubmitResult {
        self.submit(sender, LedgerUpdate::CounterInc)
    }

    pub fn register_project<'a>(
        &'a self,
        sender: Address,
        name: String,
        description: String,
        img_url: String,
    ) -> impl Future<Item = ProjectId, Error = Error> + 'a {
        self.submit(
            sender,
            LedgerUpdate::RegisterProject {
                name,
                description,
                img_url,
            },
        )
        .map(move |receipt| {
            let block = receipt
                .block_number
                .expect("Receipt must have block number");
            compute_project_id(sender.as_fixed_bytes().into(), block.as_u64())
        })
    }

    pub fn get_project(
        &self,
        project_id: ProjectId,
    ) -> QueryResult<Option<oscoin_ledger::Project>> {
        self.query(LedgerQuery::GetProject { project_id })
    }
}

// Private methods
impl Client {
    /// Queries the ledger contract by calling a method with the given parameters.
    fn query<R: serde::de::DeserializeOwned + 'static>(
        &self,
        query: LedgerQuery,
    ) -> QueryResult<R> {
        let data = LedgerCall::Query(query).serialize();
        let future = self
            .web3
            .eth()
            .call(
                web3::types::CallRequest {
                    from: None,
                    to: self.ledger_address,
                    gas: None,
                    gas_price: None,
                    value: None,
                    data: Some(web3::types::Bytes(data)),
                },
                None,
            )
            .and_then(|web3::types::Bytes(vec)| {
                serde_cbor::from_slice(&vec).map_err(|err| {
                    web3::error::Error::InvalidResponse(format!(
                        "Failed to decode CBOR response: {}",
                        err
                    ))
                })
            });
        QueryResult {
            future: Box::new(future),
        }
    }

    /// Submit a ledger transaction that calls the given method with the given parameters on the
    /// ledger contract.
    ///
    /// Note that an error is only visible as a zero status in the [TransactionReceipt].
    fn submit(&self, sender: Address, update: LedgerUpdate) -> SubmitResult {
        let data = LedgerCall::Update(update).serialize();
        let transaction_request = web3::types::TransactionRequest {
            from: sender,
            to: Some(self.ledger_address),
            gas: None,
            gas_price: None,
            value: None,
            nonce: None,
            data: Some(web3::types::Bytes(data)),
            condition: None,
        };

        let poll_interval = core::time::Duration::from_secs(1);
        let future = self
            .web3
            .personal()
            .sign_transaction(transaction_request, "")
            .and_then(move |signed_tx| {
                web3::confirm::send_raw_transaction_with_confirmation(
                    self.web3.transport().clone(),
                    signed_tx.raw,
                    poll_interval,
                    0,
                )
            })
            .map_err(Error::Web3)
            .and_then(move |tx_receipt| match tx_receipt.status {
                Some(U64([0])) => Err(Error::TransactionFailure(tx_receipt.transaction_hash)),
                _ => Ok(tx_receipt),
            });

        SubmitResult {
            future: Box::new(future),
        }
    }
}

/// Returned by queries to the ledger contract.
///
/// The [Future] interfaces allows one to retrieve the result of the query.
pub struct QueryResult<'a, T> {
    future: Box<dyn Future<Item = T, Error = web3::error::Error> + 'a>,
}

impl<'a, T> Future for QueryResult<'a, T> {
    type Item = T;
    type Error = web3::error::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        self.future.poll()
    }
}

/// Returned by the client when submiting transactions to the ledger.
///
/// The [Future] interfaces allows one to retrieve the result of the query.
pub struct SubmitResult<'a> {
    future: Box<dyn Future<Item = TransactionReceipt, Error = Error> + 'a>,
}

impl<'a> Future for SubmitResult<'a> {
    type Item = TransactionReceipt;
    type Error = Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        self.future.poll()
    }
}

/// [Future] for API call results with error [web3::error::Error].
pub type CallFuture<T> = web3::helpers::CallFuture<T, <Http as web3::Transport>::Out>;
