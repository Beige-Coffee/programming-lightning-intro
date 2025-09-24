#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use crate::exercises;
use internal::bitcoind_client::BitcoindClient;
use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::Hash;
use bitcoin::hashes::HashEngine;
use bitcoin::secp256k1;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::{SecretKey, PublicKey, Scalar};
use bitcoin::PublicKey as BitcoinPublicKey;
use bitcoin::script::{ScriptBuf};
use bitcoin::{OutPoint, Sequence, Transaction, TxIn, TxOut, Witness};
use bitcoin::amount::Amount;
use bitcoin::transaction::Version;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::{PubkeyHash};
use bitcoin::{Network};
use bitcoin::consensus::encode::serialize_hex;
use internal::hex_utils;
use bitcoin::consensus::{encode};
use bitcoin::hash_types::Txid;
use std::env;
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::secp256k1::Message;
use bitcoin::sighash::EcdsaSighashType;
use bitcoin::sighash::SighashCache;
use exercises::exercises::{ two_of_two_multisig_witness_script};

pub async fn get_bitcoind_client() -> BitcoindClient {
  let bitcoind = BitcoindClient::new(
      "0.0.0.0".to_string(),
      18443,
      "bitcoind".to_string(),
      "bitcoind".to_string(),
      Network::Regtest,
  )
  .await
  .unwrap();

  bitcoind
}

pub fn get_funding_input(input_tx_id_str: String, vout: usize) -> TxIn {

    // Get an unspent output to spend
    let mut tx_id_bytes = hex::decode(input_tx_id_str).expect("Valid hex string");
    tx_id_bytes.reverse();
    let input_txid = Txid::from_byte_array(tx_id_bytes.try_into().expect("Expected 32 bytes"));

    // Create a transaction spending this UTXO
    TxIn {
        previous_output: OutPoint {
            txid: input_txid,
            vout: vout as u32,
        },
        sequence: Sequence::MAX,
        script_sig: ScriptBuf::new(),
        witness: Witness::new(),
    }

}

pub async fn get_unspent_output(bitcoind: BitcoindClient) -> TxIn {
  let utxos = bitcoind.list_unspent().await;
  let utxo = utxos
      .0
      .iter()
      .find(|utxo| utxo.amount > 4_999_999 && utxo.amount < 6_000_000)
      .expect("No UTXOs with positive balance found");

    let tx_input = TxIn {
        previous_output: OutPoint {
            txid: utxo.txid,
            vout: utxo.vout,
        },
        sequence: Sequence::MAX,
        script_sig: ScriptBuf::new(),
        witness: Witness::new(),
    };

    tx_input
}

pub fn get_htlc_funding_input(input_tx_id_str: String, vout: usize) -> TxIn {

    // Get an unspent output to spend
    let mut tx_id_bytes = hex::decode(input_tx_id_str).expect("Valid hex string");
    tx_id_bytes.reverse();
    let input_txid = Txid::from_byte_array(tx_id_bytes.try_into().expect("Expected 32 bytes"));

    // Create a transaction spending this UTXO
    TxIn {
        previous_output: OutPoint {
            txid: input_txid,
            vout: vout as u32,
        },
        sequence: Sequence(0),
        script_sig: ScriptBuf::new(),
        witness: Witness::new(),
    }

}

pub fn build_unsigned_input(txid: String, vout: u32, sequence: Sequence) -> TxIn {

    // Get an unspent output to spend
    let mut tx_id_bytes = hex::decode(txid).expect("Valid hex string");
    tx_id_bytes.reverse();
    let input_txid = Txid::from_byte_array(tx_id_bytes.try_into().expect("Expected 32 bytes"));

    TxIn {
        previous_output: OutPoint {
            txid: input_txid,
            vout: vout,
        },
        sequence: sequence,
        script_sig: ScriptBuf::new(),
        witness: Witness::new(),
    }
}

pub fn get_arg() -> String {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure the correct number of arguments
    if args.len() < 2 {
        eprintln!("Make sure to include the <txid>!");
        std::process::exit(1);
    }

    // Parse the second argument as txid
    let txid = &args[1];

    // Validate if txid is a valid hex string
    if txid.len() % 2 != 0 {
        eprintln!("Error: txid must be a valid hexadecimal string of even length.");
        std::process::exit(1);
    }

    txid.to_string()
}

