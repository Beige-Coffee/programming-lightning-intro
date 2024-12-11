use pl_00_intro::internal::bitcoind_client::BitcoindClient;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::consensus::encode::serialize_hex;
use pl_00_intro::internal::hex_utils;
use bitcoin::{Network};
use bitcoin::blockdata::script::ScriptBuf;
use bitcoin::consensus::{encode};
use bitcoin::{OutPoint, Sequence, TxIn, Witness};
use bitcoin::hash_types::Txid;
use bitcoin::hashes::Hash;
use std::env;
use bitcoin::secp256k1;
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::secp256k1::Message;
use bitcoin::sighash::EcdsaSighashType;
use bitcoin::sighash::SighashCache;
use bitcoin::secp256k1::Scalar;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::amount::Amount;

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
