# Channel Management

Wow! While it may feel like we've implemented a lot so far (we have!), we've really only scratched the surface. As of now, our Lightning node can accomplish tasks such as:
- Fetch new blocks
- Broadcast transactions
- Estimate transaction fees
- Manage our node's keys
- Communicate with peers via Lightning's Gossip protocol

However, we're still missing two major pieces of the puzzle. We need to equip our node with the ability to leverage what we've built so far and coordinate the core operations of our Lightning node, including:
- Managing channel lifecycles (opening, closing, updating)
- Processing payments (sending, receiving, forwarding)
- Maintaining and advancing channel commitment transactions
- Responding to on-chain events that affect our channels

To implement this functionality, LDK provides a `ChainMonitor`, `ChannelMonitor`, and `ChannelManager`. The `ChainMonitor` and `ChannelMonitor` monitor on-chain activity, ensuring our node is able to react to any events that require our attention, such as a counterparty broadcasting an old commitment state. On the other hand, the `ChannelManager` focuses on off-chain activity, such as communicating with peers to advance channels state (ex: opening channels, sending payments, etc.). 


<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/node_setup/channel_management.png" alt="channel_management" width="100%" height="auto">
</p>

## ChainMonitor

LDK provides a `ChainMonitor` structure to connect ***off-chain*** **channel management** with ***on-chain*** **transaction monitoring**. To accomplish this, the `ChainMonitor` will track one or more `ChannelMonitor`s - one for each channel. The `ChannelMonitor` will instruct the `ChainMonitor` which outputs it should be monitoring for and, in return, the `ChainMonitor` will inform the `ChannelMonitor` if any events take place that require action. For example, if a counterparty publishes an old commitment transaction, our `ChainMonitor` will identify the output on-chain and inform the `ChannelMonitor`. The `ChannelMonitor` will proceed to publish the punishment transaction, claiming all of the channel's funds.

It's also important to note that the `ChannelMonitor` is responsible for storing extensive and cricical information needed to manage each channel. For example, in addition to storing basic information such as a channel's funding transaction output, it will store information such as counterparty revocation basepoints, HTLC statuses, payment preimages for inbound payments, and much more. In short, the `ChannelMonitor` is absolutely vital to our node's continued operations (more on this soon!).

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/node_setup/chain_monitor.png" alt="chain_monitor" width="100%" height="auto">
</p>


### Initializing The ChainMonitor

To instantiate a new ChainMonitor, LDK provides a new method on the ChainMonitor structure. All you need to do is provide the following inputs:
- An optional chain source that implements the chain::Filter trait. For example, if your node is obtaining pre-filtered blocks or only fetching blocks if the compact filter matches, then it is important to notify the chain source which transactions are of interest. This option is particularly important for nodes that are seeking to minimize bandwidth usage and/or data processing, such as mobile clients.
  - If you put None as this input, it indicates that we are not pre-filtering blocks and, instead, fetching full blocks.
- A transaction broadcaster - responsible for broadcasting transactions to the Bitcoin network
- A logger - handles logging of events and errors
- A fee estimator - provides fee rate estimates for transactions
- A persister - handles persistent storage of channel state
);

```rust
let chain_monitor: ChainMonitor::new(
  None,
  broadcaster,
  logger,
  fee_estimator,
  persister,
);
```

### ChannelMonitors & Persistance

We briefly touched on this ealier, but it's certainly worth digging into before moving on. Since the `ChannelMonitor` is responsible for storing extensive amounts of mission-critical data, it's vital that it is persisted every time  our channel-state changes. This is because, if our node were to go offline, we need to be able to access the `ChannelMonitor` data in the exact state is was left off in. Otherwise, we are at risk of losing access to our funds. 

Given the extreme importance of backing up each `ChannelMonitor`, LDK will actually enforce that Lightning channels cannot update to a new state until the channel's `ChannelMonitor` has been persisted and is, therefore, recoverable. This ensures that full channel recovery is possible if our node were to crash or restart. 

Unlike the `ChainMonitor` and some of the other components we've looked at previously, we do not explicity create a `ChannelMonitor` ourselves. Instead, LDK will create a `ChannelMonitor` for us when applicable.

