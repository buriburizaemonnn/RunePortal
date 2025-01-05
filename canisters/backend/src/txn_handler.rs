use std::time::Duration;

use bitcoin::{
    hashes::Hash,
    script::{Builder, PushBytesBuf},
    sighash::SighashCache,
    Address, Amount, EcdsaSighashType, Transaction,
};
use candid::CandidType;
use ic_cdk::api::management_canister::bitcoin::{
    bitcoin_get_utxos, bitcoin_send_transaction, GetUtxosRequest, SendTransactionRequest, Utxo,
};
use icrc_ledger_types::icrc1::account::Account;
use ordinals::Runestone;
use slotmap::Key;

use crate::{
    chain::btc::{
        signer::ecdsa::ecdsa_sign,
        utils::{account_to_derivation_path, derive_public_key, sec1_to_der},
    },
    state::{
        queue::ScheduledTransaction, read_config, utxo_manager::RunicUtxo, write_scheduled_state,
        write_utxo_manager,
    },
    updater::RuneId,
};

#[derive(CandidType, PartialEq, Eq)]
pub enum SubmittedTxidType {
    Bitcoin { txid: String },
}

pub enum TransactionType {
    Etching {
        commit_tx_address: Address,
        commit: Transaction,
        reveal: Transaction,
        fee_utxos: Vec<Utxo>,
        fee_payer: Address,
    },
    Bitcoin {
        utxos: Vec<Utxo>,
        txn: Transaction,
        sender: Address,
        sender_account: Account,
    },
    Rune {
        runic_utxos: Vec<RunicUtxo>,
        runeid: RuneId,
        rune_amount: u128,
        rune_sender: Box<Address>,
        rune_receiver: Box<Address>,
        rune_sender_account: Account,
        fee_utxos: Vec<Utxo>,
        fee: u64,
        fee_payer: Box<Address>,
        fee_payer_account: Account,
        postage: Amount,
    },
}

