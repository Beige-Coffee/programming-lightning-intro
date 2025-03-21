#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use bitcoin::script::{Builder as BitcoinBuilder, ScriptBuf, ScriptHash};
use bitcoin::opcodes;
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::secp256k1::PublicKey;
use bitcoin::PublicKey as BitcoinPublicKey;
use bitcoin::script::PushBytes;
use bitcoin::PubkeyHash;
use bitcoin::hashes::Hash;

pub struct Builder {
    inner: BitcoinBuilder,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            inner: BitcoinBuilder::new(),
        }
    }

    pub fn push_script(mut self, script: ScriptBuf) -> Self {
        let mut combined_script = self.inner.into_script().into_bytes();
        combined_script.append(&mut script.into_bytes());
        self.inner = combined_script.into();
        self
    }

    pub fn push_signature(mut self, signature: Signature) -> Self {
        self.inner = self.inner.push_slice(&signature.serialize_compact());
        self
    }

    pub fn push_opcode(mut self, opcode: opcodes::Opcode) -> Self {
        self.inner = self.inner.push_opcode(opcode);
        self
    }

    pub fn push_slice<T: AsRef<PushBytes>>(mut self, data: T) -> Self {
        self.inner = self.inner.push_slice(data);
        self
    }

    pub fn push_script_hash(mut self, script_hash: &ScriptHash) -> Self {
      self.inner = self.inner.push_slice(&script_hash);
      self
    }

    pub fn push_pubkey_hash(mut self, key: &PublicKey) -> Self {
        let pubkey_hash = &PubkeyHash::hash(&key.serialize());
        self.inner = self.inner.push_slice(&pubkey_hash);
        self
    }

    pub fn push_int(mut self, int: i64) -> Self {
        self.inner = self.inner.push_int(int);
        self
    }

    pub fn push_key(mut self, key: &PublicKey) -> Self {
        let serialized_key = key.serialize();
        self.inner = self.inner.push_slice(serialized_key);
        self
    }

    pub fn push_key2(mut self, key: &BitcoinPublicKey) -> Self {
        self.inner = self.inner.push_key(key);
        self
    }

    pub fn into_script(self) -> ScriptBuf {
        self.inner.into_script()
    }
}
