Oscoin Parity Ethereum WASM prototype
=====================================

This repository hosts a prototype of Oscoin's Ledger API using WASM and Parity Ethereum.

<details>
  <summary>Table of contents</summary>

<!-- toc -->

- [Requirements](#requirements)
- [Deploying the Ledger](#deploying-the-ledger)
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
* [Latest version][peth-release] of the Parity Ethereum node on the OATH
* `cargo install pwasm-utils-cli --version 0.6.0 --bin wasm-build`

[peth-release]: https://github.com/paritytech/parity-ethereum/releases/latest

Deploying the Ledger
--------------------

1. Run a development node with `./dev-node/run`.
1. Build the ledger with `./tools/build-ledger-wasm`
1. Deploy the ledger to the node with `cargo run --package oscoin_deploy`. This will
   write the contract address to `.oscoin_ledger_address`.
1. Test the ledger with `cargo run --bin osc-ping`.

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
3. Run `cargo test --all`
