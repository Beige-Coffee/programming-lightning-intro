#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
pub mod internal;

use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::secp256k1;
use bitcoin::secp256k1::PublicKey as Secp256k1PublicKey;
use bitcoin::secp256k1::Scalar;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::script::{ScriptBuf, ScriptHash};
use bitcoin::amount::Amount;
use bitcoin::{Block, OutPoint, PublicKey, Transaction, TxOut};
use internal::bitcoind_client::BitcoindClient;
use internal::builder::Builder;
use internal::channel_manager::ChannelManager;
use internal::helper::{
    pubkey_multiplication_tweak,
    sha256_hash,
};

fn p2pkh(pubkey: &PublicKey) -> ScriptBuf {
    Builder::new()
        .push_opcode(opcodes::OP_DUP)
        .push_opcode(opcodes::OP_HASH160)
        .push_pubkey_hash(pubkey)
        .push_opcode(opcodes::OP_EQUALVERIFY)
        .push_opcode(opcodes::OP_CHECKSIG)
        .into_script()
}

fn p2sh(script_hash: &ScriptHash) -> ScriptBuf {
    Builder::new()
        .push_opcode(opcodes::OP_HASH160)
        .push_script_hash(script_hash)
        .push_opcode(opcodes::OP_EQUAL)
        .into_script()
}

fn two_of_two_multisig(alice_pubkey: &PublicKey, bob_pubkey: &PublicKey) -> ScriptBuf {
    Builder::new()
    .push_int(2)
    .push_key(alice_pubkey)
    .push_key(bob_pubkey)
    .push_int(2)
    .push_opcode(opcodes::OP_CHECKMULTISIG)
    .into_script()
}

fn two_of_three_multisig_redeem_script(pubkey: &PublicKey, pubkey2: &PublicKey,
                                       pubkey3: &PublicKey) -> ScriptBuf {
    Builder::new()
        .push_int(2)
        .push_key(pubkey)
        .push_key(pubkey2)
        .push_key(pubkey3)
        .push_int(3)
        .push_opcode(opcodes::OP_CHECKMULTISIG)
    .into_script()
}

fn cltv_p2pkh(pubkey: &PublicKey, height_or_timestamp: i64) -> ScriptBuf {
    Builder::new()
        .push_int(height_or_timestamp)
        .push_opcode(opcodes::OP_CLTV)
        .push_opcode(opcodes::OP_DROP)
        .push_opcode(opcodes::OP_DUP)
        .push_opcode(opcodes::OP_HASH160)
        .push_pubkey_hash(pubkey)
        .push_opcode(opcodes::OP_EQUALVERIFY)
        .push_opcode(opcodes::OP_CHECKSIG)
        .into_script()
}

fn csv_p2pkh(pubkey: &PublicKey, height_or_timestamp: i64) -> ScriptBuf {
    Builder::new()
        .push_int(height_or_timestamp)
        .push_opcode(opcodes::OP_CSV)
        .push_opcode(opcodes::OP_DROP)
        .push_opcode(opcodes::OP_DUP)
        .push_opcode(opcodes::OP_HASH160)
        .push_pubkey_hash(pubkey)
        .push_opcode(opcodes::OP_EQUALVERIFY)
        .push_opcode(opcodes::OP_CHECKSIG)
        .into_script()
}

fn payment_channel_funding_output(
    alice_pubkey: &PublicKey,
    bob_pubkey: &PublicKey,
    height: i64,
) -> ScriptBuf {
    Builder::new()
        .push_opcode(opcodes::OP_IF)
        .push_script(two_of_two_multisig(alice_pubkey, bob_pubkey))
        .push_opcode(opcodes::OP_ELSE)
        .push_script(csv_p2pkh(alice_pubkey, height))
        .push_opcode(opcodes::OP_ENDIF)
        .into_script()
}

fn block_connected(funding_output: ScriptBuf, channel_amount_sats: Amount, block: Block) -> bool {
    let mut connected = false;
    for tx in block.txdata {
        for output in tx.output {
            if output.script_pubkey == funding_output && output.value == channel_amount_sats {
                connected = true
            }
        }
    }
    connected
}

fn spend_multisig(alice_signature: Signature, bob_signature: Signature) -> ScriptBuf {
    Builder::new()
        .push_signature(alice_signature)
        .push_signature(bob_signature)
        .push_int(0)
    .into_script()
}

fn spend_refund(alice_pubkey: &PublicKey, alice_signature: Signature) -> ScriptBuf {
    Builder::new()
        .push_signature(alice_signature)
        .push_key(alice_pubkey)
        .push_int(1)
    .into_script()
}

pub fn generate_revocation_pubkey(countersignatory_basepoint: Secp256k1PublicKey,
    per_commitment_point: Secp256k1PublicKey) -> Secp256k1PublicKey {

    let rev_append_commit_hash_key = sha256_hash(&countersignatory_basepoint,&per_commitment_point);

    let commit_append_rev_hash_key = sha256_hash(&per_commitment_point, &countersignatory_basepoint);

    let countersignatory_contrib = pubkey_multiplication_tweak(countersignatory_basepoint, rev_append_commit_hash_key);

    let broadcaster_contrib = pubkey_multiplication_tweak(per_commitment_point, commit_append_rev_hash_key);

    let pk = countersignatory_contrib.combine(&broadcaster_contrib)
        .expect("Addition only fails if the tweak is the inverse of the key. This is not possible when the tweak commits to the key.");

    pk

    }

fn channel_closed(funding_outpoint: OutPoint, block: Block) -> bool {
    for tx in block.txdata {
        for input in tx.input {
            if input.previous_output == funding_outpoint {
                return true;
            }
        }
    }
    false
}

fn handle_funding_generation_ready(
    channel_manager: &ChannelManager,
    bitcoind_client: &BitcoindClient,
    temporary_channel_id: &[u8; 32],
    counterparty_node_id: &PublicKey,
    channel_value_satoshis: u64,
    output_script: ScriptBuf,
    user_channel_id: u128,
) {
}

#[cfg(test)]
mod test;
