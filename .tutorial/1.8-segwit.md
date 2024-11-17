# Segretated Witness (SegWit)

Segregated Witness, also known as "SegWit", was a soft-fork upgrade to Bitcoin that was activated in 2017. Why are we mentioning SegWit in this workshop? Well, SegWit was actually a crucial upgrade to Bitcoin and was required for the Lightning Network to work properly.

The SegWit upgrade moved the signature data from its previous location, the **scriptSig**, to a separate location, called the **witness stack**. After the upgrade, many SegWit transactions now leave the **scriptSig** blank or enter ```00```, indicating that there is no signature present for that input.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/SegWit.png" alt="SegWit" width="100%" height="auto">
</p>

## Transaction IDs

Each transaction has a unique identifier for the transaction, called the **"Transaction ID"** (**TXID**). The TXID is calculated by hashing the data within each transaction, including the inputs and outputs. The TXID is very important within Bitcoin, as it tells us which previous transaction we are spending our bitcoin from. This is precicely why it is listed in the **input** field of our transaction. 

By creating a separarate "witness" field in the transaction, SegWit changed which data is included when we are calculating the TXID for each transaction. Effectively, this means that the signature is **NOT** included when calculating the TXID for SegWit transactions. In the simplified transaction below, the information within the red boxes represents which data is included when calculating the TXID for both Pre-SegWit (also known as "Legacy") and SegWit transactions.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/TxID.png" alt="TxID" width="100%" height="auto">
</p>

## Why Was SegWit Required For Lightning?

Great question—glad you asked!

Lightning relies on channel parties being able to create valid, unbroadcasted off-chain transactions. For this to work, each party must trust that their counterparty cannot alter the transaction in any meaningful way.

Before SegWit, Bob could take a transaction from Alice and subtly modify her signature to produce a new transaction ID while leaving the transaction content unchanged. This issue, called transaction malleability, made it unsafe to build secure off-chain protocols like Lightning.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/MultipleSigs.png" alt="MultipleSigs" width="100%" height="auto">
</p>