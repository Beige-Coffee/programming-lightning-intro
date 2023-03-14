# Payment Channels


You can conceptually think of a payment channel as a way to perform batching of your payments over a potentially unlimited length of time.  This makes it a more viable option for consumers who have sporadic payment requirements and on-chain batching is not a useful solution.

Simple payment channels are a way for two parties to make potentially unlimited number of payments between them with only ever needing to make two on-chain bitcoin transactions.  Not only does this drastically reduce the cost to the payer and the imputed cost to the network but payments over a payment channel are settled nearly instantly in most cases.

They help solve all three of the problems we identified with on-chain payments.

There are many different ways you can structure payment channels but they all share some common components.

We will start this workshop by exploring one of the simplest constructions to help you understand the core concepts as we build towards the more useful but more complicated channels used by the lightning network.

## Core concept

So how is this possible? What conceptually is going on when payments are made in a payment channel?

The idea is that the two parties will still construct bitcoin transactions for each payment being made but they agree not to broadcast them to the chain.  

By not broadcasting each transaction to the chain the parties do not need to wait for confirmations, do not need to pay miner fees, and do not need to force resource costs onto the entire network.

## 2-of-2 Multisig Payment Channel

We want to be able to construct bitcoin transactions for each payment without having to go to the chain for each one.  

### How might this be possible?

One of the simplest ways to construct a payment channel is to lock some bitcoin into a 2-of-2 multisignature output where each party controls one of the keys.

Once this output is funded, we can construct a second transaction that spends from this 2-of-2 multisig with two outputs, one output representing each parties balance.

So if Alice and Bob enter into a 2-of-2 multisig and Alice locks 5 bitcoin into the funding output the second transaction would spend this output into one output paying Alice 5 bitcoin and Bob 0 bitcoin.

If Alice wanted to pay Bob a bitcoin all she would need to do is construct and sign a transaction that spends the funding output to two outputs where one output pays Alice 4 bitcoin and Bob 1 bitcoin.

If Alice wants to pay Bob another bitcoin she can construct and sign yet another transaction that spends the same 2-of-2 funding output but now has two outputs where one output pays Alice 3 and Bob 2.

### TODO: INSERT DIAGRAM
<br/>

## Question

Why can Bob safely accept these payments without the transactions being included a block: 

- How does he know he can spend the payments Alice made? 
- How can Bob be sure Alice cannot take her money back?

<details>
  <summary>Answer</summary>
  <br/>
  Bob can accept these payments because he knows he can spend them and Alice cannot.
  
  <br/>

  As long as Alice provides her signature for the transaction Bob can at any point add his signature and broadcast the transaction to the chain.  
<br/>
  Bob has never provided Alice with his signature so there's no way she can spend the funds and therefore no need for Bob to go to the chain with the intermediate transactions if he expects further payment from Alice. 
</details>

