#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use bitcoin::hash_types::{BlockHash};
use bitcoin::hashes::Hash;
use bitcoin::{Network };
use lightning_block_sync::http::HttpEndpoint;
use lightning_block_sync::rpc::RpcClient;
use lightning_block_sync::http::JsonResponse;
use bitcoin::{PublicKey, PrivateKey};
use bitcoin::address::Address;
use bitcoin::consensus::encode::serialize_hex;
use lightning_block_sync::{AsyncBlockSourceResult, BlockData, BlockHeaderData, BlockSource};
use serde_json;
use std::str::FromStr;
use bitcoin::blockdata::transaction::Transaction;
use std::sync::Arc;
use bitcoin::consensus::{encode, Decodable, Encodable};
use crate::internal::convert::{
    ListUnspentResponse, NewAddress, SignedTx, BlockchainInfo, AddressPubkey
};
use lightning::chain::chaininterface::{BroadcasterInterface, ConfirmationTarget, FeeEstimator};
use tokio::runtime::Handle;

#[derive(Clone)]
pub struct BitcoindClient {
    pub bitcoind_rpc_client: Arc<RpcClient>,
    pub handle: tokio::runtime::Handle,
}

impl BlockSource for BitcoindClient {
    fn get_header<'a>(
        &'a self,
        header_hash: &'a BlockHash,
        height_hint: Option<u32>,
    ) -> AsyncBlockSourceResult<'a, BlockHeaderData> {
        Box::pin(async move {
            self.bitcoind_rpc_client
                .get_header(header_hash, height_hint)
                .await
        })
    }

    fn get_block<'a>(
        &'a self,
        header_hash: &'a BlockHash,
    ) -> AsyncBlockSourceResult<'a, BlockData> {
        Box::pin(async move { self.bitcoind_rpc_client.get_block(header_hash).await })
    }

    fn get_best_block<'a>(&'a self) -> AsyncBlockSourceResult<(BlockHash, Option<u32>)> {
        Box::pin(async move { self.bitcoind_rpc_client.get_best_block().await })
    }
}

/// The minimum feerate we are allowed to send, as specify by LDK.
const MIN_FEERATE: u32 = 253;

impl BitcoindClient {
    pub async fn new(
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
            handle: tokio::runtime::Handle::current(),
        };

        Ok(client)
    }

    pub async fn list_unspent(&self) -> ListUnspentResponse {
        self.bitcoind_rpc_client
            .call_method::<ListUnspentResponse>("listunspent", &vec![])
            .await
            .unwrap()
    }

    pub async fn get_new_address(&self) -> Address {
        let addr_args = vec![serde_json::json!("LDK output address")];
        let addr = self
            .bitcoind_rpc_client
            .call_method::<NewAddress>("getnewaddress", &addr_args)
            .await
            .unwrap();
        Address::from_str(addr.0.as_str()).unwrap().require_network(Network::Regtest).unwrap()
    }

    pub async fn get_pubkey(&self, address: Address) -> PublicKey {
        let addr_args = vec![serde_json::json!(address.to_string())];
        let pubkey = self
            .bitcoind_rpc_client
            .call_method::<AddressPubkey>("getaddressinfo", &addr_args)
            .await
            .unwrap();
        pubkey.0
        }

    pub async fn sign_raw_transaction_with_wallet(&self, tx_hex: String) -> SignedTx {
        let tx_hex_json = serde_json::json!(tx_hex);
        let signed_tx: SignedTx = self.bitcoind_rpc_client
            .call_method("signrawtransactionwithwallet", &vec![tx_hex_json])
            .await
            .unwrap();
        //println!("Signed Tx: {}", &signed_tx.hex);
        signed_tx
    }
}

impl BroadcasterInterface for BitcoindClient {
    fn broadcast_transactions(&self, txs: &[&Transaction]) {
        let txn = txs.iter().map(|tx| encode::serialize_hex(tx)).collect::<Vec<_>>();
        
        //println!("txn: {:?}", txn);
        let bitcoind_rpc_client = Arc::clone(&self.bitcoind_rpc_client);
        //println!("txn len: {:?}", txn.len());
        self.handle.spawn(async move {
            let res = if txn.len() == 1 {
                let tx_json = serde_json::json!(txn[0]);
                //println!("Broadcasting transaction with raw hex: {}", tx_json);  // Added debug print
                bitcoind_rpc_client
                    .call_method::<serde_json::Value>("sendrawtransaction", &[tx_json])
                    .await
            } else {
                let tx_json = serde_json::json!(txn);
                //println!("Broadcasting transactions with raw hex: {}", tx_json);  // Added debug print
                bitcoind_rpc_client
                    .call_method::<serde_json::Value>("submitpackage", &[tx_json])
                    .await
            };

        });
    }
}
