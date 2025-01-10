#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use internal::bitcoind_client::BitcoindClient;
use crate::ch2_setup::exercises::{
    BitcoindClientExercise,poll_for_blocks,poll_for_blocks2
};
use crate::ch2_setup::persist_exercise::{
    SimpleStore
};
use crate::ch2_setup::fee_estimator_exercise::{
    get_est_sat_per_1000_weight
};
use crate::ch2_setup::helpers::{get_tx_hex};
use lightning::util::persist::KVStore;
use base64;
use bitcoin::{Network};
use lightning_block_sync::http::HttpEndpoint;
use lightning_block_sync::{AsyncBlockSourceResult, BlockData, BlockHeaderData, BlockSource};
use lightning_block_sync::rpc::RpcClient;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use lightning::chain::Listen;
use lightning_block_sync::init::validate_best_block_header;
use lightning_block_sync::poll::ChainPoller;
use lightning_block_sync::SpvClient;
use bitcoin::blockdata::block::Header;
use lightning::chain::transaction::TransactionData;
use bitcoin::blockdata::block::Block;
use bitcoin::consensus::{encode};
use lightning::chain::chaininterface::{BroadcasterInterface, ConfirmationTarget, FeeEstimator};
use tokio::time::{sleep, Duration};

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

#[tokio::test]
async fn get_bitcoind() {
    let http_endpoint = HttpEndpoint::for_host("0.0.0.0".to_string()).with_port(18443);
    let rpc_credentials = base64::encode(format!(
        "{}:{}",
        "bitcoind".to_string(),
        "bitcoind".to_string()
    ));
    let bitcoind_rpc_client = RpcClient::new(&rpc_credentials, http_endpoint).unwrap();

    // Test get_best_block
    let (hash, height) = bitcoind_rpc_client.get_best_block().await.unwrap();
    assert!(height > Some(1), "Height should be greater than 1");

    // Test get_header
    let header_data = bitcoind_rpc_client
        .get_header(&hash, height)
        .await
        .expect("Should fetch header");
    assert!(header_data.height > 1, "Height should be greater than 1");

    // Test get_block
    let block_data = bitcoind_rpc_client
        .get_block(&hash)
        .await
        .expect("Should fetch block");
    
}

#[tokio::test]
async fn test_new_bitcoind() {

    let host = "0.0.0.0".to_string();
    let port: u16 = 18443;
    let rpc_user = "bitcoind".to_string();
    let rpc_password = "bitcoind".to_string();
    let network = Network::Regtest;
    
    let bitcoind_rpc_client = BitcoindClientExercise::new(host, port, rpc_user, rpc_password, network).await.unwrap();
}

#[tokio::test]
async fn test_poll_for_blocks() {

    let host = "0.0.0.0".to_string();
    let port: u16 = 18443;
    let rpc_user = "bitcoind".to_string();
    let rpc_password = "bitcoind".to_string();
    let network = Network::Regtest;

    let bitcoind_rpc_client = BitcoindClientExercise::new(host, port, rpc_user, rpc_password, network).await.unwrap();

    let listener = Listener{};

    poll_for_blocks(bitcoind_rpc_client, network, listener); 
}

#[tokio::test]
async fn test_poll_for_blocks2() {

    let host = "0.0.0.0".to_string();
    let port: u16 = 18443;
    let rpc_user = "bitcoind".to_string();
    let rpc_password = "bitcoind".to_string();
    let network = Network::Regtest;

    let bitcoind_rpc_client = BitcoindClientExercise::new(host, port, rpc_user, rpc_password, network).await.unwrap();

    let listener = Listener{};

    poll_for_blocks2(bitcoind_rpc_client, network, listener); 
}

#[tokio::test]
async fn test_simple_store() {

    let simple_store = SimpleStore::new(); 

    // Create some example data
    let data = vec![1, 2, 3, 4, 5]; 
    simple_store.write("test", "test2", "key1", &data);
    simple_store.write("test", "test2", "key2", &data);
    
    match simple_store.read("test", "test2", "key1") {
        Ok(data) => println!("Read data: {:?}", data),
        Err(e) => println!("Error reading data: {}", e),
    }

    simple_store.remove("test", "test2", "key1", true);

    let keys = simple_store.list("test", "test2");
    assert_eq!(keys.expect("Keys should be returned").len(), 1);
}
#[tokio::test]
async fn test_fees() {

    //let fee_estimator = MyAppFeeEstimator::new();
    let high_fee_target = ConfirmationTarget::UrgentOnChainSweep;
    let low_fee_target = ConfirmationTarget::MinAllowedAnchorChannelRemoteFee;

    let host = "0.0.0.0".to_string();
    let port: u16 = 18443;
    let rpc_user = "bitcoind".to_string();
    let rpc_password = "bitcoind".to_string();
    let network = Network::Regtest;

    let bitcoind_rpc_client = BitcoindClientExercise::new(host.clone(), port, rpc_user.clone(), rpc_password.clone(), network).await.unwrap();

    // check UrgentOnChainSweep
    let high_fees = bitcoind_rpc_client.get_est_sat_per_1000_weight(high_fee_target);
    assert_eq!(high_fees, 6);

    // check MinAllowedAnchorChannelRemoteFee
    let low_fees = bitcoind_rpc_client.get_est_sat_per_1000_weight(low_fee_target);
    assert_eq!(low_fees, 2);
}

#[tokio::test]
async fn test_broadcast() {

    let host = "0.0.0.0".to_string();
    let port: u16 = 18443;
    let rpc_user = "bitcoind".to_string();
    let rpc_password = "bitcoind".to_string();
    let network = Network::Regtest;

    let bitcoind_rpc_client = BitcoindClientExercise::new(host.clone(), port, rpc_user.clone(), rpc_password.clone(), network).await.unwrap();

    let internal_bitcoind = BitcoindClient::new(host.clone(), port, rpc_user.clone(), rpc_password.clone(), network).await.unwrap();

    let tx = get_tx_hex().await;

    let tx_hex = encode::serialize_hex(&tx);

    bitcoind_rpc_client.broadcast_transactions(&[&tx]);

    tokio::time::sleep(Duration::from_millis(250)).await;

    let mempool = internal_bitcoind.get_raw_mempool().await;

    let txid = tx.compute_txid().to_string();

    //assert_eq!(mempool.transaction_ids, vec!["0.0.0.0".to_string()]);

    assert!(mempool.transaction_ids.contains(&txid));
}