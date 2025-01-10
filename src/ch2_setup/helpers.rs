#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use internal::convert::BlockchainInfo;
use internal::bitcoind_client::BitcoindClient;
use internal::hex_utils;
use base64;
use crate::ch2_setup::exercises::{
    BitcoindClientExercise
};
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::amount::Amount;
use bitcoin::transaction::Version;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::{Network, OutPoint, Sequence, TxIn, TxOut, WPubkeyHash, Witness};
use lightning_block_sync::poll::ValidatedBlockHeader;
use bitcoin::hash_types::{BlockHash};
use bitcoin::blockdata::script::ScriptBuf;
use lightning_block_sync::http::HttpEndpoint;
use lightning_block_sync::rpc::RpcClient;
use lightning_block_sync::{AsyncBlockSourceResult, BlockData, BlockHeaderData, BlockSource};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use lightning::chain::Listen;
use lightning_block_sync::init::validate_best_block_header;
use lightning_block_sync::poll::ChainPoller;
use lightning_block_sync::SpvClient;
use lightning_block_sync::UnboundedCache;
use std::ops::Deref;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::consensus::{encode, Decodable, Encodable};

pub fn get_http_endpoint(host: &String, port: u16) -> HttpEndpoint {
  HttpEndpoint::for_host(host.clone()).with_port(port)
}

pub fn format_rpc_credentials(rpc_user: &String, rpc_password: &String) -> String {
  base64::encode(format!("{}:{}", rpc_user.clone(), rpc_password.clone()))
}

pub fn new_rpc_client(rpc_credentials: &String, http_endpoint: HttpEndpoint) -> Arc<RpcClient> {
  let client = RpcClient::new(&rpc_credentials, http_endpoint).unwrap();
  Arc::new(client)
}

pub async fn test_rpc_call(bitcoind_rpc_client: &RpcClient) -> std::io::Result<BlockchainInfo> {
  let _dummy = bitcoind_rpc_client
      .call_method::<BlockchainInfo>("getblockchaininfo", &vec![])
      .await
      .map_err(|_| {
          std::io::Error::new(std::io::ErrorKind::PermissionDenied,
          "Failed to make initial call to bitcoind - please check your RPC user/password and access settings")
      })?;
  Ok(_dummy)
}


pub async fn get_best_block(bitcoind: BitcoindClientExercise) -> ValidatedBlockHeader {
  let best_block_header = validate_best_block_header(&bitcoind).await.unwrap();
  best_block_header
}

pub fn get_chain_poller(bitcoind: BitcoindClientExercise, network: Network) 
    -> ChainPoller<Arc<BitcoindClientExercise>, BitcoindClientExercise> {
    let bitcoind = Arc::new(bitcoind);
    ChainPoller::new(bitcoind, network)
}

pub fn get_new_cache() -> HashMap<BlockHash, ValidatedBlockHeader> {
  UnboundedCache::new()
}

pub fn get_spv_client<'a, L: Deref>(
  best_block_header: ValidatedBlockHeader,
  poller: ChainPoller<Arc<BitcoindClientExercise>, BitcoindClientExercise>,
  cache: &'a mut UnboundedCache,  // Take a mutable reference instead
  listener: L
) -> SpvClient<'a,  // Use the lifetime parameter here
             ChainPoller<Arc<BitcoindClientExercise>, BitcoindClientExercise>,
             UnboundedCache,
             L> 
where
  L::Target: Listen,
{
  SpvClient::new(best_block_header, poller, cache, listener)  // No need for &mut here
}



pub trait ToHex {
    fn to_hex(&self) -> String;
}

impl ToHex for Transaction {
    fn to_hex(&self) -> String {
        encode::serialize_hex(self)
    }
}

pub async fn get_tx_hex() -> Transaction {
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

    // Sign the transaction
    let signed_tx = bitcoind.sign_raw_transaction_with_wallet(tx_hex).await;

    let final_tx: Transaction =
        encode::deserialize(&hex_utils::to_vec(&signed_tx.hex).unwrap()).unwrap();

    final_tx
}