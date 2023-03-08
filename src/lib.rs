#![allow(dead_code, unused_imports, unused_variables)]
use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::blockdata::script::{Builder, Script};
use bitcoin::PublicKey;

fn p2pkh(pubkey: &PublicKey) -> Script {
   todo!()
}

fn two_of_two_multisig(alice_pubkey: &PublicKey, bob_pubkey: &PublicKey) -> Script {
    todo!()
}

fn cltv_p2pkh(pubkey: &PublicKey, height: i64) -> Script {
    todo!()
}

fn csv_p2pkh(pubkey: &PublicKey, timestamp: i64) -> Script {
    todo!()
}

fn payment_channel_funding_output(
    alice_pubkey: &PublicKey,
    bob_pubkey: &PublicKey,
    height: i64,
) -> Script {
    todo!()
}

#[cfg(test)]
mod test;
