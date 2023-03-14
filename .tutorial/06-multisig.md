# Multisig

## Pay-to-pubkey-hash

Before we dive into building a multisig script, let's walk through building a pay-to-public-key-hash script so we can get our bearing of how to work with the bitcoin library we will be using throughout this workshop. 

## Building a function to generate pay-to-pubkey-hash output scripts

Let's build a function that takes in a bitcoin PublicKey and returns a Script that can be used as an output in a pay-to-pubkey-hash transaction.

We can define a function that takes in a public key and outputs a script like this:
```rust
fn p2pkh(pubkey: &PublicKey) -> Script {}
```
<br/>
The bitcoin library we are using provides a `Builder` object we can use to construct any `Script` we want. It offers a handful of helper functions for adding opcodes, ints, bytes, and keys to a Script:
<br/><br/>

* `Builder::new()` - construct a new builder object
* `.push_opcode(op)` - add an opcode to the script
* `.push_int(num)` - add a number to the script
* `.push_key(public_key)` - add a `PublicKey` to the script
* `.push_pubkey_hash(public_key)` - add a `PubkeyHash` of a `PublicKey` to the script
* `.push_signature(signature)` - add a signature to the script
* `.push_script(script)` - add another script to this script
* `.into_script()` - return the resulting `Script` from the `Builder`  

<br/><br/>
If you recall a pay-to-pubkey-hash Script has the form: 

```
OP_DUP OP_HASH160 <pubkey_hash> OP_EQUALVERIFY OP_CHECKSIG
```


We can build the entire p2pkh script using the methods available on the Builder like this:

```rust
  Builder::new()
    .push_opcode(opcodes::OP_DUP)
    .push_opcode(opcodes::OP_HASH160)
    .push_pubkey_hash(pubkey)
    .push_opcode(opcodes::OP_EQUALVERIFY)
    .push_opcode(opcodes::OP_CHECKSIG)
    .into_script()
```

You can see we use `Builder::new()` to construct a new empty Builder object.  From there we can chain calls to `push_opcode` and `push_pubkey_hash` to build up the script we want.  Finally, we call the `into_script()` method to convert the Builder into the Script that our function needs to return.

<br/><br/>
## 2-of-2 Multisig

At the root of every payment channel is a two-of-two multisig output that the channel's funds are locked into.  This will be a foundational component of the payment channels we look at in this workshop.  If you recall a 2-of-2 multisig script typically takes the form:
<br/>
```
 2 <PUBKEY_1> <PUBKEY_2> 2 OP_CHECKMULTISIG 
```
<br/>

⚡️ Try to implement the `two_of_two_multisig` function in `lib.rs`:
```rust
fn two_of_two_multisig(alice_pubkey: &PublicKey, bob_pubkey: &PublicKey) -> Script {
    // TODO: build 2-of-2 multisig Script using alice_pubkey and bob_pubkey
}
```

When you think you have the solution click the big green `Run` button at the top of the screen to make sure the unit tests are still passing.

<br/><br/><br/><br/>