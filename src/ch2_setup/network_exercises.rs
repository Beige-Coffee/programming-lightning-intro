#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use internal::convert::BlockchainInfo;
use crate::ch2_setup::helpers::{get_http_endpoint, format_rpc_credentials, 
                                new_rpc_client, test_rpc_call, get_best_block,
                                get_chain_poller, get_new_cache, get_spv_client, ToHex
};
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
use tokio::time::{sleep, Duration};
use lightning::chain::Listen;
use lightning::routing::gossip::{P2PGossipSync, NetworkGraph};
use lightning::routing::scoring::{ProbabilisticScoringDecayParameters, ProbabilisticScorer, ProbabilisticScoringFeeParameters};
use lightning::routing::router::{DefaultRouter};
use lightning::util::logger::Logger;
use lightning::sign::{EntropySource};

// Wrapper structs to hide complexity
pub struct SimpleNetworkGraph {
    inner: Arc<NetworkGraph<Arc<dyn Logger>>>
}

pub struct SimpleGossipSync {
    inner: P2PGossipSync<Arc<NetworkGraph<Arc<dyn Logger>>>, Arc<dyn Logger>>
}

pub struct SimpleScorer {
    inner: ProbabilisticScorer<Arc<NetworkGraph<Arc<dyn Logger>>>, Arc<dyn Logger>>
}

pub struct SimpleRouter {
    inner: DefaultRouter<
        Arc<NetworkGraph<Arc<dyn Logger>>>,
        Arc<dyn Logger>,
        Arc<dyn EntropySource>,
        Arc<ProbabilisticScorer<Arc<NetworkGraph<Arc<dyn Logger>>>, Arc<dyn Logger>>>,
        ProbabilisticScoringFeeParameters,
        ProbabilisticScorer<Arc<NetworkGraph<Arc<dyn Logger>>>, Arc<dyn Logger>>
    >
}

// Simple constructors
impl SimpleNetworkGraph {
    pub fn new(network: Network, logger: Arc<dyn Logger>) -> Self {
        Self {
            inner: Arc::new(NetworkGraph::new(network, logger))
        }
    }

    // Helper to get Arc'd inner for other components
    pub(crate) fn inner(&self) -> Arc<NetworkGraph<Arc<dyn Logger>>> {
        self.inner.clone()
    }
}

impl SimpleGossipSync {
    pub fn new(network_graph: &SimpleNetworkGraph, logger: Arc<dyn Logger>) -> Self {
        Self {
            inner: P2PGossipSync::new(
                network_graph.inner(),
                None,
                logger
            )
        }
    }
}

impl SimpleScorer {
    pub fn new(network_graph: &SimpleNetworkGraph, logger: Arc<dyn Logger>) -> Self {
        let decay_params = ProbabilisticScoringDecayParameters::default();
        Self {
            inner: ProbabilisticScorer::new(
                decay_params,
                network_graph.inner(),
                logger
            )
        }
    }

    // Helper to get Arc'd inner for router
    pub(crate) fn inner(&self) -> Arc<ProbabilisticScorer<Arc<NetworkGraph<Arc<dyn Logger>>>, Arc<dyn Logger>>> {
        Arc::new(self.inner.clone())
    }
}

impl SimpleRouter {
    pub fn new(
        network_graph: &SimpleNetworkGraph,
        logger: Arc<dyn Logger>,
        entropy_source: Arc<dyn EntropySource>,
        scorer: &SimpleScorer
    ) -> Self {
        let fee_params = ProbabilisticScoringFeeParameters::default();
        Self {
            inner: DefaultRouter::new(
                network_graph.inner(),
                logger,
                entropy_source,
                scorer.inner(),
                fee_params,
            )
        }
    }
}

// Now the workshop function becomes much simpler!
pub fn complete_network_setup(
    network: Network,
    logger: Arc<dyn Logger>,
    entropy_source: Arc<dyn EntropySource>,
) -> (SimpleNetworkGraph, SimpleGossipSync, SimpleScorer, SimpleRouter) {
    // Create network graph
    let network_graph = SimpleNetworkGraph::new(network, logger.clone());

    // Create gossip sync
    let gossip_sync = SimpleGossipSync::new(&network_graph, logger.clone());

    // Create scorer
    let scorer = SimpleScorer::new(&network_graph, logger.clone());

    // Create router
    let router = SimpleRouter::new(&network_graph, logger, entropy_source, &scorer);

    (network_graph, gossip_sync, scorer, router)
}
