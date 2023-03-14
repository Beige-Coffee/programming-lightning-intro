# Refund the Channel

If Bob becomes unresponsive or stops cooperating with Alice then Alice can use the refund spending path to get all of her funds out of the channel.  

<br/><br/>

## ⚡️ Write a function `spend_refund` that takes Alice's pubkey and signature and constructs the input script we need to use.


Remember, we locked our funding output in a script that looked like this:

```
OP_IF <spending_path_1> OP_ELSE <spending_path_2> OP_ENDIF 
```

One of the spending paths was for spending the 2-of-2 multisig and the other was for Alice's refund path.  In this case we want to spend the refund path.

The refund spending path script looked like this:

```
<blockheight_or_timestamp> OP_CSV OP_DROP OP_DUP OP_HASH160 <pubkey_hash> OP_EQUALVERIFY OP_CHECKSIG
```

```rust
fn spend_refund(alice_pubkey: &PublicKey, alice_signature: Signature) -> Script {}
```