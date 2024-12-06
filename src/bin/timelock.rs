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
use pl_00_intro::ch1_intro_htlcs::exercises::{build_timelocked_transaction, csv_p2pkh, build_output, build_transaction};
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
use helper::{get_bitcoind_client, get_unspent_output, sign_raw_transaction};

pub async fn create_timelock_tx(bitcoind: BitcoindClient,
                                tx_input: TxIn,
                                tx_out_amount: u64) {

    // get a public key to lock funds to
    let public_key = secp256k1_pubkey_from_private_key(&[0x01; 32]);

    // define a csv delay for the output
    let csv_delay: i64 = 14;
    let version = Version::ONE;
    let locktime = LockTime::ZERO;

    let output_script = csv_p2pkh(&public_key, csv_delay);
    
    let txout = build_output(tx_out_amount, output_script.to_p2wsh());

    let tx = build_transaction(version, locktime, vec![tx_input], vec![txout]);

    let signed_tx = sign_raw_transaction(bitcoind.clone(), tx).await;

    println!("Tx Hex: {}", serialize_hex(&signed_tx));
    //println!("Tx ID: {}", signed_tx.compute_txid());
}

#[tokio::main]
async fn main() {

    // get bitcoin client
    let bitcoind = get_bitcoind_client().await;

    // get an unspent output for funding transaction
    let tx_input = get_unspent_output(bitcoind.clone()).await;

    let tx_in_amount = 4_999_800;

    create_timelock_tx(bitcoind, tx_input, tx_in_amount).await;

    // Add a delay to allow the spawned task to complete
    sleep(Duration::from_secs(2)).await;
}