#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use crate::ch2_setup::persist_exercise_v2::{FileStore};
use crate::ch2_setup::channel_exercises::{ChannelManager};
use crate::ch2_setup::bitcoin_client::{BitcoinClient};
use crate::ch3_keys::exercises::{SimpleKeysManager};
use lightning::events::{Event};

async fn handle_ldk_events(
    mut channel_manager: ChannelManager, 
    bitcoin_client: BitcoinClient,
    keys_manager: SimpleKeysManager, 
    fs_store: FileStore,
    event: Event
) {
    match event {
        Event::OpenChannelRequest {
            temporary_channel_id,
            counterparty_node_id,
            funding_satoshis,
            params,
            ..
        } => {
            if params.to_self_delay < 24 {
                println!("Reject")
            }

                                  if funding_satoshis < 5_000_000 {
                                                            println!("Reject")
                                  }

                                  channel_manager.create_channel(counterparty_node_id, funding_satoshis)
        },
        Event::FundingTxBroadcastSafe { .. } => {},
        Event::PaymentClaimable { .. } => {},
        Event::PendingHTLCsForwardable { .. } => {},
        Event::SpendableOutputs { .. } => {},
        Event::ChannelReady { .. } => {},
        Event::ChannelClosed { .. } => {},
        _ => {},
    }
}