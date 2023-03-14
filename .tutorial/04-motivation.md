# Motivation for Payment Channels

Before we dive into payment channels and eventually the lightning network lets review how on-chain payments work to better understand the motivation behind payment channels.

## Bitcoin Transaction

A bitcoin transaction primarily consists of a list of inputs and a list of outputs.

An transaction input is a pointer to a previous transaction's output and a bitcoin script that unlocks or proves you have the information necessary to spend that bitcoin.

A transaction output is an amount of bitcoin and a script that locks or determines how that amount of bitcoin can be spent in the future.

## How do you make an on-chain payment in bitcoin? 

You create a transaction that adds inputs for bitcoin (utxos) that you control and add an output that only the person you are paying can spend in the future.

## Problems with on-chain payments?

Why are we exploring payment channels? 

- They're slow.  The person being paid should wait for the transaction to be mined and receive a sufficient number of confirmations before being able to consider the payment settled.

- They're expensive.  The payer needs to pay the miners to include it in a block.

- They don't scale.  The entire bitcoin network needs to relay, verify, and store every transaction ever made, forever.  Every payment made imputes a cost on every other participant on the network.


## Can we do any better on-chain?

We can actually do a little better on-chain by batching multiple payments into a single transaction.  This is mostly useful for entities who have large payment volumes, such as exchanges, who might batch all of their customer withdrawal requests in a given time period into a single transaction.  This transaction will have only enough inputs to fund the transaction and one output for each customer's withdrawal amount.

This means a batch transaction is generally smaller than the same set of transctions that make individual payment because the number of inputs don't grow linearly with the number of payments being made.

This helps reduce the fees paid by the payer and the imputed cost on the network.  However, it doesn't help much with the speed of the payments.  Most importantly, the spending patterns of most consumers is not consudice to employing batching and therefore is not a viable improvement to any of these concerns.