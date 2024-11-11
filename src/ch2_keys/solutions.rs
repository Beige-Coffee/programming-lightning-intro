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

pub struct SimpleKeysManager {
    secp_ctx: Secp256k1<secp256k1::All>,
    node_secret: SecretKey,
    node_id: PublicKey,
    destination_script: ScriptBuf,
    shutdown_pubkey: PublicKey,
    channel_master_key: Xpriv,
    channel_child_index: AtomicUsize,
    seed: [u8; 32],
    starting_time_secs: u64,
    starting_time_nanos: u32,
}

fn get_master_key(seed: &[u8; 32]) -> Xpriv {
    let master_key = match Xpriv::new_master(Network::Regtest, seed) {
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

fn new_simple_key_manager(seed: &[u8; 32], starting_time_secs: u64, starting_time_nanos: u32) {

    let master_key = get_master_key(seed);

    let node_secret = get_hardened_child_private_key(master_key, 0);
}