#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use internal::builder::Builder;
use crate::internal;
use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::blockdata::script::ScriptBuf;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::hashes::Hash;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::sighash::EcdsaSighashType;
use bitcoin::transaction::Version;
use bitcoin::secp256k1::{PublicKey};
use bitcoin::{TxIn};
use bitcoin::hashes::sha256::Hash as Sha256;
use std::time::Duration;
use tokio::time::sleep;
use bitcoin::hashes::ripemd160::Hash as Ripemd160;
use internal::bitcoind_client::{BitcoindClient, get_bitcoind_client};
use internal::key_utils::{add_pubkeys, pubkey_multipication_tweak, pubkey_from_secret, add_privkeys, privkey_multipication_tweak, hash_pubkeys,
      pubkey_from_private_key, secp256k1_private_key};
use internal::tx_utils::{build_output,get_unspent_output, build_transaction, get_funding_input, get_htlc_funding_input};
use internal::script_utils::{build_htlc_offerer_witness_script, p2wpkh_output_script};
use internal::sign_utils::{sign_raw_transaction, sign_funding_transaction, generate_p2wsh_signature};


pub async fn create_broadcast_funding_tx(bitcoind: BitcoindClient,
                                        txid: String,
                                        funding_amount: u64) {

    let txid_index = 2;
    let txin = get_htlc_funding_input(txid.to_string(), txid_index);
    let our_public_key = pubkey_from_private_key(&[0x01; 32]);

    let tx = build_p2wpkh_tx(txin, our_public_key);

    let signed_tx = sign_transaction(tx);

    println!("\n");
    println!("Tx ID: {}", signed_tx.compute_txid());
    println!("\n");
    println!("Tx Hex: {}", serialize_hex(&signed_tx));

}


pub async fn run(funding_txid: String) {

    // Parse the argument as txid
    let txid = funding_txid;

    // get bitcoin client
    let bitcoind = get_bitcoind_client().await;

    // Get our keys
    let our_public_key = pubkey_from_private_key(&[0x01; 32]);
    
    let funding_amount = 5_000_000;
    
    create_broadcast_funding_tx(bitcoind, txid.clone(), funding_amount).await;

    // Add a delay to allow the spawned task to complete
    sleep(Duration::from_secs(2)).await;
}

pub fn sign_transaction(tx: Transaction)-> Transaction {

    let funding_amount = 405_000;
    let txid_index = 2;
    
    let our_public_key = pubkey_from_private_key(&[0x01; 32]);
    let our_private_key = secp256k1_private_key(&[0x01; 32]);

    let secret = "ProgrammingLightning".to_string();
    let secret_bytes = secret.as_bytes();
    let payment_hash = Sha256::hash(secret_bytes).to_byte_array();
    let payment_hash160 = Ripemd160::hash(&payment_hash).to_byte_array();

    // build funding transaction using the function we created
    let redeem_script = build_hash_locked_script(&our_public_key,
                                                &payment_hash160);

    let path = true;

    let mut signed_tx = tx.clone();

    if path {

        // Push secret
        signed_tx.input[0].witness.push(secret_bytes);


        signed_tx.input[0].witness.push(vec!(1));

    } else {

    let signature = generate_p2wsh_signature(
         tx.clone(), 
         txid_index,
         &redeem_script,
         funding_amount,
         EcdsaSighashType::All,
        our_private_key);

    // Convert signature to DER and append SigHashType
    let mut signature_der = signature.serialize_der().to_vec();
    signature_der.push(EcdsaSighashType::All as u8);

    signed_tx.input[0].witness.push(signature_der);

    signed_tx.input[0].witness.push(vec![]);

    }

    // push witness
    signed_tx.input[0]
        .witness
        .push(redeem_script.clone().into_bytes());

    signed_tx
}

fn build_p2wpkh_tx(txin: TxIn, pubkey: PublicKey) -> Transaction {
    let output_script = p2wpkh_output_script(pubkey);
    let output = build_output(405_000, output_script);
    
    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(version,
                      locktime,
                      vec![txin],
                      vec![output]);
    tx
    
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