pub fn generate_p2wsh_signature(
    transaction: Transaction,
    input_idx: usize,
    witness_script: &ScriptBuf,
    value: u64,
    sighash_type: EcdsaSighashType,
    private_key: secp256k1::SecretKey,
) -> Signature {
    let secp = Secp256k1::new();

    let message =
        generate_p2wsh_message(transaction, input_idx, witness_script, value, sighash_type);
    let signature = secp.sign_ecdsa(&message, &private_key);

    signature
}

fn generate_p2wsh_message(
    transaction: Transaction,
    input_idx: usize,
    witness_script: &ScriptBuf,
    value: u64,
    sighash_type: EcdsaSighashType,
) -> Message {
    let secp = Secp256k1::new();

    let mut cache = SighashCache::new(&transaction);

    let amount = Amount::from_sat(value);

    let sighash = cache
        .p2wsh_signature_hash(input_idx, &witness_script, amount, sighash_type)
        .unwrap();

    let message = Message::from_digest_slice(&sighash[..]).unwrap();

    message
}

pub async fn sign_raw_transaction(bitcoind: BitcoindClient,
                                tx: Transaction) -> Transaction {

  // we need to serialize the tx before passing it into
  //    `sign_raw_transaction_with_wallet`
  let tx_hex = serialize_hex(&tx);

  // sign the transaction
  let signed_tx = bitcoind.sign_raw_transaction_with_wallet(tx_hex).await;

  // convert signed transaction hex into a Transaction type
  let final_tx: Transaction =
      encode::deserialize(&hex_utils::to_vec(&signed_tx.hex).unwrap()).unwrap();

  final_tx
}


pub fn pubkey_multipication_tweak(pubkey1: PublicKey, sha_bytes: [u8; 32]) -> PublicKey {
    let secp = Secp256k1::new();
    pubkey1.mul_tweak(&secp, &Scalar::from_be_bytes(sha_bytes).unwrap()).unwrap()
}

pub fn privkey_multipication_tweak(secret: SecretKey, sha_bytes: [u8; 32]) -> SecretKey {
    secret.mul_tweak(&Scalar::from_be_bytes(sha_bytes).unwrap()).unwrap()
}

pub fn hash_pubkeys(key1: PublicKey, key2: PublicKey) -> [u8; 32] {
    let mut sha = Sha256::engine();

    sha.input(&key1.serialize());
    sha.input(&key2.serialize());

    Sha256::from_engine(sha).to_byte_array()
}

pub fn add_pubkeys(key1: PublicKey, key2: PublicKey) -> PublicKey {
    let pk = key1.combine(&key2).unwrap();

    pk
}

pub fn add_privkeys(key1: SecretKey, key2: SecretKey) -> SecretKey {
    let tweak = Scalar::from_be_bytes(key2.secret_bytes()).unwrap();
    key1.add_tweak(&tweak).unwrap()
}

pub fn secp256k1_private_key(private_key_bytes: &[u8; 32]) -> secp256k1::SecretKey {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(private_key_bytes).unwrap();
    secret_key
}

pub fn pubkey_from_secret(secret: SecretKey) -> PublicKey {
    let secp = Secp256k1::new();
    secp256k1::PublicKey::from_secret_key(&secp, &secret)
}

pub fn pubkey_from_private_key(private_key: &[u8; 32]) -> PublicKey {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(private_key).unwrap();
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
    public_key
}

pub fn bitcoin_pubkey_from_private_key(private_key: &[u8; 32]) -> BitcoinPublicKey {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(private_key).unwrap();
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
    BitcoinPublicKey::new(public_key)
}

pub fn p2wpkh_output_script(public_key: PublicKey) -> ScriptBuf {
    let pubkey = BitcoinPublicKey::new(public_key);
    ScriptBuf::new_p2wpkh(&pubkey.wpubkey_hash().unwrap())
}

pub fn build_output(amount: u64, output_script: ScriptBuf) -> TxOut {

    TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: output_script,
    }
}

