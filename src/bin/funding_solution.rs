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
use pl_00_intro::ch1_intro_htlcs::exercises::build_funding_transaction;
use pl_00_intro::internal::helper::{secp256k1_pubkey_from_private_key, pubkey_from_private_key, secp256k1_private_key};
use pl_00_intro::internal::convert;
use pl_00_intro::internal::convert::{BlockchainInfo, ListUnspentUtxo};
use pl_00_intro::internal::hex_utils;
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use bitcoin::PublicKey;

pub async fn get_bitcoind_client() -> BitcoindClient {
    let bitcoind = BitcoindClient::new(
        "0.0.0.0".to_string(),
        18443,
        "bitcoind".to_string(),
        "bitcoind".to_string(),
        Network::Regtest,
    )
    .await
    .unwrap();

    bitcoind
}

pub async fn get_unspent_output(bitcoind: BitcoindClient) -> ListUnspentUtxo {
    let utxos = bitcoind.list_unspent().await;
    let utxo = utxos
        .0
        .iter()
        .find(|utxo| utxo.amount > 4_999_999 && utxo.amount < 6_000_000)
        .expect("No UTXOs with positive balance found");

    utxo.clone()
}

pub async fn create_broadcast_funding_tx() {
    // get bitcoin client
    let bitcoind = get_bitcoind_client().await;

    // get an unspent output for funding transaction
    let utxo = get_unspent_output(bitcoind.clone()).await;

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

    // we're locking to a 2-of-2 multisig, so we need two public keys
    // normally, we would generate our own public key
    //   and the counterparty would send us their
    let our_public_key = secp256k1_pubkey_from_private_key(&[0x01; 32]);
    let counterparty_pubkey = secp256k1_pubkey_from_private_key(&[0x02; 32]);

    // build funding transaction using the function we created
    let tx = build_funding_transaction(
            vec![tx_input],
            &our_public_key,
            &counterparty_pubkey,
            Amount::from_sat(4_999_800),
        );

    // we need to serialize the tx before passing it into
    //    `sign_raw_transaction_with_wallet`
    let tx_hex = serialize_hex(&tx);

    // sign the transaction
    let signed_tx = bitcoind.sign_raw_transaction_with_wallet(tx_hex).await;

    // convert signed transaction hex into a Transaction type
    let final_tx: Transaction =
        encode::deserialize(&hex_utils::to_vec(&signed_tx.hex).unwrap()).unwrap();

    println!("Tx Hex: {}", serialize_hex(&final_tx));
    println!("Tx ID: {}", final_tx.compute_txid());

    // broadcast transaction
    bitcoind.broadcast_transactions(&[&final_tx]);
}

#[tokio::main]
async fn main() {
    create_broadcast_funding_tx().await;

    // Add a delay to allow the spawned task to complete
    sleep(Duration::from_secs(2)).await;
}
