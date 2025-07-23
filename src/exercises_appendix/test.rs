#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use crate::exercises_appendix::exercises::{NodeKeysManager, Basepoint};
use bitcoin::secp256k1;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::PublicKey;
use bitcoin::secp256k1::SecretKey;
use internal::key_utils::{add_pubkeys, pubkey_multipication_tweak, pubkey_from_secret, add_privkeys, pubkey_from_private_key, privkey_multipication_tweak, hash_pubkeys};
use internal::tx_utils::{build_output, build_transaction};
use internal::script_utils::{build_htlc_offerer_witness_script, p2wpkh_output_script};

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
    let channel_id: u32 = 1;

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
    let channel_id: u32 = 1;
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
fn test_derive_revocation_public_key() {
    let countersignatory_basepoint = pubkey_from_private_key(&[0x01; 32]);
    let seed = &[0x02; 32];
    let keys_manager = NodeKeysManager::new(*seed);
    let secp_ctx = &Secp256k1::new();
    let commitment_idx = 5;
    let channel_id: u32 = 1;

    let channel_keys_manager = keys_manager.derive_channel_keys(channel_id);

    let pubkey = channel_keys_manager.derive_revocation_public_key(
            countersignatory_basepoint,
            commitment_idx,
            secp_ctx);

    let actual = pubkey.to_string();

    let expected = "02042982acd4701a09a3c589d66344559e338e532a343f87576e1595c863895811";

    assert_eq!(
        actual, expected,
        "Revocation pubkey doesn't match expected value"
    );
}