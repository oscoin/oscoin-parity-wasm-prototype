//! End-to-end tests for the ledger and the client.
//!
//! Requires running a node with `./dev-node/run` and compiling the ledger withs
//! `./tools/build-ledger-wasm`.
//!
//! The tests will deploy the ledger contract to the node and submit transactions to it to test the
//! counter.
use oscoin_deploy::dev_account_address;
use web3::futures::Future;
use web3::types::U256;

#[test]
fn counter() {
    let ledger = oscoin_deploy::deploy().unwrap();
    let client = oscoin_client::Client::new(dev_account_address(), ledger.address());

    for _ in 0..10 {
        client.counter_inc().wait().unwrap();
    }
    let counter = client.counter_value().wait().unwrap();
    assert_eq!(counter, U256::from(10));
}

#[test]
fn register_project() {
    let ledger = oscoin_deploy::deploy().unwrap();
    let client = oscoin_client::Client::new(dev_account_address(), ledger.address());

    let url = "https://example.com";
    client
        .register_project(dev_account_address(), url.to_string())
        .wait()
        .unwrap();
    let url2 = client
        .get_project_url(dev_account_address())
        .wait()
        .unwrap();
    assert_eq!(url, url2);
}
