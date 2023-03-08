# On-chain Payments

Before we dive into payment channels and eventually the lightning network lets review how on-chain payments work to better understand the motivation behind payment channels.

## Bitcoin Transaction

A bitcoin transaction is mostly made up of a list of inputs and a list of outputs.  

An transaction input is a pointer to a previous transaction's output and a bitcoin script that unlocks or proves you have the information necessary to spend that bitcoin.

A transaction output is an amount of bitcoin and a script that locks or determines how that amount of bitcoin can be spent in the future.

## Bitcoin Payments

Okay, so how do you typically make a payment in bitcoin? 

Generally, you would create a transaction that spends some bitcoin you control and create an output that only the person you are paying can spend in the future.

## Problems with on-chain payments?

There must be some issues with on-chain payments otherwise we would not be exploring payment channels.

- The person being paid needs to wait for the transaction to be mined and receive a sufficient number of confirmations before being able to consider the payment settled.


- The entire bitcoin network needs to relay, verify, and store this transaction forever.  It has a resource cost to the entire network.


- The payer needs to pay the miners to include it in a block.


## Can we do any better on-chain?

We can actually do a little better on-chain by batching payments into a single transaction.  This is mostly useful for entities such as exchanges who might batch customer withdrawals into a single transaction where each output is a payment destined for a single customer.

This helps a bit with the fees but isn't very helpful for the other problems and is not practical for every day payments people typically make.