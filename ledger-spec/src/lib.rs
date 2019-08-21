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
    ///
    /// This transaction's presence in the ledger layer is still subject to
    /// discussion.
    fn transfer_oscoin(
        // Account from which to send Oscoin.
        from_acc: types::AccountId,
        // Account to which Oscoin will be sent.
        to_acc: types::AccountId,
        // Amount of Oscoin to be sent.
        amount: types::Oscoin,
    ) -> Result<(), error::TransferError>;

    /// Registering a project in the Oscoin Ledger.
    ///
    /// This transaction's inclusion in the ledger is also subject to
    /// discussion as there is still an unclear boundary between the account
    /// layer and the ledger. This matter can be revisited.
    fn register_project(
        // Account identifier of the project to be registered.
        project_account: types::AccountId,
        // Canonical source URL of the project to be registered.
        project_source_url: types::URL,
    ) -> Result<(), error::RegisterProjectError>;

    /// Given a certain project, `addkey` adds a key to its set of keys (c.f.
    /// section 4.4.1 of the whitepaper).
    fn addkey(
        // Account identifier of the project to which a new maintainer is to be
        // added.
        project_account: types::AccountId,
        // Account identifier of the maintainer to be added to the project's
        // key set.
        maintainer_key: types::AccountId,
    ) -> Result<(), error::KeysetError>;

    /// Given a certain project, `removekey` removes a key from its set of
    /// keys (c.f. section 4.4.1 of the whitepaper).
    fn removekey(
        // Account identifier of the project from which a maintainer is to be
        // removed.
        project_account: types::AccountId,
        // Account identifier of the maintainer to be removed from the
        // project's key set.
        maintainer_key: types::AccountId,
    ) -> Result<(), error::KeysetError>;

    /// Unregistering a project from the Oscoin Ledger.
    ///
    /// As is the case above, this transaction may also be handled outside the
    /// ledger.
    fn unregister_project(
        // Account identifier of the project to be removed from the ledger.
        project_account: types::AccountId,
    ) -> Result<(), error::UnregisterProjectError>;

    /// Checkpointing a project in Oscoin's ledger.
    fn checkpoint(
        // Account identifier of the project to which a new checkpoint will be
        // added.
        project_account: types::AccountId,
        // New project hash - if the checkpoint succeeds, this will become its
        // current hash.
        new_project_hash: types::Hash,
        // Canonical source project URL of the project to which a new
        // checkpoint will be added.
        project_url: types::URL,
        // Hash-linked list of the checkpoint's contributions. To see more
        // about this type, go to types.Contribution.
        contribution_list: types::HashLinkedList<types::Contribution>,
        // A vector of dependency updates. See types.DependencyUpdate
        // for more information.
        //
        // It is to be treated as a list i.e. processed from left to right.
        dependency_updates: Vec<types::DependencyUpdate>,
    ) -> Result<(), error::CheckpointError>;

    /// Transaction used to update a project's smart contract.
    ///
    /// Can be used to e.g. modify rules for a project's fund management and
    /// distribution.
    fn update_contract(
        // Account identifier of the project whose contract is to be updated.
        project_account: types::AccountId,
        // Smart contract handler that is to be updated.
        //
        // Note that a smart
        // contract is a set of handlers, each being a function that has a
        // certain role.
        handler: types::Handler,
        // New code for the handler that is to be updated.
        code: types::Code,
        // Set of votes gathered by the update's proposer in favor of the
        // contract update.
        votes: types::VoteSet,
    ) -> Result<(), error::ContractUpdateError>;
}
