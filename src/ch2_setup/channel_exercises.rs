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
use bitcoin::block::Header;
use bitcoin::{Network};
use bitcoin::hashes::Hash;

//
//Channel Monitor
//
#[derive(Clone)]
pub struct CounterpartyCommitmentSecrets {
  old_secrets: [([u8; 32], u64); 49],
}

#[derive(Hash, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Preimage(pub [u8; 32]);

pub type TransactionData = Vec<Transaction>;

#[derive(Clone)]
pub struct ChannelMonitor {
  channel_id: ChannelId,
  funding_outpoint: OutPoint,
  channel_value_sats: u64,
  current_commitment_tx: Option<Transaction>,
  best_block: BestBlock,
  commitment_secrets: Vec<[u8; 32]>,
  preimages: Vec<Preimage>,
  outputs_to_watch: HashMap<Txid, Vec<(u32, ScriptBuf)>>,
}

enum ChannelMonitorUpdate {
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
    broadcaster: BitcoindClient
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
  monitors: HashMap<OutPoint, ChannelMonitor>,
  persister: FileStore,
  broadcaster: BitcoindClient
}

impl ChainMonitor {
  fn watch_channel(&mut self, funding_outpoint: OutPoint, channel_monitor: ChannelMonitor) -> Result<ChannelMonitorUpdateStatus, ()> {
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

  fn update_channel(&mut self, funding_outpoint: OutPoint, update: ChannelMonitorUpdate) {
    let channel_monitor = self.monitors.get_mut(&funding_outpoint).unwrap();
    channel_monitor.update_monitor(update);
    self.persister.persist_channel(funding_outpoint, channel_monitor.clone());
  }

  fn transactions_confirmed(&mut self,
    header: Header,
    txdata: &TransactionData,
    height: u32,
    broadcaster: &BitcoindClient
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
struct OutboundV1Channel {
  their_network_key: PublicKey,
  channel_value_satoshis: u64,
}

enum ChannelOpenStatus {
  Success{
  funding_outpoint: OutPoint,
  channel_monitor: ChannelMonitor
  },
  Failure
}

impl OutboundV1Channel{
  pub fn new(their_network_key: PublicKey, channel_value_satoshis: u64) -> Self {
    Self {
      their_network_key,
      channel_value_satoshis
    }
  }
  
  pub fn open_channel(&mut self) -> ChannelOpenStatus {
    ChannelOpenStatus::Success {
      funding_outpoint: OutPoint { txid: Txid::from_slice(&[43; 32]).unwrap(), index: 0 },
      channel_monitor: ChannelMonitor::new()
    }
  }
  }

struct ChannelManager {
  chain_monitor: ChainMonitor,
}

impl ChannelManager {
  pub fn create_channel(&mut self, their_network_key: PublicKey, channel_value_satoshis: u64) {

    let mut channel = OutboundV1Channel::new(their_network_key, channel_value_satoshis);

    let result = channel.open_channel();

    match result {

      ChannelOpenStatus::Success {funding_outpoint, channel_monitor} => {

        if let Err(_) = self.chain_monitor.watch_channel(funding_outpoint, channel_monitor) {
          panic!("Failed to watch channel")
        }

      },
      ChannelOpenStatus::Failure => {

        panic!("Open Channel Failed")
      }
    }
  }
}