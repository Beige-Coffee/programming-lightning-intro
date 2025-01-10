#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use bitcoin::{Address, BlockHash, Txid};
use lightning_block_sync::http::JsonResponse;
use std::convert::TryInto;
use std::str::FromStr;
use bitcoin::secp256k1::PublicKey;
use serde_json::Value;

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
pub struct MempoolInfo {
    pub transaction_ids: Vec<String>,
}

impl TryInto<MempoolInfo> for JsonResponse {
    type Error = std::io::Error;

    fn try_into(self) -> std::io::Result<MempoolInfo> {
        // Ensure the response is a JSON array
        let array = self.0.as_array().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Expected a JSON array")
        })?;

        // Convert the array items into a Vec<String>
        let transaction_ids = array
            .iter()
            .map(|item| {
                item.as_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Non-string item found"))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(MempoolInfo { transaction_ids })
    }
}

#[derive(Debug)]
pub struct AddressPubkey(pub PublicKey);

impl TryInto<AddressPubkey> for JsonResponse {
  type Error = std::io::Error;
  
  fn try_into(self) -> std::io::Result<AddressPubkey> {
    
    let pubkey_str = self.0.get("pubkey").ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing pubkey in response")
    })?;

    let pubkey = pubkey_str.as_str().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid pubkey format")
    })?;

    let pubkey = PublicKey::from_str(pubkey).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid pubkey: {}", e))
    })?;

    Ok(AddressPubkey(pubkey))
    
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

#[derive(Debug, Clone)]
pub struct ListUnspentUtxo {
  pub txid: Txid,
  pub vout: u32,
  pub amount: u64,
  pub address: Address,
}