#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use bitcoin::blockdata::script::ScriptBuf;
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::transaction::Version;
use bitcoin::{TxIn};
use internal::bitcoind_client::{BitcoindClient, get_bitcoind_client};
use internal::key_utils::{add_pubkeys, pubkey_multipication_tweak, pubkey_from_secret, add_privkeys, privkey_multipication_tweak, hash_pubkeys,
      pubkey_from_private_key, secp256k1_private_key};
use internal::tx_utils::{build_output,get_unspent_output, build_transaction, get_funding_input, get_htlc_funding_input};
use internal::script_utils::{build_htlc_offerer_witness_script, p2wpkh_output_script};
use internal::sign_utils::{sign_raw_transaction, sign_funding_transaction, generate_p2wsh_signature};
use std::time::Duration;
use tokio::time::sleep;
use clap::{ValueEnum};
use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::script::{Builder};

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


    let outputs = match mempool_command {
        MempoolCommand::NonStandard => {

            let output_script = build_non_standard_output2();

            let output1 = build_output(5_000_000, output_script);

            vec![output1]
            
        },

        MempoolCommand::Consensus => {
            let output_script = p2wpkh_output_script(our_public_key);

            let output1 = build_output(5_500_000, output_script);
            
            vec![output1]
        },

        MempoolCommand::Policy => {
            let output_script = p2wpkh_output_script(our_public_key);

            let output1 = build_output(4_900_000, output_script.clone());
            let output2 = build_output(100, output_script.clone());

            vec![output1, output2]
        }

    };

    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(version,
                      locktime,
                      vec![tx_input],
                      outputs);

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