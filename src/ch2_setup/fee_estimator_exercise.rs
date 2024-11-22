#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use lightning::chain::chaininterface::{
    ConfirmationTarget, FeeEstimator, FEERATE_FLOOR_SATS_PER_KW,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::cmp;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MempoolFeeRate {
    pub fastest_fee: u32,
    pub half_hour_fee: u32,
    pub hour_fee: u32,
    pub economy_fee: u32,
    pub minimum_fee: u32,
}

fn fetch_fee_from_mempool() -> MempoolFeeRate {
    MempoolFeeRate {
        fastest_fee: 6,
        half_hour_fee: 5,
        hour_fee: 5,
        economy_fee: 4,
        minimum_fee: 2,
    }
}

pub fn get_est_sat_per_1000_weight(confirmation_target: ConfirmationTarget) -> u32 {
  let fee_rates = fetch_fee_from_mempool();
    match confirmation_target {
        ConfirmationTarget::MaximumFeeEstimate => fee_rates.fastest_fee * 250 as u32,

        ConfirmationTarget::UrgentOnChainSweep => fee_rates.fastest_fee * 250 as u32,

        ConfirmationTarget::OutputSpendingFee => fee_rates.fastest_fee * 250 as u32,

        ConfirmationTarget::NonAnchorChannelFee => fee_rates.economy_fee * 250 as u32,

        ConfirmationTarget::AnchorChannelFee => fee_rates.minimum_fee * 250 as u32,

        ConfirmationTarget::ChannelCloseMinimum => fee_rates.minimum_fee * 250 as u32,

        ConfirmationTarget::MinAllowedNonAnchorChannelRemoteFee => fee_rates.minimum_fee * 250 as u32,

        ConfirmationTarget::MinAllowedAnchorChannelRemoteFee => fee_rates.minimum_fee * 250 as u32,
    }
}
