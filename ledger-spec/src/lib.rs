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

    /// Registers a project on the Oscoin Ledger and returns the new project’s ID.
    ///
    /// The transaction’s sender account becomes the initial maintainer of the project.
    ///
    /// The project ID is computed by hashing the sender’s nonce and the arguments. In the current
    /// implementation we use ethereum’s contract creation logic which generates the project ID.
    fn register_project(
        // Canonical source URL of the project to be registered.
        project_source_url: types::URL,
    ) -> Result<types::ProjectId, error::RegisterProjectError>;

    /// Given a certain project, `addkey` adds a key to its set of keys (c.f.
    /// section 4.4.1 of the whitepaper).
    fn addkey(
        id: types::ProjectId,
        // Account identifier of the maintainer to be added to the project's
        // key set.
        maintainer_key: types::AccountId,
    ) -> Result<(), error::KeysetError>;

    /// Given a certain project, `removekey` removes a key from its set of
    /// keys (c.f. section 4.4.1 of the whitepaper).
    fn removekey(
        id: types::ProjectId,
        // Account identifier of the maintainer to be removed from the
        // project's key set.
        maintainer_key: types::AccountId,
    ) -> Result<(), error::KeysetError>;

    /// Unregistering a project from the Oscoin Ledger.
    ///
    /// As is the case above, this transaction may also be handled outside the
    /// ledger.
    fn unregister_project(id: types::ProjectId) -> Result<(), error::UnregisterProjectError>;

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
        // about this type, go to types::Contribution.
        contribution_list: types::HashLinkedList<types::Contribution>,
        // A vector of dependency updates. See types::DependencyUpdate
        // for more information.
        //
        // It is to be treated as a list i.e. processed from left to right.
        dependency_updates: Vec<types::DependencyUpdate>,
    ) -> Result<(), error::CheckpointError>;
}

/// Functions to access information from the ledger state.
pub trait LedgerView {
    /// Returns the project registered at the given address.
    ///
    /// Returns `None` if no project was registered or the project was unregistered.
    fn get_project(project_address: types::AccountId) -> Option<types::Project>;

    /// Returns the [Account] at the given address.
    ///
    /// An account exists for every address. If it has not receveived any money the empty account
    /// with zero nonce and balance is returned.
    fn get_account(address: types::AccountId) -> types::Account;
}
