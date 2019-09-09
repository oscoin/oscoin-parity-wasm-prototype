//! Register a project with a URL and verifies that the project URL is set.
//!
//! This is a copy of a test case in `./tests/end_to_end.rs`.
use futures::Future;
use oscoin_client::Client;

fn main() {
    let client = Client::new_from_file().unwrap();

    let sender = client.new_account().wait().unwrap();
    let url = "https://example.com";
    let project_id = client
        .register_project(sender, url.to_string())
        .wait()
        .unwrap();
    let project = client.get_project(project_id).wait().unwrap().unwrap();
    assert_eq!(url, project.url);
}
