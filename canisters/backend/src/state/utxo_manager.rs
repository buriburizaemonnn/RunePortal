use std::collections::{HashMap, HashSet};

use candid::{CandidType, Decode, Encode};
use ic_cdk::api::management_canister::bitcoin::Utxo;
use ic_stable_structures::{storable::Bound, StableBTreeMap, Storable};
use serde::{Deserialize, Serialize};

use crate::{memory::MemoryIds, updater::RuneId, Memory};

use super::read_memory_manager;

#[derive(CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct RunicUtxo {
    pub balance: u128,
    pub utxo: Utxo,
}

impl std::hash::Hash for RunicUtxo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.utxo.hash(state)
    }
}

impl std::borrow::Borrow<Utxo> for RunicUtxo {
    fn borrow(&self) -> &Utxo {
        &self.utxo
    }
}

#[derive(CandidType, Deserialize)]
pub struct RunicToUtxoMapping(HashMap<RuneId, HashSet<RunicUtxo>>);

impl Storable for RunicToUtxoMapping {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).expect("should encode"))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).expect("should decode")
    }

    const BOUND: Bound = Bound::Unbounded;
}

pub type RunicMapping = StableBTreeMap<String, RunicToUtxoMapping, Memory>;

pub fn init_runic_mapping() -> RunicMapping {
    read_memory_manager(|manager| {
        let memory = manager.get(MemoryIds::Runic.into());
        RunicMapping::init(memory)
    })
}

#[derive(CandidType, Deserialize)]
pub struct UtxoMapping(HashSet<Utxo>);

impl Storable for UtxoMapping {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).expect("should encode"))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).expect("should decode")
    }

    const BOUND: Bound = Bound::Unbounded;
}

pub type BitcoinMapping = StableBTreeMap<String, UtxoMapping, Memory>;

pub fn init_bitcoin_mapping() -> BitcoinMapping {
    read_memory_manager(|manager| {
        let memory = manager.get(MemoryIds::Bitcoin.into());
        BitcoinMapping::init(memory)
    })
}

#[derive(Serialize, Deserialize)]
pub struct UtxoManager {
    #[serde(skip, default = "init_runic_mapping")]
    pub runic: RunicMapping,
    #[serde(skip, default = "init_bitcoin_mapping")]
    pub bitcoin: BitcoinMapping,
}

impl Default for UtxoManager {
    fn default() -> Self {
        Self {
            runic: init_runic_mapping(),
            bitcoin: init_bitcoin_mapping(),
        }
    }
}
