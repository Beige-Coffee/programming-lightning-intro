# LDK Components

Great, we've implemented the `Persist` interface and are well on our way towards building a fully functioning Lightning node. At this point, we have a few interfaces left that need to be implemented - namely, handling events and connecting to peers. However, before we can implement those interfaces, we'll need to instantiate some LDK components that implement the Lightning protocol for us.

So far, we've focused our attention on implementing **interfaces**, which empower developers to customize LDK so that it fits their specific use case. Now that we've completed some of these integrations, we can begin initializing LDK components that rely upon these interfaces to be defined before they can do their job.

The main components that we'll explore as part of this workshop can be mentally compartmentalized into the following categories:
- **Network Management**: These components handle our node's internal representation of the Lightning Network graph, as well as finding the optimal route for a given payment.
- **Channel Management**: These components help run the Lightning state machine, assisting in vital operations such as opening/closing channels, sending payments, and monitoring the blockchain so that our node can broadcast punishment transaction if needed. 
- **Peer Management**: These components implement Lightning's P2P messaging layer, sending and processing gossip messages for our Lightning node.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/node_setup/ldk_major_components.png" alt="ldk_major_components" width="100%" height="auto">
</p>

