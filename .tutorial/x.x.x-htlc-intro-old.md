# Introduction to Hash-Time-Locked-Contracts (HTLCs)

## Starting With The End In Mind

Introducing the famous ***HTLC*** will get a little complicated! While it's nothing we can't handle, it will be helpful if we start with the end in mind.

Routing a payment across the Lightning network just means that the **channel balance distributions** will change for each channel, such that, when the payment is complete:
- The sender will have less Bitcoin on their side of the channel
- The receiver will have more Bitcoin on their side of the channel

The above is true for **all** nodes in the route. For example, imagine Alice routes 400,000 sats to Dianne through Bob. You can see the original channel balances and updated channel balances in the visual below. **Take a moment and verify the following**:
- The total amount of Bitcoin in the payment route has *not changed*. More specifically, Alice and Bob's channel has 5M sats in both Channel States, while Bob and Dianne have 4.5M.
- The channel balances are updated such that Alice now has 400,000 less sats than she had prior to the payment, while Dianne has 400,000 more. Bob's balance remains unchanged.

**NOTE**: In reality, Bob would have received a fee for "forwarding" the payment (i.e., letting Alice us his channel liquidity to pay Dianne). If Bob didn't recieve a fee, he would have no incentive to pay Dianne.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/alice_dianne_overview.png" alt="alice_dianne_overview" width="80%" height="auto">
</p>

## Invoice
Let's build our intuition of Lightning payments by going through an example. Imagine Alice goes to the local coffee shop, which Dianne owns. She is interested in buying a double espresso with raw milk, since that's what the influencers on Twitter are recommending.

She asks Dianne to generate an **invoice** for her. This invoice will provide basic payment information, such as the product that Alice is buying, the cost, and how long this offer is valid for. 

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/dianne_invoice.png" alt="dianne_invoice" width="80%" height="auto">
</p>

## Contracts
When we think of payments, we may think of simply sending money and getting something in return, but there is more going on here. Each payment is actually a **contract**. For instance, when Alice buys a coffee, she sets up the followinig informal agreement with the coffee shop:

**If Alice pays 5k sats, then the vendor will give her coffee. This offer is valid for 8 hours, as the vendor may change their prices tomorrow.**

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/contract.png" alt="contract" width="60%" height="auto">
</p>

## Contracts on Lightning
Since Alice does not have a channel open with Dianne, the coffee shop owner, Alice will create a payment contract with Bob instead, since Bob has a channel open with Dianne. This contract will have the following condition: **If Bob pays Dianne 5,000 sats, Alice will send Bob 5,050 sats**, effectively refunding him.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/alice_dianne_contract.png" alt="alice_dianne_contract" width="100%" height="auto">
</p>

#### Question: Why is Alice sending 5,050 sats to Bob? Isn't the payment only for 5,000 sats?
<details>
  <summary>Answer</summary>

The 50 sats are fees paid to Bob! We'll discuss the nuances of fees later in this course, but, for now, it's important to note that Alice will have to pay fees to each hop along the payment route to incentivize the node to forward the payment. Otherwise, Bob has no real reason to adjust is liquidity between his channels with Alice and Bob.

</details>

#### Question: What could go wrong here?
<details>
  <summary>Answer</summary>

There are many issues with this payment construction. Below are a few major ones:

1) Bob has no guarentee that Alice will refund him. For example, if he sent Dianne 5,000 sats and Alice refused to refund him, he would lose 5,000 sats.
2) Similar to the above issue, Alice has no guarentee that Bob paid Dianne. He could attempt to lie to Alice and say he sent 5,000 sats when he did not. If Alice believes him and sends Bob 5,050 sats, she would lose those funds.

**How can we fix this**?

</details>


## Proof of Payment
What we really need is a mechanism to prove that Bob paid Dianne. For example, if Bob is able to recieve a *verifiable* **receipt** from Dianne after sending her 400k sats, then Alice can be assured that Bob actually paid Dianne.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/alice_dianne_proof_of_payment.png" alt="alice_dianne_proof_of_payment" width="100%" height="auto">
</p>

