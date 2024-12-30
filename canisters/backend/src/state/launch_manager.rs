use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct Launch {
    pub created_by: Principal,
    pub launch_id: u128,
    pub runename: String,
    pub divisibility: u8,
}
