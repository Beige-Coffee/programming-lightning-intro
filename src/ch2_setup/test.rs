#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use internal::bitcoind_client::BitcoindClient;
use crate::ch2_setup::exercises::{
    BitcoindClientExercise,poll_for_blocks,poll_for_blocks2
};
use lightning::ln::types::ChannelId;
use bitcoin::secp256k1::{self, Secp256k1};
use crate::ch2_setup::peer_manager_exercise::{
    OpenChannelMsg, OpenChannelStatus};
use crate::ch3_keys::exercises::{
    SimpleKeysManager,
};
use crate::ch2_setup::peer_manager_structs::{
    PeerManager, SocketDescriptor};

use crate::ch2_setup::network_exercise_v2::{
    start_listener, PeerManager as PeerManagerNetworkEx
};
use crate::ch2_setup::bitcoin_client::{
    BitcoinClient,
};
use crate::ch2_setup::bitcoin_client_solutions::BitcoinClient as BitcoinClientSolutions;
use crate::ch2_setup::persist_exercise::{
    SimpleStore
};
//use crate::ch2_setup::payment_exercise::{
//    send_payment
//};
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
use bitcoin::blockdata::block::Block;
use bitcoin::consensus::{encode};
use lightning::chain::chaininterface::{BroadcasterInterface, ConfirmationTarget, FeeEstimator};
use tokio::time::{sleep, Duration};
use crate::internal::helper::{
    bitcoin_pubkey_from_private_key, pubkey_from_private_key, secp256k1_private_key,
};
use crate::ch2_setup::channel_exercises_v2::{ChannelMonitor, MockBroadcaster, MockFileStore,
                                            ChainMonitor, Header as HeaderExercise, TransactionData,
                                            ChannelManager, ChannelMonitorUpdate, Preimage};
use lightning::chain::transaction::OutPoint;
use bitcoin::Transaction;
use bitcoin::consensus::{deserialize, serialize};
use hex::{FromHex};
use bitcoin::script::ScriptBuf;
use bitcoin::hashes::Hash;
use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hash_types::{Txid, BlockHash};
use crate::ch2_setup::persist_exercise_v2::{ChannelMonitorUpdateStatus};
use internal::messages::{OpenChannel, AcceptChannel,
    FundingCreated, FundingSigned,
    ChannelReady};
use bitcoin::secp256k1::{ecdsa::Signature};
use bitcoin::secp256k1::ffi::Signature as FFISignature;

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
    unsafe { crate::ch2_setup::network_exercise_v2::CALL_COUNT = 0; }

    // Pick a random high port to avoid conflicts
    let port = 9735;

    // Create a fake PeerManager
    let peer_manager = PeerManagerNetworkEx{
        id: "test_node".to_string()
};

    // Spawn the listener in the background
    tokio::spawn(async move {
        start_listener(peer_manager).await;
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
            crate::ch2_setup::network_exercise_v2::CALL_COUNT,
            1,
            "setup_inbound should be called once"
        );
    }
}

