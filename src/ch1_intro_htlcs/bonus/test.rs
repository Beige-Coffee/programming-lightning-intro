#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::ch1_intro_htlcs::bonus::revocation_keys::{NodeKeysManager};
use crate::ch1_intro_htlcs::bonus::exercise::{revocation_pubkey};
use bitcoin::secp256k1;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::PublicKey;
use bitcoin::secp256k1::SecretKey;
use crate::ch1_intro_htlcs::exercises::{
  generate_revocation_pubkey
};
use crate::internal::helper::{
    bitcoin_pubkey_from_private_key, pubkey_from_private_key, secp256k1_private_key,
};

#[test]
fn test_revocation_pubkey_deep_dive() {
    let countersignatory_basepoint = pubkey_from_private_key(&[0x01; 32]);
    let seed = &[0x02; 32];
    let keys_manager = NodeKeysManager::new(*seed);
    let secp_ctx = &Secp256k1::new();
    let channel_id = &[0x05; 32];

    let pubkey = revocation_pubkey(
        countersignatory_basepoint,
        keys_manager,
        secp_ctx,
        channel_id);

    let actual = pubkey.to_string();

    let expected = "03b4f6d4c1bde8a10e20db379e68946cfc90d5e2316ee43a28e95cd594a4488c57";
        
    assert_eq!(
        actual, expected,
        "Revocation pubkey doesn't match expected value"
    );
}