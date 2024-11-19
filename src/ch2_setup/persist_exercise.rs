#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use lightning::util::persist::KVStore;
use bitcoin::script::{ScriptBuf};
use lightning::ln::channel_keys::{RevocationBasepoint};
use bitcoin::transaction::{OutPoint, Transaction};
use bitcoin::hash_types::{Txid};
use lightning::ln::types::ChannelId;
use std::collections::HashMap;
use lightning::io::{Result, Error};
use std::sync::RwLock;

pub struct SimpleStore {
  data: RwLock<HashMap<String, Vec<u8>>>,
}

impl SimpleStore {
    pub fn new() -> Self {
        SimpleStore {
            data: RwLock::new(HashMap::new())
        }
    }
}

pub struct SimpleChannelMonitor {
  // Tracks the version/sequence number of monitor updates
  latest_update_id: u64,
  // Script used to send funds back to us when channel closes
  destination_script: ScriptBuf,
  // Script used to send funds to counterparty when channel closes
  counterparty_payment_script: ScriptBuf,
  // Unique identifier for deriving channel-specific keys
  channel_keys_id: [u8; 32],
  // Base point for generating revocation keys (used to punish cheating)
  holder_revocation_basepoint: RevocationBasepoint,
  // Unique identifier for this channel
  channel_id: ChannelId,
  // The transaction output that funded this channel and its script
  funding_info: (OutPoint, ScriptBuf),
  // Current commitment transaction from counterparty (None if not yet received)
  current_counterparty_commitment_txid: Option<Txid>,
  // Previous commitment transaction from counterparty (for revocation)
  prev_counterparty_commitment_txid: Option<Txid>,
  // Script that controls the funding output (2-of-2 multisig)
  funding_redeemscript: ScriptBuf,
  // Total value of the channel in satoshis
  channel_value_satoshis: u64,
}

impl KVStore for SimpleStore {
  fn write(
    & self,
    primary_namespace: &str,
    secondary_namespace: &str,
    key: &str,
    buf: &[u8],
  ) -> Result<()> {
    // Combine namespaces and key into a single string
    let full_key = if secondary_namespace.is_empty() {
        format!("{}/{}", primary_namespace, key)
    } else {
        format!("{}/{}/{}", primary_namespace, secondary_namespace, key)
    };
  
    // Get write access to the HashMap
    let mut data = self.data.write().unwrap();
    data.insert(full_key, buf.to_vec());
    Ok(())
  }

  fn read(
    &self, primary_namespace: &str, secondary_namespace: &str, key: &str,
  ) -> Result<Vec<u8>> {
    // Combine namespaces into full key
    let full_key = if secondary_namespace.is_empty() {
        format!("{}/{}", primary_namespace, key)
    } else {
        format!("{}/{}/{}", primary_namespace, secondary_namespace, key)
    };

    // Get read access to the HashMap
    let data = self.data.read().unwrap();

    // Clone the value if it exists
     Ok(data.get(&full_key).cloned().expect("Key exists"))
    }

  fn remove(
      &self, primary_namespace: &str, secondary_namespace: &str, key: &str, lazy: bool,
    ) -> Result<()>{
    // Combine namespaces into full key
    let full_key = if secondary_namespace.is_empty() {
        format!("{}/{}", primary_namespace, key)
    } else {
        format!("{}/{}/{}", primary_namespace, secondary_namespace, key)
    };

    // Get write access to remove the key
    let mut data = self.data.write().unwrap();
    data.remove(&full_key);

    Ok(())
    }

  fn list(
      &self, primary_namespace: &str, secondary_namespace: &str,
    ) -> Result<Vec<String>> {

    // Get read access to the HashMap
    let data = self.data.read().unwrap();
  
    // Create the prefix to match against
    let prefix = if secondary_namespace.is_empty() {
        format!("{}/", primary_namespace)
    } else {
        format!("{}/{}/", primary_namespace, secondary_namespace)
    };
  
    // Filter keys that match the prefix and extract the final key component
    let matching_keys: Vec<String> = data.keys()
        .filter(|k| k.starts_with(&prefix))
        .map(|k| k[prefix.len()..].to_string())  // Remove prefix to get final key
        .collect();
  
    Ok(matching_keys)

      }
  }

