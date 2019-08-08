//! Register a project with a URL and verifies that the project URL is set.
//!
//! This is a copy of a test case in `./tests/end_to_end.rs`.
use futures::Future;
use oscoin_client::{Address, Client};

fn main() {
    let client = Client::new_from_file(oscoin_deploy::dev_account_address()).unwrap();

    let project_address = Address::zero();
    let url = "https://example.com";
    client
        .register_project(project_address, url.to_string())
        .wait()
        .unwrap();
    let url2 = client.get_project_url(project_address).wait().unwrap();
    assert_eq!(url, url2);
}
