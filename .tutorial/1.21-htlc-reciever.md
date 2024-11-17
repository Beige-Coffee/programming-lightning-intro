# HTLC Reciever

Bob, the HTLC reciever, has to create a transaction where: 
- Bob can spend the output if he can prove he has the preimage *before* the HTLC expires at the pre-specified block height of```cltv_expiry```. Since Bob is sending an output to himself, it has to be timelocked by ```to_self_delay``` blocks.
- Alice can spend the output if she has the revocation key. This protects Alice in the future if Bob attempts to publish this commitment transaction after they had agreed to move to a new channel state.
- Alice can spend (effectively, reclaim) the output if the HTLC expires.

However, similar to the HTLC Offerer transaction, there is a dilema here! Bob's spending path must be delayed by ```to_self_delay``` blocks to give the Alice time to sweep the funds if Bob attempts to cheat in the future. 


#### Question: Looking at the simplified transaction below, can you spot why this commitment transaction structure would be a problem?
<details>
  <summary>
    Answer
</summary>
  
Bob can only claim the output *before* the `cltv_expiry` timelock expires. However, since Bob also has to wait `to_self_delay` to spend the output from the preimage spending path, there is a chance that the `to_self_delay` is longer than the `cltv_expiry`. If this happens, Alice could potentially spend the output along her expiry path when, in fact, Bob did have the preimage before expiry.  Do you know how we can fix this?

</details>

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/BobNoSuccessTx.png" alt="BobNoSuccessTx" width="50%" height="auto">
</p>

## Addressing The Timelock Dilema
To fix this timelock dilema, we'll add a second transaction for Bob, just like we did for Alice. However, this transaction will be called the **HTLC Success Transaction**. Just like the HTLC Timeout transaction, this will use the same script as our ```to_local``` output and the input for this transaction is the HTLC output from Bob's commitment transaction. That said, unlike the HTLC Timeout transaction, this transaction will not have an ```nLocktime``` timelock. Instead, the absolute timelock will be referenced in the output script.

Together, these changes allow for Bob to claim the HTLC funds as long as he has the preimage before the ```cltv_expiry```. The funds will then move to the second stage success transaction, where they will sit until Bob's ```to_self_delay``` passes. At that point, he can spend this output.


<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/Bob2ndStageTx.png" alt="Bob2ndStageTx" width="60%" height="auto">
</p>


## Putting It All Together

Putting it all together, the HTLC output has the following spending conditions:

1) **Revocation Path**: If Alice holds the revocation key (in case Bob cheats by broadcasting an old transaction), she can immediately spend the output.
2) **Timeout Path**: If the ```cltv_expiry``` passes, Alice can spend the output.
3) **Preimage Path**: If Bob provides the preimage, he can spend the output via the HTLC Success Transaction, which is set up in advance with Alice's signature for the 2-of-2 multisig condition. This allows Bob claim the funds before the ```cltv_expiry``` and also enforce his ```to_self_delay```.

For the HTLC Success:
- **Revocation Path**: Alice can spend the output immediately with the revocation key.
- **To_self_delay Path**: Bob can spend the output after the to_self_delay.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/HTLCReceiverTx.png" alt="HTLCReceiverTx" width="100%" height="auto">
</p>

## ⚡️ Write Function `build_htlc_receiver_witness_script` To Generate An HTLC Receiver Output Script

`build_htlc_receiver_witness_script` will take the following arguments:
- `revocation_pubkey160`: This is the HASH160 of the revocation public key. As a reminder, the HASH160 is calculated by first applying SHA256 to a public key and then applying RIPEMD160 to the result. This is represented as the `PubkeyHash` data type in Rust Bitcoin.
- `remote_htlc_pubkey`: This is the HTLC public key of our remote counterparty.
- `local_htlc_pubkey`: This is our HTLC public key.
- `payment_hash160`: This is the RIPEMD160 of the payment hash. Since the result of this is 20 bytes, it is represented as `&[u8; 20]`, a 20-byte array in Rust.
- `cltv_expiry`: This is the block height or timestamp at which the HTLC expires. Once this passes, the remote node (HTLC Offerer) can claim the HTLC.

Below are a few```Builder``` functions that will use useful in this excercise.
<br/><br/>
* `Builder::new()` - construct a new builder object
* `.push_opcode(op)` - add an opcode to the script
* `.push_int(num)` - add a number to the script
* `.push_key(public_key)` - add a `PublicKey` to the script
* `.push_slice(public_key)` - add bytes, such as a `PubkeyHash` or array of bytes, to the script.
* `.into_script()` - return the resulting `Script` from the `Builder`  

```rust
fn build_htlc_receiver_witness_script(
    revocation_pubkey160: &PubkeyHash,
    remote_htlc_pubkey: &PublicKey,
    local_htlc_pubkey: &PublicKey,
    payment_hash160: &[u8; 20],
    cltv_expiry: i64,
) -> ScriptBuf
```