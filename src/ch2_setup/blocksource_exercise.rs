#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use bitcoin::hash_types::{BlockHash};
use lightning_block_sync::{BlockData, BlockHeaderData, AsyncBlockSourceResult};
use lightning_block_sync::rpc::RpcClient;

// Step 1: Start with a simpler trait for learning

trait SimpleBlockSource {
    async fn get_header(&self, header_hash: &BlockHash) -> AsyncBlockSourceResult<BlockHeaderData>;
    async fn get_block(&self, header_hash: &BlockHash) -> AsyncBlockSourceResult<BlockData>;
    async fn get_best_block(&self) -> AsyncBlockSourceResult<(BlockHash, Option<u32>)>;
}

// Step 2: Implement the simple version first
#[async_trait::async_trait]
impl SimpleBlockSource for RpcClient {
    async fn get_header(&self, header_hash: &BlockHash) -> AsyncBlockSourceResult<BlockHeaderData> {
        let header_hash = serde_json::json!(header_hash.to_string());
        self.call_method("getblockheader", &[header_hash]).await?
    }

    async fn get_block(&self, header_hash: &BlockHash) -> AsyncBlockSourceResult<BlockData> {
        let header_hash = serde_json::json!(header_hash.to_string());
        let verbosity = serde_json::json!(0);
        self.call_method("getblock", &[header_hash, verbosity]).await?
    }

    async fn get_best_block(&self) -> AsyncBlockSourceResult<(BlockHash, Option<u32>)> {
        self.call_method("getblockchaininfo", &[]).await?
    }
}