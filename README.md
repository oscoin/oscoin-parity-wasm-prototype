[![Build status](https://badge.buildkite.com/67ab5ada52f2a9b06fa6ac8bf77093b5cb29dd0ef740839cd8.svg?=branch=buildkite)](https://buildkite.com/monadic/oscoin-parity-wasm-prototype)

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
- [Ledger Specification](#ledger-spec)

<!-- tocstop -->

</details>


Requirements
------------

* [`rustup`](https://github.com/rust-lang/rustup.rs/)
* [Latest version][peth-release] of the Parity Ethereum node on the PATH
* Setup Rust toolchain with `./tools/rustup-setup`
* `cargo build --package pwasm-utils-cli --bin wasm-build`

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

To compile the `oscoin_client` package a nightly Rust release is required.

To use the client you need the `.oscoin_ledger_address` in your current working
directory. This file is created by `osc-deploy`.

~~~rust
let client = Client::new_from_file().unwrap();
let sender = client.new_account().wait().unwrap();
let url = "https://example.com";
let project_id = client
    .register_project(sender, url.to_string())
    .wait()
    .unwrap();
let project = client.get_project(project_id).wait().unwrap().unwrap();
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

Ledger Specification
--------------------

In the `ledger-spec` folder, there is a Rust crate that details the Oscoin
ledger specification with traits, types and a sizable amount of documentation.

It is intended to bridge the formal description of the ledger from the
whitepaper with the ledger's future implementation, providing a "sandbox"
with which to test and discuss design ideas before implementing them in
earnest.

The `ledger-spec` crate is meant to evolve with the project, and at each point
in time its contents will reflect the team's requirements from and
understanding of the Oscoin ledger.

Note that although there is no actual implementation of any function or
datatype in the crate, it compiles and is part of the build process.

### Structure

`ledger-spec` is a library with three modules:
* `lib.rs`, defining the main traits with which to interact with the Oscoin
  ledger
* `error.rs` defining errors that may arise when interacting with the ledger.
* `types.rs`, defining the primitive types that will populate the ledger state.