impl TransactionType {
    pub async fn submit(self) -> SubmittedTxidType {
        match self {
            Self::Etching {
                commit_tx_address,
                commit,
                reveal,
                fee_utxos,
                fee_payer,
            } => {
                /*
                let mut txn: Transaction = *commit;
                let (path, pubkey) = read_config(|config| {
                    let ecdsa_pubkey = config.ecdsa_public_key();
                    let path = account_to_derivation_path(&fee_payer_account);
                    let pubkey = derive_public_key(&ecdsa_pubkey, &path).public_key;
                    (
                        path.iter().map(|x| x.to_vec()).collect::<Vec<Vec<u8>>>(),
                        pubkey,
                    )
                });

                let txn_cache = SighashCache::new(txn.clone());
                let network = read_config(|config| config.bitcoin_network());

                for (index, input) in txn.input.iter_mut().enumerate() {
                    let sighash = txn_cache
                        .legacy_signature_hash(
                            index,
                            &fee_payer.script_pubkey(),
                            EcdsaSighashType::All.to_u32(),
                        )
                        .unwrap();

                    let signature = ecdsa_sign(sighash.as_byte_array().to_vec(), path.clone())
                        .await
                        .signature;
                    let mut signature = sec1_to_der(signature);
                    signature.push(EcdsaSighashType::All.to_u32() as u8);
                    let signature = PushBytesBuf::try_from(signature).unwrap();
                    let pubkey = PushBytesBuf::try_from(pubkey.clone()).unwrap();
                    input.script_sig = Builder::new()
                        .push_slice(signature)
                        .push_slice(pubkey)
                        .into_script();
                    input.witness.clear();
                }
                let txid = txn.compute_txid().to_string();
                let txn_bytes = bitcoin::consensus::serialize(&txn);
                ic_cdk::println!("commit txn bytes after signing:");
                ic_cdk::println!("{}", hex::encode(&txn_bytes));
                ic_cdk::println!("reveal txn bytes:");
                let reveal_txn_bytes = bitcoin::consensus::serialize(reveal.as_ref());
                ic_cdk::println!("{}", hex::encode(&reveal_txn_bytes));
                bitcoin_send_transaction(SendTransactionRequest {
                    transaction: txn_bytes,
                    network,
                })
                .await
                .unwrap();
                write_scheduled_state(|state| {
                    let id = state.get_id();
                    let timer = read_config(|config| config.get_timer_for_txn_submission());
                    let timer_id =
                        ic_cdk_timers::set_timer_interval(Duration::from_secs(timer), move || {
                            ic_cdk::spawn(submit_txn(id))
                        });
                    state.record_txn(
                        id,
                        ScheduledTransaction {
                            txn: *reveal.clone(),
                            commit_tx_address: commit_tx_address.to_string(),
                            timer_id: timer_id.data(),
                        },
                    );
                });
                Some(SubmittedTxidType::Bitcoin { txid })
                */

                let (network, timer) = read_config(|config| {
                    (
                        config.bitcoin_network(),
                        config.get_timer_for_txn_submission(),
                    )
                });
                let txid = commit.compute_txid().to_string();
                let txn = bitcoin::consensus::serialize(&commit);
                if bitcoin_send_transaction(SendTransactionRequest {
                    transaction: txn,
                    network,
                })
                .await
                .is_err()
                {
                    write_utxo_manager(|manager| {
                        manager.record_bitcoin_utxos(fee_payer.to_string().as_ref(), fee_utxos);
                    });
                    ic_cdk::trap("failed submitting the transaction")
                }
                write_scheduled_state(|state| {
                    let id = state.get_id();
                    let timer_id =
                        ic_cdk_timers::set_timer_interval(Duration::from_secs(timer), move || {
                            ic_cdk::spawn(submit_txn(id))
                        });
                    state.record_txn(
                        id,
                        ScheduledTransaction {
                            txn: reveal,
                            commit_tx_address: commit_tx_address.to_string(),
                            timer_id: timer_id.data(),
                        },
                    );
                });
                SubmittedTxidType::Bitcoin { txid }
            }
            Self::Bitcoin {
                utxos,
                txn,
                sender,
                sender_account,
            } => {
                let mut txn: Transaction = txn;
                let (path, pubkey) = read_config(|config| {
                    let ecdsa_pubkey = config.ecdsa_public_key();
                    let path = account_to_derivation_path(&sender_account);
                    let pubkey = derive_public_key(&ecdsa_pubkey, &path).public_key;
                    (
                        path.iter().map(|x| x.to_vec()).collect::<Vec<Vec<u8>>>(),
                        pubkey,
                    )
                });

                let txn_cache = SighashCache::new(txn.clone());
                let network = read_config(|config| config.bitcoin_network());

                for (index, input) in txn.input.iter_mut().enumerate() {
                    let sighash = txn_cache
                        .legacy_signature_hash(
                            index,
                            &sender.script_pubkey(),
                            EcdsaSighashType::All.to_u32(),
                        )
                        .unwrap();

                    let signature = ecdsa_sign(sighash.as_byte_array().to_vec(), path.clone())
                        .await
                        .signature;
                    let mut signature = sec1_to_der(signature);
                    signature.push(EcdsaSighashType::All.to_u32() as u8);
                    let signature = PushBytesBuf::try_from(signature).unwrap();
                    let pubkey = PushBytesBuf::try_from(pubkey.clone()).unwrap();
                    input.script_sig = Builder::new()
                        .push_slice(signature)
                        .push_slice(pubkey)
                        .into_script();
                    input.witness.clear();
                }
                let txid = txn.compute_txid().to_string();
                let txn_bytes = bitcoin::consensus::serialize(&txn);
                ic_cdk::println!("bitcoin transaction bytes:");
                ic_cdk::println!("{}", hex::encode(&txn_bytes));
                if bitcoin_send_transaction(SendTransactionRequest {
                    transaction: txn_bytes,
                    network,
                })
                .await
                .is_err()
                {
                    write_utxo_manager(|manager| {
                        manager.record_bitcoin_utxos(sender.to_string().as_ref(), utxos);
                    });
                    ic_cdk::trap("failed submitting the transaction")
                }
                SubmittedTxidType::Bitcoin { txid }
            }
            _ => todo!(),
        }
    }
}

async fn submit_txn(id: u128) {
    let txn = write_scheduled_state(|state| state.remove_txn(id));
    ic_cdk::println!("commit tx address: {}", txn.commit_tx_address);
    let network = read_config(|config| config.bitcoin_network());
    let utxos_response = bitcoin_get_utxos(GetUtxosRequest {
        network,
        address: txn.commit_tx_address.clone(),
        filter: None,
    })
    .await
    .unwrap()
    .0;
    let utxos = utxos_response.utxos;
    for utxo in utxos.iter() {
        ic_cdk::println!("bitcoin in utxo: {}", utxo.value);
    }
    if utxos.is_empty() {
        write_scheduled_state(|state| state.record_txn(id, txn));
        ic_cdk::trap("No UTXOs Found")
    }
    if utxos_response.tip_height - utxos[0].height < Runestone::COMMIT_CONFIRMATIONS as u32 {
        write_scheduled_state(|state| state.record_txn(id, txn));
        ic_cdk::trap("Not enough commit confirmation")
    }
    let transaction = bitcoin::consensus::serialize(&txn.txn);
    if bitcoin_send_transaction(SendTransactionRequest {
        network,
        transaction,
    })
    .await
    .is_err()
    {
        ic_cdk::println!("Timer was hit for reveal txn submission but failed to submit due to err");
        write_scheduled_state(|state| state.record_txn(id, txn));
    } else {
        ic_cdk::println!("transaction was submitted");
        ic_cdk_timers::clear_timer(txn.timer_id.into());
    }
}
