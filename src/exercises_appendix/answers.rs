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
use crate::exercises::exercises::{
  generate_revocation_pubkey
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NodeKeysManager {
    pub secp_ctx: Secp256k1<secp256k1::All>,
    pub channel_master_key: Xpriv,
    pub node_secret: SecretKey,
    pub node_id: PublicKey,
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

        let master_key = get_master_key(seed);

        let node_secret = master_key
            .derive_priv(&secp_ctx, &ChildNumber::from_hardened_idx(0).unwrap())
            .expect("Your RNG is busted")
            .private_key;
        let node_id = PublicKey::from_secret_key(&secp_ctx, &node_secret);

        let channel_master_key = get_hardened_extended_child_private_key(master_key, 3);

        NodeKeysManager {
            secp_ctx: secp_ctx,
            channel_master_key: channel_master_key,
            node_secret: node_secret,
            node_id: node_id,
            seed: seed,
        }
    }

    pub fn derive_channel_keys(&self, channel_id_params: &[u8; 32]) -> ChannelKeysManager {
        let chan_id = u64::from_be_bytes(channel_id_params[0..8].try_into().unwrap());
        let mut unique_start = Sha256::engine();
        unique_start.input(channel_id_params);
        unique_start.input(&self.seed);

        // We only seriously intend to rely on the channel_master_key for true secure
        // entropy, everything else just ensures uniqueness. We rely on the unique_start (ie
        // starting_time provided in the constructor) to be unique.
        let child_privkey = self
            .channel_master_key
            .derive_priv(
                &self.secp_ctx,
                &ChildNumber::from_hardened_idx((chan_id as u32) % (1 << 31))
                    .expect("key space exhausted"),
            )
            .expect("Your RNG is busted");
        unique_start.input(&child_privkey.private_key[..]);

        let seed = Sha256::from_engine(unique_start).to_byte_array();

        let commitment_seed = {
            let mut sha = Sha256::engine();
            sha.input(&self.seed);
            sha.input(&b"commitment seed"[..]);
            Sha256::from_engine(sha).to_byte_array()
        };

        let revocation_base_key = key_step_derivation(&seed, &b"revocation base key"[..], &commitment_seed[..]);
        let payment_key = key_step_derivation(&seed, &b"payment key"[..], &revocation_base_key[..]);
        let delayed_payment_base_key = key_step_derivation(&seed, &b"delayed payment key"[..], &payment_key[..]);
        let htlc_base_key = key_step_derivation(&seed, &b"HTLC base key"[..], &delayed_payment_base_key[..]);

        ChannelKeysManager {
            commitment_seed,
            revocation_base_key,
            payment_key,
            delayed_payment_base_key,
            htlc_base_key,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Basepoint {
    Revocation,
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
            Basepoint::Payment => &self.payment_key,
            Basepoint::DelayedPayment => &self.delayed_payment_base_key,
            Basepoint::HTLC => &self.htlc_base_key,
            Basepoint::Revocation => &self.revocation_base_key,
        };

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
