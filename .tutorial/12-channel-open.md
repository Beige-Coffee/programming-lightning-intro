# Channel Open

Now that Alice and Bob have created their funding transaction and, crucially, refund transaction, they are ready to open their channel. Remember, Alice cannot broadcast the funding transaction without having a refund transaction ready, as this would open Alice up to the risk that Bob stops responding or refuses to cooporate, effectively locking Alice's funds in this channel forever! Also, Bob has every incentive to sign the refund transaction before opening the channel, as he has no bitcoin on his side of the channel at this point anyway, so he is not exposing himself to any risk of loss.

Once these transactions are created, the next step is to broadcast the funding transaction to the Bitcoin network and wait for it to be included in a block. To ensure the transaction is considered final and irreversible, it’s standard practice to wait until the block containing the transaction has at least 6 confirmations (i.e., 6 additional blocks mined on top of it). This is because the Bitcoin network follows the longest chain with the most accumulated proof of work, which is typically the chain with the most blocks. As a result, a transaction could initially be included in a block that gets replaced if a longer chain emerges. By waiting for 6 confirmations, the likelihood of such a reorganization occurring becomes extremely low. 

Therefore as part of our payment channel protocol, we will need to be able to monitor the chain and watch for a block that contains this funding transaction output before we can consider the payment channel safe to operate.

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