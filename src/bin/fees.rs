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

pub struct MyAppFeeEstimator {
  fee_rate_cache: HashMap<ConfirmationTarget, u32>,
}

impl MyAppFeeEstimator {
    fn fetch_fee_from_mempool() -> MempoolFeeRate {
        MempoolFeeRate {
            fastest_fee: 6,
            half_hour_fee: 5,
            hour_fee: 5,
            economy_fee: 4,
            minimum_fee: 2,
        }
    }

    pub fn new() -> Self {
        let mut fee_rate_cache = HashMap::new();

        let fee_rates = Self::fetch_fee_from_mempool();

        fee_rate_cache.insert(ConfirmationTarget::MaximumFeeEstimate, 
                              fee_rates.fastest_fee * 250 as u32);
        fee_rate_cache.insert(ConfirmationTarget::UrgentOnChainSweep, 
                              fee_rates.fastest_fee * 250 as u32);
        fee_rate_cache.insert(ConfirmationTarget::OutputSpendingFee, 
                              fee_rates.fastest_fee * 250 as u32);
        fee_rate_cache.insert(ConfirmationTarget::NonAnchorChannelFee, 
                              fee_rates.economy_fee * 250 as u32);
        fee_rate_cache.insert(ConfirmationTarget::AnchorChannelFee, 
                              fee_rates.minimum_fee * 250 as u32);
        fee_rate_cache.insert(ConfirmationTarget::ChannelCloseMinimum, 
                              fee_rates.minimum_fee * 250 as u32);
        fee_rate_cache.insert(ConfirmationTarget::MinAllowedNonAnchorChannelRemoteFee, 
                              fee_rates.minimum_fee * 250 as u32);
        fee_rate_cache.insert(ConfirmationTarget::MinAllowedAnchorChannelRemoteFee, 
                              fee_rates.minimum_fee * 250 as u32);
        
        MyAppFeeEstimator {
            fee_rate_cache
        }
    }
}

impl FeeEstimator for MyAppFeeEstimator {
  fn get_est_sat_per_1000_weight(&self, confirmation_target: ConfirmationTarget) -> u32 {
      *self.fee_rate_cache.get(&confirmation_target).expect("all ConfirmationTargets should be present")
    }

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

fn get_est_sat_per_1000_weight(confirmation_target: ConfirmationTarget) -> u32 {
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

fn main() {
    //let fee_estimator = MyAppFeeEstimator::new();
    let high_fee_target = ConfirmationTarget::UrgentOnChainSweep;
    let low_fee_target = ConfirmationTarget::MinAllowedAnchorChannelRemoteFee;
    //println!("Feerate for {:?}: {:?}", high_fee_target, fee_estimator.get_est_sat_per_1000_weight(high_fee_target));
    //println!("Feerate for {:?}: {:?}", low_fee_target, fee_estimator.get_est_sat_per_1000_weight(low_fee_target));
    println!("Feerate for low fee target: {:?}", get_est_sat_per_1000_weight(low_fee_target)); 
    println!("Feerate for high fee target: {:?}", get_est_sat_per_1000_weight(high_fee_target)); 
}