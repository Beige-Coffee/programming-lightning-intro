#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
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
use std::time::Duration;

/// The payment hash is the hash of the [`PaymentPreimage`] which is the value used to lock funds
/// in HTLCs while they transit the lightning network.
pub struct PaymentHash(pub [u8; 32]);

// A user-provided identifier for this payment. LDK uses this to ensure that each payment is only sent once. For example, if you app restarts or there are network issues, it will not attempt to send the same payment ID multiple times.
#[derive(Copy, Clone)]
pub struct PaymentId(pub [u8; 32]);

/// The payment hash is the hash of the [`PaymentPreimage`] which is the value used to lock funds
/// in HTLCs while they transit the lightning network.
pub struct PaymentSecret(pub [u8; 32]);

/// Information which is provided, encrypted, to the payment recipient when sending HTLCs.
///
/// This should generally be constructed with data communicated to us from the recipient (via a
/// BOLT11 or BOLT12 invoice).
pub struct RecipientOnionFields {
  // arbitrary 32-byte secret provided by reciient for us to repeat in onion. It is used to authenticate the sender to the recipient.
  pub payment_secret: Option<PaymentSecret>,
  // Arbitrary length metadata that can be provided to the recipient as part of a payment. 
  pub payment_metadata: Option<Vec<u8>>,
  // additional Type-Length-Value information to be conveyed in onion
  pub custom_tlvs: Vec<(u64, Vec<u8>)>,
}

/// Information used to route a payment.
pub struct PaymentParameters {
  /// Expiration of a payment to the payee, in seconds relative to the UNIX epoch.
  pub expiry_time: Option<u64>,

  /// The maximum total CLTV delta we accept for the route.
  /// Defaults to [`DEFAULT_MAX_TOTAL_CLTV_EXPIRY_DELTA`].
  pub max_total_cltv_expiry_delta: u32,
}

/// Parameters needed to find a [`Route`].
pub struct RouteParameters {
  /// The parameters of the failed payment path.
  pub payment_params: PaymentParameters,
  /// The amount in msats sent on the failed payment path.
  pub final_value_msat: u64,
  /// The maximum total fees, in millisatoshi, that may accrue during route finding.
  ///
  /// This limit also applies to the total fees that may arise while retrying failed payment
  /// paths.
  ///
  /// Note that values below a few sats may result in some paths being spuriously ignored.
  pub max_total_routing_fee_msat: Option<u64>,
}

pub fn payment_parameters_from_invoice(
invoice: Bolt11Invoice
) -> (PaymentHash, RecipientOnionFields, RouteParameters) {
// PaymentHash
let payment_hash = PaymentHash([1; 32]);  

// RecipientOnionFields
let payment_secret = PaymentSecret([2; 32]);  
let custom_tlvs = vec![  
    (2, vec![1, 2, 3, 4]),
    (4, vec![5, 6, 7, 8])
];

let recipient_onion = RecipientOnionFields {
    payment_secret: Some(payment_secret),
    payment_metadata: None,
    custom_tlvs,
};

// RouteParameters
let expiry_time: u64 = 1000000;
let max_total_cltv_expiry_delta: u32 = 144;  
let payment_params = PaymentParameters { 
    expiry_time: Some(expiry_time),
    max_total_cltv_expiry_delta,  
};
let final_value_msat: u64 = 500_000;

let max_total_routing_fee_msat: u64 = 100_000;

let route_parameters = RouteParameters {
    payment_params, 
    final_value_msat,
    max_total_routing_fee_msat: Some(max_total_routing_fee_msat),
};

(payment_hash, recipient_onion, route_parameters)
}

pub struct Bolt11Invoice {
  pub payment_id: PaymentId
}

pub enum Retry {
    /// Maximum number of attempts to retry payment
    Attempts(u32),
    /// Time elapsed before abandoning retries
    Timeout(Duration),
}


pub enum RetryableSendFailure {
    /// The provided [`PaymentParameters::expiry_time`] indicated that the payment has expired.
    /// [`PaymentParameters::expiry_time`]: crate::routing::router::PaymentParameters::expiry_time
    PaymentExpired,
    /// We were unable to find a route to the destination.
    RouteNotFound,
    /// Indicates that a payment for the provided [`PaymentId`] is already in-flight and has not
    /// yet completed (i.e. generated an [`Event::PaymentSent`] or [`Event::PaymentFailed`]).
    ///
    /// [`PaymentId`]: crate::ln::channelmanager::PaymentId
    /// [`Event::PaymentSent`]: crate::events::Event::PaymentSent
    /// [`Event::PaymentFailed`]: crate::events::Event::PaymentFailed
    DuplicatePayment,
    /// The [`RecipientOnionFields::payment_metadata`], [`RecipientOnionFields::custom_tlvs`], or
    /// [`BlindedPaymentPath`]s provided are too large and caused us to exceed the maximum onion
    /// packet size of 1300 bytes.
    ///
    /// [`BlindedPaymentPath`]: crate::blinded_path::payment::BlindedPaymentPath
    OnionPacketSizeExceeded,
}

pub struct ChannelManager {
    pub last_funding_tx_gen: Mutex<Option<(Vec<u8>, PublicKey, String)>>,
}

impl ChannelManager {
    pub fn new() -> Self {
        Self {
            last_funding_tx_gen: Mutex::new(None),
        }
    }

    pub fn funding_transaction_generated(
        &self,
        temporary_channel_id: &[u8; 32],
        counterparty_node_id: &PublicKey,
        funding_transaction: String,
    ) {
        let mut last_funding_tx_gen = self.last_funding_tx_gen.lock().unwrap();
        *last_funding_tx_gen = Some((
            temporary_channel_id.to_vec(),
            counterparty_node_id.clone(),
            funding_transaction,
        ));
    }

    pub fn send_payment(&self, payment_hash: PaymentHash, recipient_onion: RecipientOnionFields, payment_id: PaymentId, route_params: RouteParameters, retry_strategy: Retry) -> Result<(), RetryableSendFailure> {
        Ok(())
        }
}
