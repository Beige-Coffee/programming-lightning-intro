#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use base64;
pub mod helper;
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
use pl_00_intro::ch1_intro_htlcs::solutions::{
    build_htlc_commitment_transaction, two_of_two_multisig_redeem_script,generate_p2wsh_signature
};
use bitcoin::PublicKey as BitcoinPubKey;
use pl_00_intro::internal::bitcoind_client;
use pl_00_intro::internal::bitcoind_client::BitcoindClient;
use pl_00_intro::internal::convert;
use pl_00_intro::internal::convert::BlockchainInfo;
use pl_00_intro::internal::hex_utils;
use pl_00_intro::internal::helper::{secp256k1_pubkey_from_private_key, pubkey_from_private_key, secp256k1_private_key};
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use hex;
use helper::{get_bitcoind_client, get_unspent_output, sign_raw_transaction, get_funding_input, get_arg};

/// hash160 of the empty string
const HASH160_DUMMY: [u8; 20] = [
    0xb4, 0x72, 0xa2, 0x66, 0xd0, 0xbd, 0x89, 0xc1, 0x37, 0x06, 0xa4, 0x13, 0x2c, 0xcf, 0xb1, 0x6f,
    0x7c, 0x3b, 0x9f, 0xcb,
];


pub struct KeyManager{
    pub funding_private_key: SecretKey,
    pub funding_public_key: PublicKey,
    pub htlc_pubkey: PublicKey,
    pub delayed_pubkey: PublicKey,
    pub pubkey: PublicKey,
    pub revocation_pubkey: PublicKey,
}

pub async fn create_broadcast_funding_tx(bitcoind: BitcoindClient,
                                         txid: String,
                                        our_key_manager: KeyManager,
                                        counterparty_key_manager: KeyManager) {

    let txid_index = 0;
    let funding_txin = get_funding_input(txid.to_string(), txid_index);
    let funding_amount = 4_999_800;

    let payment_hash160 = HASH160_DUMMY;
    let to_self_delay: i64 = 144;

    let htlc_amount = 390_000;
    let local_amount = 2_550_000;
    let remote_amount = 2_000_000;


    let tx = build_htlc_commitment_transaction(
        funding_txin,
        &our_key_manager.revocation_pubkey,
        &counterparty_key_manager.htlc_pubkey,
        &our_key_manager.htlc_pubkey,
        &our_key_manager.delayed_pubkey,
        counterparty_key_manager.pubkey,
        to_self_delay,
        &payment_hash160,
        htlc_amount,
        local_amount,
        remote_amount);

    // Prepare the redeem script for signing (e.g., P2PKH or P2WPKH)
    let redeem_script =
        two_of_two_multisig_redeem_script(
            &our_key_manager.funding_public_key,
            &counterparty_key_manager.funding_public_key);

    let our_signature = generate_p2wsh_signature(
         tx.clone(), 
         txid_index,
         &redeem_script,
         funding_amount,
         EcdsaSighashType::All,
        our_key_manager.funding_private_key);

    let counterparty_signature = generate_p2wsh_signature(
         tx.clone(), 
         txid_index,
         &redeem_script,
         funding_amount,
         EcdsaSighashType::All,
        counterparty_key_manager.funding_private_key);

    // Convert signature to DER and append SigHashType
    let mut our_signature_der = our_signature.serialize_der().to_vec();
    our_signature_der.push(EcdsaSighashType::All as u8);

    let mut counterparty_signature_der = counterparty_signature.serialize_der().to_vec();
    counterparty_signature_der.push(EcdsaSighashType::All as u8);

    // Determine signature order based on pubkey comparison
    let our_sig_first = our_key_manager.funding_public_key.serialize()[..] > counterparty_key_manager.funding_public_key.serialize()[..];

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

    println!("\n");
    println!("Tx ID: {}", signed_tx.compute_txid());
    println!("\n");
    println!("Tx Hex: {}", serialize_hex(&signed_tx));

}

#[tokio::main]
async fn main() {

    // get bitcoin client
    let bitcoind = get_bitcoind_client().await;

    // Parse the argument as txid
    let txid = get_arg();

    // Get our keys
    let our_funding_private_key = secp256k1_private_key(&[0x01; 32]);
    let our_funding_public_key = secp256k1_pubkey_from_private_key(&[0x01; 32]);
    let local_htlc_pubkey = secp256k1_pubkey_from_private_key(&[0x11; 32]);
    let revocation_pubkey = secp256k1_pubkey_from_private_key(&[0x12; 32]);
    let to_local_delayed_pubkey = secp256k1_pubkey_from_private_key(&[0x13; 32]);
    let local_pubkey = secp256k1_pubkey_from_private_key(&[0x14; 32]);

    let our_key_manager = KeyManager{
            funding_private_key: our_funding_private_key,
            funding_public_key: our_funding_public_key,
            htlc_pubkey: local_htlc_pubkey,
            delayed_pubkey: to_local_delayed_pubkey,
            pubkey: local_pubkey,
            revocation_pubkey: revocation_pubkey,
        };

    // Get our Counterparty Pubkey
    let counterparty_funding_private_key = secp256k1_private_key(&[0x02; 32]);
    let counterparty_funding_public_key = secp256k1_pubkey_from_private_key(&[0x02; 32]);
    let counterparty_htlc_pubkey = secp256k1_pubkey_from_private_key(&[0x21; 32]);
    let counterparty_pubkey = secp256k1_pubkey_from_private_key(&[0x22; 32]);
    let counterparty_delayed_key = secp256k1_pubkey_from_private_key(&[0x23; 32]);
    let counterparty_revocation_key = secp256k1_pubkey_from_private_key(&[0x24; 32]);

    let counterparty_key_manager = KeyManager{
            funding_private_key: counterparty_funding_private_key,
            funding_public_key: counterparty_funding_public_key,
            htlc_pubkey: counterparty_htlc_pubkey,
            delayed_pubkey: counterparty_delayed_key,
            pubkey: counterparty_pubkey,
            revocation_pubkey: counterparty_revocation_key,
        };

    create_broadcast_funding_tx(bitcoind, txid, our_key_manager, counterparty_key_manager).await;

    // Add a delay to allow the spawned task to complete
    sleep(Duration::from_secs(2)).await;
}
