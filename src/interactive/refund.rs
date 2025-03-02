#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use base64;
use crate::interactive::helper;
use crate::internal;
use crate::ch1_intro_htlcs;
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
    build_htlc_commitment_transaction, two_of_two_multisig_witness_script, build_refund_transaction
};
use bitcoin::PublicKey as BitcoinPubKey;
use internal::bitcoind_client;
use internal::bitcoind_client::BitcoindClient;
use internal::convert;
use internal::convert::BlockchainInfo;
use internal::hex_utils;
use internal::helper::{pubkey_from_private_key, bitcoin_pubkey_from_private_key, secp256k1_private_key};
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use hex;
use helper::{get_bitcoind_client, get_unspent_output, generate_p2wsh_signature, sign_raw_transaction, get_funding_input, get_arg};


pub struct KeyManager{
    pub funding_private_key: SecretKey,
    pub funding_public_key: PublicKey,
    pub commitment_pubkey: PublicKey,
}

pub async fn create_broadcast_funding_tx(bitcoind: BitcoindClient,
                                        txid: String,
                                        our_key_manager: KeyManager,
                                        counterparty_key_manager: KeyManager,
                                        funding_amount: u64,
                                        our_balance: u64,
                                        counterparty_balance: u64) {

    let txid_index = 0;
    let funding_txin = get_funding_input(txid.to_string(), txid_index);

    let tx = build_refund_transaction(
        funding_txin,
        our_key_manager.commitment_pubkey,
        counterparty_key_manager.commitment_pubkey,
        our_balance,
        counterparty_balance);

    let signed_tx = sign_funding_transaction(tx, our_key_manager, counterparty_key_manager);

    println!("\n");
    println!("Tx ID: {}", signed_tx.compute_txid());
    println!("\n");
    println!("Tx Hex: {}", serialize_hex(&signed_tx));

    // Broadcast it
    bitcoind.broadcast_transactions(&[&signed_tx]);
}


pub async fn run(funding_txid: String) {

    // Parse the argument as txid
    let txid = funding_txid;

    // get bitcoin client
    let bitcoind = get_bitcoind_client().await;

    // Get our keys
    let our_funding_private_key = secp256k1_private_key(&[0x01; 32]);
    let our_funding_public_key = pubkey_from_private_key(&[0x01; 32]);
    let our_commitment_pubkey = pubkey_from_private_key(&[0x11; 32]);

    let our_key_manager = KeyManager{
            funding_private_key: our_funding_private_key,
            funding_public_key: our_funding_public_key,
            commitment_pubkey: our_commitment_pubkey
        };

    // Get our Counterparty Pubkey
    let counterparty_funding_private_key = secp256k1_private_key(&[0x02; 32]);
    let counterparty_funding_public_key = pubkey_from_private_key(&[0x02; 32]);
    let counterparty_commitment_pubkey = pubkey_from_private_key(&[0x21; 32]);

    let counterparty_key_manager = KeyManager{
            funding_private_key: counterparty_funding_private_key,
            funding_public_key: counterparty_funding_public_key,
            commitment_pubkey: counterparty_commitment_pubkey,
        };
    
    let funding_amount = 5_000_000;
    let our_balance = 4_999_500;
    let counterparty_balance = 500;
    
    create_broadcast_funding_tx(bitcoind, txid.clone(), our_key_manager, counterparty_key_manager, funding_amount,
                               our_balance, counterparty_balance).await;

    // Add a delay to allow the spawned task to complete
    sleep(Duration::from_secs(2)).await;
}

pub fn sign_funding_transaction(tx: Transaction,
                                our_key_manager: KeyManager,
                               counterparty_key_manager: KeyManager)-> Transaction {

    let funding_amount = 5_000_000;
    let txid_index = 0;
    
    // Prepare the redeem script for signing (e.g., P2PKH or P2WPKH)
    let redeem_script =
        two_of_two_multisig_witness_script(
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

    signed_tx
}