use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::{token_type::TokenType, updater::RuneId};

#[derive(CandidType, Deserialize)]
pub struct Launch {
    pub created_by: Principal,
    pub launch_id: u128,
    pub runeid: Option<RuneId>, // rune id will be provided later
    pub runename: String,
    pub divisibility: u8,
    pub logo: Option<Vec<u8>>,
    pub content_type: Option<Vec<u8>>,
    pub website: Option<String>,
    pub x: Option<String>,
    pub telegram: Option<String>,
    pub openchat: Option<String>,
    pub hard_cap: u64,
    pub soft_cap: u64,
    pub starts_in: u8, // should be in days
    pub duration: u8,  // should be in days
    pub raise_in: TokenType,
    pub price_per_token: u64,
}
