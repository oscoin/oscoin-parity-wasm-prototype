/// This is a specification document meant to approximate the Ledger described in
/// Oscoin whitepaper into concrete Rust code.
/// However, it is not meant to be an exact implementation.
///
/// It is to serve as a form of documentation that will change over
/// time with the project.

/// # Ledger Operations

/// Abstract type for Ledger addresses and public keys. Its specific data
/// structure is not important here.
pub struct ProjectAddress;

/// Description of errors that may occur when registering a project in the
/// Oscoin ledger (`register` transaction). Not exhaustive, but should cover
/// most common cases.
pub enum RegisterProjectInLedgerError {
    /// The project address used to register it is already present in the
    /// ledger.
    AddressInUseError(),

    /// The canonical source URL used to register the project is invalid.
    ///
    /// The 1.0 version of the whitepaper does not explain it means by a "valid"
    /// URL, but it can be assumed tentatively that
    /// 1. it is a proper URL as defined in [RFC 3986](https:///tools.ietf.org/html/rfc3986#section-1.1.3), and
    /// 2. it hosts the repository's page in a distributed version control
    ///   system's website e.g. GitLab, BitBucket, SourceForge, GitHub, and
    /// 3. it can be accessed without restrictions [*]
    /// [*] This part can be harder to define - if the URL returns `404`s
    /// *after* it has been inducted into the ledger, but not before, is it
    /// still valid?
    InvalidURLError(),
}

/// Representation of errors that may occur in `addkey` or `removekey`
/// transactions.
pub enum KeysetError {
    /// Version 1.0 of the whitepaper does not mention what happens when
    /// `addkey`/`removekey` are called with projects that have not yet been
    /// added to the ledger, so here that is tentatively treated as an error.
    AddressNotInUseError(),
}

/// Errors that may happen when unregistering a project.
/// Empty for now.
pub enum UnregisterProjectLedgerError;

pub trait LedgerTransactions {

    /// Registering a project in the Oscoin Ledger.
    fn register ( project_address : ProjectAddress
                , project_source_url : ProjectAddress)
        -> Result<(), RegisterProjectInLedgerError> {}

    /// Given a certain project, `addkey` adds a key to its set of keys (c.f.
    /// section 4.4.1 of the whitepaper).
    fn addkey (project_address : ProjectAddress, maintainer_key : ProjectAddress)
        -> Result<(), KeysetError> {}

    /// Given a certain project, `removekey` removes a key from its set of
    /// keys (c.f. section 4.4.1 of the whitepaper).
    fn removekey (project_address : ProjectAddress, maintainer_key : ProjectAddress)
        -> Result<(), KeysetError> {}

    /// Unregistering a project in the Oscoin Ledger.
    /// 
    fn unregister ( project_address : ProjectAddress)
        -> Result<(), UnregisterProjectLedgerError> {}

}