#[tokio::test]
async fn test_handle_open_channel() {

    let secp = Secp256k1::new();
    
    let seed = [1_u8; 32];
    let child_index: usize = 0;
    let keys_manager = SimpleKeysManager::new(seed);
    
    let peer_manager = PeerManager::new();

    let their_node_id = pubkey_from_private_key(&[0x01; 32]);

    // Test case 1: Valid channel (should accept)
    let msg1 = OpenChannelMsg {
        temporary_channel_id: ChannelId::new_zero(),
        funding_satoshis: 150_000,      // ≥ 100,000
        commitment_feerate_sat_per_vbyte: 15, // ≥ 10
        to_self_delay: 100,             // ≤ 144
        funding_pubkey: their_node_id, 
        revocation_basepoint: their_node_id,
        payment_basepoint: their_node_id
    };
    let result = peer_manager.handle_open_channel(their_node_id, &msg1);
    assert_eq!(result, OpenChannelStatus::Accept, "Valid channel should be accepted");

    // Test case 2: Invalid channel (should fail)
    let msg2 = OpenChannelMsg {
        temporary_channel_id: ChannelId::new_zero(),
        funding_satoshis: 90_000,      // ≥ 100,000
        commitment_feerate_sat_per_vbyte: 15, // ≥ 10
        to_self_delay: 100,             // ≤ 144
        funding_pubkey: their_node_id, 
        revocation_basepoint: their_node_id,
        payment_basepoint: their_node_id
    };
    let result = peer_manager.handle_open_channel(their_node_id, &msg2);
    assert_eq!(result, OpenChannelStatus::Reject, "Invalid channel should be rejected");

    // Test case 2: Invalid channel (should fail)
    let msg2 = OpenChannelMsg {
        temporary_channel_id: ChannelId::new_zero(),
        funding_satoshis: 99_999,      // ≥ 100,000
        commitment_feerate_sat_per_vbyte: 15, // ≥ 10
        to_self_delay: 100,             // ≤ 144
        funding_pubkey: their_node_id, 
        revocation_basepoint: their_node_id,
        payment_basepoint: their_node_id
    };
    let result2 = peer_manager.handle_open_channel(their_node_id, &msg2);
    assert_eq!(result2, OpenChannelStatus::Reject, "Invalid channel should be rejected");

    // Test case 3: Invalid channel (should fail)
    let msg3 = OpenChannelMsg {
        temporary_channel_id: ChannelId::new_zero(),
        funding_satoshis: 150_000,      // ≥ 100,000
        commitment_feerate_sat_per_vbyte: 9, // ≥ 10
        to_self_delay: 100,             // ≤ 144
        funding_pubkey: their_node_id, 
        revocation_basepoint: their_node_id,
        payment_basepoint: their_node_id
    };
    let result3 = peer_manager.handle_open_channel(their_node_id, &msg3);
    assert_eq!(result3, OpenChannelStatus::Reject, "Invalid channel should be rejected");

    // Test case 4: Invalid channel (should fail)
    let msg4 = OpenChannelMsg {
        temporary_channel_id: ChannelId::new_zero(),
        funding_satoshis: 150_000,      // ≥ 100,000
        commitment_feerate_sat_per_vbyte: 10, // ≥ 10
        to_self_delay: 145,             // ≤ 144
        funding_pubkey: their_node_id, 
        revocation_basepoint: their_node_id,
        payment_basepoint: their_node_id
    };
    let result4 = peer_manager.handle_open_channel(their_node_id, &msg4);
    assert_eq!(result4, OpenChannelStatus::Reject, "Invalid channel should be rejected");

    
}

static TX_RAW: &str = 
    "01000000000103fc9aa70afba04da865f9821734b556cca9fb5710\
     fc1338b97fba811033f755e308000000000000000019b37457784d\
     d04936f011f733b8016c247a9ef08d40007a54a5159d1fc62ee216\
     00000000000000004c4f2937c6ccf8256d9711a19df1ae62172297\
     0bf46be925ff15f490efa1633d01000000000000000002c0e1e400\
     0000000017a9146983f776902c1d1d0355ae0962cb7bc69e9afbde\
     8706a1e600000000001600144257782711458506b89f255202d645\
     e25c41144702483045022100dcada0499865a49d0aab8cb113c5f8\
     3fd5a97abc793f97f3f53aa4b9d1192ed702202094c7934666a30d\
     6adb1cc9e3b6bc14d2ffebd3200f3908c40053ef2df640b5012103\
     15434bb59b615a383ae87316e784fc11835bb97fab33fdd2578025\
     e9968d516e0247304402201d90b3197650569eba4bc0e0b1e2dca7\
     7dfac7b80d4366f335b67e92e0546e4402203b4be1d443ad7e3a5e\
     a92aafbcdc027bf9ccf5fe68c0bc8f3ebb6ab806c5464c012103e0\
     0d92b0fe60731a54fdbcc6920934159db8ffd69d55564579b69a22\
     ec5bb7530247304402205ab83b734df818e64d8b9e86a8a75f9d00\
     5c0c6e1b988d045604853ab9ccbde002205a580235841df609d6bd\
     67534bdcd301999b18e74e197e9e476cdef5fdcbf822012102ebb3\
     e8a4638ede4721fb98e44e3a3cd61fecfe744461b85e0b6a6a1017\
     5d5aca00000000";

