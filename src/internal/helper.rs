use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::Hash;
use bitcoin::hashes::HashEngine;
use bitcoin::secp256k1;
use bitcoin::secp256k1::PublicKey;
use bitcoin::secp256k1::Scalar;
use bitcoin::secp256k1::Secp256k1;

pub fn pubkey_multiplication_tweak(pubkey1: PublicKey, sha_bytes: [u8; 32]) -> PublicKey {
    let secp = Secp256k1::new();
    pubkey1.mul_tweak(&secp, &Scalar::from_be_bytes(sha_bytes).unwrap())
    .expect("Multiplying a valid public key by a hash is expected to never fail per secp256k1 docs")
}

pub fn sha256_hash(key1: &PublicKey, key2: &PublicKey) -> [u8; 32] {
    let mut sha = Sha256::engine();
    sha.input(&key1.serialize());
    sha.input(&key2.serialize());

    Sha256::from_engine(sha).to_byte_array()
}