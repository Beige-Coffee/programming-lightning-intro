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
    Builder::new()
        .push_int(2)
        .push_key(pubkey1)
        .push_key(pubkey2)
        .push_int(2)
        .push_opcode(opcodes::OP_CHECKMULTISIG)
        .into_script()
}

pub fn build_funding_transaction(
    txins: Vec<TxIn>,
    alice_pubkey: &PublicKey,
    bob_pubkey: &PublicKey,
    amount: u64,
) -> Transaction {
    let multisig_witness = two_of_two_multisig_witness_script(alice_pubkey, bob_pubkey);

    let output = build_output(amount, multisig_witness.to_p2wsh());

    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(version, locktime, txins, vec![output]);

    tx
}

pub fn build_refund_transaction(
    funding_txin: TxIn,
    alice_pubkey: PublicKey,
    bob_pubkey: PublicKey,
    alice_balance: u64,
    bob_balance: u64,
) -> Transaction {
    
    let alice_script = p2wpkh_output_script(alice_pubkey);

    let bob_script = p2wpkh_output_script(bob_pubkey);

    let alice_output = build_output(alice_balance, alice_script);

    let bob_output = build_output(bob_balance, bob_script);

    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(version,
                      locktime,
                      vec![funding_txin],
                      vec![alice_output, bob_output]);
    tx
}


pub fn generate_revocation_pubkey(
    countersignatory_revocation_basepoint: PublicKey,
    per_commitment_point: PublicKey,
) -> PublicKey {
    
    let h1 = hash_pubkeys(countersignatory_revocation_basepoint, per_commitment_point);

    let h2 = hash_pubkeys(per_commitment_point, countersignatory_revocation_basepoint);

    let key1 = pubkey_multipication_tweak(countersignatory_revocation_basepoint, h1);

    let key2 = pubkey_multipication_tweak(per_commitment_point, h2);

    add_pubkeys(key1, key2)
}

pub fn generate_revocation_privkey(per_commitment_secret: SecretKey, countersignatory_revocation_base_secret: SecretKey) -> SecretKey {

    let R = pubkey_from_secret(countersignatory_revocation_base_secret);

    let P = pubkey_from_secret(per_commitment_secret);

    let h1 = hash_pubkeys(R, P);

    let h2 = hash_pubkeys(P, R);

    let key1 = privkey_multipication_tweak(countersignatory_revocation_base_secret, h1);

    let key2 = privkey_multipication_tweak(per_commitment_secret, h2);

    add_privkeys(key1, key2)
}

pub fn to_local(
    revocation_key: &PublicKey,
    to_local_delayed_pubkey: &PublicKey,
    to_self_delay: i64,
) -> ScriptBuf {
    Builder::new()
        .push_opcode(opcodes::OP_IF)
        .push_key(revocation_key)
        .push_opcode(opcodes::OP_ELSE)
        .push_int(to_self_delay)
        .push_opcode(opcodes::OP_CSV)
        .push_opcode(opcodes::OP_DROP)
        .push_key(to_local_delayed_pubkey)
        .push_opcode(opcodes::OP_ENDIF)
        .push_opcode(opcodes::OP_CHECKSIG)
        .into_script()
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
    let local_script = to_local(revocation_pubkey, to_local_delayed_pubkey, to_self_delay);
    let remote_scropt = p2wpkh_output_script(remote_pubkey);

    let local_output = build_output(local_amount, local_script.to_p2wsh());

    let remote_output = build_output(remote_amount, remote_scropt);

    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(version,
                      locktime,
                      vec![funding_txin],
                      vec![local_output, remote_output]);
    tx
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
    let local_script = to_local(revocation_pubkey, to_local_delayed_pubkey, to_self_delay);

    let remote_script = p2wpkh_output_script(remote_pubkey);

    let htlc_script = build_htlc_offerer_witness_script(
            revocation_pubkey,
            remote_htlc_pubkey,
            local_htlc_pubkey,
            payment_hash160);

    let local_output = build_output(local_amount, local_script.to_p2wsh());

    let remote_output = build_output(remote_amount, remote_script);

    let htlc_output = build_output(htlc_amount, htlc_script.to_p2wsh());

    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(version,
                      locktime,
                      vec![funding_txin],
                      vec![local_output, remote_output, htlc_output]);
    tx
}

pub fn build_htlc_timeout_transaction(
    htlc_txin: TxIn,
    revocation_pubkey: &PublicKey,
    to_local_delayed_pubkey: &PublicKey,
    to_self_delay: i64,
    cltv_expiry: u32,
    htlc_amount: u64,
) -> Transaction {
    let local_script = to_local(revocation_pubkey, to_local_delayed_pubkey, to_self_delay);

    let local_output = build_output(htlc_amount, local_script.to_p2wsh());

    let version = Version::TWO;
    let locktime = LockTime::from_consensus(cltv_expiry);

    let tx = build_transaction(version,
                      locktime,
                      vec![htlc_txin],
                      vec![local_output]);
    tx
    
}
