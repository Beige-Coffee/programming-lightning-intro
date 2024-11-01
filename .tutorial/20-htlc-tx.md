# HTLC & Transactions

Now that we've built our intuition around how HTLCs work, let's dive down to the transaction level to see how they are implemented. Remember, Alice is sending a 400,000 sat payment to Dianne through Bob and Charlie. Since Alice has a channel with Bob, Alice and Bob will begin by updating their respective commitment transactions to include a new output for the HTLC.

Before we dig into the locking script for the HTLC outputs, let's review the important changes here.
1) There is a new commitment TXID. This means we would have revoked the previous commitment transaction, and we are commiting to a new transaction.
2) The value for Alice's output has decreased by 400,000, the amount she is forwarding. In practice, this would decrease by more than the 400,000 because Alice needs to pay Bob and Charlie routing fees, but we can ignore that for now.
3) There is a new HTLC output for 400,000. This is where we will place the new conditional logic that describes how these funds can be spent.
4) The pubkeys used in the output scripts have changed. Remember, each commitment transaction has its own unique public keys.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/AliceBobCommitHTLCs.png" alt="AliceBobCommitHTLCs" width="100%" height="auto">
</p>

So far, much is still similar to the previous commitment transactions we looked at. For example:
- The ```to_local``` output for Alice and Bob still has two spending paths.
  - One spendable by the local node after ```to_self_delay``` blocks have passed.
  - The other spendable by the remote node if they have the revocation key.
- The ```to_remote``` output for Alice and Bob are still immediately spendable by the owner of those funds.

## HTLC Output
For the HTLC output, we have to come up with an output script that adheres to the HTLC contract rules. HTLC outputs will be slightly different depending on if you are ***offering*** the HTLC or if you are ***recieving*** the HTLC. 