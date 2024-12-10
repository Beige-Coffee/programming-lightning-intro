# Timelocks

Timelocks are a neat feature in Bitcoin that will become **very** useful as we begin to build our super fast payment protocol, Lightning.

Timelocks allow us to lock bitcoin such that they can only be spent after a certain amount of time passes. For example, we can lock bitcoin so that they can only be spent after 144 blocks from when they are mined. Or, we can accidentally lock them  forever - fun!

## Relative & Absolute Timelocks
There are multiple ways to specify time-based conditions on our transactions. Generally speaking, timelocks can either be **relative** or **absolute**.
- A **relative** timelock will lock bitcoin until either a certain **number of blocks** or **seconds** have passed since the input or output is first mined in a block. For example, if I add a relative timlock of 144 blocks to an output, then that output is only spendable 144 blocks (~1 day) *after* it is first mined within a block.
- An **absolute** timelock will lock bitcoin until either a given **block height** or **unix timestamp** is reached. For example, if I timelock an output to block height 2,000,000, then the output cannot be spent until after block 2,000,000.

## Timelocks in Bitcoin Transactions
Timelock conditions can be placed in the following three locations within a transaction:
- **```nLocktime```**: This is a **transaction level** timelock that locks the entire transaction to an **absolute** timelock. If a timelock condition is placed in this field, then the transaction itself cannot be mined until **after** a specific block height or time has passed.

- **```nSequence```**: This is an **input level** timelock that locks an input to a **relative** timelock. If a timelock condition is placed on an input, then the input cannot be spent until that **amount of blocks or time has passed since the output it's referencing was mined**. Therefore, this input is only able to be mined after that time period has passed! It's also important to note that the ```nSquence``` field plays a par

- **```scriptPubKey```**: Using the script programming language, you can lock **outputs** with both a **relative** and **absolute** timelock. This ensures that the outputs cannot be spent until either the block height (or timestamp) has been reached or that the outputs cannot be spent until the specified number of block (or seconds) have passed.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/TXLocktimes.png" alt="TXLocktimes" width="90%" height="auto">
</p>

The **```nSequence```** field is quite important when configuring locktimes, so let's take a closer look at it.
- To enable **```nLocktime```** to be used on a transaction, we need to configure the **```nSequence```** for at least one input to be `0xFFFFFFFE` or less.
- To enable **Replace By Fee**, we need to configure the **```nSequence```** for at least one input to be `0xFFFFFFFD` or less.
- To enable **Relative Locktime**, we need to configure the **```nSequence```** to be:
  - **`0x00000000`** to **`0x0000FFFF`**
  - **`0x00400000`** to **`0x0040FFFF`**
  
<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/nsequence.png" alt="nsequence" width="90%" height="auto">
</p>



## ⚡️ Generate a Relative Timelocked OP_CSV Output

For this exercise, we'll generate a **relative timelocked** output. To do this, we'll need to complete the ```csv_p2pkh``` function in the ```lib.rs```. This function will take a ```PublicKey``` and ```height_or_timestamp``` as an input and produce a CSV locked output script.

<details>
  <summary>Click here to see the output script</summary>
  
`<blockheight_or_timestamp> OP_CLTV OP_DROP OP_DUP OP_HASH160 <pubkey_hash> OP_EQUALVERIFY OP_CHECKSIG`

</details>

```rust
fn csv_p2pkh(pubkey: &PublicKey, height_or_timestamp: i64) -> ScriptBuf {
}
```