#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use bitcoin::{Address, BlockHash, Txid};
use lightning_block_sync::http::JsonResponse;
use std::convert::TryInto;
use std::str::FromStr;

#[derive(Debug)]
pub struct BlockchainInfo {
  pub latest_height: usize,
  pub latest_blockhash: BlockHash,
  pub chain: String,
}

impl TryInto<BlockchainInfo> for JsonResponse {
  type Error = std::io::Error;
  fn try_into(self) -> std::io::Result<BlockchainInfo> {
    Ok(BlockchainInfo {
      latest_height: self.0["blocks"].as_u64().unwrap() as usize,
      latest_blockhash: BlockHash::from_str(self.0["bestblockhash"].as_str().unwrap())
        .unwrap(),
      chain: self.0["chain"].as_str().unwrap().to_string(),
    })
  }
}