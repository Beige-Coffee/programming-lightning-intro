# Channel Close

How does Alice know when Bob closes the channel? She needs to monitor blocks as they are connected to the chain and check to see if the channel funding output has been spent.  Similar to the `block_connected` we wrote earlier to detect the funding transaction:


<br/><br/>

## ⚡️ Write a function `channel_closed` that is called when new blocks are found and returns whether or not the funding outpoint was spent in the block

<br/>

A `Block` has the following structure:

```rust
pub struct Block {
    pub header: BlockHeader,
    pub txdata: Vec<Transaction>,
}
```

<br/>

A `Transaction` has the following structure:

```rust
pub struct Transaction {
    pub version: i32,
    pub lock_time: PackedLockTime,
    pub input: Vec<TxIn>,
    pub output: Vec<TxOut>,
}
```

<br/>

A `TxIn` has the following structure:

```rust
pub struct TxIn {
    pub previous_output: OutPoint,
    pub script_sig: Script,
    pub sequence: Sequence,
    pub witness: Witness,
}
```

<br/>

A `OutPoint` has the following structure:

```rust
pub struct OutPoint {
    pub txid: Txid,
    pub vout: u32,
}
```

<br/>

We want to iterate over the blocks transactions and for each transaction iterate over all of it's inputs.  We are looking for an input that has the a `script_pubkey` equal to the `funding_output` script and an `amount` equal to the `channel_amount_sats`.

```rust
fn channel_closed(funding_outpoint: OutPoint, block: Block) -> bool {}
```