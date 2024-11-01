# Timelocks

Timelocks are a neat feature in Bitcoin that allow us to lock bitcoin such that they can only be spent after a certain amount of time passes. For example, we can lock bitcoin so that they can only be spent after 144 blocks from when they are mined. That said, if we're not careful, we can also lock our bitcoin forever. Ouch!

There are multiple ways to specify time-based conditions on our transactions. Generally speaking, timelocks can either be **relative** or **absolute**.
- A **relative** timelock will lock bitcoin until either a certain number of blocks or seconds have passed since the input or output is first mined in a block. For example, if I add a relative timlock of 144 blocks to an output, then that output is only spendable 144 blocks (~1 day) *after* it is first mined within a block.
- An **absolute** timelock will lock bitcoin until either a given **block height** or **unix timestamp** is reached. For example, if I timelock an output to block height 2,000,000, then the output cannot be spent until after block 2,000,000.

Timelock conditions can be placed in the following three locations within a transaction:
- **```nLocktime```**: This is a **transaction level** timelock that locks the entire transaction to an **absolute** timelock. If a timelock condition is placed in this field, then the transaction itself cannot be mined until **after** a specific block height or time has passed.

- **```nSquence```**: This is an **input level** timelock that locks an input to a **relative** timelock. If a timelock condition is placed on an input, then the input cannot be spent until that **amount of blocks or time has passed**. It's important to note that this "timer" only starts when the transaction it mined!

- **```scriptPubKey```**: Using the script programming language, you can lock **outputs** with both a **relative** and **absolute** timelock. This ensures that the outputs cannot be spent until either the block height (or timestamp) has been reached or that the outputs cannot be spent until the specified number of block (or seconds) have passed.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/TXLocktimes.png" alt="TXLocktimes" width="95%" height="auto">
</p>

<br/>

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


## ⚡️ Generating an absolute timelocked Pay-to-Public-Key-Hash locking script

Can you write the implementation for a function that takes in a pubkey and a blockheight and generates a cltv-p2pkh locking script? As a reminder, a P2PKH script looks like the follwoing:

```
OP_DUP OP_HASH160 <pubkey_hash> OP_EQUALVERIFY OP_CHECKSIG
```

To add an absolute timelock, we'll need to add the ```OP_CLTV``` conditions to the beginning of the script. If you need a hint, you can click "hint" below.

<br/>
<details>
  <summary>Hint</summary>

Try placing ```<blockheight_or_timestamp> OP_CLTV OP_DROP``` before the P2PKH script you created in an earlier exercise.

</details>

<br/>


```rust
fn cltv_p2pkh(pubkey: &PublicKey, height: i64) -> Script {}
```


<br/><br/>

## ⚡️ Generating an relative timelocked Pay-to-Public-Key-Hash locking script

Can you write the implementation for a function that takes in a pubkey and a unix timestamp (as miiliseconds since epoch) and generates a csv-p2pkh locking script? 
```rust
fn csv_p2pkh(pubkey: &PublicKey, timestamp: i64) -> Script {}
```

<br/><br/>
### Question: TO-DO 

- What are the trade-offs of the two types of locks we can use?