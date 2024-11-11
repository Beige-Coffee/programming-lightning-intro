#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use std::sync::Mutex;

use bitcoin::{BlockHash, TxOut};

pub struct BitcoindClient {
    pub create_raw_tx: Mutex<Option<Vec<TxOut>>>,
    pub fund_raw_tx: Mutex<Option<String>>,
    pub sign_raw_tx: Mutex<Option<String>>,
}

impl BitcoindClient {
    pub fn new() -> Self {
        Self {
            create_raw_tx: Mutex::new(None),
            fund_raw_tx: Mutex::new(None),
            sign_raw_tx: Mutex::new(None),
        }
    }

    pub fn create_raw_transaction(&self, outputs: Vec<TxOut>) -> String {
        let mut create_raw_tx = self.create_raw_tx.lock().unwrap();
        *create_raw_tx = Some(outputs);
        "rawtxhex".to_string()
    }

    pub fn fund_raw_transaction(&self, raw_tx: String) -> String {
        let mut fund_raw_tx = self.fund_raw_tx.lock().unwrap();
        *fund_raw_tx = Some(raw_tx);
        "fundedtxhex".to_string()
    }

    pub fn sign_raw_transaction_with_wallet(&self, tx_hex: String) -> String {
        let mut sign_raw_tx = self.sign_raw_tx.lock().unwrap();
        *sign_raw_tx = Some(tx_hex);
        "signedtxhex".to_string()
    }

    pub fn get_new_address(&self) -> String {
        "randomaddress".to_string()
    }
}
