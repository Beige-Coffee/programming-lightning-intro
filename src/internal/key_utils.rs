#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use crate::exercises;
use internal::bitcoind_client::BitcoindClient;
use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::Hash;
use bitcoin::hashes::HashEngine;
use bitcoin::secp256k1;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::{SecretKey, PublicKey as secp256k1PublicKey, Scalar};
use bitcoin::PublicKey;
use bitcoin::script::{ScriptBuf};
use bitcoin::{OutPoint, Sequence, Transaction, TxIn, TxOut, Witness};
use bitcoin::amount::Amount;
use bitcoin::transaction::Version;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::{PubkeyHash};
use bitcoin::{Network};
use bitcoin::consensus::encode::serialize_hex;
use internal::hex_utils;
use bitcoin::consensus::{encode};
use bitcoin::hash_types::Txid;
use std::env;
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::secp256k1::Message;
use bitcoin::sighash::EcdsaSighashType;
use bitcoin::sighash::SighashCache;
use exercises::exercises::{ two_of_two_multisig_witness_script};

pub fn secp256k1_private_key(private_key_bytes: &[u8; 32]) -> SecretKey {
    let secp = Secp256k1::new();
    SecretKey::from_slice(private_key_bytes).unwrap()
}

pub fn pubkey_from_secret(secret: SecretKey) -> secp256k1PublicKey {
    let secp = Secp256k1::new();
    secp256k1PublicKey::from_secret_key(&secp, &secret)
}

pub fn secp256k1pubkey_from_private_key(private_key: &[u8; 32]) -> secp256k1PublicKey {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key).unwrap();
    secp256k1PublicKey::from_secret_key(&secp, &secret_key)
}

pub fn pubkey_from_private_key(private_key: &[u8; 32]) -> PublicKey {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key).unwrap();
    let public_key = secp256k1PublicKey::from_secret_key(&secp, &secret_key);
    PublicKey::new(public_key)
}

pub fn pubkey_multipication_tweak(pubkey1: secp256k1PublicKey, sha_bytes: [u8; 32]) -> secp256k1PublicKey {
    let secp = Secp256k1::new();
    pubkey1.mul_tweak(&secp, &Scalar::from_be_bytes(sha_bytes).unwrap()).unwrap()
}

pub fn privkey_multipication_tweak(secret: SecretKey, sha_bytes: [u8; 32]) -> SecretKey {
    secret.mul_tweak(&Scalar::from_be_bytes(sha_bytes).unwrap()).unwrap()
}

pub fn hash_pubkeys(key1: secp256k1PublicKey, key2: secp256k1PublicKey) -> [u8; 32] {
    let mut sha = Sha256::engine();

    sha.input(&key1.serialize());
    sha.input(&key2.serialize());

    Sha256::from_engine(sha).to_byte_array()
}

pub fn add_pubkeys(key1: secp256k1PublicKey, key2: secp256k1PublicKey) -> secp256k1PublicKey {
    let pk = key1.combine(&key2).unwrap();

    pk
}

pub fn add_privkeys(key1: SecretKey, key2: SecretKey) -> SecretKey {
    let tweak = Scalar::from_be_bytes(key2.secret_bytes()).unwrap();
    key1.add_tweak(&tweak).unwrap()
}