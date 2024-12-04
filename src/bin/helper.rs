use pl_00_intro::internal::bitcoind_client::BitcoindClient;
use pl_00_intro::internal::convert::{ListUnspentUtxo};
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::consensus::encode::serialize_hex;
use pl_00_intro::internal::hex_utils;
use bitcoin::{Network};
use bitcoin::blockdata::script::ScriptBuf;
use bitcoin::consensus::{encode};
use bitcoin::{OutPoint, Sequence, TxIn, Witness};
use bitcoin::hash_types::Txid;
use bitcoin::hashes::Hash;

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

pub async fn get_unspent_output(bitcoind: BitcoindClient) -> ListUnspentUtxo {
  let utxos = bitcoind.list_unspent().await;
  let utxo = utxos
      .0
      .iter()
      .find(|utxo| utxo.amount > 4_999_999 && utxo.amount < 6_000_000)
      .expect("No UTXOs with positive balance found");

  utxo.clone()
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
        sequence: Sequence::from_height(14),
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

