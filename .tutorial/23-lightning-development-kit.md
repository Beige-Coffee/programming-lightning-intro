# Lightning Development Kit

It would be a massive undertaking to attempt to implement the entire functionality specified in the BOLTS in order to operate a node on the lightning network.  There are a few of out-of-the-box implementations available to you if all you would like to do is operate a node on the lightning network.  If you want to customize the behavior of how they work you either need to fork and then maintain the project or write plugins that would alter the default behavior. Regardless of the path you take they all are still delivered as a binary and need to be run and controlled over various RPC interfaces.

The lightning development kit is a library that implements the lightning network protocol but gives you the ability to integrate it directly into your application in exactly the fashion that best suits your needs.

It exposes a broad but extremely powerful set of objects and events for you to utilize when customizing your users' lightning experience.  

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/ldk-architecture.svg" alt="ldk-architecture" width="100%" height="auto">
</p>

A better description and overview of these components can be found on their website and corresponding documentation but here's a quick glimpse into the high level components we will be interacting with:

### ChannelManager

Exposes dozens of methods for opening, making payments over, and closing channels.  This handles the majority of the logic required for operating channels on the lightning network.

### PeerManager

Handles all of the p2p protocol messaging layer.  Flexible enough to allow you to bring your own networking stack.  Provide LDK with raw TCP/IP socket data and the library will handle the rest for you.  LDK does provide a default implementation should your needs not require a custom networking layer. 

### KeysInterface

An abstraction over all of the key material you will need to manage.  Again, LDK provides sane default implementation for some applications while giving you the flexibility to customize to fit whatever key management requirements you might have.

### Persist interface

An abstraction over key-value storage to persist essential lightning protocol data.  LDK provides an implementation that uses a filesystem to persist all of your data but you can easily plug any database or storage solution.

### Listen / Confirm interfaces

LDK needs to be notified of new blocks and/or confirmed transactions.  If you have access to full blocks via bitcoind then you'd want to implement the `Listen` interface.  If you only have access to transactions relevant to specific outputs via electrum or esplora then you'd want to implement the `Confirm` interface.

LDK does provide implementations for using bitcoind (`lightning-block-sync`) and for esplora (`lightning-transaction-sync`) out-of-the-box.

