use bitcoin::blockdata::opcodes;
use bitcoin::blockdata::script::{Builder as BitcoinBuilder, Script};
use bitcoin::PublicKey;
use bitcoin::secp256k1::ecdsa::Signature;

pub struct Builder {
    inner: BitcoinBuilder
}

impl Builder {
  pub fn new() -> Self {
    Self {
      inner: BitcoinBuilder::new()
    }
  }

  pub fn push_script(mut self, script: Script) -> Self {
    let mut combined_script = self.inner.into_script().into_bytes();
    combined_script.append(&mut script.into_bytes());
    self.inner = combined_script.into();
    self
  }

  pub fn push_signature(mut self, signature: Signature) -> Self {
    self.inner = self.inner.push_slice(&signature.serialize_der());
    self
  }

  pub fn push_opcode(mut self, opcode: opcodes::All) -> Self {
    self.inner = self.inner.push_opcode(opcode);
    self
  }

  pub fn push_bytes(mut self, bytes: &[u8]) -> Self {
    self.inner = self.inner.push_slice(bytes);
    self
  }

  pub fn push_pubkey_hash(mut self, key: &PublicKey) -> Self {
    self.inner = self.inner.push_slice(&key.pubkey_hash());
    self
  }

  pub fn push_int(mut self, int: i64) -> Self {
    self.inner = self.inner.push_int(int);
    self
  }

  pub fn push_key(mut self, key: &PublicKey) -> Self {
    self.inner = self.inner.push_key(key);
    self
  }

  pub fn into_script(self) -> Script {
    self.inner.into_script()
  }
}