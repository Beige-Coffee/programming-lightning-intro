# Making a Payment

Once the channel funding transaction is confirmed on chain we can begin to make payments over our channel.  A payment can be made by constructing a second transaction that spends the funding output to two outputs, one representing the current balance of each recipient.


<br/><br/>

## ⚡️ Write a function `spend_multisig` that takes Alice and Bob's signatures and constructs the input script we need to use.


Remember, we locked our funding output in a script that looked like this:

```
OP_IF <spending_path_1> OP_ELSE <spending_path_2> OP_ENDIF 
```

One of the spending paths was for spending the 2-of-2 multisig and the other was for Alice's refund path.  In this case we want to spend the 2-of-2 multisig.

The multisig spending path script looked like this:

```
 2 <PUBKEY_1> <PUBKEY_2> 2 OP_CHECKMULTISIG 
```

Also recall our Script `Builder` object has the following methods:

* `Builder::new()` - construct a new builder object
* `.push_opcode(op)` - add an opcode to the script
* `.push_int(num)` - add a number to the script
* `.push_key(public_key)` - add a `PublicKey` to the script
* `.push_pubkey_hash(public_key)` - add a `PubkeyHash` of a `PublicKey` to the script
* `.push_signature(signature)` - add a signature to the script
* `.push_script(script)` - add another script to this script
* `.into_script()` - return the resulting `Script` from the `Builder`  


```rust
fn spend_multisig(alice_signature: Signature, bob_signature: Signature) -> Script {}
```