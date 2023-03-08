# Funding Output

We have learned that we want our payment channel funding transaction output to have two spending paths.  One spending path is the 2-of-2 multisig that requires both parties signatures to spend and the other path is a relative timelocked pay-to-pubkey-hash that only requires the funders signature after some amount of time.

The 2-of-2 multisig path will be spent by payments that are made during normal channel operation and the timelocked path will be spent by Alice in a refund transaction if Bob becomes unresponsive.

## ⚡️⚡️⚡️ Building a function to generate our simple payment channel's funding output script given the two parties public keys and a maximum age of the channel in blocks.

Let's bring it all together and a function we can use to generate a funding transaction for our simple payment channels 
```rust
fn payment_channel_funding_output(alice_pubkey: &PublicKey, bob_pubkey: &PublicKey, height: i64) -> Script {}
```