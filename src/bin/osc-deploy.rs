//! Command line tool to deploy a Wasm contract to a parity ethereum dev node.
use env_logger;
use std::fs;

use web3::api::Eth;
use web3::contract::{Contract, Options};
use web3::futures::Future;
use web3::transports::http::Http;
use web3::Web3;

use clap::crate_version;
use clap::App;

/// Path to the contract Wasm code.
const CONTRACT_CODE_PATH: &str = "./target/oscoin_ledger.wasm";

/// Maximum gas used to deploy the contract
const DEPLOY_GAS: u32 = 8_000_000;

/// Contract ABI JSON. This is empty because there are no constructor arguments.
const CONTRACT_ABI: &[u8] = b"[]";

fn main() {
    env_logger::init();

    let _matches = App::new("Oscoin Ledger Deployment")
        .version(crate_version!())
        .max_term_width(80)
        .about(format!(
            "\nDeploys the Wasm contract in {}. Outputs the address of ledger contract and writes it to {}",
            CONTRACT_CODE_PATH,
            oscoin::deploy::CONTRACT_ADDRESS_FILE
            ).as_ref())
        .get_matches();

    let web3 = prepare_web3();

    oscoin::deploy::unlock_dev_account(&web3);

    let contract = deploy(web3.eth());

    let contract_address_hex = hex::encode(contract.address());
    println!("Ledger contract address: {}", contract_address_hex);
    fs::write(oscoin::deploy::CONTRACT_ADDRESS_FILE, contract_address_hex).unwrap();
    println!(
        "Ledger contract address written to {}",
        oscoin::deploy::CONTRACT_ADDRESS_FILE
    );
}

fn deploy(eth: Eth<Http>) -> Contract<Http> {
    let contract_code_hex = hex::encode(fs::read(CONTRACT_CODE_PATH).unwrap());

    let builder = Contract::deploy(eth, CONTRACT_ABI)
        .expect("contract ABI is hardcoded and valid")
        .confirmations(0)
        .options(Options::with(|opt| {
            opt.gas = Some(DEPLOY_GAS.into());
        }));

    let pending_contract = builder
        .execute(contract_code_hex, (), oscoin::deploy::dev_account_addr())
        .expect("Correct parameters are passed to the constructor.");

    pending_contract.wait().unwrap()
}

fn prepare_web3() -> Web3<Http> {
    let (eloop, http) =
        web3::transports::Http::new(oscoin::deploy::NODE_URL).expect("URL is hardcoded and valid");
    // run the event loop in the background
    eloop.into_remote();
    web3::Web3::new(http)
}