#### Question: How can we use cryptography to create a verifiable receipt?
<details>
  <summary>Answer</summary>

To create a **proof of payment** mechanism, Dianne can generate a large, 256-bit random number (**Preimage**) and then take the SHA256 hash of it, which would be the **Preimage Hash**. For example, Dianne could generate the following:
- Preimage (Secret): `34523948796532148976321459876321459876321459876321459876321459871`
- Preimage Hash: `a7c4e9f2b5d1a8c6e3f0d7b9a4c2e5f8d1b6a9c3e0f7d4b2a5c8e1f9d6b3e0f4`

Dianne would then take the **Preimage Hash** and include it in the invoice that she gives Alice, but Dianne will keep the **Preimage** to herself for now!

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/preimage_invoice.png" alt="preimage_invoice" width="50%" height="auto">
</p>

Alice can now update the contract with Bob, requiring that Bob provide the **Preimage** in order to claim the 5,000 from Alice. Since the **Preimage** is only known by Dianne, Bob will set up a contract with Dianne with the same **Preimage Hash** that Alice gave him.

This ensures that Bob will only pay Dianne if she provides Bob with the **Preimage**, which is exactly what Bob needs to be able to claim the 5,050 from Alice.

Notice, in the below contract, we now track the the timeout period in terms of **block height**, as opposed to hours.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/alice_bob_dianne_preimage.png" alt="alice_bob_dianne_preimage" width="100%" height="auto">
</p>

Together, the above components enable Alice to create a **Hash-Time-Locked-Contract** (**HTLC**), meaning that the contract is "locked" such that the reciever of the contract must provide the **Preimage** within a specific amount of blocks (time) to be able to claim the locked funds.

**Take a minute to think through how we can set up these contracts in Bitcoin. How will we represent them? When you're ready, head over to the next section to learn how to implement a simple HTLC!**

</details>

#
#
#
#
#
#
#
#
#
#
#
#

However, recall that Alice is *not* directly connected to Dianne over the Lightning Network. So, instead, Alice will have to forward payments across the network via the following channels:
- Alice to Bob
- Bob to Dianne

**In other words, Alice will pay Bob. Bob will pay Charlie. Charlie will pay Dianne.**

We can imagine a situation where each pair of channel partners, **A** and **B**, set up the following contract:
- A will pay B ***if and only if*** B proves (ex: provides a receipt) that they forwarded the funds to C, where C is the next hop in the route from B to the final destination.

***PRO TIP!***: When reading the payment flows below, try thinking about the flow from **right-to-left**. Ultimately, this is how the payment would have to work. For example, in the contract below, Alice will only provide funds to Bob ***if*** Bob pays Charlie, BUT Bob will only pay Charlie ***if*** Charlie pays Dianne. So, it may be helpful to think of the payment flow in reverse.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/proven_payment_chain.png" alt="proven_payment_chain" width="100%" height="auto">
</p>

#### Question: What could go wrong?
<details>
  <summary>Answer</summary>

A major issue with this payment construction is that any participant in this route could refuse to cooporate with the agreement. For example, Charlie could pay Dianne, but Bob can refuse to pay Charlie. This would mean Charlie is now at a loss of 400k sats.

In the "real world", Bob may be able to contact authorities and take legal action to get his funds back. However, Lightning operates across the world, and you may not know who you're forwarding payments through or where they live, making it very difficult to take legal action.

</details>

#### Question: Why does Charlie & Dianne's contract have "...." in the contract? You're not expected to know this, as we haven't covered it, but here is a hint: do you think Charlie know's the final destination for this payment?
<details>
  <summary>Answer</summary>

We haven't covered this yet, so don't worry if you don't know the answer!

