#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use lightning::ln::types::ChannelId;
use bitcoin::secp256k1::{self, Secp256k1, PublicKey};
use std::collections::HashMap;
use crate::ch3_keys::exercises::{
    SimpleKeysManager as KeysManager,
};
use internal::messages::{OpenChannel, AcceptChannel,
  FundingCreated, FundingSigned,
  ChannelReady};
use internal::events::MessageSendEvent;
use crate::ch2_setup::peer_manager_structs::{MessageHandler, SocketDescriptor, Peer,
                                            PeerManager};
use internal::messages::Message;

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


impl Peer {
  pub fn decode_messsage(message: &[u8]) {
    unimplemented!()
  }
}

#[derive(Debug, PartialEq)]
pub enum OpenChannelStatus{
  Accept,
  Reject
}

struct NoiseXK {
  
}

impl NoiseXK {
  fn perform_handshake(peer_descriptor: &SocketDescriptor) -> bool {
    true
  }
}

impl PeerManager {
  pub fn read_event(&mut self, peer_descriptor: SocketDescriptor, data: &[u8]) {
    
  if ! NoiseXK::perform_handshake(&peer_descriptor) {
    return
  }

  let peer = self.peers.get_mut(&peer_descriptor).unwrap();

  let peer_pubkey = peer.public_key;

  let message = peer.decrypt_message(data);

  match message {
  Message::OpenChannel(message) =>
    self.message_handler.channel_message_handler.handle_open_channel(peer_pubkey, &message)
    ,
  Message::NodeAnnouncement(message) =>
    self.message_handler.route_message_handler.handle_node_announcement(Some(peer_pubkey), &message)
    ,
  Message::OnionMessage(message) =>
    self.message_handler.onion_message_handler.handle_onion_message(peer_pubkey, &message)
  }
  }
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

}