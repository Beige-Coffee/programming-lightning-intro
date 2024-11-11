#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::ch2_keys::exercises::{
  new_simple_key_manager,
};
use crate::internal::bitcoind_client::BitcoindClient;
use crate::internal::channel_manager::ChannelManager;
use std::time::{Duration, SystemTime};

use crate::internal::helper::{pubkey_multiplication_tweak, sha256_hash};
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
use bitcoin::PubkeyHash;
use bitcoin::{OutPoint, PublicKey, Sequence, Transaction, TxIn, Witness};
use rand::{thread_rng, Rng};

#[test]
fn test_new_simple_key_manager() {
  let seed = [1_u8; 32];
  let start_time: u64 = 1731279131;
  let start_time_subsec: u32 = 661666707;
  let keys_interface_impl = new_simple_key_manager(seed, start_time, start_time_subsec);

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
    keys_interface_impl.shutdown_pubkey.to_string(),
      "033469d7e6bf878c18f7d52fa2cad8ac4efc329dc1413c486a979f012ffb606110".to_string()
  );
  // check channel_master_key
  assert_eq!(
    keys_interface_impl.channel_master_key.to_string(),
"tprv8c8LX21WH7wWXe79pUjDm1XKxEK2bNZNq6yd8eYfEfLLA6r4TkJAEcBthdbQjJ4UYcBDBku6H6hdWQzKHUhrbNQn71RFjNzmD8Tf7ZGC6zH".to_string()
  );
  // check channel_child_index
  assert_eq!(
    keys_interface_impl.channel_child_index,
      0
  );
  // check starting_time_secs
  assert_eq!(
    keys_interface_impl.starting_time_secs,
    start_time
  );
  // check starting_time_nanos
  assert_eq!(
    keys_interface_impl.starting_time_nanos,
    start_time_subsec
  );
}

