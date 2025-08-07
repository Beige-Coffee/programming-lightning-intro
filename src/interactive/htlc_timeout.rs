#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use crate::exercises;
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::secp256k1::{PublicKey, SecretKey};
use exercises::exercises::{build_htlc_timeout_transaction,
};
use bitcoin::sighash::EcdsaSighashType;
use bitcoin::PublicKey as BitcoinPubKey;
use internal::bitcoind_client::{BitcoindClient, get_bitcoind_client};
use internal::key_utils::{add_pubkeys, pubkey_multipication_tweak, pubkey_from_secret, add_privkeys, privkey_multipication_tweak, hash_pubkeys,
      pubkey_from_private_key, secp256k1_private_key, bitcoin_pubkey_from_private_key};
use internal::tx_utils::{build_output,get_unspent_output, build_transaction, get_funding_input, get_htlc_funding_input};
use internal::script_utils::{build_htlc_offerer_witness_script, p2wpkh_output_script};
use internal::sign_utils::{sign_raw_transaction, sign_funding_transaction, generate_p2wsh_signature};
use std::time::Duration;
use tokio::time::sleep;

/// hash160 of the empty string
const HASH160_DUMMY: [u8; 20] = [
    0xb4, 0x72, 0xa2, 0x66, 0xd0, 0xbd, 0x89, 0xc1, 0x37, 0x06, 0xa4, 0x13, 0x2c, 0xcf, 0xb1, 0x6f,
    0x7c, 0x3b, 0x9f, 0xcb,
];

pub struct KeyManager{
    pub funding_private_key: SecretKey,
    pub funding_public_key: PublicKey,
    pub htlc_pubkey: PublicKey,
    pub htlc_private_key: SecretKey,
    pub delayed_pubkey: PublicKey,
    pub pubkey: BitcoinPubKey,
    pub revocation_pubkey: PublicKey,
}

pub async fn create_broadcast_funding_tx(bitcoind: BitcoindClient,
                                         txid: String,
                                        our_key_manager: KeyManager,
                                        counterparty_key_manager: KeyManager) {

    let txid_index = 2;
    let funding_txin = get_htlc_funding_input(txid.to_string(), txid_index);
    let funding_amount = 400_000;

    let payment_hash160 = HASH160_DUMMY;
    let to_self_delay: i64 = 144;
    let cltv_expiry: u32 = 300;
    let htlc_amount = 400_000;


    let tx = build_htlc_timeout_transaction(
        funding_txin,
        &our_key_manager.revocation_pubkey,
        &our_key_manager.delayed_pubkey,
        to_self_delay,
        cltv_expiry,
        htlc_amount
        );

    // Prepare the redeem script for signing (e.g., P2PKH or P2WPKH)
    let redeem_script =
        build_htlc_offerer_witness_script(
            &our_key_manager.revocation_pubkey,
            &counterparty_key_manager.htlc_pubkey,
            &our_key_manager.htlc_pubkey,
            &payment_hash160);

    let our_signature = generate_p2wsh_signature(
         tx.clone(), 
         0,
         &redeem_script,
         funding_amount,
         EcdsaSighashType::All,
        our_key_manager.htlc_private_key);

    let counterparty_signature = generate_p2wsh_signature(
         tx.clone(), 
         0,
         &redeem_script,
         funding_amount,
         EcdsaSighashType::All,
        counterparty_key_manager.htlc_private_key);

    // Convert signature to DER and append SigHashType
    let mut our_signature_der = our_signature.serialize_der().to_vec();
    our_signature_der.push(EcdsaSighashType::All as u8);

    let mut counterparty_signature_der = counterparty_signature.serialize_der().to_vec();
    counterparty_signature_der.push(EcdsaSighashType::All as u8);

    // Determine signature order based on pubkey comparison
    let our_sig_first = our_key_manager.funding_public_key.serialize()[..] > counterparty_key_manager.funding_public_key.serialize()[..];

    // Add the signature and public key to the witness
    let mut signed_tx = tx.clone();

    // First push empty element for NULLDUMMY compliance
    signed_tx.input[0].witness.push(Vec::new());


    signed_tx.input[0].witness.push(counterparty_signature_der);
    signed_tx.input[0].witness.push(our_signature_der);

    signed_tx.input[0].witness.push(vec![]);

    signed_tx.input[0]
        .witness
        .push(redeem_script.clone().into_bytes());

    println!("\n");
    println!("Tx ID: {}", signed_tx.compute_txid());
    println!("\n");
    println!("Tx Hex: {}", serialize_hex(&signed_tx));

    // Broadcast it
    //bitcoind.broadcast_transactions(&[&signed_tx]);
}

pub async fn run(htlc_txid: String) {

    // get bitcoin client
    let bitcoind = get_bitcoind_client().await;

    // Parse the argument as txid
    let txid = htlc_txid;

    // Get our keys
    let our_funding_private_key = secp256k1_private_key(&[0x01; 32]);
    let our_funding_public_key = pubkey_from_private_key(&[0x01; 32]);
    let local_htlc_pubkey = pubkey_from_private_key(&[0x11; 32]);
    let local_htlc_private_key = secp256k1_private_key(&[0x11; 32]);
    let revocation_pubkey = pubkey_from_private_key(&[0x12; 32]);
    let to_local_delayed_pubkey = pubkey_from_private_key(&[0x13; 32]);
    let local_pubkey = bitcoin_pubkey_from_private_key(&[0x14; 32]);

    let our_key_manager = KeyManager{
            funding_private_key: our_funding_private_key,
            funding_public_key: our_funding_public_key,
            htlc_pubkey: local_htlc_pubkey,
            htlc_private_key: local_htlc_private_key,
            delayed_pubkey: to_local_delayed_pubkey,
            pubkey: local_pubkey,
            revocation_pubkey: revocation_pubkey,
        };

    // Get our Counterparty Pubkey
    let counterparty_funding_private_key = secp256k1_private_key(&[0x02; 32]);
    let counterparty_funding_public_key = pubkey_from_private_key(&[0x02; 32]);
    let counterparty_htlc_pubkey = pubkey_from_private_key(&[0x21; 32]);
    let counterparty_htlc_private_key = secp256k1_private_key(&[0x21; 32]);
    let counterparty_pubkey = bitcoin_pubkey_from_private_key(&[0x22; 32]);
    let counterparty_delayed_key = pubkey_from_private_key(&[0x23; 32]);
    let counterparty_revocation_key = pubkey_from_private_key(&[0x24; 32]);

    let counterparty_key_manager = KeyManager{
            funding_private_key: counterparty_funding_private_key,
            funding_public_key: counterparty_funding_public_key,
            htlc_pubkey: counterparty_htlc_pubkey,
            htlc_private_key: counterparty_htlc_private_key,
            delayed_pubkey: counterparty_delayed_key,
            pubkey: counterparty_pubkey,
            revocation_pubkey: counterparty_revocation_key,
        };

    create_broadcast_funding_tx(bitcoind, txid, our_key_manager, counterparty_key_manager).await;

    // Add a delay to allow the spawned task to complete
    sleep(Duration::from_secs(2)).await;
}
