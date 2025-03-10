#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use lightning::ln::types::ChannelId;
use lightning::types::payment::{PaymentHash};
use bitcoin::hash_types::{Txid};
use bitcoin::secp256k1::{self, Secp256k1,ecdsa::Signature};
use bitcoin::secp256k1::PublicKey;

/// BOLT 4 onion packet including hop data for the next peer.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct OnionPacket {
  /// BOLT 4 version number.
  pub version: u8,
  /// In order to ensure we always return an error on onion decode in compliance with [BOLT
  /// #4](https://github.com/lightning/bolts/blob/master/04-onion-routing.md), we have to
  /// deserialize `OnionPacket`s contained in [`UpdateAddHTLC`] messages even if the ephemeral
  /// public key (here) is bogus, so we hold a [`Result`] instead of a [`PublicKey`] as we'd
  /// like.
  pub public_key: Result<PublicKey, secp256k1::Error>,
  /// 1300 bytes encrypted payload for the next hop.
  pub hop_data: [u8; 20*65],
  /// HMAC to verify the integrity of hop_data.
  pub hmac: [u8; 32],
}

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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct UpdateAddHTLC {
  /// The channel ID
  pub channel_id: ChannelId,
  /// The HTLC value in milli-satoshi
  pub amount_msat: u64,
  /// The payment hash, the pre-image of which controls HTLC redemption
  pub payment_hash: PaymentHash,
  /// The expiry height of the HTLC
  pub cltv_expiry: u32,
  /// The onion routing packet with encrypted data for the next hop.
  pub onion_routing_packet: OnionPacket,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CommitmentSigned {
  /// The channel ID
  pub channel_id: ChannelId,
  /// A signature on the commitment transaction
  pub signature: Signature,
  /// Signatures on the HTLC transactions
  pub htlc_signatures: Vec<Signature>,
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