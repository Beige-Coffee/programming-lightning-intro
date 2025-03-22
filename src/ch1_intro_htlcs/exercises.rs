#![allow(dead_code, unused_imports, unused_variables, unused_must_use, non_snake_case)]
use crate::internal;
use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::hashes::Hash;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::script::ScriptBuf;
use bitcoin::secp256k1::{SecretKey, PublicKey, Scalar};
use bitcoin::transaction::Version;
use bitcoin::PublicKey as BitcoinPublicKey;
use bitcoin::{Block, OutPoint, PubkeyHash, Sequence, Transaction, TxIn, TxOut, Witness};
use internal::builder::Builder;
use internal::helper::{
    add_pubkeys, build_htlc_offerer_witness_script, build_output, build_transaction, hash_pubkeys,
    p2wpkh_output_script, pubkey_multipication_tweak,pubkey_from_secret, add_privkeys,privkey_multipication_tweak
};
use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::HashEngine;

pub fn two_of_two_multisig_witness_script(pubkey1: &PublicKey, pubkey2: &PublicKey) -> ScriptBuf {
    unimplemented!()
}

pub fn build_funding_transaction(
    txins: Vec<TxIn>,
    alice_pubkey: &PublicKey,
    bob_pubkey: &PublicKey,
    amount: u64,
) -> Transaction {
    unimplemented!()
}

pub fn build_refund_transaction(
    funding_txin: TxIn,
    alice_pubkey: PublicKey,
    bob_pubkey: PublicKey,
    alice_balance: u64,
    bob_balance: u64,
) -> Transaction {
    
    unimplemented!()
}


pub fn generate_revocation_pubkey(
    countersignatory_revocation_basepoint: PublicKey,
    per_commitment_point: PublicKey,
) -> PublicKey {
    
    unimplemented!()
}

pub fn generate_revocation_privkey(per_commitment_secret: SecretKey, countersignatory_revocation_base_secret: SecretKey) -> SecretKey {

    unimplemented!()
}

pub fn to_local(
    revocation_key: &PublicKey,
    to_local_delayed_pubkey: &PublicKey,
    to_self_delay: i64,
) -> ScriptBuf {
    unimplemented!()
}

pub fn build_commitment_transaction(
    funding_txin: TxIn,
    revocation_pubkey: &PublicKey,
    to_local_delayed_pubkey: &PublicKey,
    remote_pubkey: PublicKey,
    to_self_delay: i64,
    local_amount: u64,
    remote_amount: u64,
) -> Transaction {
    unimplemented!()
}

pub fn build_htlc_commitment_transaction(
    funding_txin: TxIn,
    revocation_pubkey: &PublicKey,
    remote_htlc_pubkey: &PublicKey,
    local_htlc_pubkey: &PublicKey,
    to_local_delayed_pubkey: &PublicKey,
    remote_pubkey: PublicKey,
    to_self_delay: i64,
    payment_hash160: &[u8; 20],
    htlc_amount: u64,
    local_amount: u64,
    remote_amount: u64,
) -> Transaction {
    unimplemented!()
}

pub fn build_htlc_timeout_transaction(
    htlc_txin: TxIn,
    revocation_pubkey: &PublicKey,
    to_local_delayed_pubkey: &PublicKey,
    to_self_delay: i64,
    cltv_expiry: u32,
    htlc_amount: u64,
) -> Transaction {
    unimplemented!()
}
