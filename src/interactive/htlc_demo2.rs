#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use base64;
use crate::interactive::helper;
use internal::builder::Builder;
use crate::internal;
use crate::ch1_intro_htlcs;
use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::address::Address;
use bitcoin::amount::Amount;
use bitcoin::blockdata::block::Block;
use bitcoin::blockdata::block::Header;
use bitcoin::blockdata::constants::WITNESS_SCALE_FACTOR;
use bitcoin::blockdata::script::ScriptBuf;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::consensus::{encode, Decodable, Encodable};
use bitcoin::hash_types::{BlockHash, Txid};
use bitcoin::hashes::Hash;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::secp256k1::Message;
use bitcoin::secp256k1::{self, Secp256k1};
use bitcoin::sighash::EcdsaSighashType;
use bitcoin::sighash::SighashCache;
use bitcoin::transaction::Version;
use bitcoin::secp256k1::{PublicKey, SecretKey};
use bitcoin::{Network, OutPoint, Sequence, TxIn, TxOut, WPubkeyHash, Witness};
use lightning::chain::chaininterface::{BroadcasterInterface, ConfirmationTarget, FeeEstimator};
use lightning::chain::transaction::TransactionData;
use lightning::events::bump_transaction::{Utxo, WalletSource};
use lightning::log_error;
use lightning::routing::scoring::{ProbabilisticScorer, ProbabilisticScoringDecayParameters};
use lightning::sign::ChangeDestinationSource;
use lightning::util::logger::Logger;
use lightning_block_sync::http::HttpEndpoint;
use lightning_block_sync::http::JsonResponse;
use lightning_block_sync::init::validate_best_block_header;
use lightning_block_sync::poll;
use lightning_block_sync::poll::ChainPoller;
use lightning_block_sync::poll::ChainTip;
use lightning_block_sync::rpc::RpcClient;
use lightning_block_sync::SpvClient;
use lightning_block_sync::{AsyncBlockSourceResult, BlockData, BlockHeaderData, BlockSource};
use ch1_intro_htlcs::solutions::{
    build_htlc_commitment_transaction, build_commitment_transaction, two_of_two_multisig_witness_script, build_refund_transaction
};
use bitcoin::PublicKey as BitcoinPubKey;
use internal::bitcoind_client;
use internal::bitcoind_client::BitcoindClient;
use internal::convert;
use internal::convert::BlockchainInfo;
use internal::hex_utils;
use bitcoin::hashes::sha256::Hash as Sha256;
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use hex;
use bitcoin::hashes::ripemd160::Hash as Ripemd160;
use internal::helper::{pubkey_from_private_key, secp256k1_private_key,
                      p2wpkh_output_script, build_output, build_transaction};
use helper::{get_bitcoind_client, get_unspent_output, sign_raw_transaction, generate_p2wsh_signature, get_htlc_funding_input, get_arg};


pub async fn create_broadcast_funding_tx(bitcoind: BitcoindClient,
                                        txid: String,
                                        funding_amount: u64) {

    let txid_index = 2;
    let txin = get_htlc_funding_input(txid.to_string(), txid_index);
    let our_public_key = pubkey_from_private_key(&[0x01; 32]);

    let tx = build_p2wpkh_tx(txin, our_public_key);

    let signed_tx = sign_transaction(tx);

    println!("\n");
    println!("Tx ID: {}", signed_tx.compute_txid());
    println!("\n");
    println!("Tx Hex: {}", serialize_hex(&signed_tx));

}


pub async fn run(funding_txid: String) {

    // Parse the argument as txid
    let txid = funding_txid;

    // get bitcoin client
    let bitcoind = get_bitcoind_client().await;

    // Get our keys
    let our_public_key = pubkey_from_private_key(&[0x01; 32]);
    
    let funding_amount = 5_000_000;
    
    create_broadcast_funding_tx(bitcoind, txid.clone(), funding_amount).await;

    // Add a delay to allow the spawned task to complete
    sleep(Duration::from_secs(2)).await;
}

pub fn sign_transaction(tx: Transaction)-> Transaction {

    let funding_amount = 405_000;
    let txid_index = 2;
    
    let our_public_key = pubkey_from_private_key(&[0x01; 32]);
    let our_private_key = secp256k1_private_key(&[0x01; 32]);

    let secret = "ProgrammingLightning".to_string();
    let secret_bytes = secret.as_bytes();
    let payment_hash = Sha256::hash(secret_bytes).to_byte_array();
    let payment_hash160 = Ripemd160::hash(&payment_hash).to_byte_array();

    // build funding transaction using the function we created
    let redeem_script = build_hash_locked_script(&our_public_key,
                                                &payment_hash160);

    let path = true;

    let mut signed_tx = tx.clone();

    if path {

        // Push secret
        signed_tx.input[0].witness.push(secret_bytes);


        signed_tx.input[0].witness.push(vec!(1));

    } else {

    let signature = generate_p2wsh_signature(
         tx.clone(), 
         txid_index,
         &redeem_script,
         funding_amount,
         EcdsaSighashType::All,
        our_private_key);

    // Convert signature to DER and append SigHashType
    let mut signature_der = signature.serialize_der().to_vec();
    signature_der.push(EcdsaSighashType::All as u8);

    signed_tx.input[0].witness.push(signature_der);

    signed_tx.input[0].witness.push(vec![]);

    }

    // push witness
    signed_tx.input[0]
        .witness
        .push(redeem_script.clone().into_bytes());

    signed_tx
}

fn build_p2wpkh_tx(txin: TxIn, pubkey: PublicKey) -> Transaction {
    let output_script = p2wpkh_output_script(pubkey);
    let output = build_output(405_000, output_script);
    
    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(version,
                      locktime,
                      vec![txin],
                      vec![output]);
    tx
    
}

fn build_hash_locked_script(pubkey: &PublicKey,
                     payment_hash160: &[u8; 20]) -> ScriptBuf {
    Builder::new()
        .push_opcode(opcodes::OP_IF)
        .push_opcode(opcodes::OP_HASH160)
        .push_slice(payment_hash160)
        .push_opcode(opcodes::OP_EQUAL)
        .push_opcode(opcodes::OP_ELSE)
        .push_int(200)
        .push_opcode(opcodes::OP_CLTV)
        .push_opcode(opcodes::OP_DROP)
        .push_key(pubkey)
        .push_opcode(opcodes::OP_CHECKSIG)
        .push_opcode(opcodes::OP_ENDIF)
    .into_script()
}