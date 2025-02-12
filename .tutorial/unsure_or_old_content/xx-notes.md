For this workshop, we will build a **Simple Payment Verification (SPV)** client that will periodically poll our Replit's Regtest network for the best chain tip. The below diagram shows a very high-level view of what this architecure will look like. Since LDK is a modular software development kit, it's up to us to implement the actual connection to the Bitcoin blockchain. We can do this by completing the `BlockSource` trait with our chosen method (RPC, REST, etc.).


```rust
pub struct SimpleChannelMonitor {
  // Tracks the version/sequence number of monitor updates
  latest_update_id: u64,
  // Script used to send funds back to us when channel closes
  destination_script: ScriptBuf,
  // Script used to send funds to counterparty when channel closes
  counterparty_payment_script: ScriptBuf,
  // Unique identifier for deriving channel-specific keys
  channel_keys_id: [u8; 32],
  // Base point for generating revocation keys (used to punish cheating)
  holder_revocation_basepoint: RevocationBasepoint,
  // Unique identifier for this channel
  channel_id: ChannelId,
  // The transaction output that funded this channel and its script
  funding_info: (OutPoint, ScriptBuf),
  // Current commitment transaction from counterparty (None if not yet received)
  current_counterparty_commitment_txid: Option<Txid>,
  // Previous commitment transaction from counterparty (for revocation)
  prev_counterparty_commitment_txid: Option<Txid>,
  // Script that controls the funding output (2-of-2 multisig)
  funding_redeemscript: ScriptBuf,
  // Total value of the channel in satoshis
  channel_value_satoshis: u64,
}
```