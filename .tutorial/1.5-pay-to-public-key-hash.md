# Pay-to-Public-Key-Hash (P2PKH)

Now that we've reviewed the basic concepts of a transaction, let's start peeking under the hood. This will help build our intuition as we begin to dive into more advanced transaction types. 

On the previous page, we saw the below diagram. In this diagram, the bitcoin was "locked" to the **gold lock**, meaning that it can ***only*** be spent if someone is able to provide the **gold key** to "unlock" it. These keys and locks can become quite complex, so we'll start with a simpler transaction type: **Pay-to-Public-Key-Hash (P2PKH)**. In a P2PKH transaction, bitcoin is "locked" to the hash of a public key. To unlock the bitcoin, the spender must provide:
- The **public key** corresponding to the **public key hash** in the output script.
- A valid **signature**, which is created using the private key associated with the public key. This signature will prove that the spender controls the private key without revealing the private key itself.

As you will see in the below diagram, a **locking script** is placed on each output. Each output can have a different locking script.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/scriptPubKey.png" alt="scriptPubKey" width="60%" height="auto">
</p>

To spend the bitcoin, a valid **unlocking script** must be provided. As we saw earlier, for a P2PKH transaction, the unlocking script must include the unhashed public key and a valid signature.
<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/pubkeySig.png" alt="pubkeySig" width="100%" height="auto">
</p>

The Bitcoin protocol will then proceed to concatenate the ```scriptSig``` data with the ```scriptPubKey``` data such that the ```scriptSig``` comes first.
<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/p2pkhStackScript.png" alt="p2pkhStackScript" width="100%" height="auto">
</p>

From here, we're ready to execute the script to see if the information provided is valid. An in-depth review of script execution is outside the scope of this workshop, however, if you would like a brief review of how this works, please see "More Details" below.

<br/>
<details>
  <summary>More Details</summary>
  
1) Start with an empty "stack". This stack will hold all of our data, and we will perform operations on that data.
2) We read our concatenated scripts from left to right. We begin by adding the first two elements (the ``` <signature>``` and ``` <pubkey>```) to the stack.
3) The next item in our script is ```OP_DUP```. This informs us that we should duplicate the top item on the stack and add the result back to the top of the stack.
4) Now we have ```OP_HASH160```. This informs us that we should calculate the HASH160 of the top item on the stack and, again, add the result back to the top of the stack. This step ensures that the public key corresponds to the correct hash stored in the locking script, verifying ownership.
5) The next item in our script is a ```pubkey_hash```. We'll go ahead and add this to the top of our stack.
6) Now we see ```OP_EQUALVERIFY```. This means we should compare the two top elements of the stack and check if they are equal. If they are equal, we take them off the stack. If they are not equal, we fail our execution, and we are not allowed to spend these bitcoin.
7) Finally, we run ```OP_CHECKSIG```. At this point, the only two items on the stack are the ``` <signature>``` and ``` <pubkey>```. ```OP_CHECKSIG``` will check if the signature is valid for the public key provided. If so, it returns a 1 - meaning we successfully exectuted our script and can send the bitcoin. If false, it returns 0, and we cannot spend the bitcoin.
<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/p2pkhStack.png" alt="p2pkhStack" width="100%" height="auto">
</p>

</details>

<br/>

## ⚡️ Build A Function To Generate Pay-to-Public-Key-Hash Output Scripts

Now that we've briefly reviewed how a Pay-to-Public-Key-Hash script works, let's get our hands dirty with a little coding. For this exercise (and for much of the workshop), we'll be using the [Rust bitcoin library](https://docs.rs/bitcoin/latest/bitcoin/), a popular Rust libary for interacting with the Bitcoin protocol.

Let's build a function that takes in a bitcoin PublicKey and returns a Script that can be used as an output in a Pay-to-Public-Key-Hash transaction. As we saw above, a Pay-to-Public-Key-Hash **script** has the form: 

```
OP_DUP OP_HASH160 <pubkey_hash> OP_EQUALVERIFY OP_CHECKSIG
```

We can define a function that takes in a public key and outputs a script like this:
```rust
fn p2pkh(pubkey: &PublicKey) -> Script {}
```
<br/>

The bitcoin library we are using provides a ```Builder``` object we can use to construct any ```Script``` we want. It offers a handful of helper functions for adding opcodes, ints, bytes, and keys to a Script:
<br/><br/>

* `Builder::new()` - construct a new builder object
* `.push_opcode(op)` - add an opcode to the script
* `.push_int(num)` - add a number to the script
* `.push_key(public_key)` - add a `PublicKey` to the script
* `.push_pubkey_hash(public_key)` - add a `PubkeyHash` of a `PublicKey` to the script
* `.push_signature(signature)` - add a signature to the script
* `.push_script(script)` - add another script to this script
* `.into_script()` - return the resulting `Script` from the `Builder`  

<br/>

We can build the entire P2PKH script using the methods available on the Builder like this:

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

### When you think you have the solution, click the big green ```Run``` button at the top of the screen to make sure the unit tests are still passing.


# P2WPKH

Now that we have a handle on **P2PKH**, it's worth noting how **P2PKH** changes for Segwit transactions, called **Pay-to-Witness-Public-Key-Hash (P2WPKH)**.

**P2WPKH** is a special type of transaction that does not use the traditional script language. Instead, Bitcoin Core will see that this output is locked to a **P2WPKH** script and evaluate it accordingly. We just have to put the following in the `scriptPubKey` field:
- `OP_0`: This is called a **version byte**, and it indicates that this output is either **Pay-to-Witness-Public-Key-Hash (P2WPKH)** or **Pay-to-Witness-Script-Hash (P2WSH)**, which we'll learn about shortly.
- `<20 byte public key hash>`: To produce this, we hash the public key using the **HASH160** function, which will produce a 20-byte-hash.

<p align="center" style="width: 50%; max-width: 300px;">
  <img src="./tutorial_images/intro_to_htlc/p2wpkh.png" alt="p2wpkh" width="80%" height="auto">
</p>