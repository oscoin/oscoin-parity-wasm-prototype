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

    let name = "monokol";
    let description = "Looking glass into the future.";
    let img_url = "https://monok.el/img/logo.svg";
    let project_id = client
        .register_project(
            sender,
            name.to_string(),
            description.to_string(),
            img_url.to_string(),
        )
        .wait()
        .unwrap();

    let project = client.get_project(project_id).wait().unwrap().unwrap();

    assert_eq!(project.name, name);
    assert_eq!(project.description, description);
    assert_eq!(project.img_url, img_url);
    assert_eq!(project.members, vec![sender.to_fixed_bytes()]);
}