In the Lightning network, when payments are routed across the network, the given hop only knows who they are sending to next. They don't know the final destination, even if their counterparty is the final destination. So, in this case, Charlie is routing a payment to Dianne by moving funds from his side to her side, **but he doesn't know Dianne is the recipient**.

</details>

When it comes to payment routing, we need a way to ensure that the **entire payment either succeeds or fails**. This fancy word for this is "atomicity".


## Atomic Payments

### Gentle Introduction

We need to make sure that the entire payment either succeeds or fails. In other words, either *everyone* will succeed in moving funds across their channels or *nobody* will.

To build our intuition of how atomic payments work, let's temporarily introduce a trusted third party. Just as we did earlier with revocation keys, we'll replace our trusted third party with cryptographic skillz shortly!

When setting up contracts, the ***sender*** will deposit their funds into an account with a third party - say, an escrow service. The ***reciever*** will only be able to retrieve these funds if a final receipt is provided to the third party, proving the recipient recieved the funds. Once this happens, the third party will relinquish control of the funds to their new owners.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/escrow.png" alt="escrow" width="100%" height="auto">
</p>

#### Question: What is a good "Proof of Payment"? Can you think of something that leverages cryptography?
<details>
  <summary>Answer</summary>

In this case, the **Proof of Payment** will act as a receipt, and it means that Alice has recieved the funds she requested. In this case, 400k sats.

We can use cryptographic hash functions create a **Proof of Payment** mechansism in the following way:
1) Dianne, the recipient, generates a veryyyyy large random number (256 bits). This is called the **preimage**, and will act as the **secret** or **proof of payment**.
2) Dianne will take the **SHA256** of the **preimage** and send it to Alice.
3) Alice will provide the **preimage hash** to the third party and lock up 400k sats. Alice will tell the third party escrow service that they should only release the funds **if and only if** they receive the **preimage** that, when hashed, equals the **preimage hash**.
4) Bob will do the same when setting up a contract with Charlie.
5) Charlie will do the same when setting up a contract with Dianne.
6) When Dianne receives this contract, she will recognize the **preimage hash**, so she will provide the **preimage** to the third party.
7) At this point, everyone will be able to claim their funds from the third party.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/escrow_claim.png" alt="escrow_claim" width="100%" height="auto">
</p>

</details>

### Bitcoin Script

Now that we've reviewed, generally, how atomic payment work with a third party, let's discuss them in the context of Bitcoin.

First and foremost, we're going to have to have to move our contracts into Bitcoin script. The person offering the funds will set up a "contract" with the receiver such that:
- The receiver can claim the funds if they are able to provide the preimage to the preimage hash, which is specified in the Bitcoin script.
- The sender can claim the funds if the offer times out. The time period is specified as an **absolute timelock**. So each contract will only be valid until a certain block height is reached.

The above contract construction actually enables atomic payments. This will become more clear in the next section.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/PreimageHash.png" alt="PreimageHash" width="100%" height="auto">
</p>

Together, the above components enable Alice to create a **Hash-Time-Locked-Contract** (**HTLC**), meaning that the contract is "locked" such that the reciever of the contract must provide the preimage within a specific amount of time to be able to claim the locked funds.

#### Question: Why are the block height timeouts decreasing along the path from sender (Alice) to reciever (Dianne)?
<details>
  <summary>Answer</summary>

Decreasing block height timeouts is crucial to achieving atomicity. If all channels in the route had the same timeout, then there is a chance that Dianne reveals the preimage right before the timeout. In this scenario, she may have enough time to claim the HTLC funds from Charlie, but Charlie may not have enough time to claim his funds from Bob before the contract expires. To ensure that all participants have time to claim the funds, we decrease the timeout with each step along the route towards the final destination.

</details>

**Take a minute to think through how we can set up a new commitment transaction that does this. How will we represent this new output? Will it be included in the ```to_local``` or ```to_remote``` outputs, or will it be its own output?**