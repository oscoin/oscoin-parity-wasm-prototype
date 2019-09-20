//! End-to-end tests for the ledger and the client.
//!
//! Requires running a node with `./dev-node/run` and compiling the ledger withs
//! `./tools/build-ledger-wasm`.
//!
//! The tests will deploy the ledger contract to the node and submit transactions to it to test the
//! counter.

use oscoin_client::AccountId;
use oscoin_deploy::dev_account_address;
use std::collections::BTreeSet;
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
    let project_id = client
        .register_project(sender, url.to_string())
        .wait()
        .unwrap();
    let project = client.get_project(project_id).wait().unwrap().unwrap();

    assert_eq!(url, project.url);
    assert_eq!(project.members, vec![sender.to_fixed_bytes()]);
}

#[test]
fn list_projects() {
    let ledger = oscoin_deploy::deploy().unwrap();
    let client = oscoin_client::Client::new(ledger.address());

    let sender = client.new_account().wait().unwrap();

    let url_vec: Vec<String> = (0..9)
        .map(|ix| "https://examples".to_string() + &ix.to_string() + ".com")
        .collect();
    let url_set: BTreeSet<String> = url_vec.iter().cloned().collect();

    for url in url_vec.iter().take(9) {
        client
            .register_project(sender, url.to_string())
            .wait()
            .unwrap();
    }

    let project_list = client.list_projects().wait().unwrap();

    // Check that URLs of every listed project match those that were used
    // in the start.
    // Sets are used for ease of comparison and to remove duplicates.
    assert_eq!(
        url_set,
        project_list
            .clone()
            .iter()
            .map(|project| { project.url.clone() })
            .collect()
    );

    let vec: Vec<AccountId> = vec![sender.to_fixed_bytes()];
    let mut member_vec_set = BTreeSet::new();
    member_vec_set.insert(vec);

    // Check that the members of the listed projects correspond to those
    // that were registered in the ledger.
    assert_eq!(
        project_list
            .iter()
            .map(|project| { project.members.clone() })
            .collect::<BTreeSet<Vec<AccountId>>>(),
        member_vec_set
    )
}
