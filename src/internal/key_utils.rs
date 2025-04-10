
use bitcoin::secp256k1::{PublicKey, SecretKey, Scalar};
use bitcoin::PublicKey as BitcoinPublicKey;
use bitcoin::secp256k1::Secp256k1;

pub fn secp256k1_private_key(private_key_bytes: &[u8; 32]) -> SecretKey {
    let secp = Secp256k1::new();
    SecretKey::from_slice(private_key_bytes).unwrap()
}

pub fn pubkey_from_secret(secret: SecretKey) -> PublicKey {
    let secp = Secp256k1::new();
    PublicKey::from_secret_key(&secp, &secret)
}

pub fn pubkey_from_private_key(private_key: &[u8; 32]) -> PublicKey {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key).unwrap();
    PublicKey::from_secret_key(&secp, &secret_key)
}

pub fn bitcoin_pubkey_from_private_key(private_key: &[u8; 32]) -> BitcoinPublicKey {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key).unwrap();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    BitcoinPublicKey::new(public_key)
}

pub fn pubkey_multipication_tweak(pubkey1: PublicKey, sha_bytes: [u8; 32]) -> PublicKey {
    let secp = Secp256k1::new();
    pubkey1.mul_tweak(&secp, &Scalar::from_be_bytes(sha_bytes).unwrap()).unwrap()
}

pub fn privkey_multipication_tweak(secret: SecretKey, sha_bytes: [u8; 32]) -> SecretKey {
    secret.mul_tweak(&Scalar::from_be_bytes(sha_bytes).unwrap()).unwrap()
}
