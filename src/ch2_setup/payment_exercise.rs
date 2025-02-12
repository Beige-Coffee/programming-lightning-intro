use std::time::Duration;
use internal::channel_manager::{ChannelManager, Bolt11Invoice, payment_parameters_from_invoice, Retry};
use crate::internal;

pub fn send_payment(channel_manager: ChannelManager, invoice: Bolt11Invoice) {

  let payment_id = invoice.payment_id;

  let (payment_hash, recipient_onion, route_params) = payment_parameters_from_invoice(invoice);
  
  
  match channel_manager.send_payment(
    payment_hash,
    recipient_onion,
    payment_id,
    route_params,
    Retry::Timeout(Duration::from_secs(10)),
  ) {
    Ok(_) => {
      println!("Success");
    },
    Err(_e) => {
      println!("Fail");
    }
  }
}