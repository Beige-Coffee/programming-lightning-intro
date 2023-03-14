// use lightning::ln::channelmanager;
// use lightning::ln::channelmanager::{
// 	ChainParameters, ChannelManagerReadArgs, SimpleArcChannelManager,
// };
// use lightning::chain::{chainmonitor, ChannelMonitorUpdateStatus};
// use lightning::chain::keysinterface::{EntropySource, InMemorySigner, KeysManager};

// type ChainMonitor = chainmonitor::ChainMonitor<
// 	InMemorySigner,
// 	Arc<dyn Filter + Send + Sync>,
// 	Arc<BitcoindClient>,
// 	Arc<BitcoindClient>,
// 	Arc<FilesystemLogger>,
// 	Arc<FilesystemPersister>,
// >;

// pub(crate) type ChannelManager =
// 	SimpleArcChannelManager<ChainMonitor, BitcoindClient, BitcoindClient, FilesystemLogger>;

use std::sync::Mutex;

use bitcoin::{PublicKey, Transaction};
use lightning::util::errors::APIError;

pub struct ChannelManager {
  pub last_funding_tx_gen: Mutex<Option<(Vec<u8>, PublicKey, String)>>
}

impl ChannelManager {
  pub fn new() -> Self {
    Self {
      last_funding_tx_gen: Mutex::new(None)
    }
  }

  pub fn funding_transaction_generated(
    &self,
    temporary_channel_id: &[u8; 32],
    counterparty_node_id: &PublicKey,
    funding_transaction: String
  ) -> Result<(), APIError> {
    let mut last_funding_tx_gen = self.last_funding_tx_gen.lock().unwrap();
    if last_funding_tx_gen.is_some() {
      return Err(APIError::APIMisuseError {
        err: "funding_transaction_generated called more than once".to_string()
      });
    }
    *last_funding_tx_gen = Some((temporary_channel_id.to_vec(), counterparty_node_id.clone(), funding_transaction));
    Ok(())
  }
}