#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use bitcoin::bip32::{ChildNumber, Xpriv, Xpub};
use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::{Hash, HashEngine};
use bitcoin::network::Network;
use bitcoin::secp256k1;
use bitcoin::secp256k1::PublicKey;
use bitcoin::secp256k1::Scalar;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::SecretKey;
use serde::ser::Serialize;
use crate::exercises::solutions::{
  generate_revocation_pubkey
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NodeKeysManager {
    pub secp_ctx: Secp256k1<secp256k1::All>,
    pub node_secret: SecretKey,
    pub node_id: PublicKey,
    pub shutdown_xpub: Xpub,
    pub channel_master_key: Xpriv,
    pub seed: [u8; 32],
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChannelKeysManager {
    pub commitment_seed: [u8; 32],
    pub revocation_base_key: SecretKey,
    pub payment_key: SecretKey,
    pub delayed_payment_base_key: SecretKey,
    pub htlc_base_key: SecretKey,
}

impl NodeKeysManager {
    pub(crate) fn new(seed: [u8; 32]) -> NodeKeysManager {
        let secp_ctx = Secp256k1::new();

        let network = Network::Regtest;
        let master_key = Xpriv::new_master(network, &seed).unwrap();

        let node_secret = master_key
            .derive_priv(&secp_ctx, &ChildNumber::from_hardened_idx(0).unwrap())
            .expect("Your RNG is busted")
            .private_key;

        let node_id = PublicKey::from_secret_key(&secp_ctx, &node_secret);

        let shutdown_xpub = Xpub::from_priv(
            &secp_ctx, 
            &master_key.derive_priv(&secp_ctx, &ChildNumber::from_hardened_idx(1).unwrap())
                .expect("Your RNG is busted")
        );

        let channel_master_key = master_key
            .derive_priv(&secp_ctx, &ChildNumber::from_hardened_idx(2).unwrap())
            .expect("Your RNG is busted");

        NodeKeysManager {
            secp_ctx: secp_ctx,
            node_secret: node_secret,
            node_id: node_id,
            shutdown_xpub: shutdown_xpub,
            channel_master_key: channel_master_key,
            seed: seed,
        }
    }

    pub fn derive_channel_keys(&self, channel_id: u32) -> ChannelKeysManager {
        
        let mut unique = Sha256::engine();
            unique.input(&channel_id.to_be_bytes());
            unique.input(&self.seed);

        let child_privkey = self
            .channel_master_key
            .derive_priv(
                &self.secp_ctx,
                &ChildNumber::from_hardened_idx(channel_id)
                    .expect("key space exhausted"),
            )
            .expect("Your RNG is busted");

            unique.input(&child_privkey.private_key[..]);

        let channel_seed = Sha256::from_engine(unique).to_byte_array();

        let commitment_seed = {
            let mut sha = Sha256::engine();
            sha.input(&channel_seed);
            sha.input(&b"commitment seed"[..]);
            Sha256::from_engine(sha).to_byte_array()
        };

        let revocation_base_key = key_step_derivation(&channel_seed, &b"revocation base key"[..], &commitment_seed[..]);
        let payment_key = key_step_derivation(&channel_seed, &b"payment key"[..], &revocation_base_key[..]);
        let delayed_payment_base_key = key_step_derivation(&channel_seed, &b"delayed payment key"[..], &payment_key[..]);
        let htlc_base_key = key_step_derivation(&channel_seed, &b"HTLC base key"[..], &delayed_payment_base_key[..]);

        ChannelKeysManager {
            commitment_seed,
            revocation_base_key,
            payment_key,
            delayed_payment_base_key,
            htlc_base_key,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Basepoint {
    Payment,
    DelayedPayment,
    HTLC,
}

/// Build the commitment secret from the seed and the commitment number
impl ChannelKeysManager {
    pub fn build_commitment_secret(&self, idx: u64) -> [u8; 32] {
        let mut res: [u8; 32] = self.commitment_seed.clone();
        for i in 0..48 {
            let bitpos = 47 - i;
            if idx & (1 << bitpos) == (1 << bitpos) {
                res[bitpos / 8] ^= 1 << (bitpos & 7);
                res = Sha256::hash(&res).to_byte_array();
            }
        }
        res
    }

    pub fn derive_private_key(
        &self,
        basepoint_type: Basepoint,
        commitment_index: u64,
        secp_ctx: &Secp256k1<secp256k1::All>,
    ) -> SecretKey {
        
        // First, get the appropriate base key based on the basepoint type
        let basepoint_secret = match basepoint_type {
            Basepoint::Payment => self.payment_key,
            Basepoint::DelayedPayment => self.delayed_payment_base_key,
            Basepoint::HTLC => self.htlc_base_key,
        };

        // if basepoint is payment, return secret itself
        if basepoint_type == Basepoint::Payment {
            return basepoint_secret;
        }

        // Second, convert basepoint to public key
        let basepoint = PublicKey::from_secret_key(&secp_ctx, &basepoint_secret);

        // Third, get per-commitment-point with index
        let per_commitment_secret = self.build_commitment_secret(commitment_index);
        let per_commitment_privkey = SecretKey::from_slice(&per_commitment_secret).unwrap();
        let per_commitment_point = PublicKey::from_secret_key(secp_ctx, &per_commitment_privkey);

        // Forth, create scalar tweak
        let mut sha = Sha256::engine();
        sha.input(&per_commitment_point.serialize());
        sha.input(&basepoint.serialize());
        let res = Sha256::from_engine(sha).to_byte_array();
        let scalar = Scalar::from_be_bytes(res).unwrap();

        // Finally, add scalar
        basepoint_secret.add_tweak(&scalar).expect("works")
        
    }

    pub fn derive_revocation_public_key(
        &self,
        countersignatory_basepoint: PublicKey,
        commitment_index: u64,
        secp_ctx: &Secp256k1<secp256k1::All>) -> PublicKey {
        
        let per_commitment_secret = self.build_commitment_secret(commitment_index);
        
        let per_commitment_private_key = SecretKey::from_slice(&per_commitment_secret).unwrap();
        
        let per_commitment_point = PublicKey::from_secret_key(secp_ctx, &per_commitment_private_key);
        
        let revocation_pubkey = generate_revocation_pubkey(countersignatory_basepoint, per_commitment_point);
        
        revocation_pubkey

      }
}

fn key_step_derivation(seed: &[u8; 32], bytes: &[u8], previous_key: &[u8]) -> SecretKey {
    let mut sha = Sha256::engine();
    sha.input(seed);
    sha.input(&previous_key[..]);
    sha.input(&bytes[..]);
    SecretKey::from_slice(&Sha256::from_engine(sha).to_byte_array())
        .expect("SHA-256 is busted")
}

fn get_master_key(seed: [u8; 32]) -> Xpriv {
    let master_key = match Xpriv::new_master(Network::Regtest, &seed) {
        Ok(key) => key,
        Err(_) => panic!("Your RNG is busted"),
    };
    master_key
}

fn get_hardened_extended_child_private_key(master_key: Xpriv, idx: u32) -> Xpriv {
    let secp_ctx = Secp256k1::new();
    let hardened_extended_child = master_key
        .derive_priv(&secp_ctx, &ChildNumber::from_hardened_idx(idx).unwrap())
        .expect("Your RNG is busted");
    hardened_extended_child
}
