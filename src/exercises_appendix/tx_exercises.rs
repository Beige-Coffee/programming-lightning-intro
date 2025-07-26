#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use bitcoin::locktime::absolute::LockTime;
use bitcoin::secp256k1::{PublicKey};
use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::Hash;
use bitcoin::hashes::HashEngine;
use bitcoin::script::{ScriptBuf};
use bitcoin::{OutPoint, Sequence, Transaction, TxIn, TxOut, Witness};

fn extract_lower_48_bits(input: [u8; 32]) -> u64 {
  ((input[26] as u64) << 5 * 8)
    | ((input[27] as u64) << 4 * 8)
    | ((input[28] as u64) << 3 * 8)
    | ((input[29] as u64) << 2 * 8)
    | ((input[30] as u64) << 1 * 8)
    | ((input[31] as u64) << 0 * 8)
}

pub fn get_commitment_transaction_number_obscure_factor(
  channel_open_payment_basepoint: &PublicKey, channel_accept_payment_basepoint: &PublicKey,
) -> u64 {
  let mut sha = Sha256::engine();

    sha.input(&channel_open_payment_basepoint.serialize());
    sha.input(&channel_accept_payment_basepoint.serialize());

  let res = Sha256::from_engine(sha).to_byte_array();

  extract_lower_48_bits(res)
}

pub fn build_commitment_input(
  funding_outpoint: OutPoint,
  commitment_transaction_number_obscure_factor: &u64,
  commitment_number: &u64,
) -> TxIn {

  let obscured_commitment_transaction_number = 
    commitment_transaction_number_obscure_factor ^ commitment_number;
  
  TxIn {
    previous_output: funding_outpoint,
    script_sig: ScriptBuf::new(),
    sequence: Sequence(((0x80 as u32) << 8 * 3)
      | ((obscured_commitment_transaction_number >> 3 * 8) as u32)),
    witness: Witness::new(),
  }
  
}

pub fn build_commitment_locktime(
  commitment_transaction_number_obscure_factor: &u64,
  commitment_number: &u64,
) -> LockTime {

  let obscured_commitment_transaction_number = 
    commitment_transaction_number_obscure_factor ^ commitment_number;

  LockTime::from_consensus(((0x20 as u32) << 8 * 3) | ((obscured_commitment_transaction_number & 0xffffffu64) as u32))

}