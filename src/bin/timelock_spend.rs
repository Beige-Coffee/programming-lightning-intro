#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
pub mod helper;
use base64;
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
use bitcoin::sighash::EcdsaSighashType;
use bitcoin::sighash::SighashCache;
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
use pl_00_intro::internal::bitcoind_client;
use pl_00_intro::internal::bitcoind_client::BitcoindClient;
use pl_00_intro::ch1_intro_htlcs::exercises::{build_timelocked_transaction, csv_p2pkh, build_output, build_transaction,
                                             generate_p2wsh_signature, build_csv_input};
use pl_00_intro::internal::helper::{secp256k1_pubkey_from_private_key, pubkey_from_private_key, secp256k1_private_key};
use pl_00_intro::internal::convert;
use pl_00_intro::internal::convert::{BlockchainInfo, ListUnspentUtxo, SignedTx};
use pl_00_intro::internal::hex_utils;
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use bitcoin::PublicKey;
use helper::{get_bitcoind_client, get_unspent_output, sign_raw_transaction,
            get_funding_input, build_unsigned_input};

pub async fn spend_timelock_tx(bitcoind: BitcoindClient,
                                output_amount: u64) {

    // get utxo
    let txid = "75607cf4f4c5c63391766c8864daacc1723450fd4afdfdb7b6a75714c5528178";
    let txid_index = 0;
    let sequence = Sequence(14);
    let txin = build_unsigned_input(txid.to_string(), txid_index, sequence);

    // get a public key to lock funds to
    let our_public_key = pubkey_from_private_key(&[0x01; 32]);

    let output_script = our_public_key.p2wpkh_script_code().unwrap();

    let txout = build_output(output_amount, output_script.to_p2wsh());

    let version = Version::TWO;
    let locktime = LockTime::ZERO;
    let tx = build_transaction(version, locktime, vec![txin], vec![txout]);

    let signed_tx = sign_transaction(tx);

    println!("Tx Hex: {}", serialize_hex(&signed_tx));
    //println!("Tx ID: {}", signed_tx.compute_txid());
}

#[tokio::main]
async fn main() {

    // get bitcoin client
    let bitcoind = get_bitcoind_client().await;

    let output_amount = 4_999_800 - 10_000;

    spend_timelock_tx(bitcoind, output_amount).await;

    // Add a delay to allow the spawned task to complete
    sleep(Duration::from_secs(2)).await;
}

pub fn sign_transaction(tx: Transaction)-> Transaction {

    let our_secp256k1_public_key = secp256k1_pubkey_from_private_key(&[0x01; 32]);
    let our_private_key = secp256k1_private_key(&[0x01; 32]); 

    // define a csv delay for the output
    let csv_delay: i64 = 14;
    
    // Prepare the redeem script for signing (e.g., P2PKH or P2WPKH)
    let redeem_script =
        csv_p2pkh(
            &our_secp256k1_public_key,
            csv_delay);

    println!("Decoded script: {}", redeem_script);

    let signature = generate_p2wsh_signature(
         tx.clone(), 
         0,
         &redeem_script,
        4_999_800,
        EcdsaSighashType::All,
       our_private_key);

    // Convert signature to DER and append SigHashType
    let mut signature_der = signature.serialize_der().to_vec();
    signature_der.push(EcdsaSighashType::All as u8);

    // Add the signature and public key to the witness
    let mut signed_tx = tx.clone();

    signed_tx.input[0].witness.push(signature_der);

    signed_tx.input[0].witness.push(our_secp256k1_public_key.serialize());       

    signed_tx.input[0]
        .witness
        .push(redeem_script.clone().into_bytes());

    signed_tx
}