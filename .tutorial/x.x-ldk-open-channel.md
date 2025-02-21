# Opening A Channel

LDK makes opening a Lightning channel quite simple, but there is actually a *lot* going on under the hood. Since *real* learning is supposed to be *hard*, let's dig into the protocol before reviewing LDK's higher-level code abstractions for opening a channel. To do this, we'll have to open up our textbooks to [BOLT #2](https://github.com/lightning/bolts/blob/master/02-peer-protocol.md#channel-establishment-v2).

## BOLT #2

BOLT #2, titled "Peer Protocol for Channel Management", describes the communication protocol that peers will use to set up a channel between themselves. This protocol has the following three phases:
1) Channel Establishment
2) Channel Operation
3) Channel Closing

We'll focus on **Channel Establishment** in this section.

It's important to note that there are the following two version of channel establishment:
- **Channel Establishment v1**: This version is the original protocol for opening a channel between two channel parties.
- **Channel Establishment v2**: This is the updated channel open protocol that allows for dual-funding channel where both channel parties contribute funds to the channel. Note, you can also create single-fundeded channels with this updated protocol.

# Channel Establishment V1
For simplicity, we'll focus on **Channel Establishment v1** for this section. Also, note that, when initiating a channel, we've already autheniticated ([BOLT #8](https://github.com/lightning/bolts/blob/master/08-transport.md)). and initialized ([BOLT #1](https://github.com/lightning/bolts/blob/master/01-messaging.md)) a connection with our peer.

To begin our journey, let's start with the end. Imagine Alice and Bob decide to open a channel between themselves. Alice is funding this channel, so she will publish the funding transaction. Once the channel is established, we'll have the following three transactions:
- Funding Transaction
- Alice's Commitment Transaction
- Bob's Commitment Transaction

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/operations/open_channel_keys.png" alt="open_channel_keys" width="40%" height="auto">
</p>

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/operations/open_channel_txs.png" alt="open_channel_txs" width="100%" height="auto">
</p>

#### Question: Look at the above diagram. What information will Alice and Bob have to provide each other? For instance, Alice will create the Funding Transaction and her Commitment Transaction - what information does she need from Bob to do this?


<details>
  <summary>Answer</summary>

**Funding Transaction**
- Alice needs Bob's funding public key for the funding transaction. Since the Transaction ID is a hash of a subset of the transaction data (not the witness!), we'll also need Bob's funding public key before we can calculate this.

</details>

## Open Channel Message

Since Alice is funding the channel (providing the input UTXO for the funding transcaion), she will begin the process by sending Bob an `open_channel` message.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/operations/open_channel_msg.png" alt="open_channel_msg" width="100%" height="auto">
</p>

In the above picture, you'll notice that some fields have been grayed out. This is because, at this point in the channel establishment process, these fields are not yet known to Alice or Bob. For example, on Alice's side, she does not yet have Bob's funding public key, so she does not yet know the transaction ID.

When Alice sends Bob the `open_channel` message, she is essentially proposing a Lightning channel contract to Bob. The contract will stipulate important requirements information that Alice requires (or desires) to operate a channel with Bob. For example, consider the following fields in the `open_channel` message:
- `max_htlc_value_in_flight_msat`: The maximum value of *outstanding* HTLCs that Bob can offer.
- `max_accepted_htlcs`: The maximum number of *outstanding* HTLCs that Bob can offer.
- `channel_reserve_satoshis`: The minimum value that Bob must keep on his side of the channel. In other words, in outputs that pay directly to him.

Bob will then evaluate Alice's proposed channel and, if acceptable, he will send back a `accept_channel` message.

#### Question: Why would Alice request a channel reserve for Bob?

<details>
  <summary>Answer</summary>

**Funding Transaction**
- Alice needs Bob's funding public key for the funding transaction. Since the Transaction ID is a hash of a subset of the transaction data (not the witness!), we'll also need Bob's funding public key before we can calculate this.

</details>


## Accept Channel Message

If Bob agrees to Alice's channel proposition, he will send back an `accept_channel` message. His `accept_channel` message will it's own set of requirements that Alice must agree to. For example, one field that Bob proposes is `minimum_depth`, which provides the minimum number of blocks that must be mined on top of the funding transaction before the channel is live. This parameter is provided by the node which is *not* the funder (ex: Bob) because it's meant to protect Bob against Alice double-spending the funding transaction. See below for an example for how such an attack could be carried out:
1) Alice publishes the funding transaction with a low feerate.
2) Bob see the funding transaction in the mempool and, incorrectly, assumes it's safe to start operating the channel
3) Alice sends bob a payment, updating their channel state.
4) Alice secretly creates a new transaction with a high feerate, double-spending the funds in the funding transaction (which is still not yet confirmed)

Ouch!

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/operations/accept_channel_msg.png" alt="accept_channel_msg" width="100%" height="auto">
</p>

#### Question: Sometimes, channel partners may agree to operate a "zero-conf" channel, where they start sending payment to eachother once the funding transaction is in both of their mempools (before it's mined). Why would they do this? What are other risks in addition to the above?

