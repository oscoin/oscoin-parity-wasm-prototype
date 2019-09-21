//! Register a project with a URL and verifies that the project URL is set.
//!
//! This is a copy of a test case in `./tests/end_to_end.rs`.
use futures::Future;
use oscoin_client::Client;

fn main() {
    let client = Client::new_from_file().unwrap();

    let sender = client.new_account().wait().unwrap();
    let name = "monokol";
    let description = "Looking glass into the future.";
    let img_url = "https://monok.el/img/logo.svg";
    let project_id = client
        .register_project(
            sender,
            name.to_owned(),
            description.to_owned(),
            img_url.to_owned(),
        )
        .wait()
        .unwrap();
    let project = client.get_project(project_id).wait().unwrap().unwrap();
    assert_eq!(project.name, name);
    assert_eq!(project.description, description);
    assert_eq!(project.img_url, img_url);
    assert_eq!(project.members, vec![sender.to_fixed_bytes()]);
}
