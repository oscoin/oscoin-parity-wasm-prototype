Oscoin Parity Ethereum WASM prototype
=====================================

This repository hosts a prototype of Oscoin's Ledger API using WASM and Parity Ethereum.

<details>
  <summary>Table of contents</summary>

<!-- toc -->

- [Requirements](#requirements)
- [Deploying the Ledger](#deploying-the-ledger)
- [Using the Client](#using-the-client)
- [Commands and Tools](#commands-and-tools)
    + [`osc-ping`](#osc-ping)
    + [`osc-deploy` from `oscoin_deploy` crate](#osc-deploy-from-oscoin_deploy-crate)
    + [`./tools/build-ledger-wasm`](#toolsbuild-ledger-wasm)
- [Testing](#testing)

<!-- tocstop -->

</details>


Requirements
------------

* [`rustup`](https://github.com/rust-lang/rustup.rs/)
* [Latest version][peth-release] of the Parity Ethereum node on the PATH
* `cargo install pwasm-utils-cli --version 0.6.0 --bin wasm-build`
* `rustup target add wasm32-unknown-unknown`

[peth-release]: https://github.com/paritytech/parity-ethereum/releases/latest

Deploying the Ledger
--------------------

1. Run a development node with `./dev-node/run`.
1. Build the ledger with `./tools/build-ledger-wasm`
1. Deploy the ledger to the node with `cargo run --package oscoin_deploy`. This will
   write the contract address to `.oscoin_ledger_address`.
1. Test the ledger with `cargo run --bin osc-ping`.

Using the Client
----------------

The `oscoin_client` package in `./client` provides an API to read and manipulate
the ledger hosted on a Parity Ethereum node.

To use the client you need the `.oscoin_ledger_address` in your current working
directory. This file is created by `osc-deploy`.

~~~rust
let client = oscoin_client::Client::new_from_file().unwrap();
let sender = client.new_account().wait().unwrap();
let project_address = Address::zero();
let url = "https://example.com";
client
    .register_project(sender, project_address, url.to_string())
    .wait()
    .unwrap();
~~~

You can find a full example in `examples/project-registration.rs`

Account management is currently handled by the Parity Ethereum node.

Commands and Tools
------------------

#### `osc-ping`

Calls the ledger’s `ping` method and prints the result.

#### `osc-deploy` from `oscoin_deploy` crate

Deploys the ledger contract and sets the ledger contract address.

#### `./tools/build-ledger-wasm`

Build the ledger contract Wasm code and output it to `./target/oscoin_ledger.wasm`.

Testing
-------

To run the tests
1. Build the ledger with `./tools/build-ledger-wasm`
2. Run the dev node with `./dev-node/run`
3. Run `cargo test --all -- --test-threads=1`. (We need to run the test single
   threaded because of [issue #13][issue-13])

[issue-13]: https://github.com/oscoin/oscoin-parity-wasm-prototype/issues/13
