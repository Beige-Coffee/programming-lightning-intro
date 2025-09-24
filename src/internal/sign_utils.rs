#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use crate::exercises;
use internal::bitcoind_client::BitcoindClient;
use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::Hash;
use bitcoin::hashes::HashEngine;
use bitcoin::secp256k1;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::{SecretKey, PublicKey as secp256k1PublicKey, Scalar};
use bitcoin::PublicKey;
use bitcoin::script::{ScriptBuf};
use bitcoin::{OutPoint, Sequence, Transaction, TxIn, TxOut, Witness};
use bitcoin::amount::Amount;
use bitcoin::transaction::Version;
use bitcoin::locktime::absolute::LockTime;
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

pub fn sign_funding_transaction(tx: Transaction,
                                our_funding_public_key: PublicKey,
                                our_funding_private_key: SecretKey,
                                counterparty_funding_public_key: PublicKey,
                                counterparty_funding_private_key: SecretKey,
                               )-> Transaction {

    let funding_amount = 5_000_000;
    let txid_index = 0;

    // Prepare the redeem script for signing (e.g., P2PKH or P2WPKH)
    let redeem_script =
        two_of_two_multisig_witness_script(
            &our_funding_public_key,
            &counterparty_funding_public_key);

    let our_signature = generate_p2wsh_signature(
         tx.clone(), 
         txid_index,
         &redeem_script,
         funding_amount,
         EcdsaSighashType::All,
        our_funding_private_key);

    let counterparty_signature = generate_p2wsh_signature(
         tx.clone(), 
         txid_index,
         &redeem_script,
         funding_amount,
         EcdsaSighashType::All,
    counterparty_funding_private_key);

    // Convert signature to DER and append SigHashType
    let mut our_signature_der = our_signature.serialize_der().to_vec();
    our_signature_der.push(EcdsaSighashType::All as u8);

    let mut counterparty_signature_der = counterparty_signature.serialize_der().to_vec();
    counterparty_signature_der.push(EcdsaSighashType::All as u8);

    // Determine signature order based on pubkey comparison
    let our_sig_first = our_funding_public_key.inner.serialize()[..] > counterparty_funding_public_key.inner.serialize()[..];

    // Add the signature and public key to the witness
    let mut signed_tx = tx.clone();

    // First push empty element for NULLDUMMY compliance
    signed_tx.input[0].witness.push(Vec::new());

    // Push signatures in correct order
    if our_sig_first {
        signed_tx.input[0].witness.push(our_signature_der);
        signed_tx.input[0].witness.push(counterparty_signature_der);
    } else {
        signed_tx.input[0].witness.push(counterparty_signature_der);
        signed_tx.input[0].witness.push(our_signature_der);
    }

    signed_tx.input[0]
        .witness
        .push(redeem_script.clone().into_bytes());

    signed_tx
}

pub fn generate_p2wsh_signature(
    transaction: Transaction,
    input_idx: usize,
    witness_script: &ScriptBuf,
    value: u64,
    sighash_type: EcdsaSighashType,
    private_key: secp256k1::SecretKey,
) -> Signature {
    let secp = Secp256k1::new();

    let message =
        generate_p2wsh_message(transaction, input_idx, witness_script, value, sighash_type);
    let signature = secp.sign_ecdsa(&message, &private_key);

    signature
}

pub async fn sign_raw_transaction(bitcoind: BitcoindClient,
                                tx: Transaction) -> Transaction {

  // we need to serialize the tx before passing it into
  //    `sign_raw_transaction_with_wallet`
  let tx_hex = serialize_hex(&tx);

  // sign the transaction
  let signed_tx = bitcoind.sign_raw_transaction_with_wallet(tx_hex).await;

  // convert signed transaction hex into a Transaction type
  let final_tx: Transaction =
      encode::deserialize(&hex_utils::to_vec(&signed_tx.hex).unwrap()).unwrap();

  final_tx
}

fn generate_p2wsh_message(
    transaction: Transaction,
    input_idx: usize,
    witness_script: &ScriptBuf,
    value: u64,
    sighash_type: EcdsaSighashType,
) -> Message {
    let secp = Secp256k1::new();

    let mut cache = SighashCache::new(&transaction);

    let amount = Amount::from_sat(value);

    let sighash = cache
        .p2wsh_signature_hash(input_idx, &witness_script, amount, sighash_type)
        .unwrap();

    let message = Message::from_digest_slice(&sighash[..]).unwrap();

    message
}
