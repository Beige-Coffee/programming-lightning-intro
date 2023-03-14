# Channel Open

If you recall, the channel opening process is a multi-step process requiring communication and agreement with your counterparty.  You need to agree on some parameters (what chain, min/max htlc values and amounts, etc), exchange signatures, and then fund and broadcast the funding transaction.

In LDK the process begins with the `ChannelManager`'s [create_channel()](https://docs.rs/lightning/0.0.114/lightning/ln/channelmanager/struct.ChannelManager.html#method.create_channel) method.

This method will return a temporary channel id that is used until it can be replaced by the actual channel id that is specified by the funding transaction.

ChannelManager will also handle all of the initial `channel_open` and `accept_channel` negotiations for you based on the configuration provided in the last parameter.

If the negotiations are successful then LDK will notify you by publishing an event called `FundingGenerationReady`.  This event is LDK's way of telling you that things are looking good but we now need you to construct and sign a funding transaction that correctly funds the channel you asked for.  

Once the funding transaction is ready you can provide it back to LDK via `ChannelManager`'s [funding_transaction_generated()](https://docs.rs/lightning/0.0.114/lightning/ln/channelmanager/struct.ChannelManager.html#method.funding_transaction_generated) method.

This will enable LDK to complete the next steps of the channel creation protocol with your counterparty by exchanging `funding_created` and `funding_signed` messages.

From there LDK will use the chain data you provide it to monitor for confirmations and when the specified number of confirmations are achieved on the funding transaction it will automatically finish the negotiation for you by exchanging `channel_ready` messages.

Finally, LDK will emit a `ChannelReady` event that signals the channel is ready to be used for making payments.

Let's implement parts of this channel opening flow using `bitcoind` as our on-chain wallet.  If you're not familiar with bitcoind's wallet rpc's you can review them on [bitcoin.org's documentation site](https://developer.bitcoin.org/reference/rpc/#wallet-rpcs).

We will be using a bitcoind client to make these rpc calls and it has the following api:

```rust

/// Outputs is a Vec of TxOut from our previous channel open section
/// returns the raw tx in hex formatted String
async fn create_raw_transaction(&self, outputs: Vec<TxOut>) -> String {}

/// This takes a raw tx and will have bitcoind do coin selection from it's wallet
/// and attach enough inputs to satisfy the outputs in the transaction
/// returns the funded tx as a raw hex String
async fn fund_raw_transaction(&self, raw_tx_hex: String) -> String {}

/// Uses the keys in bitcoind wallet to sign the inputs of the transaction so they can be spent
/// returns the signed tx as a raw hex String
async fn sign_raw_transaction_with_wallet(&self, tx_hex: String) -> String {}

/// Gets a new address from the bitcoind wallet
async fn get_new_address(&self) -> Address {}

/// Returns information about bitcoind
/// pub struct BlockchainInfo {
///	   pub latest_height: usize,
///	   pub latest_blockhash: BlockHash,
///	   pub chain: String,
/// }
async fn get_blockchain_info(&self) -> BlockchainInfo {}
```


## ⚡️ Write a function `handle_funding_generation_ready` that will construct the correct channel funding transaction and provide it to LDK

<br/>

The `FundingGenerationReady` event that LDK emits provides us with the following parameters:

- `temporary_channel_id` -- this is used to identify the channel before the funding tx exists. we will need to give this back to LDK when we provide the signed funding transaction so LDK knows what channel this funding transaction is for.

- `counterparty_node_id` -- this identifies what counterparty this channel is being created with. you might use this to perform lookups and apply special logic based on the peer.  in this example we'll just pass this back to LDK when we're ready

- `channel_value_satoshis` -- this is the total value of the channel in satoshis. we will need to make sure our funding transaction has an output with this amount

- `output_script` -- this is the funding tx output script. it includes the 2-of-2 multisig spending path just like our simple payment channel example. we will need to make sure our funding transaction has an output with this as the `scriptPubKey`.

- `user_channel_id` -- this is a user provided identifier for this channel. we won't need it for this example but it can be useful for tracking channel opens within your application

<br/>

```rust
fn handle_funding_generation_ready(
  channel_manager: ChannelManager, 
  bitcoind_client: BitcoindClient, 
  temporary_channel_id: [u8; 32], 
  counterparty_node_id: PublicKey, 
  channel_value_satoshis: u64, 
  output_script: Script, 
  user_channel_id: u128) {}
```
