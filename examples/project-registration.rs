//! Register a project with a URL and verifies that the project URL is set.
//!
//! This is a copy of a test case in `./tests/end_to_end.rs`.
use futures::Future;
use oscoin_client::{Address, Client};

fn main() {
    let client = Client::new_from_file().unwrap();

    let project_address = Address::zero();
    let sender = client.new_account().wait().unwrap();
    let url = "https://example.com";
    client
        .register_project(sender, project_address, url.to_string())
        .wait()
        .unwrap();
    let project = client.get_project(project_address).wait().unwrap().unwrap();
    assert_eq!(url, project.url);
}
