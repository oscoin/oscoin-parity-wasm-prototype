// These two module-wide attributes are used to disable "unused variable" and
// "field is never used" warnings when compiling this file.
#![allow(dead_code)]
#![allow(unused_variables)]

use std::marker::PhantomData;

/// This is a specification document meant to approximate the Ledger described in
/// Oscoin whitepaper into concrete Rust code.
/// However, it is not meant to be an exact implementation.
///
/// It is to serve as a form of documentation that will change over
/// time with the project.

/// # Ledger Types and Operations

/// Type for Ledger addresses. Its specific data structure is not
/// important here.
pub struct Address;

/// Type for Ledger public keys. Its specific data structure is not
/// important here, just as it is with `Address`es.
pub struct PublicKey;

/// Type for the hash digest used in the Oscoin Ledger. Useful to represent
/// commit hashes.
pub struct Hash;

/// Representation of a URL. It can represent e.g. a project's page.
pub struct URL;

/// Representation of a project in the Oscoin ledger.
/// It is still unclear whether the project's keyset should be present in this
/// data structure, or if it will be in a different layer of the protocol.
pub struct Project {
    addr : Address,
    /// A project's latest commit hash.
    hash : Hash,
    url  : URL
}

/// Description of errors that may occur when registering a project in the
/// Oscoin ledger (`register` transaction). Not exhaustive, but should cover
/// most common cases.
pub enum RegisterProjectError {
    /// The project address used to register it is already present in the
    /// ledger.
    AddressInUseError(),

    /// The canonical source URL used to register the project is invalid.
    ///
    /// The 1.0 version of the whitepaper establishes only one condition for
    /// the validity of the URL - the source code retrieved from it must
    /// always hash to the `hash` field of the `Project` structure -
    /// but this definition of validity can be tentatively extended to include:
    ///
    /// 1. it is a proper URL as defined in [RFC 3986](https:///tools.ietf.org/html/rfc3986#section-1.1.3), and
    /// 2. it hosts the repository's page in a distributed version control
    ///    system's website e.g. GitLab, BitBucket, SourceForge, GitHub, and
    /// 3. it can be accessed without restrictions [*]
    /// [*] This part can be harder to define - if the URL returns permanently
    /// returns `404`s *after* it has been inducted into the ledger, but not
    /// before, is it still valid?
    InvalidURLError(),
}

/// Representation of errors that may occur in `addkey` or `removekey`
/// transactions.
pub enum KeysetError {
    /// Version 1.0 of the whitepaper does not mention what happens when
    /// `addkey`/`removekey` are called with projects that have not yet been
    /// added to the ledger, so here that is tentatively treated as an error.
    AddressNotInUseError()
}

/// Errors that may happen when unregistering a project.
///
/// Empty for now.
pub enum UnregisterProjectError {}

/// Errors that may occur when checkpointing a project.
///
/// Question:
/// * Does an invalid dependency update list in a checkpoint invalidate it
/// entirely?
pub enum CheckpointError {
    /// A dependency update is invalid if it adds a dependency the project
    /// already uses.
    UsedDependencyAddedError(),

    /// A dependency update is invalid if it removes a dependency the project
    /// does not use.
    UnusedDependencyRemovedError(),

    /// As the whitepaper says, a checkpoint is invalid if the dependency
    /// update list containts duplicate dependencies.
    DuplicateDependenciesError()
}

/// Datatype representing a hash-linked-list. Used in the whitepaper to
/// organize contributions when checkpointing.
/// 
/// The type it abstracts over - in the context of the whitepaper's Ledger
/// section, contributions, here abbreviated as `C` - should be a tuple,
/// struct or equivalent with at least two fields e.g. `prev` and `commit`
/// such that for every two consecutive members of the hash-linked-list
/// `C {prev1 = hash1, commit1 = hash2 .. }, C {prev2 = hash3, commit2 = hash4 ..}`:
/// * it is true that `hash2 == hash3`;
/// * if the `hash1` is the first hash present in the list, it is either
///   * the same as the last hash present in the last contribution of the last
///     checkpoint, or
///   * an empty hash, in case this is a project's first checkpoint
///
/// In practice, it may not necessarily be a list, but conceptually the name
/// is explanatory.
pub struct HashLinkedList <T> {
    contributions : PhantomData<T>
}

/// Representation of a contribution's author.
pub struct Author;

/// Datatype representing a contribution, one of the data required by a
/// checkpoint.
///
/// Questions that arose from the whitepaper:
/// * what is C_sig for? It is defined but not used anywhere
/// * what is the type of C_author? Is it the GPG public key used to sign the
///   commit? Is it a string with their name?
pub struct Contribution {
    prev : Hash,
    commit : Hash,
    author : Author,
    signoff : PublicKey
}

/// Datatype representing a dependency update, another segment of data require
/// in order to checkpoint a project in the Oscoin ledger.
pub enum DependencyUpdate {
    /// Constructor to add a dependency.
    Depend {
        /// Address of the project being added to the dependency list.
        addr : Address,
        /// Zero-based index of the current checkpoint, in which the
        /// dependency is being added.
        cp_index : u64
    },
    /// Constructor to remove a dependency.
    Undepend {
        /// Address of the project being removed from the dependency list.
        addr : Address,
        /// Zero-based index of the current checkpoint, in which the
        /// dependency is being removed.
        cp_index : u64
    }
}

pub trait LedgerTransactions {

    /// Registering a project in the Oscoin Ledger.
    fn register_project ( project_address : Address
                        , project_source_url : URL)
        -> Result<(), RegisterProjectError> { unimplemented!() }

    /// Given a certain project, `addkey` adds a key to its set of keys (c.f.
    /// section 4.4.1 of the whitepaper).
    fn addkey (project_address : Address, maintainer_key : Address)
        -> Result<(), KeysetError> { unimplemented!() }

    /// Given a certain project, `removekey` removes a key from its set of
    /// keys (c.f. section 4.4.1 of the whitepaper).
    fn removekey (project_address : Address, maintainer_key : Address)
        -> Result<(), KeysetError> { unimplemented!() }

    /// Unregistering a project in the Oscoin Ledger.
    fn unregister_project ( project_address : Address)
        -> Result<(), UnregisterProjectError> { unimplemented!() }

    /// Checkpointing a project in Oscoin's ledger.
    fn checkpoint ( project_address : Address
                  , new_project_hash : Hash
                  , project_url : URL
                  , contribution_list : HashLinkedList<Contribution>
                  , dependency_updates : Vec<DependencyUpdate>
                  ) -> Result<(), CheckpointError> { unimplemented!() }
}