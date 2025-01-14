#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;

use bitcoin::amount::Amount;
use bitcoin::bip32::{ChildNumber, Xpriv, Xpub};
use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::hashes::ripemd160::Hash as Ripemd160;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::network::Network;
use bitcoin::script::{ScriptBuf, ScriptHash};
use bitcoin::secp256k1;
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::secp256k1::PublicKey;
use bitcoin::secp256k1::Scalar;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::SecretKey;
use bitcoin::transaction::Version;
use bitcoin::PubkeyHash;
use bitcoin::{Block, OutPoint, Transaction, TxIn, TxOut};
use core::sync::atomic::{AtomicUsize, Ordering};
use internal::bitcoind_client::BitcoindClient;
use internal::builder::Builder;
use internal::channel_manager::ChannelManager;
use lightning::sign::KeysManager;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct SimpleKeysManager {
    pub node_secret: SecretKey,
    pub node_id: PublicKey,
    pub unilateral_close_pubkey: PublicKey,
    pub coop_close_pubkey: PublicKey,
    pub channel_master_key: Xpriv,
    pub inbound_payment_key: SecretKey,
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

fn get_public_key(private_key: SecretKey) -> PublicKey {
    let secp_ctx = Secp256k1::new();
    let public_key = PublicKey::from_secret_key(&secp_ctx, &private_key);
    public_key
}

fn get_current_time() -> Duration {
    let cur = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    cur
}

pub fn new_simple_key_manager(seed: [u8; 32]) -> SimpleKeysManager {
    let master_key = get_master_key(seed);

    let node_secret = get_hardened_child_private_key(master_key, 0);

    let node_id = get_public_key(node_secret);

    let unilateral_close_private_key = get_hardened_child_private_key(master_key, 1);
    let unilateral_close_pubkey = get_public_key(unilateral_close_private_key);

    let coop_close_private_key = get_hardened_child_private_key(master_key, 2);
    let coop_close_pubkey = get_public_key(unilateral_close_private_key);

    let channel_master_key = get_hardened_extended_child_private_key(master_key, 3);

    let inbound_payment_key = get_hardened_child_private_key(master_key, 5);

    SimpleKeysManager {
        node_secret: node_secret,
        node_id: node_id,
        unilateral_close_pubkey: unilateral_close_pubkey,
        coop_close_pubkey: coop_close_pubkey,
        channel_master_key: channel_master_key,
        inbound_payment_key: inbound_payment_key,
        seed: seed,
    }
}

pub fn unified_onchain_offchain_wallet2(seed: [u8; 64]) -> KeysManager {

    let master_xprv = Xpriv::new_master(Network::Regtest, &seed).unwrap();
    let secp = Secp256k1::new();
    let xprv: Xpriv = master_xprv
        .derive_priv(&secp, &ChildNumber::from_hardened_idx(535).unwrap())
        .expect("Your RNG is busted");
    let ldk_seed: [u8; 32] = xprv.private_key.secret_bytes();

    // Seed the LDK KeysManager with the private key at m/535h
    let cur = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let keys_manager = KeysManager::new(&ldk_seed, cur.as_secs(), cur.subsec_nanos());
    keys_manager
}

pub fn unified_onchain_offchain_wallet(seed: [u8; 32]) -> KeysManager {

    let master_key = get_master_key(seed);

    let ldk_key = get_hardened_extended_child_private_key(master_key, 535);

    let ldk_seed = ldk_key.private_key.secret_bytes();

    let cur = get_current_time();
    
    let keys_manager = KeysManager::new(&ldk_seed, cur.as_secs(), cur.subsec_nanos());
    
    keys_manager
}