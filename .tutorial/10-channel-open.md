# Channel Open

With the ability to construct the funding transaction output that we will lock our channel funds into the next steps would be to build the transaction and broadcast it to the chain.

As part of our payment channel protocol we will need to be able to monitor the chain and watch for a block that contains this funding transaction output before we can consider the payment channel safe to operate.

<br/><br/>

## ⚡️ Write a function `block_connected` that is called when new blocks are found and returns whether or not a valid funding output was found in this block

We want to make sure the funding output script is included in the block *and* that the channel amount is correct.

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

A `TxOut` has the following structure:

```rust
pub struct TxOut {
    pub value: u64,
    pub script_pubkey: Script,
}
```

<br/>

A `Vec` in rust is an array type and can be iterated using `for` item `in` array loop like this:

```rust
let arr = vec![1,2,3,4,5];

for num in arr {
  println!("{num}"); 
}
```

<br/>

We want to iterate over the blocks transactions and for each transaction iterate over all of it's outputs.  We are looking for an output that has the a `script_pubkey` equal to the `funding_output` script and an `amount` equal to the `channel_amount_sats`.

```rust
fn block_connected(funding_output: Script, channel_amount_sats: u64, block: Block) -> bool {}
```