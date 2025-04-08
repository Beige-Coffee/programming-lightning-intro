#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::ch1_intro_htlcs::bonus::revocation_keys::{SimpleNodeKeys};
use bitcoin::secp256k1;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::PublicKey;
use bitcoin::secp256k1::SecretKey;
use crate::ch1_intro_htlcs::exercises::{
  generate_revocation_pubkey
};

pub fn revocation_pubkey(
  countersignatory_basepoint: PublicKey,
  keys_manager: SimpleNodeKeys,
  secp_ctx: &Secp256k1<secp256k1::All>,
  channel_params: &[u8; 32]) -> PublicKey {

  let channel_keys = keys_manager.derive_channel_keys(channel_params);

  let first_commitment_secret = channel_keys.build_commitment_secret(0);

  let first_commitment_private_key = SecretKey::from_slice(&first_commitment_secret).unwrap();
  
  let first_commitment_point = PublicKey::from_secret_key(secp_ctx, &first_commitment_private_key);

  let revocation_pubkey = generate_revocation_pubkey(countersignatory_basepoint, first_commitment_point);

  revocation_pubkey
  
  }