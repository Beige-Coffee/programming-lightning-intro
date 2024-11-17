## Problems with on-chain payments?

Okay, so we've reviewed various types of Bitcoin transactions such as Pay-To-Public-Key-Hash, timelocks, and multisig. Got it! These all seem like they work quite well for sending bitcoin. So, why do we need a "payment channel"? 

While on-chain transactions are effective, they have some limitations when it comes to creating a global payment system. 

- They're **slow**: For a Bitcoin transaction to be considered final, the recipient needs to wait for the transaction to be mined into a new block. On average, a block is mined every 10 minutes. Additionally, recipients will often wait for a few additional blocks to be mined on top of the block containing the transaction (also known as "confirmations"). This is to ensure they are protected against a [block reorganization](https://river.com/learn/terms/r/reorganization/), which may render their original transaction invalid.

- They're **expensive**: The sender needs to pay (incentivize) miners to include their transaction in a block. Historically, fees have been relatively low (less than $2), but they can reach over $50 during times of extreme congestion in the network. This poses a few problems. For example, what if we want to send $1 of bitcoin to someone?

- They don't **scale**. Every transaction ever made must be relayed, verified, and stored by the entire Bitcoin network, indefinitely. For Bitcoin to scale to support 8 billion people, we can't rely solely on on-chain transactions; we need more efficient solutions!


## Can we do any better on-chain?

We can actually do a little better on-chain by batching multiple payments into a single transaction.  This is mostly useful for entities who have large payment volumes, such as exchanges, who might batch all of their customer withdrawal requests in a given time period into a single transaction.  This transaction will have only enough inputs to fund the transaction and one output for each customer's withdrawal amount.

This means a batch transaction is generally smaller than the same set of transctions that make individual payment because the number of inputs don't grow linearly with the number of payments being made.

This helps reduce the fees paid by the payer and the imputed cost on the network.  However, it doesn't help much with the speed of the payments.  Most importantly, the spending patterns of most consumers is not conducive to employing batching and therefore is not a viable improvement to any of these concerns.