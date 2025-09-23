#![allow(dead_code, unused_variables,unused_imports, unused_must_use)]
use crate::internal;
use crate::exercises;
use internal::builder::Builder;
use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::blockdata::script::ScriptBuf;
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::hashes::Hash;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::transaction::Version;
use bitcoin::{ TxIn,};
use exercises::solutions::{to_local};
use internal::bitcoind_client::{BitcoindClient, get_bitcoind_client};
use internal::key_utils::{add_pubkeys, pubkey_multipication_tweak, pubkey_from_secret, add_privkeys, privkey_multipication_tweak, hash_pubkeys,
      pubkey_from_private_key, secp256k1_private_key};
use internal::tx_utils::{build_output,get_unspent_output, build_transaction, get_funding_input};
use internal::script_utils::{build_htlc_offerer_witness_script, p2wpkh_output_script};
use internal::sign_utils::{sign_raw_transaction, sign_funding_transaction};
use std::time::Duration;
use tokio::time::sleep;
use bitcoin::secp256k1::PublicKey;
use bitcoin::hashes::ripemd160::Hash as Ripemd160;

pub async fn build_funding_tx(bitcoind: BitcoindClient,
                                        tx_input: TxIn,
                                        tx_in_amount: u64) {

    // we're locking to a 2-of-2 multisig, so we need two public keys
    // normally, we would generate our own public key
    //   and the counterparty would send us theirs
    let our_public_key = pubkey_from_private_key(&[0x01; 32]);
    let revocation_key = pubkey_from_private_key(&[0x02; 32]);
    let to_local_delayed_pubkey = pubkey_from_private_key(&[0x03; 32]);
    let counterparty_public_key = pubkey_from_private_key(&[0x04; 32]);

    let to_self_delay = 144;

    // preimage
    let secret = "ProgrammingLightning".to_string();
    let secret_bytes = secret.as_bytes();
    let payment_hash = Sha256::hash(secret_bytes).to_byte_array();
    let payment_hash160 = Ripemd160::hash(&payment_hash).to_byte_array();

    let local_output_script = to_local(&revocation_key, &to_local_delayed_pubkey,
            to_self_delay);

    let remote_output_script = p2wpkh_output_script(counterparty_public_key);

    let local_output = build_output(3_593_500, local_output_script.to_p2wsh());
    let remote_output = build_output(1_000_500, remote_output_script);

    // build funding transaction using the function we created
    let output_script = build_hash_locked_script(&our_public_key,
                                                &payment_hash160);
    println!("Witness Script (hex): {}", output_script.to_hex_string());
    

    let htlc_output = build_output(405_000, output_script.to_p2wsh());

    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(version, locktime, vec![tx_input], vec![local_output, remote_output, htlc_output]);

    let signed_tx = sign_raw_transaction(bitcoind.clone(), tx).await;

    println!("\n");
    println!("Tx ID: {}", signed_tx.compute_txid());
    println!("\n");
    println!("Tx Hex: {}", serialize_hex(&signed_tx));
}

pub async fn run() {

    // get bitcoin client
    let bitcoind = get_bitcoind_client().await;

    // get an unspent output for funding transaction
    let tx_input = get_unspent_output(bitcoind.clone()).await;

    let tx_in_amount = 5_000_000;
    
        build_funding_tx(bitcoind, tx_input, tx_in_amount).await;

    // Add a delay to allow the spawned task to complete
    sleep(Duration::from_secs(2)).await;
}

fn build_hash_locked_script(pubkey: &PublicKey,
                     payment_hash160: &[u8; 20]) -> ScriptBuf {
    Builder::new()
        .push_opcode(opcodes::OP_IF)
        .push_opcode(opcodes::OP_HASH160)
        .push_slice(payment_hash160)
        .push_opcode(opcodes::OP_EQUAL)
        .push_opcode(opcodes::OP_ELSE)
        .push_int(200)
        .push_opcode(opcodes::OP_CLTV)
        .push_opcode(opcodes::OP_DROP)
        .push_key(pubkey)
        .push_opcode(opcodes::OP_CHECKSIG)
        .push_opcode(opcodes::OP_ENDIF)
    .into_script()
}