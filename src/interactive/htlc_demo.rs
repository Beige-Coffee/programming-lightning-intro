#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::interactive::helper;
use crate::internal;
use crate::ch1_intro_htlcs;
use base64;
use internal::builder::Builder;
use bitcoin::address::Address;
use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::amount::Amount;
use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::blockdata::block::Block;
use bitcoin::blockdata::block::Header;
use bitcoin::blockdata::constants::WITNESS_SCALE_FACTOR;
use bitcoin::blockdata::script::ScriptBuf;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::consensus::{encode, Decodable, Encodable};
use bitcoin::hash_types::{BlockHash, Txid};
use bitcoin::hashes::Hash;
use bitcoin::key::XOnlyPublicKey;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::transaction::Version;
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
use internal::bitcoind_client;
use internal::bitcoind_client::BitcoindClient;
use ch1_intro_htlcs::solutions::{build_funding_transaction, to_local};
use internal::helper::{pubkey_from_private_key, build_output, build_transaction, bitcoin_pubkey_from_private_key, secp256k1_private_key, p2wpkh_output_script};
use internal::convert;
use internal::convert::{BlockchainInfo, ListUnspentUtxo, SignedTx};
use internal::hex_utils;
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use bitcoin::secp256k1::PublicKey;
use helper::{get_bitcoind_client, get_unspent_output, sign_raw_transaction};
use bitcoin::hashes::ripemd160::Hash as Ripemd160;

pub async fn build_funding_tx(bitcoind: BitcoindClient,
                                        tx_input: TxIn,
                                        tx_in_amount: u64) {

    // we're locking to a 2-of-2 multisig, so we need two public keys
    // normally, we would generate our own public key
    //   and the counterparty would send us theirs
    let our_public_key = pubkey_from_private_key(&[0x01; 32]);
    let revocation_key = pubkey_from_private_key(&[0x02; 32]);
    let to_local_delayed_pubkey = pubkey_from_private_key(&[0x03; 32]);
    let counterparty_public_key = pubkey_from_private_key(&[0x04; 32]);

    let to_self_delay = 144;

    // preimage
    let secret = "ProgrammingLightning".to_string();
    let secret_bytes = secret.as_bytes();
    let payment_hash = Sha256::hash(secret_bytes).to_byte_array();
    let payment_hash160 = Ripemd160::hash(&payment_hash).to_byte_array();

    let local_output_script = to_local(&revocation_key, &to_local_delayed_pubkey,
            to_self_delay);

    let remote_output_script = p2wpkh_output_script(counterparty_public_key);

    let local_output = build_output(3_594_500, local_output_script.to_p2wsh());
    let remote_output = build_output(1_000_500, remote_output_script);

    // build funding transaction using the function we created
    let output_script = build_hash_locked_script(&our_public_key,
                                                &payment_hash160);
    println!("Witness Script (hex): {}", output_script.to_hex_string());
    

    let htlc_output = build_output(405_000, output_script.to_p2wsh());

    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(version, locktime, vec![tx_input], vec![local_output, remote_output, htlc_output]);

    let signed_tx = sign_raw_transaction(bitcoind.clone(), tx).await;

    println!("\n");
    println!("Tx ID: {}", signed_tx.compute_txid());
    println!("\n");
    println!("Tx Hex: {}", serialize_hex(&signed_tx));
}

pub async fn run() {

    // get bitcoin client
    let bitcoind = get_bitcoind_client().await;

    // get an unspent output for funding transaction
    let tx_input = get_unspent_output(bitcoind.clone()).await;

    let tx_in_amount = 5_000_000;
    
        build_funding_tx(bitcoind, tx_input, tx_in_amount).await;

    // Add a delay to allow the spawned task to complete
    sleep(Duration::from_secs(2)).await;
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