#### Question: Imagine our node goes offline and, when it reboots, we find our ChannelManager was never persisted? What are the various ways we can lose our funds?

<details>
  <summary>Answer</summary>

- Counterparty publishes old TX?
- We no longer have scripts to spend commitment txs?
- We don't have counterparty signatures?
- We no longer have the most up-to-date information to monitor for on-chain transactions
- If it did reboot with old state, we may accidentally publish an old state, and our counterparty will punish us.

</details>

## ChannelManager

Now that we've implememnted the `ChainMonitor`, which equipts our with the ability to monitor on-chain activity, let's focus on off-chain activity. To do this, we'll need to implement the `ChannelManager`, which is our lightning node's channel state machine, handling tasks such as sending, forwarding, and receiving payments.

It's also helpful to discuss the relationship between the `ChannelManager`, `ChainMonitor`, and `ChannelMonitor` a little further. As we've reviewed, the `ChannelManger` is responsible for off-chain activity. As such, it's also responsible for exchanging messages with peers, such as opening, closing, and updating channels. During this process, it will generate a `ChannelMonitor` for each new channel and a `ChannelMonitorUpdate` for each relevant change to a channel. It will then notify the `ChainMonitor` of these updates. Once the `ChainMonitor` aware of the `ChannelMonitor`, it will begin monitoring the on-chain blockchain for relevant activity. If any on-chain activity is needed - for example, broadcasting a punishment transction if a counterparty attempts to cheat, then the `ChainMonitor` will notify the `ChannelManager` that this action has taken place, and the `ChannelManager` will act accordingly and close the channel off-chain. 

The division of on-chain and off-chain activity allows for more advanced and interesting node setups. For example, you could deploy a redundant copy of the `ChainMonitor` on a different server, providing a robust backup so that you can continue to monitor the blockchain and act accoringly, even if your primary node crashes or restarts. Alternatively, if your primary node is resource constrained, you can deploy the `ChainMonitor` on a separate server to provide higher uptime and reliability. This setup is similar to a "watchtower" setup, where you outsource 24/7 blockchain monitoring to ensure you catch any suspicious on-chain activity immediately.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/node_setup/channel_manager.png" alt="channel_manager" width="100%" height="auto">
</p>

### Initializing The ChannelManager

As you may imagine, to initialize the `ChannelManager`, we're going to need to provide it with quite a few interfaces so that it can properly manage our off-chain activity. Most of the below inputs should be familiar to you, as we've reviewed and implemented them during this workshop. The two arguments that you may not be familiar with are `user_config` and `chain_params`.
- `user_config` will contain configuration options for our Lightning node. For example, a user can use this structure to define their routing fee policy, CLTV expiry delta, whether they would like to accept inbound payments, and more. You can read more about it [here](https://docs.rs/lightning/latest/lightning/util/config/struct.UserConfig.html). For this exercise, we'll use the default options provided by LDK.
- `chain_params` will contain the network we're running our LDK node on and the current block height, which we can be fetched using LDK's `validate_best_block_header` method available in the `block_sync` [module](https://docs.rs/lightning-block-sync/latest/lightning_block_sync/init/fn.validate_best_block_header.html).


```rust
let polled_chain_tip = init::validate_best_block_header(bitcoind_client.as_ref())
  .await
  .expect("Failed to fetch best block header and best block");
let polled_best_block = polled_chain_tip.to_best_block();
let polled_best_block_hash = polled_best_block.block_hash;
let chain_params =
  ChainParameters { network: Network::Regtest, best_block: polled_best_block };

let user_config = UserConfig::default()
  
let channel_manager = ChannelManager::new(
  fee_estimator,
  chain_monitor,
  broadcaster,
  router,
  logger,
  entropy_source,
  node_signer,
  signer_provider,
  user_config,
  chain_params,
  current_timestamp
);
```

## If You're Restarting your node...
If you're restarting your node (as opposed to starting up a fresh node with no channels), there are a few extra steps you'll need to take.

### Reading ChannelMonitors

As we reviewed earlier, if our node is starting and has existing channels, we'll need to read each `ChannelMonitor` in from disk and pass them to the `ChainMonitor`.

A basic LDK restart flow would involve the following steps:
1) Read `ChannelMonitor` state from disk.
2) Sync `ChannelMonitor` to current chain tip (as it may be behind if our node has been offline). More on this step shortly!
3) Give `ChannelMonitor` to `ChainMonitor`.