#[tokio::test]
async fn test_block_connected() {
    
    let mut monitor = ChannelMonitor::new();

    let broadcaster = MockBroadcaster::new();

    let persister = MockFileStore::new();

    let tx: Transaction = deserialize(Vec::from_hex(TX_RAW).unwrap().as_slice()).unwrap();

    let header = HeaderExercise {
        version: 2
    };

    println!("outputs_to_watch len: {:?}\n\n", monitor.outputs_to_watch.len());

    let txdata: TransactionData = vec![tx.clone()];

    let height = 100;

    let script = ScriptBuf::from_hex("a9146983f776902c1d1d0355ae0962cb7bc69e9afbde87").unwrap();

    let tx_bytes: [u8; 32] = [
            0xfc, 0x9a, 0xa7, 0x0a, 0xfb, 0xa0, 0x4d, 0xa8,
            0x65, 0xf9, 0x82, 0x17, 0x34, 0xb5, 0x56, 0xcc,
            0xa9, 0xfb, 0x57, 0x10, 0xfc, 0x13, 0x38, 0xb9,
            0x7f, 0xba, 0x81, 0x10, 0x33, 0xf7, 0x55, 0xe3,
        ];

    let tx_id = Txid::from_byte_array(tx_bytes);

    monitor.outputs_to_watch.insert(tx_id, vec![(8, ScriptBuf::new())]);

    monitor.block_connected(header, txdata, height, broadcaster.clone());

    println!("tx: {:?}\n\n", tx);
    println!("outputs_to_watch: {:?}\n\n", monitor.outputs_to_watch);
    
    let broadcasted_txs = broadcaster.broadcasted_txs;
    println!("broadcasted_txs: {:?}\n\n", broadcasted_txs);

    println!("outputs_to_watch len: {:?}\n\n", monitor.outputs_to_watch.len());

    assert!(monitor.outputs_to_watch.contains_key(&tx.compute_txid()), "Student must update outputs_to_watch with new outputs");

}

#[tokio::test]
async fn test_transactions_confirmed() {

    let monitor = ChannelMonitor::new();

    let broadcaster = MockBroadcaster::new();

    let persister = MockFileStore::new();

    let chain_monitor = ChainMonitor {
        monitors: HashMap::new(),
        persister,
        broadcaster: broadcaster.clone(),
    };

    let seed = [1_u8; 32];
    let child_index: usize = 0;
    let keys_manager = SimpleKeysManager::new(seed);

    let mut channel_manager = ChannelManager { 
        chain_monitor: chain_monitor,
        pending_peer_events: Vec::new(),
        pending_user_events: Vec::new(),
        peers: HashMap::new(),
        signer_provider: keys_manager,
    };

    let tx: Transaction = deserialize(Vec::from_hex(TX_RAW).unwrap().as_slice()).unwrap();

    let header = HeaderExercise {
        version: 2
    };

    println!("outputs_to_watch len: {:?}\n\n", monitor.outputs_to_watch.len());

    let txdata: TransactionData = vec![tx.clone()];

    let height = 100;

    let script = ScriptBuf::from_hex("a9146983f776902c1d1d0355ae0962cb7bc69e9afbde87").unwrap();

    let tx_bytes: [u8; 32] = [
            0xfc, 0x9a, 0xa7, 0x0a, 0xfb, 0xa0, 0x4d, 0xa8,
            0x65, 0xf9, 0x82, 0x17, 0x34, 0xb5, 0x56, 0xcc,
            0xa9, 0xfb, 0x57, 0x10, 0xfc, 0x13, 0x38, 0xb9,
            0x7f, 0xba, 0x81, 0x10, 0x33, 0xf7, 0x55, 0xe3,
        ];

    let tx_id = Txid::from_byte_array(tx_bytes);

    let outpoint = OutPoint{txid: tx_id, index: 8};

    channel_manager.chain_monitor.watch_channel(outpoint, monitor);

    channel_manager.chain_monitor.monitors.get_mut(&outpoint).unwrap()
    .outputs_to_watch.insert(tx_id, vec![(8, ScriptBuf::new())]);

    channel_manager.chain_monitor.transactions_confirmed(header, txdata, height);

    println!("tx: {:?}\n\n", tx);

    let outputs = &channel_manager.chain_monitor.monitors.get_mut(&outpoint).unwrap().outputs_to_watch;

    println!("outputs_to_watch: {:?}\n\n", outputs);

    assert!(outputs.contains_key(&tx.compute_txid()), "Student must update outputs_to_watch with new outputs");

}

