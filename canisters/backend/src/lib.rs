mod memory;
mod state;
mod updater;

use ic_cdk::api::management_canister::ecdsa::EcdsaPublicKeyResponse as EcdsaPublicKey;
use memory::Memory;

pub fn init() {}

pub fn pre_upgrade() {}

pub fn post_upgrade() {}

ic_cdk::export_candid!();
