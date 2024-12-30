use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RuneId {
    pub block: u32,
    pub tx: u64,
}

impl Storable for RuneId {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).expect("should encode"))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).expect("should decode")
    }

    const BOUND: Bound = Bound::Unbounded;
}

mod ord_canister {}
