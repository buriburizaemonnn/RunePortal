use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

mod ord_canister {
    use candid::{CandidType, Deserialize, Principal};
    use ic_cdk::api::call::CallResult;

    use super::RuneId;

    const ORD_CANISTER: &str = "o25oi-jaaaa-aaaal-ajj6a-cai";

    #[derive(CandidType, Deserialize, Debug)]
    pub struct RuneBalance {
        pub id: RuneId,
        pub balance: u128,
    }

    #[derive(CandidType, Deserialize)]
    pub enum MintError {
        Cap(u128),
        End(u64),
        Start(u64),
        Unmintable,
    }

    #[derive(CandidType, Deserialize)]
    pub enum RpcError {
        Io(String, String, String),
        Decode(String, String, String),
        Endpoint(String, String, String),
    }

    #[derive(CandidType, Deserialize)]
    pub enum OrdError {
        Params(String),
        Overflow,
        BlockVerification(u32),
        Index(MintError),
        Rpc(RpcError),
    }

    pub type GetRunesResult = Result<Vec<RuneBalance>, OrdError>;

    pub async fn get_runes_by_utxo(txid: String, vout: u32) -> CallResult<(GetRunesResult,)> {
        let ord_canister = Principal::from_text(ORD_CANISTER).unwrap();
        ic_cdk::call(ord_canister, "get_runes_by_utxo", (txid, vout)).await
    }
}

use bitcoin::hashes::Hash;
use ic_cdk::api::management_canister::bitcoin::{bitcoin_get_utxos, GetUtxosRequest, UtxoFilter};

use crate::state::{read_config, read_utxo_manager, utxo_manager::RunicUtxo, write_utxo_manager};

fn txid_to_string(txid: &[u8]) -> String {
    bitcoin::Txid::from_raw_hash(Hash::from_slice(txid).unwrap()).to_string()
}

pub enum TargetType {
    Bitcoin { target: u64 },
    Runic { runeid: RuneId, target: u128 },
}

pub async fn fetch_utxos_and_update(addr: &str, target: TargetType) {
    let network = read_config(|config| config.bitcoin_network());
    let mut arg = GetUtxosRequest {
        address: addr.to_string(),
        network,
        filter: None,
    };
    loop {
        let utxo_response = bitcoin_get_utxos(arg.clone())
            .await
            .expect("failed getting the utxo response")
            .0;
        let mut btc_utxos = vec![];
        for utxo in utxo_response.utxos {
            if read_utxo_manager(|manager| manager.is_recorded_as_runic(addr, &utxo)) {
                continue;
            }
            let txid = txid_to_string(&utxo.outpoint.txid);
            match ord_canister::get_runes_by_utxo(txid, utxo.outpoint.vout)
                .await
                .unwrap()
                .0
            {
                Err(_) => {
                    ic_cdk::println!("err while checking for runes, recording as non runic utxo");
                    btc_utxos.push(utxo);
                    continue;
                }
                Ok(runes) => {
                    if runes.is_empty() {
                        btc_utxos.push(utxo);
                        continue;
                    }
                    for rune in runes {
                        write_utxo_manager(|manager| {
                            manager.remove_bitcoin_utxo(addr, &utxo);
                            manager.record_runic_utxos(
                                addr,
                                rune.id,
                                vec![RunicUtxo {
                                    utxo: utxo.clone(),
                                    balance: rune.balance,
                                }],
                            )
                        });
                    }
                }
            }
        }
        // recording of bitcoin utxo
        write_utxo_manager(|manager| manager.record_bitcoin_utxos(addr, btc_utxos));

        match target {
            TargetType::Runic { ref runeid, target } => {
                let balance =
                    read_utxo_manager(|manager| manager.get_runestone_balance(addr, runeid));
                if balance < target && utxo_response.next_page.is_some() {
                    arg.filter = Some(UtxoFilter::Page(utxo_response.next_page.unwrap()));
                    continue;
                } else {
                    break;
                }
            }
            TargetType::Bitcoin { target } => {
                let balance = read_utxo_manager(|manager| manager.get_bitcoin_balance(addr));
                if balance < target && utxo_response.next_page.is_some() {
                    arg.filter = Some(UtxoFilter::Page(utxo_response.next_page.unwrap()));
                    continue;
                } else {
                    break;
                }
            }
        }
    }
}
