# Simple Payment Channel Issues

At this point we have gone through the bitcoin script required to build out a simple payment channel. When we fixed the potential for Alice to lose all of her funds by introducing a timelock we introduced a new problem. 

## Question

What problem did the introduction of timelocks create?

<details>
  <summary>Answer</summary>
  <br/>
  They gave our channels a limited lifetime.  Bob must close the channel before the timelock expires otherwise Alice can take all of her funds back using the refund spending path.
</details>

<br/> <br/>

## Question

There are many other problems with this set up but one of the bigger one's is that you can only send payments in one direction.  How come Bob cannot pay Alice using this channel?

<details>
  <summary>Answer</summary>
  <br/>
  Let's say Alice open's a channel and puts 5 bitcoin into it.  She has already made a payment that pays 2 bitcoin to Bob.  This means that Bob has a signature from Alice for a tx that spends the original 5 bitcoin funding output giving 3 btc to Alice and 2 btc to Bob.  If Bob wanted to pay Alice a bitcoin by signing a new tx that brings his balance down to 1 btc, there is nothing preventing Bob from broadcasting the previous transaction that paid him the 2 btc. 
</details>


## Solutions

In order to solve both of these problems we need to come up with a way that we can cancel or invalidate old transactions.  Is this possible to do in Bitcoin? No, not really. 

## Question

Can you come up with a way to effectively invalidate or make it so Bob would not want to broadcast an old transaction? 
Do you recall the solution used to construct the channels using by the Lightning Network?

<details>
  <summary>Answer</summary>
  <br/>
  The lightning network uses a concept they call Revocable Transactions.  The basic idea is that each transaction includes an alternative spending path that makes ALL of your funds spendable by a special revocation key.  The details to construct the previous transaction's revocation key are exchanged each time someone proposes an update to the channel.  This way if Bob were to try to broadcast an old channel state then Alice could use the revocation key to steal all of his funds.  This mechanism is what gives rise for the need for lightning nodes to be online (or delegate to a watchtower).  You need to always be watching the chain to see if your counterparty is trying to cheat so that you can broadcast the penalty tx and claim their funds.
</details>