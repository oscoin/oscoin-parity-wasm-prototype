//! End-to-end tests for the ledger and the client.
//!
//! Requires running a node with `./dev-node/run` and compiling the ledger withs
//! `./tools/build-ledger-wasm`.
//!
//! The tests will deploy the ledger contract to the node and submit transactions to it to test the
//! counter.
use oscoin_deploy::dev_account_address;
use web3::futures::Future;

#[test]
fn counter() {
    let ledger = oscoin_deploy::deploy().unwrap();
    let client = oscoin_client::Client::new(ledger.address());

    for _ in 0..10 {
        client.counter_inc(dev_account_address()).wait().unwrap();
    }
    let counter = client.counter_value().wait().unwrap();
    assert_eq!(counter, 10);
}

#[test]
fn register_project() {
    let ledger = oscoin_deploy::deploy().unwrap();
    let client = oscoin_client::Client::new(ledger.address());

    let sender = client.new_account().wait().unwrap();

    let url = "https://example.com";
    client
        .register_project(sender, dev_account_address(), url.to_string())
        .wait()
        .unwrap();
    let url2 = client
        .get_project_url(dev_account_address())
        .wait()
        .unwrap();
    assert_eq!(url, url2);
}
