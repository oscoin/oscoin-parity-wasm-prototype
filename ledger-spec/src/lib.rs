// These two moducratee-wide attributes are used to disable "unused variable"
// and "field is never used" warnings when compiling this file.
// Notice the bang (`!`) before the attributes - this means they are
// crate-wide because this is a library, and they are placed in its root
// (a library's crate root is always `src/lib.rs`).
#![allow(dead_code)]
#![allow(unused_variables)]

/// This is a specification document meant to approximate the Ledger described in
/// Oscoin whitepaper into concrete Rust code.
/// However, it is not meant to be an exact implementation.
///
/// It is to serve as a form of documentation that will change over
/// time with the project.
mod error;
mod types;

/// A trait exposing the Oscoin ledger transactions described in the
/// whitepaper.
///
/// The methods here return `Result<(), E>` for some error type `E` as they
/// will be applying a modification on the Ledger global state, and won't need
/// to return any actual data if they succeed.
pub trait LedgerTransactions {
    /// Transfer Oscoin from one account to another.
    ///
    /// This transaction's presence in the ledger layer is still subject to
    /// discussion.
    fn transfer_oscoin(
        from_addr: types::AccountId,
        to_addr: types::AccountId,
        amount: types::Oscoin,
    ) -> Result<(), error::TransferError>;

    /// Registering a project in the Oscoin Ledger.
    ///
    /// This transaction's inclusion in the ledger is also subject to
    /// discussion as there is still an unclear boundary between the account
    /// layer and the ledger. This matter can be revisited.
    fn register_project(
        project_address: types::AccountId,
        project_source_url: types::URL,
    ) -> Result<(), error::RegisterProjectError>;

    /// Given a certain project, `addkey` adds a key to its set of keys (c.f.
    /// section 4.4.1 of the whitepaper).
    fn addkey(
        project_address: types::AccountId,
        maintainer_key: types::AccountId,
    ) -> Result<(), error::KeysetError>;

    /// Given a certain project, `removekey` removes a key from its set of
    /// keys (c.f. section 4.4.1 of the whitepaper).
    fn removekey(
        project_address: types::AccountId,
        maintainer_key: types::AccountId,
    ) -> Result<(), error::KeysetError>;

    /// Unregistering a project in the Oscoin Ledger.
    ///
    /// As is the case above, this transaction may also be handled outside the
    /// ledger.
    fn unregister_project(
        project_address: types::AccountId,
    ) -> Result<(), error::UnregisterProjectError>;

    /// Checkpointing a project in Oscoin's ledger.
    fn checkpoint(
        project_address: types::AccountId,
        new_project_hash: types::Hash,
        project_url: types::URL,
        contribution_list: types::HashLinkedList<types::Contribution>,
        dependency_updates: Vec<types::DependencyUpdate>,
    ) -> Result<(), error::CheckpointError>;

    /// Transaction used to update a project's smart contract.
    ///
    /// Can be used to e.g. modify rules for a project's fund management and
    /// distribution.
    fn update_contract(
        project_address: types::AccountId,
        handler: types::Handler,
        code: types::Code,
        votes: types::VoteSet<types::AccountId>,
    ) -> Result<(), error::ContractUpdateError>;
}
