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

/// Helper function to create the full key path from namespaces and key
fn create_key_path(primary: &str, secondary: &str, key: &str) -> String {
    if secondary.is_empty() {
        format!("{}/{}", primary, key)
    } else {
        format!("{}/{}/{}", primary, secondary, key)
    }
}

/// Helper function to get the last part of a path (the key name)
fn get_key_name(full_path: &str) -> String {
    full_path.split('/')
        .last()
        .unwrap_or("")
        .to_string()
}


impl KVStore for SimpleStore {
    fn write(
        &self,
        primary_namespace: &str,
        secondary_namespace: &str,
        key: &str,
        value: &[u8],
    ) -> Result<()> {
        let full_key = create_key_path(primary_namespace, secondary_namespace, key);
        let mut store = self.data.write().unwrap();
        store.insert(full_key, value.to_vec());
        Ok(())
    }

    fn read(
        &self,
        primary_namespace: &str,
        secondary_namespace: &str,
        key: &str,
    ) -> Result<Vec<u8>> {
        let full_key = create_key_path(primary_namespace, secondary_namespace, key);
        let store = self.data.read().unwrap();

        let result = store.get(&full_key)
            .cloned()
            .expect("Key exists");

        Ok(result)
    }

    fn remove(
        &self,
        primary_namespace: &str,
        secondary_namespace: &str,
        key: &str,
        _lazy: bool,
    ) -> Result<()> {
        let full_key = create_key_path(primary_namespace, secondary_namespace, key);
        let mut store = self.data.write().unwrap();
        store.remove(&full_key);
        Ok(())
    }

    fn list(
        &self,
        primary_namespace: &str,
        secondary_namespace: &str,
    ) -> Result<Vec<String>> {
        let prefix = create_key_path(primary_namespace, secondary_namespace, "");
        let store = self.data.read().unwrap();

        let mut result = Vec::new();
        for full_key in store.keys() {
            if full_key.starts_with(&prefix) {
                result.push(get_key_name(full_key));
            }
        }

        Ok(result)
    }
    }