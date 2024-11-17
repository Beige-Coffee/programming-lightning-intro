# Create Simple Payment Verification (SPV) Client

Great, we now have the ability to create a new `BitcoindClient` instance, which implements the `BlockSource` using an `RpcClient`. Now, we have to instantiate an SPV client which will periodicall poll our `BlockSource` for the best chain tip and communicate any new (or reorganized) blocks to various LDK components.

## Simple Payment Verification (SPV)
Before digging into the LDK implementation, let's briefly review the concept behind **Simple Payment Verification**.

Many Bitcoin applications are designed to run on resource-constrained machines such as phones. It's not feasible or desirable for these applications to store a full bitcoin blockchain so that they can monitor and validate transactions themselves. To get around this, they implement SPV and only download block headers, resulting in a 1000x smaller storage requirement compared to the full blockchain.

Our lightning node will leverage a SPV client to periodically fetch block chain headers from our `BlockSource` implementation. Whenever it recieves a new block, it will validate the proof-of-work and pass the block to LDK components that need to be aware of new blocks, such as the `ChannelManger` and `ChainMonitor`.

## Simple Payment Verification (SPV) in LDK
LDK provides a `SPVClient` within the `lightning-block-sync` crate that we can use to implement a SPV.

```rust
pub struct SpvClient<P: Poll, C: Cache, L: Deref>
where
    L::Target: Listen,
```
As you can see above, this client is parameterized with the following components:
- `P: Poll`: A trait for polling block sources and retrieving chain data. This is used to fetch new blocks and maintain chain synchronization. [Click here to see docs.](https://docs.rs/lightning-block-sync/latest/lightning_block_sync/poll/trait.Poll.html)
- `C: Cache`: A trait for managing the block header cache. [Click here to see docs.](
https://docs.rs/lightning-block-sync/latest/lightning_block_sync/trait.Cache.html)
- ```L``` is basically a wrapper around something that implements the ```Listen``` trait, which listens for new blocks. In practice, this is something like a `ChannelManager` or `ChainMontitor`, which needs to know when new blocks arrive or when the chain reorganizes.[Click here to see docs.](https://docs.rs/lightning/0.0.125/lightning/chain/trait.Listen.html)

## ⚡️ Build a SPV Client
For this exercise, we'll build a SPV client for our Lightning node. To do this, we'll complete the asynchronous function `poll_for_blocks`, seen below.

```rust
pub async fn poll_for_blocks<L: Listen>(bitcoind: BitcoindClientExercise, network: Network,
                   listener: L) {

    let best_block_header = validate_best_block_header(&bitcoind).await.unwrap();

    let poller = ChainPoller::new(&bitcoind, network);

    let mut cache = HashMap::new();

    let mut spv_client = SpvClient::new(best_block_header, poller, & mut cache, &listener);

    loop {
        let best_block = spv_client.poll_best_tip().await.unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
```

This function will take an instance of the `BitcoindClientExercise` we created in a prior exercise, a `Network` (in this case, Regtest), and  `listener`, which implements the `Listen` trait.

For this exercise, use the `new` method on available on the `SpvCient`. [You can read the docs here.](https://docs.rs/lightning-block-sync/latest/lightning_block_sync/struct.SpvClient.html#method.new)