<details>
  <summary>Answer</summary>

Generally, channel partners open a zero-conf channel in circumstances where they wish to start using the channel immediately. For example, imagine you're setting up a Lightning wallet with a reputable Lightning Service Provider (LSP), and you wish to start using your wallet to send payments right away. The LSP may offer zero-conf channels so that their users have a better user experience.

Notice, an important pre-requisite to zero-conf channels is some degree of trust between the two parties. This is because the funding transaction is not solidified in the blockchain until it's been mined. Additionally, due to re-organizations, it's often recommended to wait around 6 blocks before operating a channel.

</details>


## Funding Created Message
Assuming Alice agrees to the channel propositions proposed in Bob's `accept_channel` message, she will then send Bob a `funding_created` message. In this message, Alice will provide Bob with the information he needs to be able to complete his commitment transaction for the initial channel state - namely the funding transaction TXID, output index, and Alice's signature, which Bob can use in the witness stack if he ever wishes to publish his commitment transaction.

At this point, the only information needed to complete the **Channel Establishment v1** process is a signature from Bob for Alice's commitment transaction. 

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/operations/funding_created_msg.png" alt="funding_created_msg" width="100%" height="auto">
</p>

## Funding Signed Message
Finally, in response to Alice's `funding_created` message, Bob will send Alice a `funding_signed` message. This will contain a `channel_id` and Bob's signature, Which Alice can use for her commitment transaction. Note, since Alice can theoretically have multiple channels with Bob, the `channel_id` field allows Bob to specify which channel he is sending a signature for.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/operations/funding_signed_msg.png" alt="funding_created_msg" width="100%" height="auto">
</p>


## Channel Ready Message
After recieving Bob's `funding_signed` message, Alice is now able to broadcast the funding transaction safely. This is because, if Bob were to disapear, she can always publish her commitment transaction, which spends the funds from the multi-sig back to herself.

Once Alice broadcasts the funding transaction and its received sufficient confirmations, Alice will send Bob a `channel_ready` message, indicating that the channel is ready for use. Similarly, Bob will send a `channel_ready` message to Alice once he verifies himself that the funding transaction has received sufficient confirmations on-chain.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/operations/funding_signed_msg.png" alt="funding_created_msg" width="100%" height="auto">
</p>


# Opening A Channel In LDK
To open a channel in LDK, we'll *start* by using the `create_channel` function, which is made available via the `ChannelManager`. Notice how we said "start" in the first sentence? The `create_channel` function won't do everything for us. As we learned earlier, there are quite a few steps to opening a channel. Instead, this function will focus on creating an `open_channel` message for us and sending it to the desired recipient. Once it's sent, we'll have to be on the lookout for a respond from our peer. More on that soon!

When using this function, we'll have to pass in the following parameters:
- `their_network_key`: The public key of the peer we'd like to connect with
- `channel_value_satoshis`: The total channel capacity. This is the amount that we're going to fund the channel with.
- `push_msat`: If we'd like to push an amount (in sats) on our channel partner's side when opening the channel, we can specify that amount here.
- `user_channel_id`: This is an mapping that we, the developers, can create when opening a channel. It has no specific use in LDK. Instead, it is passed back to us in the `FudningGenerationReady` message and allows the developer to track channels themselves with their own mapping scheme, if they would like.
- `temporary_channel_id`: If a temporary channel ID is specified, it will be used as the temporary channel ID for this channel. Otherwise, a random one will be generated.
- `override_config`: Developers can choose to implement custom channel configurations or use LDKs defaults.

```rust
pub fn create_channel(
    their_network_key: PublicKey,
    channel_value_satoshis: u64,
    push_msat: u64,
    user_channel_id: u128,
    temporary_channel_id: Option<ChannelId>,
    override_config: Option<UserConfig>,
) -> Result<ChannelId, APIError>
```

## ⚡️ Complete `channel_open`



## Funding Generation Ready
When our counterparty responds to us, confirming that they have accepted our channel, we'll recieve a `FundingGenerationReady` `Event` from LDK. As the event name suggests, this notification informs us that our counterparty has accepted our channel, and they are ready for us to send them a `funding_created` message. However, before we can do this, we need to actually create the funding transaction so that we have the transaction ID and output index to provide to our counterparty.

To accomplish this, we'll update our **Event Handler** to properly respond when a `FundingGenerationReady` `Event` is provided to us. Specifically, we'll have to complete the following steps:
1) Use the information provided to us in the `FundingGenerationReady` event to create the funding transaction.
2) Pass the funding transaction to the channel manager via the `funding_transaction_generated` function.

```rust
match event {
  Event::FundingGenerationReady {
    temporary_channel_id,
    channel_value_satoshis,
    output_script,
    user_channel_id
  } => {
    // implement code to generate funding transaction here

    // pass to channel_manager.funding_transaction_generated()
  }
}
```
After we pass the funding transaction to `funding_transaction_generated`, LDK will take care of broadcasting the transaction for us, notifying us with a `ChannelReady` event once the funding transaction is sufficiently confirmed on-chain.



