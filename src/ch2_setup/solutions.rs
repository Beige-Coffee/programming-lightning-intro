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
use lightning::chain::Listen;
use lightning_block_sync::init::validate_best_block_header;
use lightning_block_sync::poll::ChainPoller;
use lightning_block_sync::SpvClient;

pub struct BitcoindClientExercise {
    pub(crate) bitcoind_rpc_client: Arc<RpcClient>,
    network: Network,
    host: String,
    port: u16,
    rpc_user: String,
    rpc_password: String
}

impl BlockSource for BitcoindClientExercise {
    fn get_header<'a>(
        &'a self, header_hash: &'a BlockHash, height_hint: Option<u32>,
    ) -> AsyncBlockSourceResult<'a, BlockHeaderData> {
        Box::pin(async move { self.bitcoind_rpc_client.get_header(header_hash, height_hint).await })
    }

    fn get_block<'a>(
        &'a self, header_hash: &'a BlockHash,
    ) -> AsyncBlockSourceResult<'a, BlockData> {
        Box::pin(async move { self.bitcoind_rpc_client.get_block(header_hash).await })
    }

    fn get_best_block<'a>(&'a self) -> AsyncBlockSourceResult<(BlockHash, Option<u32>)> {
        Box::pin(async move { self.bitcoind_rpc_client.get_best_block().await })
    }
}

impl BitcoindClientExercise {
    pub(crate) async fn new(
        host: String, port: u16, rpc_user: String, rpc_password: String, network: Network,
    ) -> std::io::Result<Self> {
        let http_endpoint = HttpEndpoint::for_host(host.clone()).with_port(port);
        let rpc_credentials =
            base64::encode(format!("{}:{}", rpc_user.clone(), rpc_password.clone()));
        let bitcoind_rpc_client = RpcClient::new(&rpc_credentials, http_endpoint)?;
        let _dummy = bitcoind_rpc_client
            .call_method::<BlockchainInfo>("getblockchaininfo", &vec![])
            .await
            .map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::PermissionDenied,
                "Failed to make initial call to bitcoind - please check your RPC user/password and access settings")
            })?;

        let client =Self {
            bitcoind_rpc_client: Arc::new(bitcoind_rpc_client),
            host,
            port,
            rpc_user,
            rpc_password,
            network
        };

        Ok(client)
    }
}
