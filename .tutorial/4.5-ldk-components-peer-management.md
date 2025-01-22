# Peer Management

The beauty of the Lightning network is that it's supported by an opt-in decentralized peer-to-peer infrustructure. You can route payments across the world at near-instant speeds by routing them through a network of participants.


<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/node_setup/peer_management.png" alt="peer_management" width="100%" height="auto">
</p>

We've already configured our node to be able to store this network and interact with the gossip protocol, but we're not done just yet! Now we need to configure our node with the ability to handle additional network functionality, such as processing gossip, routing onion messages, and coordinating the various communcations that our Lightning Node will have to employ.

Generally, we can compartmentalize our Lightning node's communication into the following categories:

1) **Gossip**: To maintain a graph of the public network, our node will need to process gossip messages emitted by other nodes, such as:
   - Channel announcements
   - Node announcements
   - Channel updates
2) **Routing**: To ensure that payments are routed privately across the Lightning Network, payment information is wrapped in a series of encrypted layers, called "onion messages". This ensures that, for each hop in a payment, the current hop will only know the previous hop and the next hop in the route.
3) **Channel Management**: To operate a channel ourselves, we'll have to send messages with our peers. For instance, we may ask peer to open a channel with us, stipulating our channel features (fees, delays, etc.), and, if accepted, we'll have to communicate whenever we'd like to update the balance or route HTLCs through this channel.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/node_setup/communication.png" alt="communication" width="100%" height="auto">
</p>


## Initializing Gossip Sync
Let's start implementing our communication functionality by ensuring we're up-to-date with the latest gossip. Scandalous!

To do this LDK provides a `P2PGossipSync` component that, among other helpful utilities, receives and validates P2P gossip from peers. There other ways that we can initialize and keep up-to-date with gossip, such as **Rapid Gossip Sync**, but this will be covered later in the course.

```rust
let gossip_sync = P2PGossipSync::new(network_graph, logger)
```

## Initializing Onion Messenging (Routing)

```rust
// Create the onion messenger. This must use the same `keys_manager` as is passed to your
// ChannelManager.
let onion_messenger = OnionMessenger::new(
    &keys_manager, &keys_manager, logger, &node_id_lookup, message_router,
    &offers_message_handler, &async_payments_message_handler, &dns_resolution_message_handler,
    &custom_message_handler,
);
```

## Initializing PeerManager
```
let mut ephemeral_bytes = [0; 32];
let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
rand::thread_rng().fill_bytes(&mut ephemeral_bytes);
let lightning_msg_handler = MessageHandler {
  chan_handler: channel_manager.clone(),
  route_handler: gossip_sync.clone(),
  onion_message_handler: onion_messenger.clone(),
  custom_message_handler: IgnoringMessageHandler {},
};
let peer_manager: Arc<PeerManager> = Arc::new(PeerManager::new(
  lightning_msg_handler,
  current_time.try_into().unwrap(),
  &ephemeral_bytes,
  logger.clone(),
  Arc::clone(&keys_manager),
));
```