pub fn build_transaction(
    version: Version,
    locktime: LockTime,
    tx_ins: Vec<TxIn>,
    tx_outs: Vec<TxOut>,
) -> Transaction {
    Transaction {
        version: version,
        lock_time: locktime,
        input: tx_ins,
        output: tx_outs,
    }
}

pub fn build_htlc_offerer_witness_script(
    revocation_pubkey: &PublicKey,
    remote_htlc_pubkey: &PublicKey,
    local_htlc_pubkey: &PublicKey,
    payment_hash160: &[u8; 20],
) -> ScriptBuf {
    Builder::new()
        .push_opcode(opcodes::OP_DUP)
        .push_opcode(opcodes::OP_HASH160)
        .push_slice(&PubkeyHash::hash(&revocation_pubkey.serialize()))
        .push_opcode(opcodes::OP_EQUAL)
        .push_opcode(opcodes::OP_IF)
        .push_opcode(opcodes::OP_CHECKSIG)
        .push_opcode(opcodes::OP_ELSE)
        .push_slice(&remote_htlc_pubkey.serialize())
        .push_opcode(opcodes::OP_SWAP)
        .push_opcode(opcodes::OP_SIZE)
        .push_int(32)
        .push_opcode(opcodes::OP_EQUAL)
        .push_opcode(opcodes::OP_NOTIF)
        .push_opcode(opcodes::OP_DROP)
        .push_int(2)
        .push_opcode(opcodes::OP_SWAP)
        .push_slice(&local_htlc_pubkey.serialize())
        .push_int(2)
        .push_opcode(opcodes::OP_CHECKMULTISIG)
        .push_opcode(opcodes::OP_ELSE)
        .push_opcode(opcodes::OP_HASH160)
        .push_slice(payment_hash160)
        .push_opcode(opcodes::OP_EQUALVERIFY)
        .push_opcode(opcodes::OP_CHECKSIG)
        .push_opcode(opcodes::OP_ENDIF)
        .push_opcode(opcodes::OP_ENDIF)
        .into_script()
}

pub fn sign_funding_transaction(tx: Transaction,
                                our_funding_public_key: PublicKey,
                                our_funding_private_key: SecretKey,
                                counterparty_funding_public_key: PublicKey,
                                counterparty_funding_private_key: SecretKey,
                               )-> Transaction {

    let funding_amount = 5_000_000;
    let txid_index = 0;

    // Prepare the redeem script for signing (e.g., P2PKH or P2WPKH)
    let redeem_script =
        two_of_two_multisig_witness_script(
            &our_funding_public_key,
            &counterparty_funding_public_key);

    let our_signature = generate_p2wsh_signature(
         tx.clone(), 
         txid_index,
         &redeem_script,
         funding_amount,
         EcdsaSighashType::All,
        our_funding_private_key);

    let counterparty_signature = generate_p2wsh_signature(
         tx.clone(), 
         txid_index,
         &redeem_script,
         funding_amount,
         EcdsaSighashType::All,
    counterparty_funding_private_key);

    // Convert signature to DER and append SigHashType
    let mut our_signature_der = our_signature.serialize_der().to_vec();
    our_signature_der.push(EcdsaSighashType::All as u8);

    let mut counterparty_signature_der = counterparty_signature.serialize_der().to_vec();
    counterparty_signature_der.push(EcdsaSighashType::All as u8);

    // Determine signature order based on pubkey comparison
    let our_sig_first = our_funding_public_key.serialize()[..] > counterparty_funding_public_key.serialize()[..];

    // Add the signature and public key to the witness
    let mut signed_tx = tx.clone();

    // First push empty element for NULLDUMMY compliance
    signed_tx.input[0].witness.push(Vec::new());

    // Push signatures in correct order
    if our_sig_first {
        signed_tx.input[0].witness.push(our_signature_der);
        signed_tx.input[0].witness.push(counterparty_signature_der);
    } else {
        signed_tx.input[0].witness.push(counterparty_signature_der);
        signed_tx.input[0].witness.push(our_signature_der);
    }

    signed_tx.input[0]
        .witness
        .push(redeem_script.clone().into_bytes());

    signed_tx
}