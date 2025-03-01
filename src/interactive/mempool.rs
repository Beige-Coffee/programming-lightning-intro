#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::interactive::helper;
use crate::internal;
use crate::ch1_intro_htlcs;
use base64;
use bitcoin::address::Address;
use bitcoin::amount::Amount;
use bitcoin::blockdata::block::Block;
use bitcoin::blockdata::block::Header;
use bitcoin::blockdata::constants::WITNESS_SCALE_FACTOR;
use bitcoin::blockdata::script::ScriptBuf;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::consensus::{encode, Decodable, Encodable};
use bitcoin::hash_types::{BlockHash, Txid};
use bitcoin::hashes::Hash;
use bitcoin::key::XOnlyPublicKey;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::transaction::Version;
use bitcoin::{Network, OutPoint, Sequence, TxIn, TxOut, WPubkeyHash, Witness};
use lightning::chain::chaininterface::{BroadcasterInterface, ConfirmationTarget, FeeEstimator};
use lightning::chain::transaction::TransactionData;
use lightning::events::bump_transaction::{Utxo, WalletSource};
use lightning::log_error;
use lightning::routing::scoring::{ProbabilisticScorer, ProbabilisticScoringDecayParameters};
use lightning::sign::ChangeDestinationSource;
use lightning::util::logger::Logger;
use lightning_block_sync::http::HttpEndpoint;
use lightning_block_sync::http::JsonResponse;
use lightning_block_sync::init::validate_best_block_header;
use lightning_block_sync::poll;
use lightning_block_sync::poll::ChainPoller;
use lightning_block_sync::poll::ChainTip;
use lightning_block_sync::rpc::RpcClient;
use lightning_block_sync::SpvClient;
use lightning_block_sync::{AsyncBlockSourceResult, BlockData, BlockHeaderData, BlockSource};
use internal::bitcoind_client;
use internal::bitcoind_client::BitcoindClient;
use ch1_intro_htlcs::solutions::build_funding_transaction;
use internal::helper::{pubkey_from_private_key, secp256k1_private_key, build_output, build_transaction, p2wpkh_output_script,};
use internal::convert;
use internal::convert::{BlockchainInfo, ListUnspentUtxo, SignedTx};
use internal::hex_utils;
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use bitcoin::PublicKey;
use helper::{get_bitcoind_client, get_unspent_output, sign_raw_transaction};
use clap::{ValueEnum};
use internal::builder::Builder;
use bitcoin::blockdata::opcodes::all as opcodes;

// Define the enum for Mempool's command types
#[derive(ValueEnum, Clone, Debug)]
pub enum MempoolCommand {
    #[value(name = "nonstandard")]
    NonStandard,
    #[value(name = "consensus")]
    Consensus,
    #[value(name = "policy")]
    Policy,
}

pub async fn build_funding_tx(bitcoind: BitcoindClient,
                                        tx_input: TxIn,
                                        tx_in_amount: u64,
                                        mempool_command: MempoolCommand) {

    let our_public_key = pubkey_from_private_key(&[0x01; 32]);


    let output1 = match mempool_command {
        MempoolCommand::NonStandard => {

            let output_script = build_non_standard_output2();

            build_output(5_000_000, output_script)
            
        },

        MempoolCommand::Consensus => {
            let output_script = p2wpkh_output_script(our_public_key);

            build_output(5_500_000, output_script)
        },

        MempoolCommand::Policy => {
            let output_script = p2wpkh_output_script(our_public_key);

            build_output(100, output_script)
        }

    };

    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(version,
                      locktime,
                      vec![tx_input],
                      vec![output1]);

    let signed_tx = sign_raw_transaction(bitcoind.clone(), tx).await;

    println!("\n");
    println!("Tx ID: {}", signed_tx.compute_txid());
    println!("\n");
    println!("Tx Hex: {}", serialize_hex(&signed_tx));
}

pub async fn run(mempool_command: MempoolCommand) {

    // get bitcoin client
    let bitcoind = get_bitcoind_client().await;

    // get an unspent output for funding transaction
    let tx_input = get_unspent_output(bitcoind.clone()).await;

    let tx_in_amount = 5_000_000;
    
        build_funding_tx(bitcoind, tx_input, tx_in_amount, mempool_command).await;

    // Add a delay to allow the spawned task to complete
    sleep(Duration::from_secs(2)).await;
}

fn build_non_standard_output() -> ScriptBuf {
    
    let pubkey1 = pubkey_from_private_key(&[0x01; 32]);
    let pubkey2 = pubkey_from_private_key(&[0x02; 32]);
    let pubkey3 = pubkey_from_private_key(&[0x03; 32]);
    let pubkey4 = pubkey_from_private_key(&[0x04; 32]);
    let pubkey5 = pubkey_from_private_key(&[0x05; 32]);
    let pubkey6 = pubkey_from_private_key(&[0x06; 32]);
    let pubkey7 = pubkey_from_private_key(&[0x07; 32]);
    let pubkey8 = pubkey_from_private_key(&[0x08; 32]);
    let pubkey9 = pubkey_from_private_key(&[0x09; 32]);
    let pubkey10 = pubkey_from_private_key(&[0x10; 32]);
    let pubkey11 = pubkey_from_private_key(&[0x1; 32]);
    let pubkey12 = pubkey_from_private_key(&[0x12; 32]);
    let pubkey13 = pubkey_from_private_key(&[0x13; 32]);
    let pubkey14 = pubkey_from_private_key(&[0x14; 32]);
    let pubkey15 = pubkey_from_private_key(&[0x15; 32]);
    let pubkey16 = pubkey_from_private_key(&[0x16; 32]);
    let pubkey17 = pubkey_from_private_key(&[0x17; 32]);
    let pubkey18 = pubkey_from_private_key(&[0x18; 32]);
    let pubkey19 = pubkey_from_private_key(&[0x19; 32]);
    let pubkey20 = pubkey_from_private_key(&[0x20; 32]);
    
    Builder::new()
        .push_int(15)
    
        .push_key(&pubkey1)
        .push_key(&pubkey2)
        .push_key(&pubkey3)
        .push_key(&pubkey4)
        .push_key(&pubkey5)
        .push_key(&pubkey6)
    
        .push_key(&pubkey7)
        .push_key(&pubkey8)
        .push_key(&pubkey9)
        .push_key(&pubkey10)
        .push_key(&pubkey11)
        .push_key(&pubkey12)

        .push_key(&pubkey13)
        .push_key(&pubkey14)
        .push_key(&pubkey15)
        .push_key(&pubkey16)
        .push_key(&pubkey17)
        .push_key(&pubkey18)
        .push_key(&pubkey19)
        .push_key(&pubkey20)
    
        .push_int(20)
    
        .push_opcode(opcodes::OP_CHECKMULTISIG)
        .into_script()
}


fn build_non_standard_output2() -> ScriptBuf {

    let pubkey1 = pubkey_from_private_key(&[0x01; 32]);
    let pubkey2 = pubkey_from_private_key(&[0x02; 32]);
    let pubkey3 = pubkey_from_private_key(&[0x03; 32]);
    let pubkey4 = pubkey_from_private_key(&[0x04; 32]);

    Builder::new()
        .push_int(3)

        .push_key(&pubkey1)
        .push_key(&pubkey2)
        .push_key(&pubkey3)
        .push_key(&pubkey4)

        .push_int(4)

        .push_opcode(opcodes::OP_CHECKMULTISIG)
        .into_script()
}