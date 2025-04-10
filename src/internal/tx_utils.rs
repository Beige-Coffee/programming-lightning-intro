#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use crate::exercises;
use internal::bitcoind_client::BitcoindClient;
use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::Hash;
use bitcoin::hashes::HashEngine;
use bitcoin::secp256k1;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::{SecretKey, PublicKey, Scalar};
use bitcoin::PublicKey as BitcoinPublicKey;
use bitcoin::script::{ScriptBuf};
use bitcoin::{OutPoint, Sequence, Transaction, TxIn, TxOut, Witness};
use bitcoin::amount::Amount;
use bitcoin::transaction::Version;
use bitcoin::locktime::absolute::LockTime;
use internal::builder::Builder;
use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::{PubkeyHash};
use bitcoin::{Network};
use bitcoin::consensus::encode::serialize_hex;
use internal::hex_utils;
use bitcoin::consensus::{encode};
use bitcoin::hash_types::Txid;
use std::env;
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::secp256k1::Message;
use bitcoin::sighash::EcdsaSighashType;
use bitcoin::sighash::SighashCache;
use exercises::exercises::{ two_of_two_multisig_witness_script};

pub fn get_funding_input(input_tx_id_str: String, vout: usize) -> TxIn {

    // Get an unspent output to spend
    let mut tx_id_bytes = hex::decode(input_tx_id_str).expect("Valid hex string");
    tx_id_bytes.reverse();
    let input_txid = Txid::from_byte_array(tx_id_bytes.try_into().expect("Expected 32 bytes"));

    // Create a transaction spending this UTXO
    TxIn {
        previous_output: OutPoint {
            txid: input_txid,
            vout: vout as u32,
        },
        sequence: Sequence::MAX,
        script_sig: ScriptBuf::new(),
        witness: Witness::new(),
    }

}

pub async fn get_unspent_output(bitcoind: BitcoindClient) -> TxIn {
  let utxos = bitcoind.list_unspent().await;
  let utxo = utxos
      .0
      .iter()
      .find(|utxo| utxo.amount > 4_999_999 && utxo.amount < 6_000_000)
      .expect("No UTXOs with positive balance found");

    let tx_input = TxIn {
        previous_output: OutPoint {
            txid: utxo.txid,
            vout: utxo.vout,
        },
        sequence: Sequence::MAX,
        script_sig: ScriptBuf::new(),
        witness: Witness::new(),
    };

    tx_input
}

pub fn get_htlc_funding_input(input_tx_id_str: String, vout: usize) -> TxIn {

    // Get an unspent output to spend
    let mut tx_id_bytes = hex::decode(input_tx_id_str).expect("Valid hex string");
    tx_id_bytes.reverse();
    let input_txid = Txid::from_byte_array(tx_id_bytes.try_into().expect("Expected 32 bytes"));

    // Create a transaction spending this UTXO
    TxIn {
        previous_output: OutPoint {
            txid: input_txid,
            vout: vout as u32,
        },
        sequence: Sequence(0),
        script_sig: ScriptBuf::new(),
        witness: Witness::new(),
    }

}

pub fn build_unsigned_input(txid: String, vout: u32, sequence: Sequence) -> TxIn {

    // Get an unspent output to spend
    let mut tx_id_bytes = hex::decode(txid).expect("Valid hex string");
    tx_id_bytes.reverse();
    let input_txid = Txid::from_byte_array(tx_id_bytes.try_into().expect("Expected 32 bytes"));

    TxIn {
        previous_output: OutPoint {
            txid: input_txid,
            vout: vout,
        },
        sequence: sequence,
        script_sig: ScriptBuf::new(),
        witness: Witness::new(),
    }
}

pub fn build_output(amount: u64, output_script: ScriptBuf) -> TxOut {
    TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: output_script,
    }
}

pub fn build_transaction(
    version: Version,
    locktime: LockTime,
    tx_ins: Vec<TxIn>,
    tx_outs: Vec<TxOut>,
) -> Transaction {
    Transaction {
        version,
        lock_time: locktime,
        input: tx_ins,
        output: tx_outs,
    }
}

