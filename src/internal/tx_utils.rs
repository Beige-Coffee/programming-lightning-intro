
use bitcoin::{TxIn, TxOut, Transaction, OutPoint, Sequence, Txid};
use bitcoin::amount::Amount;
use bitcoin::transaction::Version;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::script::ScriptBuf;

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
        version,
        lock_time: locktime,
        input: tx_ins,
        output: tx_outs,
    }
}

use bitcoin::Witness;

pub fn get_funding_input(input_tx_id_str: String, vout: usize) -> TxIn {
    build_unsigned_input(input_tx_id_str, vout as u32, Sequence::MAX)
}

pub fn get_htlc_funding_input(input_tx_id_str: String, vout: usize) -> TxIn {
    build_unsigned_input(input_tx_id_str, vout as u32, Sequence(0))
}

pub fn build_unsigned_input(txid: String, vout: u32, sequence: Sequence) -> TxIn {
    let mut tx_id_bytes = hex::decode(txid).expect("Valid hex string");
    tx_id_bytes.reverse();
    let input_txid = Txid::new(tx_id_bytes.try_into().expect("Expected 32 bytes"));

    TxIn {
        previous_output: OutPoint {
            txid: input_txid,
            vout,
        },
        sequence,
        script_sig: ScriptBuf::new(),
        witness: Witness::new(),
    }
}
