use crate::{memory::MemoryIds, EcdsaPublicKey, Memory, SchnorrPublicKey};
use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::{
    bitcoin::BitcoinNetwork,
    ecdsa::{EcdsaCurve, EcdsaKeyId},
    schnorr::{SchnorrAlgorithm, SchnorrKeyId},
};
use ic_stable_structures::{storable::Bound, StableCell, Storable};
use serde::Deserialize;

use super::read_memory_manager;

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct Config {
    pub auth: Option<Principal>,
    pub commission_receiver: Option<Principal>,
    pub bitcoin_network: Option<BitcoinNetwork>,
    pub ecdsa_public_key: Option<EcdsaPublicKey>,
    pub schnorr_public_key: Option<SchnorrPublicKey>,
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

impl Config {
    pub fn bitcoin_network(&self) -> BitcoinNetwork {
        match self.bitcoin_network {
            None => ic_cdk::trap("canister's config uninitialized"),
            Some(network) => network,
        }
    }
    pub fn commission_receiver(&self) -> Principal {
        match self.commission_receiver {
            None => ic_cdk::id(),
            Some(cr) => cr,
        }
    }

    pub fn ecdsa_public_key(&self) -> EcdsaPublicKey {
        if let Some(ref public_key) = self.ecdsa_public_key {
            public_key.clone()
        } else {
            ic_cdk::trap("canister's config uninitialized")
        }
    }

    pub fn schnorr_public_key(&self) -> SchnorrPublicKey {
        if let Some(ref public_key) = self.schnorr_public_key {
            public_key.clone()
        } else {
            ic_cdk::trap("canister's config uninitialized")
        }
    }

    pub fn keyname(&self) -> String {
        if let Some(ref keyname) = self.keyname {
            keyname.clone()
        } else {
            ic_cdk::trap("canister's config uninitialized")
        }
    }

    pub fn ecdsakeyid(&self) -> EcdsaKeyId {
        let name = self.keyname();
        EcdsaKeyId {
            name,
            curve: EcdsaCurve::Secp256k1,
        }
    }

    pub fn schnorrkeyid(&self) -> SchnorrKeyId {
        let name = self.keyname();
        SchnorrKeyId {
            algorithm: SchnorrAlgorithm::Bip340secp256k1,
            name,
        }
    }

    pub fn get_timer_for_txn_submission(&self) -> u64 {
        match self.bitcoin_network() {
            BitcoinNetwork::Regtest => 60,
            _ => 60 * 60,
        }
    }
}

pub type StableConfig = StableCell<Config, Memory>;

pub fn init_stable_config() -> StableConfig {
    read_memory_manager(|manager| {
        let memory = manager.get(MemoryIds::Config.into());
        StableConfig::new(memory, Config::default())
            .expect("failed to initialize memory for config")
    })
}
