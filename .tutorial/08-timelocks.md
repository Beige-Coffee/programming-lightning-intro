# Timelocks

As you'll recall from Mastering Bitcoin there are multiple ways to lock funds such that they are only spendable after a certain amount of time.

If the spending path for Alice to get her funds back were timelocked then Bob could safely hold onto these interim transactions until the timelock was about to expire.

### Absolute Timelocks (OP_CLTV)

One flavor of timelocks utilizes the opcode OP_CLTV to make it so that an output cannot be spent until a specific block height or unix timestamp has been reached.

An absolute timelocked pay-to-pubkey-hash locking script might look like this:
```
<blockheight_or_timestamp> OP_CLTV OP_DROP OP_DUP OP_HASH160 <pubkey_hash> OP_EQUALVERIFY OP_CHECKSIG
```

## ⚡️⚡️⚡️ Building a function to generate an absolute timelocked pay-to-pubkey-hash locking script

Can you write the implementation for a function that takes in a pubkey and a blockheight and generates a cltv-p2pkh locking script? 
```rust
fn cltv_p2pkh(pubkey: &PublicKey, height: i64) -> Script {}
```


### Relative Timelocks (OP_CSV)

Another flavor of timelocks utilizes the opcode OP_CSV to make it so that an output cannot be spent until the output is at least a specific amount of blocks or milliseconds old.

This means the height or time that the output becomes unlocked is relative to the height or time that the output is first mined into a block.

A relative timelocked pay-to-pubkey-hash locking script might look like this:
```
<blockheight_or_timestamp> OP_CSV OP_DROP OP_DUP OP_HASH160 <pubkey_hash> OP_EQUALVERIFY OP_CHECKSIG
```

Just to really hammer home the script Builder interface...

## ⚡️⚡️⚡️ Building a function to generate an relative timelocked pay-to-pubkey-hash locking script

Can you write the implementation for a function that takes in a pubkey and a unix timestamp (as miiliseconds since epoch) and generates a csv-p2pkh locking script? 
```rust
fn csv_p2pkh(pubkey: &PublicKey, timestamp: i64) -> Script {}
```

<br/><br/>
## What kind of timelock should we use?

So we know we want to timelock Alice's refund spending path so that she can get her funds back after some amount of time should Bob become unresponsive or uncooperative but what version should we use for our payment channels?

Well we could really go either way but I'll argue that relative timelocks give us a slightly better user experience.


### Maximum age of the channel

When Alice proposes a channel to Bob she will have to provide a blockheight or timestamp to use for the refund locking script.  

Bob might prefer a certain minimum max age for the channels he participates in.  After Bob tells Alice that he accepts the proposed channel parameters it's up to Alice to construct, fund, and broadcast the funding transaction.

*There's nothing that guarantees when Alice might actually broadcast the funding transaction.*

With absolute timelocks, the longer she waits to broadcast the transaction the less overall time the channel will be able to be active.

With relative timelocks the channel will always be able to be active for the same amount of time regardless of how long it takes Alice to broadcast the transaction.