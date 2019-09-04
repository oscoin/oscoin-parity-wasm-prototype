//! Deploy the ledger Wasm contract to a node.
//!
//! All of the parameters are provided as constants.
//!
//! ```no_run
//! let contract = oscoin_deploy::deploy().unwrap();
//! oscoin_deploy::write_contract_address(&contract.address());
//! ```
use std::fs;
use web3::contract::{Contract, Options};
use web3::futures::Future;
use web3::types::Address;
use web3::Web3;

/// Maximum gas used to deploy the contract
pub const DEPLOY_GAS: u32 = 100_000_000;

/// Path to the contract Wasm code. Is `./target/oscoin_ledger_pwasm.wasm`.
pub const CONTRACT_CODE_PATH: &str = "./target/oscoin_ledger_pwasm.wasm";

/// Contract ABI JSON. This is empty because there are no constructor arguments.
const CONTRACT_ABI: &[u8] = b"[]";

/// Development account address for our custom chainspec.
pub const DEV_ACCOUNT_ADDR: &str = "bcd6e47db1ac1f7f021988e20854d27778de6e4d";

/// Password for the dev account
pub const DEV_ACCOUNT_PASSWORD: &str = "";

pub const NODE_URL: &str = "http://localhost:8545";

/// File to write the address of the deployed contract to
pub const CONTRACT_ADDRESS_FILE: &str = "./.oscoin_ledger_address";

/// Deploys the contract.
///
/// 1. Read the contract code from [CONTRACT_CODE_PATH].
/// 2. Unlock [DEV_ACCOUNT_ADDR]
/// 3. Deploy the contract with [DEV_ACCOUNT_ADDR] as the sender
///
/// **Note:** This contract blocks on IO.
pub fn deploy() -> Result<Contract<web3::transports::Http>, String> {
    let web3 = prepare_web3();

    let contract_code = fs::read(CONTRACT_CODE_PATH)
        .map_err(|e| format!("Failed to read {}: {}", CONTRACT_CODE_PATH, e))?;

    let builder = Contract::deploy(web3.eth(), CONTRACT_ABI)
        .expect("contract ABI is hardcoded and valid")
        .confirmations(0)
        .options(Options::with(|opt| {
            opt.gas = Some(DEPLOY_GAS.into());
        }));

    web3.personal()
        .unlock_account(dev_account_address(), DEV_ACCOUNT_PASSWORD, None)
        .wait()
        .map_err(|e| format!("Failed to unlock dev account: {}", e))?;

    let pending_contract = builder
        .execute(hex::encode(contract_code), (), dev_account_address())
        .expect("Correct parameters are passed to the constructor.");

    let contract = pending_contract
        .wait()
        .map_err(|e| format!("Failed to create contract: {}", e))?;

    Ok(contract)
}

/// Returns the address of the dev account provided by the `oscoin` chain spec.
pub fn dev_account_address() -> Address {
    DEV_ACCOUNT_ADDR
        .parse()
        .expect("address is hardcoded and valid")
}

/// Writes contract address to [CONTRACT_ADDRESS_FILE].
pub fn write_contract_address(address: &Address) -> std::io::Result<()> {
    let contract_address_hex = hex::encode(address);
    fs::write(CONTRACT_ADDRESS_FILE, contract_address_hex)
}

fn prepare_web3() -> Web3<web3::transports::Http> {
    let (eloop, http) = web3::transports::Http::new(NODE_URL).expect("URL is hardcoded and valid");
    // run the event loop in the background
    eloop.into_remote();
    web3::Web3::new(http)
}
