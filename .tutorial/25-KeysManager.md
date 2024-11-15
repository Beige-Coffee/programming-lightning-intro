# KeysManager

LDK provides a simple default ```KeysManager``` implementation which provides key management and signing oprations for a Lightning node. To instantiate a ```KeysManager```, we simply provide a 32-byte `seed` and some random integers (to ensure uniqueness across restarts). The  ```KeysManager``` will use the `seed` as a BIP 32 extended key and derive all keys from that.

```rust
pub struct KeysManager {
  secp_ctx: Secp256k1<secp256k1::All>,
  node_secret: SecretKey,
  node_id: PublicKey,
  inbound_payment_key: KeyMaterial,
  destination_script: ScriptBuf,
  shutdown_pubkey: PublicKey,
  channel_master_key: Xpriv,
  channel_child_index: AtomicUsize,
  entropy_source: RandomBytes,
  seed: [u8; 32],
  starting_time_secs: u64,
  starting_time_nanos: u32,
}
```

## Key Functions
1. **Node Operations** (via `NodeSigner`)
   - Node identification
   - Invoice signing
   - Message encryption (ECDH)
   - Gossip message signing

2. **Channel Operations** (via `SignerProvider` & `ChannelSigner`)
   - Derives unique signers for each channel
   - Manages channel funding keys
   - Handles commitment transaction signing
   - Manages revocation keys

3. **On-chain Operations**
   - Manages destination scripts for receiving funds
   - Handles channel closure transactions
   - Signs justice transactions


## ⚡️ Create A Unified (On-Chain + Off-Chain) Wallet
LDK makes it simple to combine an on-chain and off-chain wallet within the same app. 

