#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_must_use,
    non_snake_case
)]
use crate::internal;
use bitcoin::script::{Builder, ScriptBuf, ScriptHash};
use internal::key_utils::{add_pubkeys, pubkey_multipication_tweak, pubkey_from_secret, add_privkeys, privkey_multipication_tweak, hash_pubkeys};
use internal::tx_utils::{build_output, build_transaction};
use internal::script_utils::{build_htlc_offerer_witness_script, p2wpkh_output_script};
use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::secp256k1::{SecretKey, PublicKey as secp256k1PublicKey, Scalar};
use bitcoin::PublicKey;
use bitcoin::hashes::Hash;
use bitcoin::{Block, OutPoint, PubkeyHash, Sequence, Transaction, TxIn, TxOut, Witness};
use bitcoin::transaction::Version;
use bitcoin::locktime::absolute::LockTime;

//
// Exercise 1
//

pub fn two_of_two_multisig_witness_script(
    pubkey1: &PublicKey,
    pubkey2: &PublicKey,
) -> ScriptBuf {
    Builder::new()
        .push_int(2)
        .push_key(pubkey1)
        .push_key(pubkey2)
        .push_int(2)
        .push_opcode(opcodes::OP_CHECKMULTISIG)
        .into_script()
}

//
// Exercise 2
//

pub fn build_funding_transaction(
    txins: Vec<TxIn>,
    alice_pubkey: &PublicKey,
    bob_pubkey: &PublicKey,
    amount: u64,
) -> Transaction {

    let witness_script = two_of_two_multisig_witness_script(alice_pubkey, bob_pubkey);

    let txout = build_output(amount, witness_script.to_p2wsh());

    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(
        version,
        locktime,
        txins,
        vec![txout],
    );

    tx
}

//
// Exercise 3
//

pub fn build_refund_transaction(
    funding_txin: TxIn,
    alice_pubkey: PublicKey,
    bob_pubkey: PublicKey,
    alice_balance: u64,
    bob_balance: u64
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
                      vec![bob_output, alice_output]);
    tx
}

//
// Exercise 4
//

pub fn generate_revocation_pubkey(
    countersignatory_basepoint: secp256k1PublicKey,
    per_commitment_point: secp256k1PublicKey,
) -> secp256k1PublicKey {
    let h1 =
        hash_pubkeys(countersignatory_basepoint, per_commitment_point);

    let h2 =
        hash_pubkeys(per_commitment_point, countersignatory_basepoint);

    let R =
        pubkey_multipication_tweak(countersignatory_basepoint, h1);

    let P =
        pubkey_multipication_tweak(per_commitment_point, h2);

    let revocation_pubkey = add_pubkeys(R, P);

    revocation_pubkey
}

//
// Exercise 5
//

pub fn generate_revocation_privkey(per_commitment_secret: SecretKey, countersignatory_revocation_base_secret: SecretKey) -> SecretKey {

    let R = pubkey_from_secret(countersignatory_revocation_base_secret);

    let P = pubkey_from_secret(per_commitment_secret);

    let h1 = hash_pubkeys(R, P);

    let h2 = hash_pubkeys(P, R);

    let key1 = privkey_multipication_tweak(countersignatory_revocation_base_secret, h1);

    let key2 = privkey_multipication_tweak(per_commitment_secret, h2);

    add_privkeys(key1, key2)
}

//
// Exercise 6
//

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

//
// Exercise 7
//

pub fn build_commitment_transaction(
    funding_txin: TxIn,
    revocation_pubkey: &PublicKey,
    to_local_delayed_pubkey: &PublicKey,
    remote_pubkey: PublicKey,
    to_self_delay: i64,
    local_amount: u64,
    remote_amount: u64,
) -> Transaction {

    let to_local_script =
        to_local(revocation_pubkey, to_local_delayed_pubkey, to_self_delay);

    let to_remote_script = p2wpkh_output_script(remote_pubkey);

    let local_output = build_output(local_amount, to_local_script.to_p2wsh());

    let remote_output = build_output(remote_amount, to_remote_script);

    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(version,
                      locktime,
                      vec![funding_txin],
                      vec![remote_output, local_output]);
    tx
}

//
// Exercise 8
//

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
    let htlc_offerer_script = build_htlc_offerer_witness_script(
        revocation_pubkey,
        remote_htlc_pubkey,
        local_htlc_pubkey,
        payment_hash160,
    );

    let to_local_script =
        to_local(revocation_pubkey, to_local_delayed_pubkey, to_self_delay);

    let to_remote_script = p2wpkh_output_script(remote_pubkey);

    let htlc_output = build_output(htlc_amount, htlc_offerer_script.to_p2wsh());

    let local_output = build_output(local_amount, to_local_script.to_p2wsh());

    let remote_output = build_output(remote_amount, to_remote_script);

    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(version,
                               locktime,
                               vec![funding_txin],
                            vec![htlc_output, remote_output, local_output]);

    tx
}

//
// Exercise 9
//

pub fn build_htlc_timeout_transaction(
    htlc_txin: TxIn,
    revocation_pubkey: &PublicKey,
    to_local_delayed_pubkey: &PublicKey,
    to_self_delay: i64,
    cltv_expiry: u32,
    htlc_amount: u64,
) -> Transaction {
    let htlc_timeout_script = to_local(
        revocation_pubkey,
        to_local_delayed_pubkey,
        to_self_delay,
    );

    let htlc_output = build_output(htlc_amount, htlc_timeout_script.to_p2wsh());

    let version = Version::TWO;
    let locktime = LockTime::from_consensus(cltv_expiry);

    let tx = build_transaction(
                version,
                locktime,
                vec![htlc_txin],
                vec![htlc_output]);

    tx
}
