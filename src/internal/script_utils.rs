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
use bitcoin::script::{Builder};

pub fn p2wpkh_output_script(public_key: PublicKey) -> ScriptBuf {
    ScriptBuf::new_p2wpkh(&public_key.wpubkey_hash().unwrap())
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
        .push_slice(revocation_pubkey.pubkey_hash())
        .push_opcode(opcodes::OP_EQUAL)
        .push_opcode(opcodes::OP_IF)
        .push_opcode(opcodes::OP_CHECKSIG)
        .push_opcode(opcodes::OP_ELSE)
        .push_key(&remote_htlc_pubkey)
        .push_opcode(opcodes::OP_SWAP)
        .push_opcode(opcodes::OP_SIZE)
        .push_int(32)
        .push_opcode(opcodes::OP_EQUAL)
        .push_opcode(opcodes::OP_NOTIF)
        .push_opcode(opcodes::OP_DROP)
        .push_int(2)
        .push_opcode(opcodes::OP_SWAP)
        .push_key(&local_htlc_pubkey)
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
