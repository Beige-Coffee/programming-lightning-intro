#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use lightning::chain::chaininterface::{BroadcasterInterface, ConfirmationTarget, FeeEstimator};
use serde::Deserialize;
use std::collections::HashMap;
use std::cmp;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeeRateEstimate {
    pub target_1_block: u32,
    pub target_3_block: u32,
    pub target_6_block: u32,
    pub target_144_block: u32,
    pub target_1008_block: u32,
}

fn fetch_fee_from_mempool() -> FeeRateEstimate {
    FeeRateEstimate {
        target_1_block: 6000,
        target_3_block: 5000,
        target_6_block: 5000,
        target_144_block: 4000,
        target_1008_block: 2000,
    }
}


pub fn get_est_sat_per_1000_weight(confirmation_target: ConfirmationTarget) -> u32 {
  let fee_rates = fetch_fee_from_mempool();
    match confirmation_target {
        ConfirmationTarget::MaximumFeeEstimate => fee_rates.target_1_block * 250 as u32,

        ConfirmationTarget::UrgentOnChainSweep => fee_rates.target_1_block * 250 as u32,

        ConfirmationTarget::OutputSpendingFee => fee_rates.target_1_block * 250 as u32,

        ConfirmationTarget::NonAnchorChannelFee => fee_rates.target_6_block * 250 as u32,

        ConfirmationTarget::AnchorChannelFee => fee_rates.target_1008_block * 250 as u32,

        ConfirmationTarget::ChannelCloseMinimum => fee_rates.target_1008_block * 250 as u32,

        ConfirmationTarget::MinAllowedNonAnchorChannelRemoteFee => fee_rates.target_1008_block * 250 as u32,

        ConfirmationTarget::MinAllowedAnchorChannelRemoteFee => fee_rates.target_1008_block * 250 as u32,
    }
}
