/*
This is a specification document meant to approximate the Ledger described in
Oscoin whitepaper into concrete Rust code.
However, it is not meant to be an exact implementation, and it is not intended
to compile. It is to serve as a form of documentation that will change over
time with the project.
*/

// Ledger Operations

// Description of errors that may occur when registering a project in the
// Oscoin ledger. Not exhaustive, but should cover most common cases.
// What each constructor actually returns may vary.
pub enum RegisterProjectInLedgerError {
    // The project address used to register it is already present in the
    // ledger.
    AddressInUseError(std::io::Error),

    // The canonical source URL used to register the project is invalid.
    // The 1.0 version of the whitepaper does not explain it means by a "valid"
    // URL, but it can be assumed tentatively that
    // 1. it is a proper URL as defined in [RFC 3986](https://tools.ietf.org/html/rfc3986#section-1.1.3), and
    // 2. it hosts the repository's page in a distributed version control
    //   system's website e.g. GitLab, BitBucket, SourceForge, GitHub, and
    // 3. it can be accessed without restrictions [*]
    // [*] This part can be harder to define - if the URL returns `404`s
    // *after* it has been inducted into the ledger, but not before, is it
    // still valid?
    InvalidURLError(std::io::Error),
}

// Ledger addresses and public keys are represented by `&str`, but in practice
// that may change.

// Registering a project in the Oscoin Ledger.
fn register (project_address : &str, project_source_url : &str)
    -> Result<(), RegisterProjectInLedgerError> {}

pub enum AddKeyToKeysetError {
    // Version 1.0 of the whitepaper does not mention what happens when
    // `addkey` is called with a project that has not yet been added to the
    // ledger, so here it is tentatively treated as an error.
    AddressNotInUseError(std::io::Error)

    // The key that was to be added to the project's key set was invalid
    // e.g. improperly formed, inexistent or already in the key set.
    InvalidKeyError(std::io::Error)
}

// Given a certain project, `addkey` adds a key to its set of keys (c.f.
// section 4.4.1 of the whitepaper).
fn addkey (project_address : &str, maintainer_key : &str)
    -> Result<(), AddKeyToKeysetError> {}