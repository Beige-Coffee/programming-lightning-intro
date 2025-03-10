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
use internal::messages::{ UpdateAddHTLC, CommitmentSigned};
use crate::ch3_keys::exercises::{
    SimpleKeysManager,
};

//funding_transaction_generated_intern
impl ChannelManager {

  pub fn send_payment(&mut self, their_network_key: PublicKey, amount_sats: u64) {
  unimplemented!()
  }

  fn send_htlc_and_commit(&mut self, amount_msat: u64, payment_hash: PaymentHash, cltv_expiry: u32) -> ChannelMonitorUpdate {
    
    self.pending_peer_events.push(
      MessageSendEvent::UpdateAddHTLC {
        node_id: their_network_key,
        msg
      }
    );

    self.pending_peer_events.push(
      MessageSendEvent::CommitmentSigned {
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

  }
}