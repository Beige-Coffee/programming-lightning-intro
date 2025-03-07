#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use lightning::ln::types::ChannelId;
use bitcoin::secp256k1::{self, Secp256k1, PublicKey};
use std::collections::HashMap;
use crate::ch3_keys::exercises::{
    SimpleKeysManager as KeysManager,
};
use internal::events::{Message};
use internal::events::MessageSendEvent;
use crate::ch2_setup::peer_manager_structs::{MessageHandler};

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


impl Peer {
  pub fn decode_messsage(message: &[u8]) {
    Message::OpenChannel
  }
}

pub struct PeerManager{
  pub peers: HashMap<SocketDescriptor, Peer>,
  pub pending_msg_events: Vec<MessageSendEvent>,
  pub message_handler: MessageHandler,
  pub node_signer: KeysManager,
  pub secp_ctx: Secp256k1<secp256k1::SignOnly>
}

#[derive(Debug, PartialEq)]
pub enum OpenChannelStatus{
  Accept,
  Reject
}

struct NoiseXK {
  
}

impl NoiseXK {
  fn perform_handshake(message: [u8]) -> bool {
    true
  }
}

struct SocketDescriptor {
  pubkey: PublicKey,
  addr: String
}

impl PeerManager {
  //pub fn read_event(peer_descriptor: SocketDescriptor, data: &[u8]) {
    // noise handshake
    // match message type to handler
      // channel handler
      // route handler
      // onion handler
    
  //if ! perform_handshake(peer_descriptor, data) {
  //return
//}
  //let peer = peers.get_mut(peer_descriptor);
  //let peer_pubkey = peer.pubkey;

  //let message_type = peer.decrypt_message(data);

  //match message_type {
  //Message::OpenChannel {
  //self.message_handler.chan_message_handler.handle_open_channel(peer_pubkey, data);
//},
  //Message::NodeAnnouncement {
  //self.message_handler.route_handler.handle_node_announcement(peer_pubkey, data);
//},
  //Message::OnionMessage {
  //self.message_handler.onion_message_handler.handle_onion_message(peer_pubkey, data);
//},

  pub fn process_events() {
    // if empty, return
    // match event
      // channel
      // event
  unimplemented!()
  }
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