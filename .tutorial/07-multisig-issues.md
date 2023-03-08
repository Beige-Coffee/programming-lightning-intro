
## Multisig Channel Problems

Obviously there are some problems with using a bare 2-of-2 multisig as a payment channel otherwise we would not need more complicated constructions!

## Problem #1: Loss of Funds

There are many but by opening this payment channel Alice is putting all of her funds at risk.

### Question

How can Alice lose all her funds in this setup?

<details>
  <summary>Answer</summary>
  If Bob stops responding or cooperating at all then there's no way Alice can ever get her funds back out of this 2-of-2 multisig because she will never be able to get a signature from Bob.
</details>


## Solution

Alice needs a way to get her funds back out of the channel if Bob becomes unresponsive.

We can do this by adding a second spending condition on the script that locks the funds into the channel.  This extra spending condition needs to give Alice a way of accessing her funds if Bob becomes unresponsive while still maintaing the ability for Bob to safely accept payments without broadcasting the interim transactions.

### Question

What's the problem with adding something like a simple pay-to-alice's-pubkey-hash spending path to the funding transaction?

<details>
  <summary>Answer</summary>
  This would mean that at ANY time Alice can withdraw ALL of her funds from the channel. If Alice gave Bob a signed transaction paying him 1 btc then he couldn't safely hold onto this transaction without broadcasting it.  If Alice broadcasts a transaction spending the funding output using the p2pkh path it would invalidate the transaction Bob is holding and he would lose his funds.
</details>