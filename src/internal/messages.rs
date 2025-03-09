#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use lightning::ln::types::ChannelId;
use bitcoin::hash_types::{Txid};
use bitcoin::secp256k1::{Secp256k1,ecdsa::Signature};
use bitcoin::secp256k1::PublicKey;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenChannel {
  pub channel_value_satoshis: u64
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AcceptChannel {
  pub channel_value_satoshis: u64,
  pub temporary_channel_id: ChannelId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FundingCreated {
  /// A temporary channel ID, until the funding is established
  pub temporary_channel_id: ChannelId,
  pub transaction_id: Txid
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FundingSigned {
  /// A temporary channel ID, until the funding is established
  pub channel_id: ChannelId,
  pub signature: Signature,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelReady {
  /// A temporary channel ID, until the funding is established
  pub channel_id: ChannelId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeAnnouncement {
  /// A temporary channel ID, until the funding is established
  pub signature: Signature,
  pub contents: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelAnnouncement {
  /// Authentication of the announcement by the first public node
  pub node_signature_1: Signature,
  /// Authentication of the announcement by the second public node
  pub node_signature_2: Signature,
  /// Proof of funding UTXO ownership by the first public node
  pub bitcoin_signature_1: Signature,
  /// Proof of funding UTXO ownership by the second public node
  pub bitcoin_signature_2: Signature,
  /// The actual announcement
  pub contents: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OnionMessage {
    /// Used in decrypting the onion packet's payload.
    pub blinding_point: PublicKey,
    /// The full onion packet including hop data, pubkey, and hmac
    pub onion_routing_packet: [u8; 32],
}

pub enum Message {
  OpenChannel(OpenChannel),
  NodeAnnouncement(NodeAnnouncement),
  OnionMessage(OnionMessage)
}