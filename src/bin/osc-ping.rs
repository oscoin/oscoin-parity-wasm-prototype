///! Calls the oscoin ledger contract’s ping method and returns the output.
use env_logger;
use std::fs;
use std::str;
use std::str::FromStr;

use web3::contract::{Contract, Options};
use web3::futures::Future;
use web3::transports::http::Http;
use web3::types::Address;
use web3::Web3;

use clap::crate_version;
use clap::App;

/// Path to the contract Wasm code.
const CONTRACT_ABI_PATH: &str = "./target/json/Ledger.json";
const CONTRACT_ADDRESS_FILE: &str = "./.oscoin_ledger_address";

fn main() {
    env_logger::init();

    let _matches = App::new("Oscoin Ledger Deployment")
        .version(crate_version!())
        .max_term_width(80)
        .about(
            format!(
                "\nCalls the ledger’s \"ping\" method and outputs the result. Reads the ledger \
                 contract address from \"{}\"",
                oscoin::deploy::CONTRACT_ADDRESS_FILE
            )
            .as_ref(),
        )
        .get_matches();

    let web3 = prepare_web3();

    let contract_address_hex = fs::read_to_string(CONTRACT_ADDRESS_FILE).unwrap();
    let contract_address = Address::from_str(&contract_address_hex).unwrap();
    let contract_abi = fs::read(CONTRACT_ABI_PATH).unwrap();
    let contract =
        Contract::from_json(web3.eth(), contract_address, contract_abi.as_ref()).unwrap();
    let s: String = contract
        .query(
            "ping",
            (),
            oscoin::deploy::dev_account_addr(),
            Options::default(),
            None,
        )
        .wait()
        .unwrap();
    println!("{}", s);
}

fn prepare_web3() -> Web3<Http> {
    let (eloop, http) =
        web3::transports::Http::new(oscoin::deploy::NODE_URL).expect("URL is hardcoded and valid");
    // run the event loop in the background
    eloop.into_remote();
    web3::Web3::new(http)
}
