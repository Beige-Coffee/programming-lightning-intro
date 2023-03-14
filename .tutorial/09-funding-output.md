# Funding Output

We have learned that we want our payment channel funding transaction output to have two spending paths.  One spending path is the 2-of-2 multisig that requires signatures from both parties to spend and the other path is a relative timelocked pay-to-pubkey-hash that only requires the funders signature after some amount of time.

The 2-of-2 multisig path will be spent by payments that are made during normal channel operation and the timelocked path will be spent by Alice in a refund transaction if Bob becomes unresponsive.

## Multiple spending paths

Recall we can create multiple spending paths in a single output by using OP_IF, OP_ELSE, and OP_ENDIF.  A typical two spending path script might look like this:

```
OP_IF <spending_path_1> OP_ELSE <spending_path_2> OP_ENDIF 
```


<br/><br/>

## ⚡️ Build a simple payment channel's funding output script given the two parties public keys and a maximum age of the channel in blocks.

Let's bring everything together and write a function we can use to generate a funding transaction for our simple payment channels

```rust
fn payment_channel_funding_output(alice_pubkey: &PublicKey, bob_pubkey: &PublicKey, max_age_in_blocks: i64) -> Script {}
```