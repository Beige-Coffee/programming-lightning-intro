# Adding Time Constraints To Revocable Transactions

We're getting close to building a robust penalty mechanism that allows us to, in practice, revoke old commitment transactions. 

We saw in the last exercise that, while there is a revocation key that the counterparty can use, there are still situations where the owner of the funds can steal them before the counterparty has a chance to act. To resolve this, we can add a relative timelock to the output such that the owner can only use that output as an input to another transaction *after* a certain amount of blocks have been mined. To do this, we can leverage the **Check Sequence Verify** ```OP_CSV``` operation code (also known as **opcode**) within our output script.

By including the ```OP_CSV``` opcode in our script, we ensure that the output cannot be sent to the original owner's address until a pre-specified amount of blocks have passed *since the transaction was mined on chain*. This solves the problem we identified previously because, if a channel partner tried to publish an old state (whether that was publically, via the mempool, or privately, by handing the transacation directly to a miner), the counterparty just has to monitor the blockchain to see if an old transaction is published. If they do see the old transaction, they now have time (usually 144 blocks) to publish claim those funds via the revocation key spending path.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/DelayedRevocationTx.png" alt="DelayedRevocationTx" width="100%" height="auto">
</p>

## Asymmetric Commitment Transactions 
Up until now, there is a pretty important detail that has been abstracted away from our commitment transactions. You may have picked it up recently if you've been wondering something like the following: 

- *"If both parties know the revocation private key for prior transactions, what is stopping them from claiming each other's balances from prior transactions?"*

In the actual Lightning Network, each party has their own version of ***each*** commitment transaction. They are mirror images of each other, but they do not contain the exact same information. For example, below you will see both Alice and Bob's respective version of the commitment transaction that reflects the first update they made to their channel: Alice sending Bob 2 bitcoin.

Before reviewing the commitment transactions, note that we've added another public key for both Alice and Bob. This public key represents each party's **revocation public key**. When Alice and Bob decide to move to a new channel state (create a new commitment transaction), they give eachother the information needed to calculate each other's **revocation private key**. This is how we effectively revoke old transactions. Since, if you publish an old transaction, you've already given your counterparty a way to spend all of your funds (which would be very bad for you!).

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/revocation_keys.png" alt="revocation_keys" width="30%" height="auto">
</p>

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/AsymmetricCommits.png" alt="AsymmetricCommits" width="100%" height="auto">
</p>

Below are some important things to note about these commitment transactions:
### To_Local Output
Both transactions have a ```to_local``` output. The ```to_local``` sends bitcoin to the party who is creating the given commitment transaction. For example, Alice's ```to_local``` will send bitcoin to herself.

### Revocation Public Keys
Both channels will add a revocation spending path to their ```to_local``` output. **This protects their counterparty, if they attempt to publish an old transaction**.

### OP_CSV Timelocks
Before setting up the payment channel, Alice and Bob will communicate their expectations for the channel. We'll discuss this in more detail later, but, for now, it's imprtant to note that, during this process, they communicate their ```to_self_delay``` expectations. In other words, they specify how long they expect their counterparty to delay any ```to_local``` outputs that are sent to themselves. For example, Bob may only open channels in which his counterparty agrees to delay their ```to_local``` outputs by at least 144 blocks (about 24 hours), so we will negotiate this ahead of time.

### Delayed Public Keys
Both channels will have generate a specific public key for each commitment transaction. This public key will be referenced within their ```to_local``` output and be encumbered with a time delay. Remember, within the Bitcoin Lightning Network, we always time-delay ```to_local``` outputs to protect our channel partner from us cheating.

### To_Remote Output
Finally, each transaction will have a ```to_remote``` output that sends our channel partner's balance to their public key. This will be a simple **Pay-To-Witness-Public-Key-Hash** (P2WPKH) transaction. The reason this output is less complicated is because, it reflects the balance that is **not** owned by the person or entity initiating the closure. Therefore, it does not have to be restricted due to concerns of cheating. Additionally, by not restricting it, we ensure that the channel funds are available immediately to the channel party.

## ⚡️ Write Function `to_remote` To Generate A ```to_remote``` Output Script For Our Commitment Transaction

`to_remote` will take a revocation public key, to_local delayed public key, and to_self_delay number of blocks  as an input. It will return the output script we need to use.

```rust
fn to_remote(revocation_key: &PublicKey, to_local_delayed_pubkey: &PublicKey, to_self_delay: &i64) -> Script {
}
```