#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;

use bitcoin::amount::Amount;
use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::hashes::ripemd160::Hash as Ripemd160;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::script::{ScriptBuf, ScriptHash};
use bitcoin::secp256k1;
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::secp256k1::PublicKey as Secp256k1PublicKey;
use bitcoin::secp256k1::Scalar;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::SecretKey;
use bitcoin::transaction::Version;
use bitcoin::PubkeyHash;
use bitcoin::{Block, OutPoint, PublicKey, Transaction, TxIn, TxOut};
use internal::bitcoind_client::BitcoindClient;
use internal::builder::Builder;
use internal::channel_manager::ChannelManager;
use internal::helper::{pubkey_multiplication_tweak, sha256_hash};
use bitcoin::bip32::{ChildNumber, Xpriv, Xpub};
use core::sync::atomic::{AtomicUsize, Ordering};
use bitcoin::network::Network;

#[derive(Debug)]
pub struct SimpleKeysManager {
    pub node_secret: SecretKey,
    pub node_id: Secp256k1PublicKey,
    pub shutdown_pubkey: Secp256k1PublicKey,
    pub channel_master_key: Xpriv,
    pub channel_child_index: AtomicUsize,
    pub seed: [u8; 32],
}

fn get_master_key(seed: [u8; 32]) -> Xpriv {
    let master_key = match Xpriv::new_master(Network::Regtest, &seed) {
        Ok(key) => key,
        Err(_) => panic!("Your RNG is busted"),
    };
    master_key
}

fn get_hardened_child_private_key(master_key: Xpriv, idx: u32) -> SecretKey {
    let secp_ctx = Secp256k1::new();
    let hardened_child = master_key
        .derive_priv(&secp_ctx, &ChildNumber::from_hardened_idx(idx).unwrap())
        .expect("Your RNG is busted")
        .private_key;
    hardened_child
}

fn get_hardened_extended_child_private_key(master_key: Xpriv, idx: u32) -> Xpriv {
    let secp_ctx = Secp256k1::new();
    let hardened_extended_child = master_key
        .derive_priv(&secp_ctx, &ChildNumber::from_hardened_idx(idx).unwrap())
        .expect("Your RNG is busted");
    hardened_extended_child
}

fn get_public_key(private_key: SecretKey) -> Secp256k1PublicKey {
    let secp_ctx = Secp256k1::new();
    let public_key = Secp256k1PublicKey::from_secret_key(&secp_ctx, &private_key);
    public_key
}

pub fn new_simple_key_manager(seed: [u8; 32]) -> SimpleKeysManager{

    let master_key = get_master_key(seed);

    let node_secret = get_hardened_child_private_key(master_key, 0);

    let node_id = get_public_key(node_secret);

    let shutdown_private_key = get_hardened_child_private_key(master_key, 2);
    let shutdown_public_key = get_public_key(shutdown_private_key);

    let channel_master_key = get_hardened_extended_child_private_key(master_key, 3);

    SimpleKeysManager {
        node_secret: node_secret,
        node_id: node_id,
        shutdown_pubkey: shutdown_public_key,
        channel_master_key: channel_master_key,
        channel_child_index: AtomicUsize::new(0),
        seed: seed,
    }
}
