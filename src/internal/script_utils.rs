
use bitcoin::secp256k1::PublicKey;
use bitcoin::script::ScriptBuf;
use bitcoin::PublicKey as BitcoinPublicKey;

pub fn p2wpkh_output_script(public_key: PublicKey) -> ScriptBuf {
    let pubkey = BitcoinPublicKey::new(public_key);
    ScriptBuf::new_p2wpkh(&pubkey.wpubkey_hash().unwrap())
}
