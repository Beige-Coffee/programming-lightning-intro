# Multisig Channel Problems

Obviously there are some problems with using a bare 2-of-2 multisig as a payment channel otherwise we would not need more complicated constructions!

<br/>

## Problem: Potential Loss of Funds

There are many problems with this construction but by opening this payment channel Alice is putting all of her funds at risk.

### Question

How can Alice lose all her funds in this setup?

<br/>
<details>
  <summary>Answer</summary>
  <br/>

  If Bob stops responding or refuses to cooperate then there's no way Alice can ever get her funds out of the 2-of-2 multisig because she will never be able to get a signature from Bob.

  <br/>

  Remember: To spend from a 2-of-2 multisig requires signatures from *both* parties.

</details>

<br/>
<br/>

## Refund spending path

Alice needs a way to get her funds back out of the channel if Bob becomes unresponsive.

We can do this by adding a second spending condition on the script that locks the funds into the channel.  This extra spending condition needs to give Alice a way of accessing her funds if Bob becomes unresponsive while still maintaing the ability for Bob to safely accept payments without broadcasting the interim transactions.

<br/>

### Question

One way we could give Alice access to her funds is by adding a second spend path to the funding output that locks the funds using a pay-to-pubkey-hash script.

What's the problem with this solution?

<br/>

<details>
  <summary>Answer</summary>
  This would mean that at *any* time Alice can withdraw *all* of her funds from the channel. If Alice gave Bob a signed transaction paying him 1 btc then he couldn't safely hold onto this transaction without broadcasting it immediately.
  
  <br/>

  If Alice broadcasts a transaction spending the funding output using the p2pkh path it would invalidate the transaction Bob is holding and he would lose his funds.
</details>


<br/>

### Question

Can you think of another way we could add a second spending path to the funding output that would give Alice a way to get her money back while allowing Bob to safely hold onto the unbroadcasted transactions?

<br/>

<details>
  <summary>Answer</summary>
  If we add a timelock to the refund spending path then Alice won't be able to immediately take her funds but she will know she can eventually get them back after the timelock expires.

  <br/>

  This would also allow Bob to safely hold onto the unbroadcasted transactions as long as the timelock was in place.