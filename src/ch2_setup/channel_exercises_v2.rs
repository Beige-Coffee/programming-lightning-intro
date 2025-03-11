#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use crate::ch2_setup::persist_exercise_v2::{FileStore, ChannelMonitorUpdateStatus};
use bitcoin::Transaction;
use bitcoin::Block;
use bitcoin::secp256k1::PublicKey;
use std::collections::HashMap;
use lightning::ln::types::ChannelId;
use lightning::chain::transaction::OutPoint;
use lightning::chain::{BestBlock};
use bitcoin::hash_types::{Txid};
use bitcoin::script::ScriptBuf;
use internal::bitcoind_client::BitcoindClient;
use lightning::chain::chaininterface::BroadcasterInterface;
use bitcoin::{Network};
use bitcoin::hashes::Hash;
use rand::Rng;
use lightning::ln::msgs;
use internal::events::{MessageSendEvent, Event};
use internal::messages::{OpenChannel, AcceptChannel,
                                            FundingCreated, FundingSigned,
                                            ChannelReady};
use crate::ch3_keys::exercises::{
    SimpleKeysManager,
};


// Mock Broadcaster
#[derive(Hash, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct MockBroadcaster {
    pub broadcasted_txs: Vec<Transaction>,
}

impl MockBroadcaster {
   pub fn new() -> Self {
        Self { broadcasted_txs: Vec::new() }
    }

   pub fn broadcast_transactions(&mut self, txs: &[&Transaction]) {
        for tx in txs {
            self.broadcasted_txs.push((*tx).clone());
        }
    }
}

// Mock FileStore
#[derive(Clone, PartialEq)]
pub struct MockFileStore {
    pub store: HashMap<String, Vec<u8>>,
}

impl MockFileStore {
    pub fn new() -> Self {
        Self { store: HashMap::new() }
    }

    fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.store.get(key).cloned()
    }
}

impl MockFileStore {
    fn write(&mut self, key: &str, value: &[u8]) -> Result<(), ()> {
        self.store.insert(key.to_string(), value.to_vec());
        Ok(())
    }

    fn read(&self, key: &str) -> Result<Vec<u8>, ()> {
        self.store.get(key).cloned().ok_or(())
    }

    pub fn persist_channel(&mut self, funding_outpoint: OutPoint, channel_monitor: ChannelMonitor) -> 
    
    ChannelMonitorUpdateStatus {
      let mut rng = rand::thread_rng();
      let random_data: Vec<u8> = (0..4).map(|_| rng.gen()).collect();
      self.store.insert("channel".to_string(), random_data);
      ChannelMonitorUpdateStatus::Completed
    }
}

//
//Channel Monitor
//
#[derive(Clone)]
pub struct CounterpartyCommitmentSecrets {
  old_secrets: [([u8; 32], u64); 49],
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Preimage(pub [u8; 32]);

pub type TransactionData = Vec<Transaction>;


#[derive(Copy, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct Header {
  pub version: u32,
}

#[derive(Debug, Clone)]
pub struct ChannelMonitor {
  pub channel_id: ChannelId,
  pub funding_outpoint: OutPoint,
  pub channel_value_sats: u64,
  pub current_commitment_tx: Option<Transaction>,
  pub best_block: BestBlock,
  pub commitment_secrets: Vec<[u8; 32]>,
  pub preimages: Vec<Preimage>,
  pub outputs_to_watch: HashMap<Txid, Vec<(u32, ScriptBuf)>>,
}

pub enum ChannelMonitorUpdate {
  LatestHolderCommitmentTXInfo {
    commitment_tx: Transaction,
  },
  PaymentPreimage {
    payment_preimage: Preimage
  },
  CommitmentSecret {
    secret: [u8; 32]
  }
}

struct WatchOutput {
  txid: Txid,
  input_idx: u32,
  script: ScriptBuf
}


//
// Exercise 1
//

