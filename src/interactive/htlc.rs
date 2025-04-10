#![allow(dead_code,unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use crate::exercises;
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::secp256k1::{PublicKey, SecretKey};
use exercises::exercises::{
    build_htlc_commitment_transaction
};
use internal::bitcoind_client::BitcoindClient;
use internal::helper::{pubkey_from_private_key, secp256k1_private_key,get_bitcoind_client, get_funding_input, sign_funding_transaction};
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
    pub delayed_pubkey: PublicKey,
    pub pubkey: PublicKey,
    pub revocation_pubkey: PublicKey,
}

pub async fn create_broadcast_funding_tx(bitcoind: BitcoindClient,
                                         txid: String,
                                        our_key_manager: KeyManager,
                                        counterparty_key_manager: KeyManager) {

    let txid_index = 0;
    let funding_txin = get_funding_input(txid.to_string(), txid_index);
    let funding_amount = 5_000_000;

    let payment_hash160 = HASH160_DUMMY;
    let to_self_delay: i64 = 144;

    let htlc_amount = 400_000;
    let local_amount = 2_600_000;
    let remote_amount = 2_000_000;


    let tx = build_htlc_commitment_transaction(
        funding_txin,
        &our_key_manager.revocation_pubkey,
        &counterparty_key_manager.htlc_pubkey,
        &our_key_manager.htlc_pubkey,
        &our_key_manager.delayed_pubkey,
        counterparty_key_manager.pubkey,
        to_self_delay,
        &payment_hash160,
        htlc_amount,
        local_amount,
        remote_amount);

    let signed_tx = sign_funding_transaction(tx,
        our_key_manager.funding_public_key,
        our_key_manager.funding_private_key,
        counterparty_key_manager.funding_public_key,
        our_key_manager.funding_private_key,
       );

    println!("\n");
    println!("Tx ID: {}", signed_tx.compute_txid());
    println!("\n");
    println!("Tx Hex: {}", serialize_hex(&signed_tx));

}

pub async fn run(funding_txid: String) {

    // get bitcoin client
    let bitcoind = get_bitcoind_client().await;

    // Parse the argument as txid
    let txid = funding_txid;

    // Get our keys
    let our_funding_private_key = secp256k1_private_key(&[0x01; 32]);
    let our_funding_public_key = pubkey_from_private_key(&[0x01; 32]);
    let local_htlc_pubkey = pubkey_from_private_key(&[0x11; 32]);
    let revocation_pubkey = pubkey_from_private_key(&[0x12; 32]);
    let to_local_delayed_pubkey = pubkey_from_private_key(&[0x13; 32]);
    let local_pubkey = pubkey_from_private_key(&[0x14; 32]);

    let our_key_manager = KeyManager{
            funding_private_key: our_funding_private_key,
            funding_public_key: our_funding_public_key,
            htlc_pubkey: local_htlc_pubkey,
            delayed_pubkey: to_local_delayed_pubkey,
            pubkey: local_pubkey,
            revocation_pubkey: revocation_pubkey,
        };

    // Get our Counterparty Pubkey
    let counterparty_funding_private_key = secp256k1_private_key(&[0x02; 32]);
    let counterparty_funding_public_key = pubkey_from_private_key(&[0x02; 32]);
    let counterparty_htlc_pubkey = pubkey_from_private_key(&[0x21; 32]);
    let counterparty_pubkey = pubkey_from_private_key(&[0x22; 32]);
    let counterparty_delayed_key = pubkey_from_private_key(&[0x23; 32]);
    let counterparty_revocation_key = pubkey_from_private_key(&[0x24; 32]);

    let counterparty_key_manager = KeyManager{
            funding_private_key: counterparty_funding_private_key,
            funding_public_key: counterparty_funding_public_key,
            htlc_pubkey: counterparty_htlc_pubkey,
            delayed_pubkey: counterparty_delayed_key,
            pubkey: counterparty_pubkey,
            revocation_pubkey: counterparty_revocation_key,
        };

    create_broadcast_funding_tx(bitcoind, txid, our_key_manager, counterparty_key_manager).await;

    // Add a delay to allow the spawned task to complete
    sleep(Duration::from_secs(2)).await;
}
