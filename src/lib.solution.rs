#![allow(dead_code, unused_imports, unused_variables)]
pub mod internal;

use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::blockdata::script::Script;
use bitcoin::PublicKey;
use internal::builder::Builder;

fn p2pkh(pubkey: &PublicKey) -> Script {
    Builder::new()
    .push_opcode(opcodes::OP_DUP)
    .push_opcode(opcodes::OP_HASH160)
    .push_pubkey_hash(pubkey)
    .push_opcode(opcodes::OP_EQUALVERIFY)
    .push_opcode(opcodes::OP_CHECKSIG)
    .into_script()
}

fn two_of_two_multisig(alice_pubkey: &PublicKey, bob_pubkey: &PublicKey) -> Script {
    Builder::new()
    .push_int(2)
    .push_key(alice_pubkey)
    .push_key(bob_pubkey)
    .push_int(2)
    .push_opcode(opcodes::OP_CHECKMULTISIG)
    .into_script()
}

fn cltv_p2pkh(pubkey: &PublicKey, height_or_timestamp: i64) -> Script {
    Builder::new()
    .push_int(height_or_timestamp)
    .push_opcode(opcodes::OP_CLTV)
    .push_opcode(opcodes::OP_DROP)
    .push_opcode(opcodes::OP_DUP)
    .push_opcode(opcodes::OP_HASH160)
    .push_pubkey_hash(pubkey)
    .push_opcode(opcodes::OP_EQUALVERIFY)
    .push_opcode(opcodes::OP_CHECKSIG)
    .into_script()
}

fn csv_p2pkh(pubkey: &PublicKey, height_or_timestamp: i64) -> Script {
    Builder::new()
    .push_int(height_or_timestamp)
    .push_opcode(opcodes::OP_CSV)
    .push_opcode(opcodes::OP_DROP)
    .push_opcode(opcodes::OP_DUP)
    .push_opcode(opcodes::OP_HASH160)
    .push_pubkey_hash(pubkey)
    .push_opcode(opcodes::OP_EQUALVERIFY)
    .push_opcode(opcodes::OP_CHECKSIG)
    .into_script()
}

fn payment_channel_funding_output(
    alice_pubkey: &PublicKey,
    bob_pubkey: &PublicKey,
    height: i64,
) -> Script {
    Builder::new()
    .push_opcode(opcodes::OP_IF)
    .push_script(two_of_two_multisig(alice_pubkey, bob_pubkey))
    .push_opcode(opcodes::OP_ELSE)
    .push_script(csv_p2pkh(alice_pubkey, height))
    .push_opcode(opcodes::OP_ENDIF)
    .into_script()
}

fn block_connected(funding_output: Script, channel_amount_sats: u64, block: Block) -> bool {
  for tx in block.txdata {
      for output in tx.output {
          if output.script_pubkey == funding_output && output.value == channel_amount_sats {
              return true;
          }
      }
  }
  false
}

fn spend_multisig(alice_signature: Signature, bob_signature: Signature) -> Script {
  Builder::new()
      .push_int(0)
      .push_signature(alice_signature)
      .push_signature(bob_signature)
  .into_script()
}

fn spend_refund(alice_signature: Signature) -> Script {
  Builder::new()
      .push_int(1)
      .push_signature(alice_signature)
  .into_script()
}

#[cfg(test)]
mod test;
