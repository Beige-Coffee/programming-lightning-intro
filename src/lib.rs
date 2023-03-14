#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
pub mod internal;

use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::blockdata::script::Script;
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::{PublicKey, Block, OutPoint, TxOut, Transaction};
use internal::bitcoind_client::BitcoindClient;
use internal::builder::Builder;
use internal::channel_manager::ChannelManager;

fn p2pkh(pubkey: &PublicKey) -> Script {
    todo!()
}

fn two_of_two_multisig(alice_pubkey: &PublicKey, bob_pubkey: &PublicKey) -> Script {
    todo!()
}

fn cltv_p2pkh(pubkey: &PublicKey, height_or_timestamp: i64) -> Script {
    todo!()
}

fn csv_p2pkh(pubkey: &PublicKey, height_or_timestamp: i64) -> Script {
    todo!()
}

fn payment_channel_funding_output(
    alice_pubkey: &PublicKey,
    bob_pubkey: &PublicKey,
    height: i64,
) -> Script {
    todo!()
}

fn block_connected(funding_output: Script, channel_amount_sats: u64, block: Block) -> bool {
    todo!()
}

fn spend_multisig(alice_signature: Signature, bob_signature: Signature) -> Script {
    todo!()
}

fn spend_refund(alice_pubkey: &PublicKey, alice_signature: Signature) -> Script {
    todo!()
}

fn channel_closed(funding_outpoint: OutPoint, block: Block) -> bool {
    todo!()
}

fn handle_funding_generation_ready(
    channel_manager: &ChannelManager, 
    bitcoind_client: &BitcoindClient, 
    temporary_channel_id: &[u8; 32], 
    counterparty_node_id: &PublicKey, 
    channel_value_satoshis: u64, 
    output_script: Script, 
    user_channel_id: u128) {
        todo!()
    }

#[cfg(test)]
mod test;
