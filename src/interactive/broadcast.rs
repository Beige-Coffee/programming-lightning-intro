#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
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
use pl_00_intro::internal::convert;
use pl_00_intro::internal::convert::BlockchainInfo;
use pl_00_intro::internal::hex_utils;
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub struct Listener {}

impl lightning::chain::Listen for Listener {
    fn filtered_block_connected(&self, header: &Header, txdata: &TransactionData<'_>, height: u32) {
        println!("Filtered Block Connected: {:?}", height);
    }
    fn block_disconnected(&self, header: &Header, height: u32) {
        println!("Block Disconnected: {:?}", height);
    }

    // Provided method
    fn block_connected(&self, block: &Block, height: u32) {
        println!("Block Connected: {:?}", height);
    }
}

fn print_chain_tip_status(chain_tip: &ChainTip) {
    match chain_tip {
        ChainTip::Common => {
            println!("Chain tip status: No change (same as current best block)");
        }
        ChainTip::Better(header) => {
            println!("Chain tip status: Found better chain tip!");
            println!("  New block Height: {}", header.to_best_block().height);
        }
        ChainTip::Worse(header) => {
            println!("Chain tip status: Found competing chain tip (worse than current)");
            println!(
                "  Competing block Height: {}",
                header.to_best_block().height
            );
        }
    }
}

pub async fn get_bitcoind2() {
    let bitcoind = BitcoindClient::new(
        "0.0.0.0".to_string(),
        18443,
        "bitcoind".to_string(),
        "bitcoind".to_string(),
        Network::Regtest,
    )
    .await
    .unwrap();

    // Get an unspent output to spend
    let utxos = bitcoind.list_unspent().await;
    let utxo = utxos
        .0
        .iter()
        .find(|utxo| utxo.amount > 1000)
        .expect("No UTXOs with positive balance found");
    //println!("This is the UTXO we're spending {:?}", utxo);

    // Create a transaction spending this UTXO
    let tx_input = TxIn {
        previous_output: OutPoint {
            txid: utxo.txid,
            vout: utxo.vout,
        },
        sequence: Sequence::MAX,
        script_sig: ScriptBuf::new(),
        witness: Witness::new(),
    };

    // Create a destination address
    let dest_address = bitcoind.get_new_address().await;
    //println!("{:?}", dest_address);

    //println!("utxo.amount: {}", utxo.amount);
    // Create the transaction
    let tx = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![tx_input],
        output: vec![TxOut {
            value: Amount::from_sat(utxo.amount - 1000), // Subtract fee
            script_pubkey: dest_address.script_pubkey(),
        }],
    };

    let tx_hex = serialize_hex(&tx);

    //println!("Unsigned Tx: {}", tx_hex);
    // Sign the transaction
    let signed_tx = bitcoind.sign_raw_transaction_with_wallet(tx_hex).await;

    //println!("Signed Tx: {}", &signed_tx.hex);

    let final_tx: Transaction =
        encode::deserialize(&hex_utils::to_vec(&signed_tx.hex).unwrap()).unwrap();

    //println!("final_tx Tx: {}", serialize_hex(&final_tx));

    println!("Tx ID: {}", final_tx.compute_txid());

    // Broadcast it
    bitcoind.broadcast_transactions(&[&final_tx]);
}

#[tokio::main]
async fn main() {
    get_bitcoind2().await;

    // Add a delay to allow the spawned task to complete
    sleep(Duration::from_secs(2)).await;
}