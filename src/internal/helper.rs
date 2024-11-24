#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::Hash;
use bitcoin::hashes::HashEngine;
use bitcoin::secp256k1;
use bitcoin::secp256k1::Scalar;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::PublicKey as Secp256k1PublicKey;
use bitcoin::PublicKey;


pub fn pubkey_multiplication_tweak(pubkey1: Secp256k1PublicKey, sha_bytes: [u8; 32]) -> Secp256k1PublicKey {
    let secp = Secp256k1::new();
    pubkey1.mul_tweak(&secp, &Scalar::from_be_bytes(sha_bytes).unwrap())
    .expect("Multiplying a valid public key by a hash is expected to never fail per secp256k1 docs")
}

pub fn sha256_hash(key1: &Secp256k1PublicKey, key2: &Secp256k1PublicKey) -> [u8; 32] {
    let mut sha = Sha256::engine();
    sha.input(&key1.serialize());
    sha.input(&key2.serialize());

    Sha256::from_engine(sha).to_byte_array()
}

pub fn secp256k1_private_key(private_key_bytes: &[u8; 32]) -> secp256k1::SecretKey {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(private_key_bytes).unwrap();
    secret_key
}

pub fn secp256k1_pubkey_from_private_key(private_key: &[u8; 32]) -> Secp256k1PublicKey {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(private_key).unwrap();
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
    public_key
}

pub fn pubkey_from_private_key(private_key: &[u8; 32]) -> PublicKey {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(private_key).unwrap();
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
    PublicKey::new(public_key)
}