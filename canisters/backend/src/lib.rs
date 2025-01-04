mod addresses;
mod chain;
mod memory;
mod state;
mod token_type;
mod txn_handler;
mod updater;

use std::time::Duration;

use addresses::Addresses;
use candid::{CandidType, Principal};
use chain::btc::{
    address_validation,
    runestone::{etch::EtchingArgs, validate_etching},
};
use ic_cdk::{
    api::management_canister::{
        bitcoin::BitcoinNetwork,
        ecdsa::{
            ecdsa_public_key, EcdsaPublicKeyArgument, EcdsaPublicKeyResponse as EcdsaPublicKey,
        },
        schnorr::{
            schnorr_public_key, SchnorrPublicKeyArgument,
            SchnorrPublicKeyResponse as SchnorrPublicKey,
        },
    },
    init, post_upgrade, pre_upgrade, query, update,
};
use memory::Memory;
use serde::Deserialize;
use state::{read_config, write_config};
use token_type::TokenType;

async fn lazy_ecdsa_schnorr_setup() {
    let (ecdsakeyid, schnorrkeyid) =
        read_config(|config| (config.ecdsakeyid(), config.schnorrkeyid()));
    let ecdsapublickey = ecdsa_public_key(EcdsaPublicKeyArgument {
        derivation_path: vec![],
        canister_id: None,
        key_id: ecdsakeyid,
    })
    .await
    .unwrap()
    .0;
    let schnorrpublickey = schnorr_public_key(SchnorrPublicKeyArgument {
        derivation_path: vec![],
        canister_id: None,
        key_id: schnorrkeyid,
    })
    .await
    .unwrap()
    .0;
    write_config(|config| {
        let mut temp = config.get().clone();
        temp.ecdsa_public_key.replace(ecdsapublickey);
        temp.schnorr_public_key.replace(schnorrpublickey);
        config.set(temp).expect("failed to set config");
    });
}

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    pub bitcoin_network: BitcoinNetwork,
    pub auth: Option<Principal>,
    pub commission_receiver: Option<Principal>,
}

#[init]
pub fn init(
    InitArgs {
        bitcoin_network,
        auth,
        commission_receiver,
    }: InitArgs,
) {
    let caller = ic_cdk::caller();
    let auth = auth.unwrap_or(caller);
    let keyname = match bitcoin_network {
        BitcoinNetwork::Mainnet => "key_1".to_string(),
        BitcoinNetwork::Testnet => "test_key_1".to_string(),
        BitcoinNetwork::Regtest => "dfx_test_key".to_string(),
    };
    write_config(|config| {
        let mut temp = config.get().clone();
        temp.keyname.replace(keyname);
        temp.auth.replace(auth);
        temp.bitcoin_network.replace(bitcoin_network);
        config.set(temp).expect("failed to set config");
    });
    ic_cdk_timers::set_timer(Duration::from_secs(0), || {
        ic_cdk::spawn(lazy_ecdsa_schnorr_setup())
    });
}

#[pre_upgrade]
pub fn pre_upgrade() {}

#[post_upgrade]
pub fn post_upgrade() {}

pub fn user_detail() {}

pub fn all_launches() {}

#[derive(CandidType, Deserialize)]
pub struct StartLaunchArgs {
    pub logo: Option<Vec<u8>>,
    pub content_type: Option<Vec<u8>>,
    pub runename: String,
    pub symbol: Option<u32>,
    pub divisibility: u8,
    pub total_supply: u128,
    pub turbo: bool,
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
    pub fee_per_vbytes: Option<u64>,
}

#[update]
pub async fn start_launch(
    StartLaunchArgs {
        logo,
        content_type,
        runename,
        symbol,
        divisibility,
        total_supply,
        turbo,
        website,
        x,
        telegram,
        openchat,
        hard_cap,
        soft_cap,
        starts_in,
        duration,
        raise_in,
        price_per_token,
        fee_per_vbytes,
    }: StartLaunchArgs,
) {
    let caller = ic_cdk::caller();
    let caller_addresses = Addresses::from(&caller);
    let caller_address = address_validation(&caller_addresses.bitcoin).unwrap();
    let (spaced_rune, total_supply, symbol) =
        match validate_etching(&runename, symbol, divisibility, total_supply) {
            Err(err) => ic_cdk::trap(&err),
            Ok((sr, total_supply, symbol)) => (sr, total_supply, symbol),
        };
    let fee_per_vbytes = fee_per_vbytes.unwrap_or(20_000); // default to 20sats per vbytes
    let arg = EtchingArgs {
        content_type,
        logo,
        reveal_address: caller_address.clone(),
        spaced_rune,
        premine: total_supply,
        divisibility,
        symbol,
        fee_payer: caller_address.clone(),
        fee_payer_account: caller_addresses.icrc1,
        turbo,
        postage: None,
        fee_per_vbytes,
    };
}

pub fn participate() {}

ic_cdk::export_candid!();
