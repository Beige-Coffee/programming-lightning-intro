#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::ch3_keys::exercises::{
  new_simple_key_manager, unified_onchain_offchain_wallet
};
use crate::internal::bitcoind_client::BitcoindClient;
use crate::internal::channel_manager::ChannelManager;
use std::time::{Duration, SystemTime};

use bitcoin::amount::Amount;
use bitcoin::hash_types::Txid;
use bitcoin::hashes::hex::FromHex;
use bitcoin::hashes::Hash;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::script::{ScriptBuf, ScriptHash};
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::secp256k1::PublicKey as Secp256k1PublicKey;
use bitcoin::secp256k1::Scalar;
use bitcoin::secp256k1::{self, Secp256k1};
use bitcoin::transaction::Version;
use core::sync::atomic::{AtomicUsize, Ordering};
use bitcoin::PubkeyHash;
use bitcoin::{OutPoint, PublicKey, Sequence, Transaction, TxIn, Witness};
use rand::{thread_rng, Rng};

#[test]
fn test_new_simple_key_manager() {
  let seed = [1_u8; 32];
  let child_index: usize = 0;
  let keys_interface_impl = new_simple_key_manager(seed);

  // check seed
  assert_eq!(
    keys_interface_impl.seed,
    seed
  );
  // check node_id
  assert_eq!(
    keys_interface_impl.node_id.to_string(),
      "0355f8d2238a322d16b602bd0ceaad5b01019fb055971eaadcc9b29226a4da6c23".to_string()
  );
  // check shutdown_pubkey
  assert_eq!(
    keys_interface_impl.unilateral_close_pubkey.to_string(),
      "02665a31546d90a812366bd637de00682d1492969da876dc1484f9b831838dcc7a".to_string()
  );
  // check channel_master_key
  assert_eq!(
    keys_interface_impl.channel_master_key.to_string(),
"tprv8c8LX21WH7wWXe79pUjDm1XKxEK2bNZNq6yd8eYfEfLLA6r4TkJAEcBthdbQjJ4UYcBDBku6H6hdWQzKHUhrbNQn71RFjNzmD8Tf7ZGC6zH".to_string()
  );
}

#[test]
fn test_unifed_onchain_offchain_wallet() {
  let seed = [1_u8; 32];
  let keys_interface_impl = unified_onchain_offchain_wallet(seed);
    // check channel_master_key
    assert_eq!(
      keys_interface_impl.get_node_secret_key().display_secret().to_string(),
"d34c19d25ca7c93b17121c6fbb77418b48a2de73080d82e8f2b4e1ba57679514".to_string()
    );
}

