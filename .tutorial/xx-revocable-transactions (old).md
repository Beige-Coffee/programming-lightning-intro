# Revocable Transactions

Recall, our motivation for creating revocable transactions is that we need a way to invalidate old commitment transaction such that Alice and Bob *are disincentivized from* broadcasting them to steal each other's bitcoin. As we saw in the previous exercise, Alice made a payment to Bob of 2 bitcoin, however, she still has a valid refund transaction that she could publish to the Bitcoin network, effectively stealing back the 2 bitcoin.

Since Bitcoin does not support the ability to cancel transactions, we need a more clever way to *disincentivize* broadcasting old transactions.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/RefundRevoked.png" alt="RefundRevoked" width="80%" height="auto">
</p>

## Penalty Mechanism
A **penalty mechanism** is a system, rule, or process that is designed to discourage undesirable or dishonest behavior. It accomplishes this by imposing a negative consequence or penalty on individuals or entities that violate the rules or norms of that system.

As we will see throughout this workshop, penalty mechanisms are a foundational part of the Lightning Network, as they provide a way to incentivize good behavior in a decentralized system that does not have a central party or entity to enforce that behavior themselves.

Within the context of commitment transactions, we can incentivize good behavior by adding the penalty mechanism rule:

- **If you publish an old commitment transaction, the counterparty is allowed to steal all of the funds you have on your side of the channel.**

Now that we have our penalty mechanism rule, we just need to identify a way to enforce it. Can you think of something?

<details>
  <summary>Answer</summary>
  <br/>

To enforce this rule, we can do the following:
1) Add an additional spending path to the output such that, if the counterparty presents a special key, called a **revocation key**, they are able to claim all of the funds. Otherwise, the funds will be sent directly to the intended owner.
2) For each new transaction, the owner of the funds will provide the counterparty with the information they need to calculate the **revocation key** for the prior transaction. This represents a promise from the owner that they will not publish an old transaction, because, if they did, the counterparty now has the ability to steal all of the owner's funds.

To add this logic to our script, we'll have to make our script more complex. Luckily, we already learned about a more flexible script type called **Pay-To-Witness-Script-Hash** that we can leverage to do this!

All we have to do is update our script to include the following conditional check:
1) If the counterparty provides the **revocation private key** for the **revocation public key** that we include in the witness script, then they can spend the funds.
2) Otherwise, if the private key corresponding to the original owner's public key is provided, then the owner can spend the funds. 

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/RevocationTx.png" alt="RevocationTx" width="100%" height="auto">
</p>

</details>

## ⚡️ Write Function `revokable_output` To Generate A Revokable Output Script For Our Commitment Transaction

`revokable_output` will take a public key as an input and constructs the output script we need to use.

```rust
fn revokable_output(revocation_key: &PublicKey, owner_pubkey: &PublicKey) -> Script {
}
```

## Another Security Flaw!

A keen eye may have already noticed that this penalty mechanism is missing something very important. Can you tell what it is?

Here is a hint: *Timing is everything!*

<details>
  <summary>Answer</summary>
  <br/>

So, a counterparty is able to claim all of the funds if the owner tries to cheat them and publish an old transaction. That's good! However, with the current construction, the counterparty would have to act immediately and, essentially, outbid the owner in the mempool. If the counterparty is unable to outbid them and get their transaction mined first, then the original owner would succeed in publishing an old transaction.

Even worse, the original owner could just go directly to a miner and hand them the transaction "under the table", meaning that it would never be broadcasted among the network, so the counterparty wouldn't know until it's too late.

Therefore, to stop this from happening, we must add a timelock to the transaction so that, if the original owner does succeed in getting an old transaction mined, the counterparty has a window of time to steal all of the funds!

We'll review how this works in the next section. Do you know which timelock feature we will use to accomplish this?

</details>