impl ChannelMonitor{
  pub fn block_connected(
    &mut self,
    header: Header,
    txdata: TransactionData,
    height: u32,
    mut broadcaster: MockBroadcaster
  ) {

    // for each transaction in data
    for tx in txdata {
      if self.spends_watched_output(tx.clone()) {
        let on_chain_tx = self.build_onchain_tx(tx);
        broadcaster.broadcast_transactions(&[&on_chain_tx]);
        self.update_outputs_to_watch(on_chain_tx);
      }

      }
    }

  fn spends_watched_output(&self, tx: Transaction) -> bool {
    for input in tx.input {
      if let Some(outputs) = self.outputs_to_watch.get(&input.previous_output.txid) {
        for (output_idx, script_pubkey) in outputs.iter() {
          if output_idx == &input.previous_output.vout {
            return true;
          }
        }
      }
    }
    false
  }

  fn update_outputs_to_watch(&mut self, tx: Transaction) {
    let mut outputs_to_add = Vec::new();

    for (index, output) in tx.output.iter().enumerate() {
      outputs_to_add.push( (index as u32, output.script_pubkey.clone()) );
    }

    self.outputs_to_watch.insert(tx.compute_txid(), outputs_to_add);
  }

  fn build_onchain_tx(&self, tx: Transaction) -> Transaction {
    tx.clone()
  }

  pub fn encode(self) -> [u8; 10] {
    [0; 10] // returning a dummy array of 10 bytes
  }

}


impl ChannelMonitor {
  pub fn update_monitor(&mut self, update: ChannelMonitorUpdate) {
    match update {
      ChannelMonitorUpdate::LatestHolderCommitmentTXInfo {commitment_tx} => {
      self.current_commitment_tx = Some(commitment_tx);
      },
      ChannelMonitorUpdate::PaymentPreimage {payment_preimage}  => {
      self.preimages.push(payment_preimage);
      },
      ChannelMonitorUpdate::CommitmentSecret {secret} => {
      self.commitment_secrets.push(secret);
      },
    }
  }

  pub fn new() -> Self {
      ChannelMonitor {
          channel_id: ChannelId::new_zero(),
          funding_outpoint: OutPoint {
              txid: Txid::from_slice(&[43; 32]).unwrap(),
              index: 0,
          },
          channel_value_sats: 0,
          current_commitment_tx: None,
          best_block: BestBlock::from_network(Network::Regtest),
          commitment_secrets: Vec::new(),
          preimages: Vec::new(),
          outputs_to_watch: HashMap::new(),
      }
  }
}

//
//Chain Monitor
//

pub struct ChainMonitor {
  pub monitors: HashMap<OutPoint, ChannelMonitor>,
  pub persister: MockFileStore,
  pub broadcaster: MockBroadcaster
}

impl ChainMonitor {
  pub fn watch_channel(&mut self, funding_outpoint: OutPoint, channel_monitor: ChannelMonitor) -> Result<ChannelMonitorUpdateStatus, ()> {
    self.monitors.insert(funding_outpoint, channel_monitor.clone());
    let result = self.persister.persist_channel(funding_outpoint, channel_monitor.clone());

    match result {
      ChannelMonitorUpdateStatus::Completed => {
        println!("Persist successful")
      },
      ChannelMonitorUpdateStatus::UnrecoverableError => {
        panic!("ChannelMonitor Persistance Failed! Cannot continue normal operations!")
      }
      }
    Ok(result)
    }

  pub fn update_channel(&mut self, funding_outpoint: OutPoint, update: ChannelMonitorUpdate) {
    let channel_monitor = self.monitors.get_mut(&funding_outpoint).unwrap();
    channel_monitor.update_monitor(update);
    self.persister.persist_channel(funding_outpoint, channel_monitor.clone());
  }

  pub fn transactions_confirmed(&mut self,
    header: Header,
    txdata: TransactionData,
    height: u32,
  ) {
    for (_, monitor) in self.monitors.iter_mut() {
      monitor.block_connected(
        header,
        txdata.clone(),
        height,
        self.broadcaster.clone());
    }
  }
}

