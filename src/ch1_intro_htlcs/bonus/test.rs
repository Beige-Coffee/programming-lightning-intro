#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::ch1_intro_htlcs::bonus::revocation_keys::{NodeKeysManager, Basepoint};
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
fn test_node_keys_manager() {
    let countersignatory_basepoint = pubkey_from_private_key(&[0x01; 32]);
    let seed = &[0x02; 32];
    let keys_manager = NodeKeysManager::new(*seed);

    let actual = keys_manager.node_id.to_string();

    let expected = "02888f2e3df341d21240f769afd83938fdc1878356769aae64141880d810c61dce";
        
    assert_eq!(
        actual, expected,
        "Revocation pubkey doesn't match expected value"
    );
}

#[test]
fn test_derive_channel_keys() {
    let countersignatory_basepoint = pubkey_from_private_key(&[0x01; 32]);
    let seed = &[0x02; 32];
    let keys_manager = NodeKeysManager::new(*seed);
    let secp_ctx = &Secp256k1::new();
    let channel_id = &[0x05; 32];

    let channel_keys_manager = keys_manager.derive_channel_keys(channel_id);

    let htlc_base_key = channel_keys_manager.htlc_base_key;

    let actual =  hex::encode(htlc_base_key.secret_bytes());

    let expected = "16bd658180e8edb308baded18cf09fc5fb831ce9ee7bd70c50e6af58dd5b160d";

    assert_eq!(
        actual, expected,
        "Revocation pubkey doesn't match expected value"
    );
}

#[test]
fn test_derive_private_key() {
    let countersignatory_basepoint = pubkey_from_private_key(&[0x01; 32]);
    let seed = &[0x02; 32];
    let keys_manager = NodeKeysManager::new(*seed);
    let secp_ctx = &Secp256k1::new();
    let channel_id = &[0x05; 32];
    let commitment_idx = 5;

    let channel_keys_manager = keys_manager.derive_channel_keys(channel_id);

    let htlc_commitment_key = channel_keys_manager.derive_private_key(
        Basepoint::HTLC,
        commitment_idx,
        secp_ctx);

    let actual =  hex::encode(htlc_commitment_key.secret_bytes());

    let expected = "4fefaebd76a2d2b98262a435dd101be4e285988d4d0c6a054954b4e96961db7b";

    assert_eq!(
        actual, expected,
        "Revocation pubkey doesn't match expected value"
    );
}

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