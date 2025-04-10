#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use crate::exercises;
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::{TxIn};
use internal::bitcoind_client::BitcoindClient;
use exercises::exercises::build_funding_transaction;
use internal::helper::{get_bitcoind_client, pubkey_from_private_key, get_unspent_output, sign_raw_transaction};
use std::time::Duration;
use tokio::time::sleep;

pub async fn build_funding_tx(bitcoind: BitcoindClient,
                                        tx_input: TxIn,
                                        tx_in_amount: u64) {

    // we're locking to a 2-of-2 multisig, so we need two public keys
    // normally, we would generate our own public key
    //   and the counterparty would send us theirs
    let our_public_key = pubkey_from_private_key(&[0x01; 32]);
    let counterparty_pubkey = pubkey_from_private_key(&[0x02; 32]);

    // build funding transaction using the function we created
    let tx = build_funding_transaction(
            vec![tx_input],
            &our_public_key,
            &counterparty_pubkey,
            tx_in_amount,
        );

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

