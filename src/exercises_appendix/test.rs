#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;
use crate::exercises_appendix::exercises::{NodeKeysManager, Basepoint};
use crate::exercises_appendix::tx_exercises::{get_commitment_transaction_number_obscure_factor,
                                             build_commitment_input,
                                             build_commitment_locktime};
use crate::exercises_appendix::solutions::{build_commitment_input as build_commitment_input_answer };
use bitcoin::secp256k1;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::PublicKey;
use bitcoin::secp256k1::SecretKey;
use bitcoin::{OutPoint, Sequence, Transaction, TxIn, Witness};
use bitcoin::hash_types::Txid;
use internal::key_utils::{add_pubkeys, pubkey_multipication_tweak, pubkey_from_secret, add_privkeys, pubkey_from_private_key, privkey_multipication_tweak, hash_pubkeys,
                         secp256k1pubkey_from_private_key};
use internal::tx_utils::{build_output, build_transaction};
use internal::script_utils::{build_htlc_offerer_witness_script, p2wpkh_output_script};
use bitcoin::locktime::absolute::LockTime;

pub const INITIAL_COMMITMENT_NUMBER: u64 = (1 << 48) - 1;

#[test]
fn test_get_commitment_transaction_number_obscure_factor() {
    let open_basepoint = secp256k1pubkey_from_private_key(&[0x01; 32]);
    let accept_basepoint = secp256k1pubkey_from_private_key(&[0x01; 32]);

    let obscure_factor = get_commitment_transaction_number_obscure_factor(
        &open_basepoint,
        &accept_basepoint
    );

    let expected: u64 = 239765233721135;

    assert_eq!(
        obscure_factor, expected,
        "Obscure factor doesn't match expected value"
    );
}

#[test]
fn test_build_commitment_input() {
    let outpoint = OutPoint::new(
        "d9334caed6503ebc710d13a5f663f03bec531026d2bc786befdfdb8ef5aad721"
            .parse::<Txid>()
            .unwrap(),
        1,
    );

    let obscure_factor: u64 = 239765233721135;
    let commitment_number = INITIAL_COMMITMENT_NUMBER - 10;

    let tx_input = build_commitment_input(
          outpoint,
          &obscure_factor,
          &commitment_number
    );

    let tx_input_answer = build_commitment_input_answer(
          outpoint,
          &obscure_factor,
          &commitment_number
    );

    assert_eq!(
        tx_input, tx_input_answer,
        "Commitment doesn't match expected value"
    );
}

#[test]
fn test_build_commitment_locktime() {
    let obscure_factor: u64 = 239765233721135;
    let commitment_number = INITIAL_COMMITMENT_NUMBER - 10;

    let obscured_commitment_transaction_number = 
        obscure_factor ^ commitment_number;

    let answer = LockTime::from_consensus(((0x20 as u32) << 8 * 3) |
                ((obscured_commitment_transaction_number & 0xffffffu64) as u32));

    let locktime = build_commitment_locktime(
        &obscure_factor,
        &commitment_number
    );

    assert_eq!(
        locktime, answer,
        "Locktime doesn't match expected value"
    );
}

#[test]
fn test_node_keys_manager() {
    let seed = &[0x02; 32];
    let keys_manager = NodeKeysManager::new(*seed);

    let actual = keys_manager.node_id.to_string();

    let expected = "02888f2e3df341d21240f769afd83938fdc1878356769aae64141880d810c61dce";
        
    assert_eq!(
        actual, expected,
        "Node ID doesn't match expected value"
    );
}

#[test]
fn test_derive_channel_keys() {
    let seed = &[0x02; 32];
    let keys_manager = NodeKeysManager::new(*seed);
    let secp_ctx = &Secp256k1::new();
    let channel_id: u32 = 1;

    let channel_keys_manager = keys_manager.derive_channel_keys(channel_id);

    let htlc_base_key = channel_keys_manager.htlc_base_key;

    let actual =  hex::encode(htlc_base_key.secret_bytes());

    let expected = "4198bf640ed19ac652ccc67180bb6e7233b42c8e7a9b9deedff400ef628be33e";

    assert_eq!(
        actual, expected,
        "htlc_base_key doesn't match expected value"
    );
}

#[test]
fn test_derive_private_key() {
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

    let expected = "112d3c7fe11adc1239484b95c879049796786eaaf56342379183a746e30f2dfc";

    assert_eq!(
        actual, expected,
        "HTLC key doesn't match expected value"
    );
}

#[test]
fn test_derive_revocation_public_key() {
    let countersignatory_basepoint = secp256k1pubkey_from_private_key(&[0x01; 32]);
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

    let expected = "03f582ebe04e6711cd72eada2c4778eb801922cf8656045d0f078f436d99c9ff47";

    assert_eq!(
        actual, expected,
        "Revocation pubkey doesn't match expected value"
    );
}