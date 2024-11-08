# Introduction to HTLCs

To begin our payment process, Alice will inform Dianne that she would like to pay her. In return, Dianne will generate a **preimage hash**, which she will send to Alice. The preimage hash is crypytographic hash of a random number, called the **preimage**. It's practically impossible to guess the preimage from the preimage hash alone, so the only way to obtain the preimage would be for Dianne to provide it.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/PreimageHash.png" alt="PreimageHash" width="100%" height="auto">
</p>

The **preimage** is crucual to the current construction of the Lightning Network because it allows for *conditional payments*. A conditional payment is one where the payment is only completed if a specific condition is met. When we combine preimage hashes with tools we learned about earlier, such as timelocks, we can create very useful **payment contracts**. For example, Alice can create a payment contract with Bob such that Bob can only claim the bitcoin if:
1) He provides the preimage that, when hashed, equals the preimage hash that Dianne gave Alice.
2) Bob provides the preimage hash within a specific time period. For instance, 150 blocks (~25 hours).
   - The time period is specified as an **absolute timelock**. So each contract will only be valid until a certain block height is reached.

Together, the above components enable Alice to create a **Hash-Time-Locked-Contract** (**HTLC**), meaning that the contract is "locked" such that the reciever of the contract must provide the preimage within a specific amount of time to be able to claim the locked funds.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/HTLCContract.png" alt="HTLCContract" width="100%" height="auto">
</p>

Now, Bob can turn around and create offer the same contract to Charlie, who can offer the same contract to Dianne. Once Dianne recieve the HTLC (payment contract) from Charlie, she'll see that she can claim Charlie's funds immediately because she has the preimage. Once she hands the preimage over to Charlie, Charlie will turn around and provide the preimage to Bob so that he can claim the funds locked in his contract with Bob. Likewise, Bob will claim the funds locked in his contract with Alice.

In this way, Alice can pay Dianne by initiating a chain of payments through each channel. Furthermore, this payment chain is **atomic**, meaning that the payment will either succeed or fail for all participants. This is because, for any recipient along this chain to claim their payment, they will have to publish the preimage to the blockchain. Therefore, the preimage would be made public and everyone in the chain would be able to claim their payment (as long as the time period has not expired).

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/HTLCRedeem.png" alt="HTLCRedeem" width="100%" height="auto">
</p>

#### Question: Why are the block height timeouts decreasing along the path from sender (Alice) to reciever (Dianne)?
<details>
  <summary>Answer</summary>
  <br/>

Decreasing block height timeouts is crucial to achieving atomicity. If all channels in the route had the same timeout, then there is a chance that Dianne reveals the preimage right before the timeout. In this scenario, she may have enough time to claimm the HTLC funds, but Bob wouldnt have time to claim the funds in his channel with Alice. To ensure that all participants have time to claim the funds, we decrease the timeout with each step along the route towards the final destination.

</details>