#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::Hash;
use bitcoin::hashes::HashEngine;
use bitcoin::secp256k1;
use bitcoin::secp256k1::Scalar;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::PublicKey;
use bitcoin::PublicKey as BitcoinPublicKey;
use bitcoin::script::{ScriptBuf};
use bitcoin::{Block, OutPoint, Sequence, Transaction, TxIn, TxOut, Witness};
use bitcoin::amount::Amount;
use bitcoin::transaction::Version;
use bitcoin::locktime::absolute::LockTime;
use internal::builder::Builder;
use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::{PubkeyHash, WPubkeyHash};


pub fn tweak_pubkey(pubkey1: PublicKey, sha_bytes: [u8; 32]) -> PublicKey {
    let secp = Secp256k1::new();
    pubkey1.mul_tweak(&secp, &Scalar::from_be_bytes(sha_bytes).unwrap())
    .expect("Multiplying a valid public key by a hash is expected to never fail per secp256k1 docs")
}

pub fn hash_pubkeys(key1: PublicKey, key2: PublicKey) -> [u8; 32] {
    let mut sha = Sha256::engine();

    sha.input(&key1.serialize());
    sha.input(&key2.serialize());

    Sha256::from_engine(sha).to_byte_array()
}

pub fn add_pubkeys(key1: PublicKey, key2: PublicKey) -> PublicKey {
    let pk = key1.combine(&key2)
        .expect("Addition only fails if the tweak is the inverse of the key. This is not possible when the tweak commits to the key.");

    pk
}

pub fn secp256k1_private_key(private_key_bytes: &[u8; 32]) -> secp256k1::SecretKey {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(private_key_bytes).unwrap();
    secret_key
}

pub fn pubkey_from_private_key(private_key: &[u8; 32]) -> PublicKey {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(private_key).unwrap();
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
    public_key
}

pub fn bitcoin_pubkey_from_private_key(private_key: &[u8; 32]) -> BitcoinPublicKey {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(private_key).unwrap();
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
    BitcoinPublicKey::new(public_key)
}

pub fn p2wpkh_output_script(public_key: PublicKey) -> ScriptBuf {
    let pubkey = BitcoinPublicKey::new(public_key);
    ScriptBuf::new_p2wpkh(&pubkey.wpubkey_hash().unwrap())
}

pub fn build_output(amount: u64, output_script: ScriptBuf) -> TxOut {

    TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: output_script,
    }
}

pub fn build_transaction(
    version: Version,
    locktime: LockTime,
    tx_ins: Vec<TxIn>,
    tx_outs: Vec<TxOut>,
) -> Transaction {
    Transaction {
        version: version,
        lock_time: locktime,
        input: tx_ins,
        output: tx_outs,
    }
}

pub fn build_htlc_offerer_witness_script(
    revocation_pubkey: &PublicKey,
    remote_htlc_pubkey: &PublicKey,
    local_htlc_pubkey: &PublicKey,
    payment_hash160: &[u8; 20],
) -> ScriptBuf {
    Builder::new()
        .push_opcode(opcodes::OP_DUP)
        .push_opcode(opcodes::OP_HASH160)
        .push_slice(&PubkeyHash::hash(&revocation_pubkey.serialize()))
        .push_opcode(opcodes::OP_EQUAL)
        .push_opcode(opcodes::OP_IF)
        .push_opcode(opcodes::OP_CHECKSIG)
        .push_opcode(opcodes::OP_ELSE)
        .push_slice(&remote_htlc_pubkey.serialize())
        .push_opcode(opcodes::OP_SWAP)
        .push_opcode(opcodes::OP_SIZE)
        .push_int(32)
        .push_opcode(opcodes::OP_EQUAL)
        .push_opcode(opcodes::OP_NOTIF)
        .push_opcode(opcodes::OP_DROP)
        .push_int(2)
        .push_opcode(opcodes::OP_SWAP)
        .push_slice(&local_htlc_pubkey.serialize())
        .push_int(2)
        .push_opcode(opcodes::OP_CHECKMULTISIG)
        .push_opcode(opcodes::OP_ELSE)
        .push_opcode(opcodes::OP_HASH160)
        .push_slice(payment_hash160)
        .push_opcode(opcodes::OP_EQUALVERIFY)
        .push_opcode(opcodes::OP_CHECKSIG)
        .push_opcode(opcodes::OP_ENDIF)
        .push_opcode(opcodes::OP_ENDIF)
        .into_script()
}