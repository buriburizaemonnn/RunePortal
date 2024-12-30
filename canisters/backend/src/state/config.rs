use crate::{memory::MemoryIds, EcdsaPublicKey, Memory};
use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use ic_stable_structures::{storable::Bound, StableCell, Storable};
use serde::Deserialize;

use super::read_memory_manager;

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct Config {
    pub auth: Option<Principal>,
    pub bitcoin_network: Option<BitcoinNetwork>,
    pub ecdsa_public_key: Option<EcdsaPublicKey>,
    pub keyname: Option<String>,
}

impl Storable for Config {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).expect("should encode"))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).expect("should decode")
    }

    const BOUND: Bound = Bound::Unbounded;
}

pub type StableConfig = StableCell<Config, Memory>;

pub fn init_stable_config() -> StableConfig {
    read_memory_manager(|manager| {
        let memory = manager.get(MemoryIds::Config.into());
        StableConfig::new(memory, Config::default())
            .expect("failed to initialize memory for config")
    })
}
