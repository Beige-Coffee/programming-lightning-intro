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

#[derive(Debug)]
pub struct SignedTx {
  pub complete: bool,
  pub hex: String,
}


impl TryInto<SignedTx> for JsonResponse {
  type Error = std::io::Error;
  fn try_into(self) -> std::io::Result<SignedTx> {
    Ok(SignedTx {
      hex: self.0["hex"].as_str().unwrap().to_string(),
      complete: self.0["complete"].as_bool().unwrap(),
    })
  }
}


#[derive(Debug)]
pub struct NewAddress(pub String);
impl TryInto<NewAddress> for JsonResponse {
  type Error = std::io::Error;
  fn try_into(self) -> std::io::Result<NewAddress> {
    Ok(NewAddress(self.0.as_str().unwrap().to_string()))
  }
}

#[derive(Debug)]
pub struct ListUnspentResponse(pub Vec<ListUnspentUtxo>);

impl TryInto<ListUnspentResponse> for JsonResponse {
  type Error = std::io::Error;
  fn try_into(self) -> Result<ListUnspentResponse, Self::Error> {
    let utxos = self
      .0
      .as_array()
      .unwrap()
      .iter()
      .map(|utxo| ListUnspentUtxo {
        txid: Txid::from_str(&utxo["txid"].as_str().unwrap().to_string()).unwrap(),
        vout: utxo["vout"].as_u64().unwrap() as u32,
        amount: bitcoin::Amount::from_btc(utxo["amount"].as_f64().unwrap())
          .unwrap()
          .to_sat(),
        address: Address::from_str(&utxo["address"].as_str().unwrap().to_string())
          .unwrap()
          .assume_checked(), // the expected network is not known at this point
      })
      .collect();
    Ok(ListUnspentResponse(utxos))
  }
}

#[derive(Debug)]
pub struct ListUnspentUtxo {
  pub txid: Txid,
  pub vout: u32,
  pub amount: u64,
  pub address: Address,
}