//
//Channel Manager
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Channel {
  their_network_key: PublicKey,
  temporary_channel_id: ChannelId,
  channel_value_satoshis: u64,
  output_script: ScriptBuf,
  funding_outpoint: OutPoint,
}

enum ChannelOpenStatus {
  Success{
  funding_outpoint: OutPoint,
  channel_monitor: ChannelMonitor
  },
  Failure
}

impl Channel{
  pub fn new(their_network_key: PublicKey, channel_value_satoshis: u64) -> Self {
    Self {
      their_network_key,
      temporary_channel_id: ChannelId::new_zero(),
      channel_value_satoshis,
      output_script: ScriptBuf::new(),
      funding_outpoint: OutPoint { txid: Txid::from_slice(&[43; 32]).unwrap(), index: 0 }
    }
  }

  pub fn open_channel(&mut self) -> ChannelOpenStatus {
    ChannelOpenStatus::Success {
      funding_outpoint: OutPoint { txid: Txid::from_slice(&[43; 32]).unwrap(), index: 0 },
      channel_monitor: ChannelMonitor::new()
    }
  }

  pub fn open_channel_msg(&mut self, channel_value_satoshis: u64) -> OpenChannel {
    OpenChannel{
      
      channel_value_satoshis: channel_value_satoshis

    }
  }

  pub fn funding_created_msg(&mut self, temporary_channel_id: ChannelId, transaction_id: Txid) -> FundingCreated {
    FundingCreated{

      temporary_channel_id: temporary_channel_id,
      transaction_id: transaction_id

    }
  }

  pub fn into_monitor(&mut self)-> ChannelMonitor {
    ChannelMonitor::new()
  }

}


pub struct ChannelManager {
  pub chain_monitor: ChainMonitor,
  pub pending_peer_events: Vec<MessageSendEvent>,
  pub pending_user_events: Vec<Event>,
  pub peers: HashMap<PublicKey, Channel>,
  pub signer_provider: SimpleKeysManager,
}
//funding_transaction_generated_intern
impl ChannelManager {
  
  pub fn create_channel(&mut self, their_network_key: PublicKey, channel_value_satoshis: u64) {

    let mut channel = Channel::new(their_network_key, channel_value_satoshis);

    self.peers.insert(their_network_key, channel.clone());

    let msg = channel.open_channel_msg(channel_value_satoshis);

    self.pending_peer_events.push(
      MessageSendEvent::SendOpenChannel {
        node_id: their_network_key,
        msg
      }
    );
  }

  pub fn handle_accept_channel(&mut self, counterparty_node_id: &PublicKey, msg: AcceptChannel) {
    let channel = self.peers.get_mut(&counterparty_node_id).unwrap();

    let channel_value_satoshis = channel.channel_value_satoshis;
    let temp_channel_id = channel.temporary_channel_id;
    let output_script = channel.output_script.clone();
    
    self.pending_user_events.push(
      Event::FundingGenerationReady {
        temporary_channel_id: msg.temporary_channel_id,
        counterparty_node_id: *counterparty_node_id,
        channel_value_satoshis: channel_value_satoshis,
        output_script: output_script,
      }
    );
  }

  pub fn handle_funding_signed(&mut self, counterparty_node_id: &PublicKey, msg: FundingSigned){
    let channel = self.peers.get_mut(&counterparty_node_id).unwrap();

    let funding_outpoint = channel.funding_outpoint;
    let channel_monitor = channel.into_monitor();
    
    self.chain_monitor.watch_channel(funding_outpoint, channel_monitor)
    .expect("Add to watch channel");
  }

  pub fn funding_transaction_generated(&mut self, temp_channel_id :ChannelId , their_network_key: PublicKey,
                                   transaction: Transaction) {

    let channel = self.peers.get_mut(&their_network_key).unwrap();

    let txid = transaction.compute_txid();
    
    let msg = channel.funding_created_msg(temp_channel_id, txid);

    self.pending_peer_events.push(
      MessageSendEvent::SendFundingCreated {
        node_id: their_network_key,
        msg
      }
    );
  
  }
}