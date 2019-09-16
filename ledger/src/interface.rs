//! Abstract interfaces for the ledger and related types.
//!
//! The abstract leder interface is encoded in the [Ledger] trait.
//!
//! Calls to the ledger a reified in the [Call] enum. Each ledger method has a corresponding
//! [Query] or [Update] constructor. With [dispatch] the method corresponding to a given [Call] is
//! called on a [Ledger] implementation.
use crate::pwasm::String;
use alloc::collections::BTreeSet;
use alloc::prelude::v1::Vec;
use serde::{Deserialize, Serialize};

pub type ProjectId = [u8; 20];
pub type AccountId = [u8; 20];

#[derive(Serialize, Deserialize, Debug, Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct Project {
    pub url: String,
    pub members: Vec<AccountId>,
}

/// Public interface of the oscoin ledger
pub trait Ledger {
    fn ping(&mut self) -> String;

    fn counter_inc(&mut self);

    fn counter_value(&mut self) -> u32;

    fn register_project(&mut self, url: String) -> ProjectId;

    fn get_project(&mut self, project_id: ProjectId) -> Option<Project>;

    fn list_projects(&mut self) -> BTreeSet<Project>;
}

/// Represents a call to a ledger method. Either a [Query] or a [Update].
///
/// Calls are serialized to byte vectors with [Call::serialize].
///
/// Each [Call] corresponds to a method on [Ledger].
#[derive(Serialize, Deserialize, Clone)]
pub enum Call {
    Query(Query),
    Update(Update),
}

/// Reified non-mutating call to the ledger
///
/// Each [Query] corresponds to a method on [Ledger].
#[derive(Serialize, Deserialize, Clone)]
pub enum Query {
    Ping,
    CounterValue,
    GetProject { project_id: ProjectId },
    ListProjects,
}

/// Reified update to the ledger
///
/// Each [Update] corresponds to a method on [Ledger].
#[derive(Serialize, Deserialize, Clone)]
pub enum Update {
    CounterInc,
    RegisterProject { url: String },
}

impl Call {
    pub fn serialize(&self) -> Vec<u8> {
        serde_cbor::to_vec(&self).expect("CBOR serialization to Vec always succeeds")
    }

    pub fn deserialize(data: &[u8]) -> serde_cbor::Result<Self> {
        let call = serde_cbor::from_slice::<Call>(data)?;
        Ok(call.clone())
    }
}

impl From<Query> for Call {
    fn from(query: Query) -> Call {
        Call::Query(query)
    }
}

impl From<Update> for Call {
    fn from(update: Update) -> Call {
        Call::Update(update)
    }
}

/// Calls the `ledger`’s method corresponding to `call` and returns the serialized result of the
/// method call.
pub fn dispatch(mut ledger: impl Ledger, call: Call) -> Vec<u8> {
    let res = match call {
        Call::Query(query) => match query {
            Query::Ping => serde_cbor::to_vec(&ledger.ping()),
            Query::CounterValue => serde_cbor::to_vec(&ledger.counter_value()),
            Query::GetProject { project_id } => serde_cbor::to_vec(&ledger.get_project(project_id)),
            Query::ListProjects => serde_cbor::to_vec(&ledger.list_projects()),
        },
        Call::Update(update) => match update {
            Update::CounterInc => serde_cbor::to_vec(&ledger.counter_inc()),
            Update::RegisterProject { url } => serde_cbor::to_vec(&ledger.register_project(url)),
        },
    };
    res.expect("CBOR serialization never fails")
}
