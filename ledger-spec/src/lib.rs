//! This is a specification document meant to approximate the Ledger described in
//! Oscoin whitepaper into concrete Rust code.
//! However, it is not meant to be an exact implementation.
//!
//! It is to serve as a form of documentation that will change over
//! time with the project.
pub mod error;
pub mod types;

/// A trait exposing the Oscoin ledger transactions described in the
/// whitepaper.
///
/// The methods here return `Result<(), E>` for some error type `E` as they
/// will be applying a modification on the Ledger global state, and won't need
/// to return any actual data if they succeed.
pub trait LedgerTransactions {
    /// Transfer Oscoin from one account to another.
    fn transfer_oscoin(
        from_addr: types::Address,
        to_addr: types::Address,
        amount: types::Oscoin,
    ) -> Result<(), error::TransferError>;

    /// Registering a project in the Oscoin Ledger.
    fn register_project(
        project_address: types::Address,
        project_source_url: types::URL,
    ) -> Result<(), error::RegisterProjectError>;

    /// Given a certain project, `addkey` adds a key to its set of keys (c.f.
    /// section 4.4.1 of the whitepaper).
    fn addkey(
        project_address: types::Address,
        maintainer_key: types::Address,
    ) -> Result<(), error::KeysetError>;

    /// Given a certain project, `removekey` removes a key from its set of
    /// keys (c.f. section 4.4.1 of the whitepaper).
    fn removekey(
        project_address: types::Address,
        maintainer_key: types::Address,
    ) -> Result<(), error::KeysetError>;

    /// Unregistering a project in the Oscoin Ledger.
    fn unregister_project(
        project_address: types::Address,
    ) -> Result<(), error::UnregisterProjectError>;

    /// Checkpointing a project in Oscoin's ledger.
    fn checkpoint(
        project_address: types::Address,
        new_project_hash: types::Hash,
        project_url: types::URL,
        contribution_list: types::HashLinkedList<types::Contribution>,
        dependency_updates: Vec<types::DependencyUpdate>,
    ) -> Result<(), error::CheckpointError>;
}
