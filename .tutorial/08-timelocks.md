# Timelocks

As you'll recall from Mastering Bitcoin there are multiple ways to lock funds such that they are only spendable after a certain amount of time.

<br/>

## Absolute Timelocks (OP_CLTV)

One flavor of timelocks utilizes the opcode OP_CLTV to make it so that an output cannot be spent until a specific block height or unix timestamp has been reached.

An absolute timelocked pay-to-pubkey-hash locking script might look like this:
```
<blockheight_or_timestamp> OP_CLTV OP_DROP OP_DUP OP_HASH160 <pubkey_hash> OP_EQUALVERIFY OP_CHECKSIG
```

<br/><br/>

## ⚡️ Generating an absolute timelocked pay-to-pubkey-hash locking script

Can you write the implementation for a function that takes in a pubkey and a blockheight and generates a cltv-p2pkh locking script? 
```rust
fn cltv_p2pkh(pubkey: &PublicKey, height: i64) -> Script {}
```


<br/><br/>

## Relative Timelocks (OP_CSV)

Another flavor of timelocks utilizes the opcode OP_CSV to make it so that an output cannot be spent until the output is at least a specific amount of blocks or milliseconds old.

This means the height or time that the output becomes unlocked is relative to the height or time that the output is first mined into a block.

A relative timelocked pay-to-pubkey-hash locking script might look like this:
```
<blockheight_or_timestamp> OP_CSV OP_DROP OP_DUP OP_HASH160 <pubkey_hash> OP_EQUALVERIFY OP_CHECKSIG
```

<br/><br/>

## ⚡️ Generating an relative timelocked pay-to-pubkey-hash locking script

Can you write the implementation for a function that takes in a pubkey and a unix timestamp (as miiliseconds since epoch) and generates a csv-p2pkh locking script? 
```rust
fn csv_p2pkh(pubkey: &PublicKey, timestamp: i64) -> Script {}
```

<br/><br/>
## Question

In our payment channel protocol Alice will propose a locktime for her refund path to Bob when opening the channel.  

- What are the trade-offs of the two types of locks we can use?

<br/><br/>

<details>
  <summary>Answer</summary>
<br/>

*There's nothing that guarantees when Alice might actually broadcast the funding transaction.*

Absolute timelocks have the advantage of being able to have a fixed height or time that the channel will have to be closed by.  A downside is that how long the channel is active is entirely dependent on when Alice broadcasts the tx.


With relative timelocks the channel can be active for a guaranteed amount of time regardless of how long it takes Alice to broadcast the funding transaction.  The downside is that the end date for the channel is unknown until the funding tx is mined.

The decision really comes down to whether you'd prefer to have a known maximum age or a known end date for the channel.

One could argue that a known maximum age is more useful but it really depends on the use-case.