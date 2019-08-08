//! Command line tool to deploy a Wasm contract to a parity ethereum dev node.
use env_logger;
use std::fs;

use clap::crate_version;
use clap::App;

fn main() {
    env_logger::init();

    let _matches = App::new("Oscoin Ledger Deployment")
        .version(crate_version!())
        .max_term_width(80)
        .about(format!(
            "\nDeploys the Wasm contract in {}. Outputs the address of ledger contract and writes it to {}",
            oscoin_deploy::CONTRACT_CODE_PATH,
            oscoin_deploy::CONTRACT_ADDRESS_FILE
            ).as_ref())
        .get_matches();

    let contract = oscoin_deploy::deploy().unwrap();

    let contract_address_hex = hex::encode(contract.address());
    println!("Ledger contract address: {}", contract_address_hex);
    fs::write(oscoin_deploy::CONTRACT_ADDRESS_FILE, contract_address_hex).unwrap();
    println!(
        "Ledger contract address written to {}",
        oscoin_deploy::CONTRACT_ADDRESS_FILE
    );
}