For example, imagine we have read in our channel monitors and stored them in a vector (list). They are wrapped within a broader struct, called `ChannelMonitorData`, which looks like this:

```rust
pub struct ChannelMonitorData {
    monitor: ChannelMonitor,
    funding_outpoint: OutPoint,
}
```

We could inform our `ChainMonitor` of these `ChannelMonitor` by calling the `watch_channel` method on the `ChainMonitor`.

```rust
for monitor_data in channel_monitors {
    chain_monitor.watch_channel(
        monitor_data.funding_outpoint,
        monitor_data.monitor,
    )
```

### Sync ChannelMonitors and ChannelManager

Whenever we're restarting a node, we need to ensure that the `ChannelManager` and all `ChannelMonitor` instances are synced to the same chain tip. During this process, if any of these instances are behind the current chain tip, LDK will catch up, processing each block along the way, to ensure that it is able to react if any on-chain activity occurred that needs our attention.

To sync the `ChannelManager` and each `ChannelMonitor`, we'll ultimately need to call `synchronize_listeners`, a method which LDK provides in its [lightning_block_sync crate](https://docs.rs/lightning-block-sync/latest/lightning_block_sync/init/fn.synchronize_listeners.html). This will perform a one-time sync, ensuring that each component is in-sync with the latest block. To do this, we'll have to pass a *trusted* block source as an argument to this function. Luckily, we implemented a block source earlier in this workshop which fetches blocks directly from our regtest environment, so we can use this block source to synchronize our components!

We'll also need to pass in a `header_cache`, which will store each block header that we fetch during the synchronization process. This cache serves two important purposes:
1. It optimizes the synchronization for multiple components (for example, multiple `ChannelMonitor`s) by avoiding redundant fetches of the same block headers
2. It maintains headers from both the main chain and any forks during chain reorganizations, ensuring we can properly handle disconnecting from one chain and connecting to another

```rust
pub async fn synchronize_listeners<B: Deref + Sized + Send + Sync, C: Cache, L: Listen + ?Sized>(
    block_source: B,
    network: Network,
    header_cache: &mut C,
    chain_listeners: Vec<(BlockHash, &L)>,
) -> BlockSourceResult<ValidatedBlockHeader>
where
    B::Target: BlockSource,
```





#### Preparing The `chain_listeners`

You'll notice above that we also need to pass in `chain_listeners`. This is a vec (list) of components that implement the `chain::Listen` [trait](https://docs.rs/lightning/latest/lightning/chain/trait.Listen.html#method.block_connected). The `Listen` trait is used to notify components when new blocks have been connected or disconnected from the chain. Unsurprisingly, the `ChannelManager` and `ChannelMonitor` implement the `Listen` trait, as they need to be notified of on-chain block activity.

Before we can pass the `ChannelManager` and `ChannelMonitor` into the `synchronize_listeners` function, we need to make sure they are in the correct structure. As we can see from the function definition, this is `Vec<(BlockHash, &L)>`, which is a vector of tuples - each tuple is a block hash and then whichever component we're syncronizing.

For example, we may pass the following tuple:

```rust
[ (block_hash1, chain_monitor1), (block_hash2, chain_monitor2), (block_hash3, channel_manager)]
```

```rust
// Sync ChannelMonitors and ChannelManager to chain tip
let mut chain_listener_channel_monitors = Vec::new();
let mut cache = UnboundedCache::new();

  let mut chain_listeners = vec![
    (channel_manager_blockhash, channel_manager),
  ];

  for monitor_listener_info in chain_listener_channel_monitors.iter_mut() {
    chain_listeners.push((
      monitor_listener_info.0,
      &monitor_listener_info.1 as &(dyn chain::Listen + Send + Sync),
    ));
  }

  init::synchronize_listeners(
    bitcoind_client.as_ref(),
    args.network,
    &mut cache,
    chain_listeners,
  );
```
