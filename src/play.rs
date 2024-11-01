use bitcoin::blockdata::transaction::{Transaction, TxIn, TxOut};
use bitcoin::blockdata::script::{Builder, Script};
use bitcoin::util::address::Address;
use bitcoin::util::key::PublicKey;
use bitcoin::network::constants::Network;
use bitcoin::hashes::hash160;
use bitcoin::hashes::Hash;
use bitcoin::secp256k1::{Secp256k1, SecretKey};

fn create_redeem_script(pubkeys: Vec<PublicKey>) -> Script {
    // A 2-of-3 multisig script
    Builder::new()
        .push_int(2) // Require 2 signatures
        .push_key(&pubkeys[0])
        .push_key(&pubkeys[1])
        .push_key(&pubkeys[2])
        .push_int(3) // Total of 3 public keys
        .push_opcode(bitcoin::blockdata::opcodes::all::OP_CHECKMULTISIG)
        .into_script()
}

fn create_p2sh_address(redeem_script: &Script, network: Network) -> Address {
    let redeem_script_hash = hash160::Hash::hash(&redeem_script.to_bytes());
    Address::p2sh(&redeem_script_hash, network)
}

fn create_p2sh_transaction(redeem_script: Script, network: Network) {
  // Create the P2SH address
  let p2sh_address = create_p2sh_address(&redeem_script, network);

  // For this example, we'll assume an existing transaction ID (utxo_txid)
  let utxo_txid = bitcoin::Txid::from_hex("your_existing_utxo_txid").unwrap();
  let vout = 0; // The index of the output in the previous transaction

  // Input (funds we're spending)
  let tx_in = TxIn {
      previous_output: bitcoin::OutPoint {
          txid: utxo_txid,
          vout,
      },
      script_sig: Script::new(),  // This will be filled with the redeem script and signatures later
      sequence: 0xFFFFFFFF, // Standard sequence
      witness: vec![], // We will add the witness stack later if needed
  };

  // Output (where we are sending the funds, in this case, back to the P2SH address)
  let tx_out = TxOut {
      value: 100_000,  // Amount of satoshis to send (adjust as needed)
      script_pubkey: p2sh_address.script_pubkey(),
  };

  // Construct the unsigned transaction
  let transaction = Transaction {
      version: 1,
      lock_time: 0,
      input: vec![tx_in],
      output: vec![tx_out],
  };

  // Print the raw transaction for verification
  println!("Unsigned P2SH transaction: {:?}", transaction);
  }

fn main() {
    // Set up the secp256k1 context
    let secp = Secp256k1::new();

    // Generate three random public keys (normally you'd derive these from actual private keys)
    let priv_key1 = SecretKey::new(&mut secp256k1::rand::thread_rng());
    let priv_key2 = SecretKey::new(&mut secp256k1::rand::thread_rng());
    let priv_key3 = SecretKey::new(&mut secp256k1::rand::thread_rng());

    let pub_key1 = PublicKey::from_secret_key(&secp, &priv_key1);
    let pub_key2 = PublicKey::from_secret_key(&secp, &priv_key2);
    let pub_key3 = PublicKey::from_secret_key(&secp, &priv_key3);

    let pubkeys = vec![pub_key1, pub_key2, pub_key3];

    // Create the 2-of-3 multisig redeem script
    let redeem_script = create_redeem_script(pubkeys);

    // Create and print the P2SH transaction on Bitcoin testnet
    create_p2sh_transaction(redeem_script, Network::Testnet);
}