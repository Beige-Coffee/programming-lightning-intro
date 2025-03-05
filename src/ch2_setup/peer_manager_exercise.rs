#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use lightning::ln::types::ChannelId;
use bitcoin::secp256k1::{self, Secp256k1, PublicKey};
use std::collections::HashMap;
use crate::ch3_keys::exercises::{
    SimpleKeysManager as KeysManager,
};

pub struct OpenChannelMsg {
  pub temporary_channel_id: ChannelId,
  pub funding_satoshis: u64,
  pub commitment_feerate_sat_per_vbyte: u32,
  pub to_self_delay: u16,
  pub funding_pubkey: PublicKey,
  pub revocation_basepoint: PublicKey,
  pub payment_basepoint: PublicKey
}

pub struct MockChannelMsg (pub String);

pub struct Peer (pub PublicKey);

pub struct PeerManager{
  pub peers: HashMap<PublicKey, Peer>,
  pub node_signer: KeysManager,
  pub secp_ctx: Secp256k1<secp256k1::SignOnly>
}

#[derive(Debug, PartialEq)]
pub enum OpenChannelStatus{
  Accept,
  Reject
}

impl PeerManager {
  /// Handle an incoming `open_channel` message from the given peer.
  pub fn handle_open_channel(&self, their_node_id: PublicKey, msg: &OpenChannelMsg) -> OpenChannelStatus {
    if msg.funding_satoshis < 100000 {
      return OpenChannelStatus::Reject
    }

    if msg.to_self_delay > 144 {
      return OpenChannelStatus::Reject
    }

    if msg.commitment_feerate_sat_per_vbyte < 10 {
      return OpenChannelStatus::Reject
    }
    return OpenChannelStatus::Accept
  }

  /// Handle an incoming `accept_channel` message from the given peer.
  fn handle_accept_channel(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `funding_created` message from the given peer.
  fn handle_funding_created(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `funding_signed` message from the given peer.
  fn handle_funding_signed(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `channel_ready` message from the given peer.
  fn handle_channel_ready(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  // Channel close:
  /// Handle an incoming `shutdown` message from the given peer.
  fn handle_shutdown(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `closing_signed` message from the given peer.
  fn handle_closing_signed(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }


  // Interactive channel construction
  /// Handle an incoming `tx_add_input message` from the given peer.
  fn handle_tx_add_input(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `tx_add_output` message from the given peer.
  fn handle_tx_add_output(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `tx_remove_input` message from the given peer.
  fn handle_tx_remove_input(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `tx_remove_output` message from the given peer.
  fn handle_tx_remove_output(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `tx_complete message` from the given peer.
  fn handle_tx_complete(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `tx_signatures` message from the given peer.
  fn handle_tx_signatures(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `tx_init_rbf` message from the given peer.
  fn handle_tx_init_rbf(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `tx_ack_rbf` message from the given peer.
  fn handle_tx_ack_rbf(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `tx_abort message` from the given peer.
  fn handle_tx_abort(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  // HTLC handling:
  /// Handle an incoming `update_add_htlc` message from the given peer.
  fn handle_update_add_htlc(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `update_fulfill_htlc` message from the given peer.
  fn handle_update_fulfill_htlc(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `update_fail_htlc` message from the given peer.
  fn handle_update_fail_htlc(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `commitment_signed` message from the given peer.
  fn handle_commitment_signed(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `revoke_and_ack` message from the given peer.
  fn handle_revoke_and_ack(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

  /// Handle an incoming `update_fee` message from the given peer.
  fn handle_update_fee(&self, their_node_id: PublicKey, msg: &MockChannelMsg){
      unimplemented!()
    }

}