#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use internal::convert::BlockchainInfo;
use base64;
use bitcoin::hash_types::{BlockHash};
use bitcoin::{Network};
use lightning_block_sync::http::HttpEndpoint;
use lightning_block_sync::rpc::RpcClient;
use lightning_block_sync::{AsyncBlockSourceResult, BlockData, BlockHeaderData, BlockSource};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub fn get_http_endpoint(host: &String, port: u16) -> HttpEndpoint {
  HttpEndpoint::for_host(host.clone()).with_port(port)
}

pub fn format_rpc_credentials(rpc_user: &String, rpc_password: &String) -> String {
  base64::encode(format!("{}:{}", rpc_user.clone(), rpc_password.clone()))
}

pub fn new_rpc_client(rpc_credentials: &String, http_endpoint: HttpEndpoint) -> RpcClient {
  let client = RpcClient::new(&rpc_credentials, http_endpoint).unwrap();
  client
}

pub async fn test_rpc_call(bitcoind_rpc_client: &RpcClient) -> std::io::Result<BlockchainInfo> {
  let _dummy = bitcoind_rpc_client
      .call_method::<BlockchainInfo>("getblockchaininfo", &vec![])
      .await
      .map_err(|_| {
          std::io::Error::new(std::io::ErrorKind::PermissionDenied,
          "Failed to make initial call to bitcoind - please check your RPC user/password and access settings")
      })?;
  Ok(_dummy)
}