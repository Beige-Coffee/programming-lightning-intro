# Timelocks

Timelocks are a neat feature in Bitcoin that allow us to lock bitcoin such that they can only be spent after a certain amount of time passes. For example, we can lock bitcoin so that they can only be spent after 144 blocks from when they are mined. Or, we can accidentally lock them  forever - fun!

There are multiple ways to specify time-based conditions on our transactions. Generally speaking, timelocks can either be **relative** or **absolute**.
- A **relative** timelock will lock bitcoin until either a certain number of blocks or seconds have passed since the input or output is first mined in a block. For example, if I add a relative timlock of 144 blocks to an output, then that output is only spendable 144 blocks (~1 day) *after* it is first mined within a block.
- An **absolute** timelock will lock bitcoin until either a given **block height** or **unix timestamp** is reached. For example, if I timelock an output to block height 2,000,000, then the output cannot be spent until after block 2,000,000.

Timelock conditions can be placed in the following three locations within a transaction:
- **```nLocktime```**: This is a **transaction level** timelock that locks the entire transaction to an **absolute** timelock. If a timelock condition is placed in this field, then the transaction itself cannot be mined until **after** a specific block height or time has passed.

- **```nSquence```**: This is an **input level** timelock that locks an input to a **relative** timelock. If a timelock condition is placed on an input, then the input cannot be spent until that **amount of blocks or time has passed since the output it's referencing was mined**. Therefore, this input is only able to be mined after that time period has passed! It's also important to note that the ```nSquence``` field plays a par

- **```scriptPubKey```**: Using the script programming language, you can lock **outputs** with both a **relative** and **absolute** timelock. This ensures that the outputs cannot be spent until either the block height (or timestamp) has been reached or that the outputs cannot be spent until the specified number of block (or seconds) have passed.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/TXLocktimes.png" alt="TXLocktimes" width="95%" height="auto">
</p>

<br/>

<details>
  <summary>Click to view how timelock conditions are processed by the Bitcoin protocol!</summary>

Timelock conditions are processed as follows:
- **```nLocktime```**:
  - If ```nLocktime``` is set to ```<=499999999```, then the ```nLocktime``` field is read as a **block height**.
  - If ```nLocktime``` is set to ```>=500000000```, then the ```nLocktime``` field is read as a **timestamp**.

- **```nSquence```**:
  - If ```nSquence``` is set to ```0x00000000 to 0x0000FFFF```, then ```nSquence``` is read as the **number of blocks**.
  - If ```nSquence``` is set to ```0x00400000 to 0x0040FFFF```, then ```nSquence``` is read as the **number of seconds**.

- **```scriptPubKey```**:
  - To lock the output to an **absolute** block height or timestamp, you would, generally, use the following script: ```<blockheight_or_timestamp> OP_CLTV OP_DROP```
  - To lock the output to a **relative** number of blocks or seconds, you would, generally, use the following script: ```<blockheight_or_timestamp> OP_CSV OP_DROP```
  - It's worth mentioning that both scripts are preceded by ```OP_DROP``` because they both leave whichever item they are checking (block height or timestamp) on the stack after the evaluation, so we need to drop (or remove) it with ```OP_DROP``` before moving on to the rest of the script.

</details>


## ⚡️ Generate a Timelocked Transaction

For this exercise, we'll generate a timelocked transaction that satisfies the following two conditions:
1) The transaction is timelocked using nTimelock (measured in blocks). This means the transaction cannot be mined until a certain block height is reached.
2) The output will be encumbered via P2PKH, but there is a required **relative** delay (measured in blocks), such that the output cannot be used as an input until the delay has passed.

To complete this exercise, we'll utilize the following structures that are provided by rust-bitcoin.

``Transaction`` provides the structure for representing a Bitcoin transaction. Each field within this struct requires that it be of a specific type. For example, the ```version``` must be a ```Version``` type. You can read more about this struct [here](https://docs.rs/bitcoin/latest/bitcoin/struct.Transaction.html).
```rust
pub struct Transaction {
    pub version: Version,
    pub lock_time: LockTime,
    pub input: Vec<TxIn>,
    pub output: Vec<TxOut>,
}
```

A `TxOut` provides the structes for representing a transaction output in rust-bitcoin. You can read more about this struct [here](https://docs.rs/bitcoin/latest/bitcoin/struct.TxOut.html).

```rust
pub struct TxOut {
    pub value: Amount,
    pub script_pubkey: ScriptBuf,
}
```

#### Exercise #1: Create P2PKH Output Script

We'll start this exercise by completing the ```csv_p2pkh``` function in the ```lib.rs```. This function will take a ```PublicKey``` and ```height_or_timestamp``` as an input and produce a CSV locked P2PKH output script.

<details>
  <summary>Click here to see the output script</summary>
  
`<blockheight_or_timestamp> OP_CLTV OP_DROP OP_DUP OP_HASH160 <pubkey_hash> OP_EQUALVERIFY OP_CHECKSIG`

</details>

```rust
fn csv_p2pkh(pubkey: &PublicKey, height_or_timestamp: i64) -> ScriptBuf {
}
```

#### Exercise #2: Build Output

Next, let's complete the ```build_output``` function. This function will take an ```Amount``` and ```output_script``` as an input and produce a TxOut structure that can be used in a rust bitcoin ```Transaction```.

```rust
fn build_output(amount: Amount, output_script: ScriptBuf) -> TxOut {
}
```

#### Exercise #3: Build Transaction

Finally, let's complete the ```build_timelocked_transaction``` function. This function will take a ```txins``` structs (transaction inputs), ```PublicKey```,  ```block_height``` (nLocktime), ```csv_delay```, and ```amount```. You can use the functions you created in the previous exercises to help. Comments have been provided below to guide you.

The below tips will help you as you complete the exercise:
- The ```version``` field in ```Transaction``` expects a ```Version``` structure. You can use ```Version::ONE``` for legacy transactions and ```Version::TWO``` for SegWit. This exercise invovles building a legacy transaction.
- The ```lock_time``` field expects a ```Locktime``` structure. This structure has a method ```LockTime::from_height()```, which can be used to create an absolute block height locktime.
- The ```output``` field expects a ```vec``` data type. you can wrap your output in ```vec![ output ]``` to satisfy this expectation. That same is true for ```input```, but that is provided as part of this function, so you don't have to worry about that.

```rust
fn build_timelocked_transaction(
    txins: Vec<TxIn>,
    pubkey: &PublicKey,
    block_height: u64,
    csv_delay: u32,
    amount: Amount,
) -> Transaction {

    // step 1. create csv p2pkh output script

    // step 2. create txout

    // step 3. create transaction
}
```

<details>
  <summary>Click here for a hint if you get stuck</summary>

```rust
fn build_timelocked_transaction(
    txins: Vec<TxIn>,
    pubkey: &PublicKey,
    block_height: u64,
    csv_delay: u32,
    amount: Amount,
) -> Transaction {

    // step 1. create csv p2pkh output script

    // step 2. create txout

    // step 3. create transaction
    Transaction {
        version: // version here,
        lock_time: // locktime here,
        input: // txins here,
        output: vec![// txout here
                    ],
    }
}
```
</details>
