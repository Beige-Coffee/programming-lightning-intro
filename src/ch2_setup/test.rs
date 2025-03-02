#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use internal::bitcoind_client::BitcoindClient;
use crate::ch2_setup::exercises::{
    BitcoindClientExercise,poll_for_blocks,poll_for_blocks2
};
use crate::ch2_setup::network_exerciseV2::{
    start_listener,PeerManager
};
use crate::ch2_setup::bitcoin_client::{
    BitcoinClient,
};
use crate::ch2_setup::bitcoin_client_solutions::BitcoinClient as BitcoinClientSolutions;
use crate::ch2_setup::persist_exercise::{
    SimpleStore
};
use crate::ch2_setup::payment_exercise::{
    send_payment
};
use crate::ch2_setup::fee_estimator_exercise::{
    get_est_sat_per_1000_weight
};
use tokio::net::TcpStream;
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


#[tokio::test]
async fn test_new_bitcoin_client() {

    let host = "0.0.0.0".to_string();
    let port: u16 = 18443;
    let rpc_user = "bitcoind".to_string();
    let rpc_password = "bitcoind".to_string();
    let network = Network::Regtest;

    let bitcoind_rpc_client = BitcoinClient::new(host, port, rpc_user, rpc_password, network).await.unwrap();
}

#[tokio::test]
async fn test_block_source() {

    let host = "0.0.0.0".to_string();
    let port: u16 = 18443;
    let rpc_user = "bitcoind".to_string();
    let rpc_password = "bitcoind".to_string();
    let network = Network::Regtest;

    let bitcoind_rpc_client = BitcoinClient::new(host.clone(), port.clone(), rpc_user.clone(), rpc_password.clone(), network).await.unwrap();
    let bitcoin_client_solution = BitcoinClientSolutions::new(host.clone(), port.clone(), rpc_user.clone(), rpc_password.clone(), network).await.unwrap();

    let best_block_user = bitcoind_rpc_client.get_best_block().await.unwrap().0;
    let best_block_answer = bitcoin_client_solution.get_best_block().await.unwrap().0;

    let block_user = bitcoind_rpc_client.get_block(&best_block_user).await.unwrap();
    let block_answer = bitcoin_client_solution.get_block(&best_block_user).await.unwrap();

    let header_user = match block_user {
        BlockData::HeaderOnly(header) => header,
        BlockData::FullBlock(block) => block.header,
    };

    let header_answer = match block_answer {
        BlockData::HeaderOnly(header) => header,
        BlockData::FullBlock(block) => block.header,
    };

    let best_header_user = bitcoind_rpc_client.get_header(&best_block_user, None).await.unwrap();
    let best_header_answer = bitcoin_client_solution.get_header(&best_block_user, None).await.unwrap();


    assert_eq!(
    best_block_user,
    best_block_answer
    );


    assert_eq!(
    header_user,
    header_answer
    );


    assert_eq!(
    best_header_user,
    best_header_answer
    );

    
}

#[tokio::test]
async fn test_list_unspent() {

    let host = "0.0.0.0".to_string();
    let port: u16 = 18443;
    let rpc_user = "bitcoind".to_string();
    let rpc_password = "bitcoind".to_string();
    let network = Network::Regtest;

    let bitcoind_rpc_client = BitcoinClient::new(host.clone(), port.clone(), rpc_user.clone(), rpc_password.clone(), network).await.unwrap();
    let bitcoin_client_solution = BitcoinClientSolutions::new(host.clone(), port.clone(), rpc_user.clone(), rpc_password.clone(), network).await.unwrap();

    let user_unspent_utxos = bitcoind_rpc_client.list_unspent().await.0.len();
    let answer_unspent_utxos = bitcoind_rpc_client.list_unspent().await.0.len();


    assert_eq!(
        user_unspent_utxos,
        answer_unspent_utxos
    );

}

#[tokio::test]
async fn test_broadcast() {

    let host = "0.0.0.0".to_string();
    let port: u16 = 18443;
    let rpc_user = "bitcoind".to_string();
    let rpc_password = "bitcoind".to_string();
    let network = Network::Regtest;

    let bitcoind_rpc_client = BitcoinClient::new(host.clone(), port, rpc_user.clone(), rpc_password.clone(), network).await.unwrap();

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

    let bitcoind_rpc_client = BitcoinClient::new(host.clone(), port, rpc_user.clone(), rpc_password.clone(), network).await.unwrap();

    // check UrgentOnChainSweep
    let high_fees = bitcoind_rpc_client.get_est_sat_per_1000_weight(high_fee_target);
    assert_eq!(high_fees, 1500);

    // check MinAllowedAnchorChannelRemoteFee
    let low_fees = bitcoind_rpc_client.get_est_sat_per_1000_weight(low_fee_target);
    assert_eq!(low_fees, 500);
}

#[tokio::test]
async fn test_start_listener() {
    // Reset call count
    unsafe { crate::ch2_setup::network_exerciseV2::CALL_COUNT = 0; }

    // Pick a random high port to avoid conflicts
    let port = 56789;

    // Create a fake PeerManager
    let peer_manager = PeerManager {
        id: "test_node".to_string(),
    };

    // Spawn the listener in the background
    tokio::spawn(async move {
        start_listener(port, peer_manager).await;
    });

    // Give the listener a moment to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Connect to it
    let stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await;
    assert!(stream.is_ok(), "Should connect to the listener");

    // Wait briefly for setup_inbound to be called
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Check if setup_inbound was called
    unsafe {
        assert_eq!(
            crate::ch2_setup::network_exerciseV2::CALL_COUNT,
            1,
            "setup_inbound should be called once"
        );
    }
}