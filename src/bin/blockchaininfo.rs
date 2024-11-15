#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use pl_00_intro::internal::bitcoind_client;
use pl_00_intro::internal::convert::BlockchainInfo;
use pl_00_intro::internal::bitcoind_client::BitcoindClient;
use lightning_block_sync::poll;
use lightning_block_sync::SpvClient;
use lightning_block_sync::http::JsonResponse;
use lightning_block_sync::poll::ChainPoller;
use bitcoin::blockdata::block::Header;
use bitcoin::blockdata::block::Block;
use lightning::chain::transaction::TransactionData;
use lightning_block_sync::poll::ChainTip;
use lightning_block_sync::init::validate_best_block_header;
use base64;
use bitcoin::address::{Address};
use bitcoin::blockdata::constants::WITNESS_SCALE_FACTOR;
use bitcoin::blockdata::script::ScriptBuf;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::consensus::{encode, Decodable, Encodable};
use bitcoin::hash_types::{BlockHash, Txid};
use bitcoin::hashes::Hash;
use bitcoin::key::XOnlyPublicKey;
use bitcoin::{Network, OutPoint, TxOut, WPubkeyHash};
use lightning::chain::chaininterface::{BroadcasterInterface, ConfirmationTarget, FeeEstimator};
use lightning::events::bump_transaction::{Utxo, WalletSource};
use lightning::routing::scoring::{ProbabilisticScorer, ProbabilisticScoringDecayParameters};
use lightning::log_error;
use lightning::sign::ChangeDestinationSource;
use lightning::util::logger::Logger;
use lightning_block_sync::http::HttpEndpoint;
use lightning_block_sync::rpc::RpcClient;
use lightning_block_sync::{AsyncBlockSourceResult, BlockData, BlockHeaderData, BlockSource};
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct Listener {

}

impl lightning::chain::Listen for Listener {
    fn filtered_block_connected(
        &self,
        header: &Header,
        txdata: &TransactionData<'_>,
        height: u32,
    ) {
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
        },
        ChainTip::Better(header) => {
            println!("Chain tip status: Found better chain tip!");
            println!("  New block Height: {}", header.to_best_block().height);
        },
        ChainTip::Worse(header) => {
            println!("Chain tip status: Found competing chain tip (worse than current)");
            println!("  Competing block Height: {}", header.to_best_block().height);
        },
    }
}

pub async fn get_bitcoind2() {
    let http_endpoint = HttpEndpoint::for_host("0.0.0.0".to_string()).with_port(18443);
    let rpc_credentials = base64::encode(format!(
        "{}:{}",
        "bitcoind".to_string(),
        "bitcoind".to_string()
    ));
    let mut bitcoind =  BitcoindClient{bitcoind_rpc_client: Arc::new(RpcClient::new(&rpc_credentials, http_endpoint).unwrap())};

    let _dummy = bitcoind.bitcoind_rpc_client
        .call_method::<BlockchainInfo>("getblockchaininfo", &vec![])
        .await
        .map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::PermissionDenied,
            "Failed to make initial call to bitcoind - please check your RPC user/password and access settings")
        });
    println!("Blockchain Info: {:?}", _dummy);

    let best_block_header = validate_best_block_header(&mut bitcoind).await.unwrap();

    let poller = ChainPoller::new(& mut bitcoind, Network::Regtest);

    let mut cache = HashMap::new();

    let listener = Listener{};

    let mut spv_client = SpvClient::new(best_block_header, poller, & mut cache, &listener);

    loop {
        let best_block = spv_client.poll_best_tip().await.unwrap();
        print_chain_tip_status(&best_block.0);
        tokio::time::sleep(Duration::from_secs(10)).await;
    }





}

#[tokio::main]
async fn main() {

    get_bitcoind2().await;
}
