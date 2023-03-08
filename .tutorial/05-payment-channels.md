# Payment Channels

In some ways a payment channel is a way to batch multiple payments into (usually two) bitcoin transactions.

Simple payment channels are a way for two parties to make potentially unlimited number of payments between them with only ever needing to broadcast two transactions to the bitcoin network/blockchain.

There are many different ways you can structure payment channels but they all share some common components.

We will start this workshop by exploring one of the simplest constructions to help you grok the core concepts as we build towards the more complicated (and much more powerful) channels used by the lightning network.

## Core concept

So how is this possible? What conceptually is going on when payments are made in a payment channel?

The idea is that the two parties will still construct bitcoin transactions for each payment being made but they agree not to broadcast them to the chain.  

By not broadcasting each transaction to the chain the parties do not need to wait for confirmations, do not need to pay miner fees, and do not push cpu/bandwidth/storage costs onto the entire network.


## 2-of-2 Multisig Payment Channel

Ok, so we know we want to be able to construct bitcoin transactions for each payment without having to go to the chain for each one.  

### How might this be possible?

One of the simplest ways to construct this is to lock some bitcoin into a 2-of-2 multisignature output where each party controls one of the keys.

Once this output is funded, we can construct a second transaction that spends from this 2-of-2 multisig with two outputs, one output representing each parties balance.

So if Alice and Bob enter into a 2-of-2 multisig and Alice locks 5 bitcoin into the funding output the second transaction would spend this output into one output paying Alice 5 bitcoin and Bob 0 bitcoin.

If Alice wanted to pay Bob a bitcoin all she would need to do is construct and sign a transaction that spends the funding output to two outputs where one output pays Alice 4 bitcoin and Bob 1 bitcoin.

If Alice wants to pay Bob another bitcoin she can construct and sign yet another transaction that spends the same 2-of-2 funding output but now has two outputs where one output pays Alice 3 and Bob 2.

### Question
How can Bob accept these payments without them having to be in a block?


<details>
  <summary>Answer</summary>
  As long as Alice provides her signature for the transaction Bob can at any point add his signature and broadcast the transaction to the chain.  
<br/><br/>
  Bob has never provided Alice with his signature so there's no way she can spend the funds and therefore no rush for Bob to go to chain. 
</details>

