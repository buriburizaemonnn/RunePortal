mod addresses;
mod chain;
mod memory;
mod state;
mod txn_handler;
mod updater;

use ic_cdk::api::management_canister::ecdsa::EcdsaPublicKeyResponse as EcdsaPublicKey;
use memory::Memory;

pub fn init() {}

pub fn pre_upgrade() {}

pub fn post_upgrade() {}

pub fn user_detail() {}

pub fn all_launches() {}

pub fn start_launch() {}

pub fn participate() {}

ic_cdk::export_candid!();
