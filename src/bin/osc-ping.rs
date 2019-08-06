///! Calls the oscoin ledger contract’s ping method and returns the output.
use env_logger;

use web3::futures::Future;

use clap::crate_version;
use clap::App;

use oscoin_client::Client;

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

    let client = Client::new_from_file().unwrap();
    let pong = client.ping().wait().unwrap();
    println!("{}", pong);
}
