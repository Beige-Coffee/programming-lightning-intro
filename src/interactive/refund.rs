#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use crate::exercises;
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::secp256k1::{PublicKey, SecretKey};
use lightning::chain::chaininterface::{BroadcasterInterface,};
use exercises::exercises::{build_refund_transaction, };
use internal::bitcoind_client::BitcoindClient;
use internal::helper::{pubkey_from_private_key, secp256k1_private_key, get_bitcoind_client, get_unspent_output, generate_p2wsh_signature, sign_raw_transaction, get_funding_input, sign_funding_transaction};
use std::time::Duration;
use tokio::time::sleep;


pub struct KeyManager{
    pub funding_private_key: SecretKey,
    pub funding_public_key: PublicKey,
    pub commitment_pubkey: PublicKey,
}

pub async fn create_broadcast_funding_tx(bitcoind: BitcoindClient,
                                        txid: String,
                                        our_key_manager: KeyManager,
                                        counterparty_key_manager: KeyManager,
                                        funding_amount: u64,
                                        our_balance: u64,
                                        counterparty_balance: u64) {

    let txid_index = 0;
    let funding_txin = get_funding_input(txid.to_string(), txid_index);

    let tx = build_refund_transaction(
        funding_txin,
        our_key_manager.commitment_pubkey,
        counterparty_key_manager.commitment_pubkey,
        our_balance,
        counterparty_balance);

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

    // Broadcast it
    bitcoind.broadcast_transactions(&[&signed_tx]);
}


pub async fn run(funding_txid: String) {

    // Parse the argument as txid
    let txid = funding_txid;

    // get bitcoin client
    let bitcoind = get_bitcoind_client().await;

    // Get our keys
    let our_funding_private_key = secp256k1_private_key(&[0x01; 32]);
    let our_funding_public_key = pubkey_from_private_key(&[0x01; 32]);
    let our_commitment_pubkey = pubkey_from_private_key(&[0x11; 32]);

    let our_key_manager = KeyManager{
            funding_private_key: our_funding_private_key,
            funding_public_key: our_funding_public_key,
            commitment_pubkey: our_commitment_pubkey
        };

    // Get our Counterparty Pubkey
    let counterparty_funding_private_key = secp256k1_private_key(&[0x02; 32]);
    let counterparty_funding_public_key = pubkey_from_private_key(&[0x02; 32]);
    let counterparty_commitment_pubkey = pubkey_from_private_key(&[0x21; 32]);

    let counterparty_key_manager = KeyManager{
            funding_private_key: counterparty_funding_private_key,
            funding_public_key: counterparty_funding_public_key,
            commitment_pubkey: counterparty_commitment_pubkey,
        };
    
    let funding_amount = 5_000_000;
    let our_balance = 4_999_500;
    let counterparty_balance = 500;
    
    create_broadcast_funding_tx(bitcoind, txid.clone(), our_key_manager, counterparty_key_manager, funding_amount,
                               our_balance, counterparty_balance).await;

    // Add a delay to allow the spawned task to complete
    sleep(Duration::from_secs(2)).await;
}