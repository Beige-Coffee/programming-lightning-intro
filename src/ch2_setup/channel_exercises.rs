#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
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

#[derive(Clone)]
pub struct CounterpartyCommitmentSecrets {
  old_secrets: [([u8; 32], u64); 49],
}

pub type TransactionData = Vec<Transaction>;

#[derive(Clone)]
pub struct ChannelMonitor {
  channel_id: ChannelId,
  funding_outpoint: OutPoint,
  channel_value_sats: u64,
  current_commitment_txid: Option<Txid>,
  best_block: BestBlock,
  commitment_secrets: CounterpartyCommitmentSecrets,
  outputs_to_watch: HashMap<Txid, Vec<(u32, ScriptBuf)>>,
}

struct WatchOutput {
  txid: Txid,
  input_idx: u32,
  script: ScriptBuf
}

impl ChannelMonitor{
  pub fn block_connected(
    self,
    header: Header,
    txdata: TransactionData,
    height: u32,
    broadcaster: BitcoindClient
  ) {
    
    // for each transaction in data
    for tx in txdata {
      if self.spends_watched_output(tx) {
        let on_chain_tx = self.build_onchain_tx(tx);
        broadcaster.broadcast_transactions(&[&on_chain_tx]);
        self.update_outputs_to_watch(on_chain_tx);
      }
        
      }
    }

  fn spends_watched_output(self, tx: Transaction) -> bool {
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

  fn update_outputs_to_watch(mut self, tx: Transaction) {
    let mut outputs_to_add = Vec::new();
    
    for (index, output) in tx.output.iter().enumerate() {
      outputs_to_add.push( (index as u32, output.script_pubkey.clone()) );
    }

    self.outputs_to_watch.insert(tx.compute_txid(), outputs_to_add);
  }

  fn build_onchain_tx(self, tx: Transaction) -> Transaction {
    tx.clone()
  }

}

  