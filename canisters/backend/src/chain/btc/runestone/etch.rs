use bitcoin::{
    absolute::LockTime,
    key::{constants::SCHNORR_SIGNATURE_SIZE, Secp256k1},
    opcodes,
    script::Builder,
    taproot::{self, ControlBlock, LeafVersion, TaprootBuilder},
    transaction::Version,
    Address, Amount, Network, OutPoint, PublicKey, Script, ScriptBuf, Sequence, Transaction, TxIn,
    TxOut, Witness, XOnlyPublicKey,
};
use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, Utxo};
use icrc_ledger_types::icrc1::account::Account;
use ordinals::{Etching, Runestone, SpacedRune};

use crate::state::read_config;

use super::{
    inscription::Inscription, DEFAULT_POSTAGE, MAX_STANDARD_OP_RETURN_SIZE, TARGET_POSTAGE,
};

fn build_reveal_transaction(
    commit_input_index: usize,
    control_block: &ControlBlock,
    fee_per_vbytes: u64,
    output: Vec<TxOut>,
    input: Vec<OutPoint>,
    script: &Script,
) -> (Transaction, Amount) {
    let reveal_txn = Transaction {
        input: input
            .into_iter()
            .map(|previous_output| TxIn {
                previous_output,
                sequence: Sequence::from_height(Runestone::COMMIT_CONFIRMATIONS - 1),
                script_sig: ScriptBuf::new(),
                witness: Witness::new(),
            })
            .collect(),
        output,
        lock_time: LockTime::ZERO,
        version: Version(2),
    };
    let fee = {
        let mut reveal_txn = reveal_txn.clone();

        for (index, txin) in reveal_txn.input.iter_mut().enumerate() {
            if index == commit_input_index {
                txin.witness.push(
                    taproot::Signature::from_slice(&[0; SCHNORR_SIGNATURE_SIZE])
                        .unwrap()
                        .to_vec(),
                );
                txin.witness.push(script);
                txin.witness.push(control_block.serialize());
            } else {
                txin.witness = Witness::from_slice(&[&[0; SCHNORR_SIGNATURE_SIZE]]);
            }
        }

        let vsize = reveal_txn.vsize() as u64;
        Amount::from_sat((vsize * fee_per_vbytes) / 1000)
    };

    (reveal_txn, fee)
}

pub struct EtchingArgs {
    pub reveal_address: Address,
    pub logo: Option<Vec<u8>>,
    pub content_type: Option<Vec<u8>>,
    pub spaced_rune: SpacedRune,
    pub premine: u128,
    pub divisibility: u8,
    pub symbol: Option<char>,
    pub turbo: bool,
    pub fee_payer: Address,
    pub fee_payer_account: Account,
    pub postage: Option<u64>,
    pub fee_per_vbytes: u64,
}

pub fn etch(
    EtchingArgs {
        reveal_address,
        logo,
        content_type,
        spaced_rune,
        premine,
        divisibility,
        symbol,
        turbo,
        fee_payer,
        fee_payer_account,
        postage,
        fee_per_vbytes,
    }: EtchingArgs,
) {
    let postage = Amount::from_sat(postage.unwrap_or(DEFAULT_POSTAGE));
    let SpacedRune { rune, spacers } = spaced_rune;
    let inscription = Inscription::new(logo, content_type, rune);

    let (mut reveal_input, mut reveal_output) = (vec![OutPoint::null()], vec![]);

    let etching = Etching {
        divisibility: Some(divisibility),
        premine: Some(premine),
        rune: Some(rune),
        spacers: Some(spacers),
        symbol,
        turbo,
        terms: None, // this will make rune unmintable
    };

    let vout;

    if premine > 0 {
        let output = reveal_output.len() as u32;
        reveal_output.push(TxOut {
            script_pubkey: reveal_address.script_pubkey(),
            value: TARGET_POSTAGE,
        });
        vout = output;
    }

    let runestone = Runestone {
        edicts: vec![],
        etching: Some(etching),
        mint: None,
        pointer: (premine > 0).then_some(reveal_output.len() as u32 - 1),
    };

    let enciphered = runestone.encipher();
    if enciphered.len() > MAX_STANDARD_OP_RETURN_SIZE {
        ic_cdk::trap("runestone greater than maximum OP_RETURN size");
    }

    reveal_output.push(TxOut {
        value: Amount::ZERO,
        script_pubkey: enciphered.clone(),
    });

    let (schnorr_public_key, network) = read_config(|config| {
        let schnorr_public_key = config.schnorr_public_key().public_key;
        let network = match config.bitcoin_network() {
            BitcoinNetwork::Mainnet => Network::Bitcoin,
            BitcoinNetwork::Testnet => Network::Testnet,
            BitcoinNetwork::Regtest => Network::Regtest,
        };
        (schnorr_public_key, network)
    });
    let secp256k1 = Secp256k1::new();
    let schnorr_public_key: XOnlyPublicKey =
        PublicKey::from_slice(&schnorr_public_key).unwrap().into();
    let reveal_script = Builder::new()
        .push_slice(schnorr_public_key.serialize())
        .push_opcode(opcodes::all::OP_CHECKSIG);
    let reveal_script = inscription
        .append_reveal_script_to_builder(reveal_script)
        .into_script();

    let taproot_spend_info = TaprootBuilder::new()
        .add_leaf(0, reveal_script.clone())
        .expect("adding leaf should work")
        .finalize(&secp256k1, schnorr_public_key)
        .expect("finalizing taproot builder should work");

    let control_block = taproot_spend_info
        .control_block(&(reveal_script.clone(), LeafVersion::TapScript))
        .expect("should compute control block");

    let commit_tx_address = Address::p2tr_tweaked(taproot_spend_info.output_key(), network);

    let commit_input_index = 0;

    let (_, reveal_fee) = build_reveal_transaction(
        commit_input_index,
        &control_block,
        fee_per_vbytes,
        reveal_output.clone(),
        reveal_input.clone(),
        &reveal_script,
    );

    let mut target_value = reveal_fee;
    // for premining
    target_value += TARGET_POSTAGE;

    build_commit_transaction_with_fee(
        &fee_payer,
        commit_tx_address.script_pubkey(),
        fee_per_vbytes,
        target_value,
    );
}

fn build_commit_transaction_with_fee(
    fee_payer: &Address,
    recipient: ScriptBuf,
    fee_per_vbytes: u64,
    target: Amount,
) -> Result<(Transaction, Vec<Utxo>), u64> {
    let (mut input, mut output) = (vec![], vec![]);

    if !recipient.is_op_return() {
        let dust_value = recipient.minimal_non_dust();

        if target < dust_value {
            ic_cdk::trap("DUST VALUE")
        }
    }

    let txn = Transaction {
        input,
        output,
        version: Version(2),
        lock_time: LockTime::ZERO,
    };
    todo!()
}