#[tokio::test]
async fn test_watch_channel() {
    let channel_monitor = ChannelMonitor::new();

    let broadcaster = MockBroadcaster::new();

    let persister = MockFileStore::new();

    let mut chain_monitor = ChainMonitor {
        monitors: HashMap::new(),
        persister,
        broadcaster: broadcaster.clone(),
    };


    let tx_bytes: [u8; 32] = [
            0xfc, 0x9a, 0xa7, 0x0a, 0xfb, 0xa0, 0x4d, 0xa8,
            0x65, 0xf9, 0x82, 0x17, 0x34, 0xb5, 0x56, 0xcc,
            0xa9, 0xfb, 0x57, 0x10, 0xfc, 0x13, 0x38, 0xb9,
            0x7f, 0xba, 0x81, 0x10, 0x33, 0xf7, 0x55, 0xe3,
        ];

    let tx_id = Txid::from_byte_array(tx_bytes);

    let funding_outpoint = OutPoint{txid: tx_id, index: 8};

    println!("monitors: {:?}", chain_monitor.monitors.len());

    let result = chain_monitor.watch_channel(funding_outpoint, channel_monitor).unwrap();

    let num_monitors_after_watch = chain_monitor.monitors.len();

    println!("monitors: {:?}", chain_monitor.monitors.len());

    assert_eq!(1, num_monitors_after_watch);
    assert_eq!(result, ChannelMonitorUpdateStatus::Completed);
}

#[tokio::test]
async fn test_update_monitor() {
    let mut channel_monitor = ChannelMonitor::new();

    let broadcaster = MockBroadcaster::new();

    let persister = MockFileStore::new();

    let chain_monitor = ChainMonitor {
        monitors: HashMap::new(),
        persister,
        broadcaster: broadcaster.clone(),
    };

    let preimage_update = ChannelMonitorUpdate::PaymentPreimage { payment_preimage: Preimage([9; 32]) };
    let secret_update = ChannelMonitorUpdate::CommitmentSecret { secret: [10; 32] };

    println!("channel_monitor - # commitment_secrets: {:?}", channel_monitor.commitment_secrets.len());
    println!("channel_monitor - # preimages: {:?}", channel_monitor.preimages.len());
    
    channel_monitor.update_monitor(preimage_update);
    channel_monitor.update_monitor(secret_update);


    assert_eq!(1, channel_monitor.commitment_secrets.len());
    assert_eq!(1, channel_monitor.preimages.len());
}

#[tokio::test]
async fn test_update_channel() {
    let channel_monitor = ChannelMonitor::new();

    let broadcaster = MockBroadcaster::new();

    let persister = MockFileStore::new();

    let mut chain_monitor = ChainMonitor {
        monitors: HashMap::new(),
        persister,
        broadcaster: broadcaster.clone(),
    };

    let tx_bytes: [u8; 32] = [
        0xfc, 0x9a, 0xa7, 0x0a, 0xfb, 0xa0, 0x4d, 0xa8,
        0x65, 0xf9, 0x82, 0x17, 0x34, 0xb5, 0x56, 0xcc,
        0xa9, 0xfb, 0x57, 0x10, 0xfc, 0x13, 0x38, 0xb9,
        0x7f, 0xba, 0x81, 0x10, 0x33, 0xf7, 0x55, 0xe3,
    ];

    let tx_id = Txid::from_byte_array(tx_bytes);

    let funding_outpoint = OutPoint{txid: tx_id, index: 8};

    let result = chain_monitor.watch_channel(funding_outpoint, channel_monitor).unwrap();

    let preimage_update = ChannelMonitorUpdate::PaymentPreimage { payment_preimage: Preimage([9; 32]) };
    let secret_update = ChannelMonitorUpdate::CommitmentSecret { secret: [10; 32] };

    let channel_mon_before = chain_monitor.monitors.get_mut(&funding_outpoint).unwrap();

    let store_before = chain_monitor.persister.store.clone();

    println!("chain_monitor: commitment_secrets {:?}", channel_mon_before.commitment_secrets);
    println!("chain_monitor: secret_update {:?}", channel_mon_before.preimages);
    println!("persister: store {:?}", chain_monitor.persister.store);

    chain_monitor.update_channel(funding_outpoint, preimage_update);
    chain_monitor.update_channel(funding_outpoint, secret_update);

    let channel_mon_after = chain_monitor.monitors.get_mut(&funding_outpoint).unwrap();
    let store_after = chain_monitor.persister.store.clone();


    println!("chain_monitor: commitment_secrets {:?}", channel_mon_after.commitment_secrets);
    println!("chain_monitor: secret_update {:?}", channel_mon_after.preimages);
    println!("persister: store {:?}", chain_monitor.persister.store);
    

    assert_eq!(1, channel_mon_after.commitment_secrets.len());
    assert_eq!(1, channel_mon_after.preimages.len());
    assert_ne!(store_before, store_after);
}

#[tokio::test]
async fn test_create_channel() {

    let monitor = ChannelMonitor::new();

    let broadcaster = MockBroadcaster::new();

    let persister = MockFileStore::new();

    let chain_monitor = ChainMonitor {
        monitors: HashMap::new(),
        persister,
        broadcaster: broadcaster.clone(),
    };

    let seed = [1_u8; 32];
    let child_index: usize = 0;
    let keys_manager = SimpleKeysManager::new(seed);

    let pubkey = pubkey_from_private_key(&[0x01; 32]);
    let channel_balance = 100_000_000;

    let mut channel_manager = ChannelManager { 
        chain_monitor: chain_monitor,
        pending_peer_events: Vec::new(),
        pending_user_events: Vec::new(),
        peers: HashMap::new(),
        signer_provider: keys_manager,
    };

    channel_manager.create_channel(pubkey, channel_balance);

    println!("channel_manager.peers.len(): {:?}\n\n", channel_manager.peers.len());

    println!("channel_manager.pending_peer_events.len(): {:?}\n\n", channel_manager.pending_peer_events.len());

    assert_eq!(1, channel_manager.peers.len());

    assert_eq!(1, channel_manager.pending_peer_events.len());

}

#[tokio::test]
async fn test_accept_channel() {

    let monitor = ChannelMonitor::new();

    let broadcaster = MockBroadcaster::new();

    let persister = MockFileStore::new();

    let chain_monitor = ChainMonitor {
        monitors: HashMap::new(),
        persister,
        broadcaster: broadcaster.clone(),
    };

    let seed = [1_u8; 32];
    let child_index: usize = 0;
    let keys_manager = SimpleKeysManager::new(seed);

    let pubkey = pubkey_from_private_key(&[0x01; 32]);
    let channel_balance = 100_000_000;
    let msg = AcceptChannel{
        channel_value_satoshis: channel_balance,
        temporary_channel_id: ChannelId::new_zero()
    };

    let mut channel_manager = ChannelManager { 
        chain_monitor: chain_monitor,
        pending_peer_events: Vec::new(),
        pending_user_events: Vec::new(),
        peers: HashMap::new(),
        signer_provider: keys_manager,
    };

    channel_manager.create_channel(pubkey, channel_balance);

    channel_manager.handle_accept_channel(&pubkey, msg);

    println!("channel_manager.pending_user_events.len(): {:?}\n\n", channel_manager.pending_user_events.len());

    assert_eq!(1, channel_manager.pending_user_events.len());

}

#[tokio::test]
async fn test_funding_signed() {

    let monitor = ChannelMonitor::new();

    let broadcaster = MockBroadcaster::new();

    let persister = MockFileStore::new();

    let chain_monitor = ChainMonitor {
        monitors: HashMap::new(),
        persister,
        broadcaster: broadcaster.clone(),
    };

    let seed = [1_u8; 32];
    let child_index: usize = 0;
    let keys_manager = SimpleKeysManager::new(seed);

    let pubkey = pubkey_from_private_key(&[0x01; 32]);
    let channel_balance = 100_000_000;
    let msg = FundingSigned{
        channel_id: ChannelId::new_zero(),
        signature: Signature::from(unsafe { FFISignature::new() })
    };

    let mut channel_manager = ChannelManager { 
        chain_monitor: chain_monitor,
        pending_peer_events: Vec::new(),
        pending_user_events: Vec::new(),
        peers: HashMap::new(),
        signer_provider: keys_manager,
    };

    channel_manager.create_channel(pubkey, channel_balance);

    channel_manager.handle_funding_signed(&pubkey, msg);

    println!("channel_manager.chain_monitor.monitors.len(): {:?}\n\n", channel_manager.chain_monitor.monitors.len());

    assert_eq!(1, channel_manager.chain_monitor.monitors.len());

}

#[tokio::test]
async fn test_read_event() {

    let data_open_channel: &[u8] = &[0x00];
    let data_node_announcement = [0x01];
    let data_onion_message = [0x02];

    let mut peer_manager = PeerManager::new();

    let pubkey = pubkey_from_private_key(&[0x01; 32]);

    let socket_descriptor = SocketDescriptor{
        pubkey: pubkey,
        addr: "test".to_string()
    };

    peer_manager.read_event(socket_descriptor, data_open_channel)

}