#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::exercises::exercises::{
    build_commitment_transaction, build_funding_transaction, build_htlc_commitment_transaction,
    build_htlc_timeout_transaction, build_refund_transaction, generate_revocation_privkey,
    generate_revocation_pubkey, to_local, two_of_two_multisig_witness_script,
};
use crate::internal;
use bitcoin::hash_types::Txid;
use bitcoin::script::ScriptBuf;
use bitcoin::secp256k1::{self, Secp256k1};
use bitcoin::secp256k1::{PublicKey as secp256k1PublicKey, Scalar, SecretKey};
use bitcoin::PublicKey;
use bitcoin::{OutPoint, Sequence, Transaction, TxIn, Witness};
use internal::key_utils::{
    add_privkeys, add_pubkeys, hash_pubkeys, privkey_multipication_tweak, pubkey_from_private_key,
    pubkey_from_secret, pubkey_multipication_tweak, secp256k1_private_key,
    secp256k1pubkey_from_private_key,
};
use internal::script_utils::{build_htlc_offerer_witness_script, p2wpkh_output_script};
use internal::tx_utils::{build_output, build_transaction};

/// hash160 of the empty string
const HASH160_DUMMY: [u8; 20] = [
    0xb4, 0x72, 0xa2, 0x66, 0xd0, 0xbd, 0x89, 0xc1, 0x37, 0x06, 0xa4, 0x13, 0x2c, 0xcf, 0xb1, 0x6f,
    0x7c, 0x3b, 0x9f, 0xcb,
];

#[test]
fn test_01_two_of_two_multisig_witness_script() {
    let pubkey1 = pubkey_from_private_key(&[0x01; 32]);
    let pubkey2 = pubkey_from_private_key(&[0x02; 32]);
    let result = two_of_two_multisig_witness_script(&pubkey1, &pubkey2);

    let their_solution = format!("{}", result.script_hash());
    println!("their solution: {}", their_solution);
    let acceptable_solutions = ["d8fecda80c30e89a9e7f0964ee79ce055288bc1c".to_string()];

    assert!(acceptable_solutions.contains(&their_solution))
}

#[test]
fn test_02_build_funding_transaction() {
    let outpoint = OutPoint::new(
        "d9334caed6503ebc710d13a5f663f03bec531026d2bc786befdfdb8ef5aad721"
            .parse::<Txid>()
            .unwrap(),
        1,
    );

    let txin = vec![TxIn {
        previous_output: outpoint,
        script_sig: ScriptBuf::new(),
        sequence: Sequence::MAX,
        witness: Witness::new(),
    }];

    let pubkey1 = pubkey_from_private_key(&[0x01; 32]);
    let pubkey2 = pubkey_from_private_key(&[0x02; 32]);

    let amount: u64 = 100000;

    let transaction = build_funding_transaction(txin, &pubkey1, &pubkey2, amount);

    let their_solution = transaction.compute_txid().to_string();

    println!("their solution: {}", their_solution);

    let acceptable_solutions =
        ["8f28cec85c8d986559c7bf5760d57d57446e26f27ac3ed623d591e4579b7bc9c".to_string()];

    assert!(acceptable_solutions.contains(&their_solution));
}

#[test]
fn test_03_build_refund_transaction() {
    let outpoint = OutPoint::new(
        "d9334caed6503ebc710d13a5f663f03bec531026d2bc786befdfdb8ef5aad721"
            .parse::<Txid>()
            .unwrap(),
        1,
    );

    let txin = TxIn {
        previous_output: outpoint,
        script_sig: ScriptBuf::new(),
        sequence: Sequence::MAX,
        witness: Witness::new(),
    };

    let pubkey1 = pubkey_from_private_key(&[0x01; 32]);
    let pubkey2 = pubkey_from_private_key(&[0x02; 32]);

    let alice_amount: u64 = 4_998_500;
    let bob_amount: u64 = 500;

    let transaction = build_refund_transaction(txin, pubkey1, pubkey2, alice_amount, bob_amount);

    let their_solution = transaction.compute_txid().to_string();

    println!("their solution: {}", their_solution);

    let acceptable_solutions =
        ["e66414f0d4dca7df235b9e2cf4855f0cf64b9d4164412fd03c8670621bd398ff".to_string()];

    assert!(acceptable_solutions.contains(&their_solution));
}

#[test]
fn test_04_generate_revocation_pubkey() {
    let countersignatory_basepoint = secp256k1pubkey_from_private_key(&[0x01; 32]);
    let per_commitment_point = secp256k1pubkey_from_private_key(&[0x02; 32]);

    let revocation_pubkey =
        generate_revocation_pubkey(countersignatory_basepoint, per_commitment_point);

    let key_str = "0c8415f91d9df28009f87442dfc897d3c22a534032d4795467206db994a3c539";
    let key_bytes = hex::decode(key_str).expect("Invalid hex string");
    let key = SecretKey::from_slice(&key_bytes).expect("Invalid 32-byte private key");
    let secp = Secp256k1::new();
    let derived_pubkey = secp256k1PublicKey::from_secret_key(&secp, &key);

    let actual = revocation_pubkey.to_string();

    let expected = "02f406b2b2ef2372c08d003e5062e4b34929b86f107a36bfbd406f5644419e9ff6";

    assert_eq!(
        actual, expected,
        "Revocation pubkey doesn't match expected value"
    );

    assert_eq!(
        derived_pubkey, revocation_pubkey,
        "Secret does not work for pubkey"
    );
}

#[test]
fn test_05_generate_revocation_privkey() {
    let countersignatory_secret = secp256k1_private_key(&[0x01; 32]);
    let per_commitment_secret = secp256k1_private_key(&[0x02; 32]);

    let revocation_privkey =
        generate_revocation_privkey(countersignatory_secret, per_commitment_secret);

    let actual = revocation_privkey.display_secret().to_string();

    let expected = "0c8415f91d9df28009f87442dfc897d3c22a534032d4795467206db994a3c539";

    assert_eq!(
        actual, expected,
        "Revocation pubkey doesn't match expected value"
    );
}

#[test]
fn test_06_to_local() {
    let revocation_key = pubkey_from_private_key(&[0x01; 32]);
    let to_local_delayed_pubkey = pubkey_from_private_key(&[0x02; 32]);
    let to_self_delay: i64 = 144;
    let result = to_local(&revocation_key, &to_local_delayed_pubkey, to_self_delay);

    let their_solution = format!("{}", result.script_hash());
    println!("their solution: {}", their_solution);
    let acceptable_solutions = ["6c61fd62fe96fc6aa5485a820ce87d662329f4ab".to_string()];

    assert!(acceptable_solutions.contains(&their_solution))
}

#[test]
fn test_07_build_commitment_transaction() {
    let outpoint = OutPoint::new(
        "d9334caed6503ebc710d13a5f663f03bec531026d2bc786befdfdb8ef5aad721"
            .parse::<Txid>()
            .unwrap(),
        1,
    );

    let txin = TxIn {
        previous_output: outpoint,
        script_sig: ScriptBuf::new(),
        sequence: Sequence::MAX,
        witness: Witness::new(),
    };

    let revocation_pubkey = pubkey_from_private_key(&[0x01; 32]);
    let to_local_delayed_pubkey = pubkey_from_private_key(&[0x02; 32]);
    let remote_pubkey = pubkey_from_private_key(&[0x03; 32]);

    let to_self_delay: i64 = 144;
    let alice_amount: u64 = 3_998_500;
    let bob_amount: u64 = 1_000_500;

    let transaction = build_commitment_transaction(
        txin,
        &revocation_pubkey,
        &to_local_delayed_pubkey,
        remote_pubkey,
        to_self_delay,
        alice_amount,
        bob_amount,
    );

    let their_solution = transaction.compute_txid().to_string();

    println!("their solution: {}", their_solution);

    let acceptable_solutions =
        ["83aef4d9008ac14c71967a7944f2f0b8bcb30ef58ae2946e0c550de6bb908cba".to_string()];

    assert!(acceptable_solutions.contains(&their_solution));
}

#[test]
fn test_08_build_htlc_commitment_transaction() {
    let outpoint = OutPoint::new(
        "d9334caed6503ebc710d13a5f663f03bec531026d2bc786befdfdb8ef5aad721"
            .parse::<Txid>()
            .unwrap(),
        1,
    );

    let txin = TxIn {
        previous_output: outpoint,
        script_sig: ScriptBuf::new(),
        sequence: Sequence::MAX,
        witness: Witness::new(),
    };

    let revocation_pubkey = pubkey_from_private_key(&[0x01; 32]);
    let remote_htlc_pubkey = pubkey_from_private_key(&[0x02; 32]);
    let local_htlc_pubkey = pubkey_from_private_key(&[0x03; 32]);
    let to_local_delayed_pubkey = pubkey_from_private_key(&[0x02; 32]);
    let remote_pubkey = pubkey_from_private_key(&[0x02; 32]);

    let to_self_delay: i64 = 144;
    let payment_hash160 = HASH160_DUMMY;
    let htlc_amount: u64 = 405_000;
    let local_amount: u64 = 3_593_500;
    let remote_amount: u64 = 1_000_500;

    let transaction = build_htlc_commitment_transaction(
        txin,
        &revocation_pubkey,
        &remote_htlc_pubkey,
        &local_htlc_pubkey,
        &to_local_delayed_pubkey,
        remote_pubkey,
        to_self_delay,
        &payment_hash160,
        htlc_amount,
        local_amount,
        remote_amount,
    );

    let their_solution = transaction.compute_txid().to_string();

    println!("their solution: {}", their_solution);

    let acceptable_solutions =
        ["cecd7f4c7bebfddbdd563fd96bab55a1d5d72b672518104aaa68e9bbf99a4acb".to_string()];

    assert!(acceptable_solutions.contains(&their_solution));
}

#[test]
fn test_09_build_htlc_timeout_transaction() {
    let outpoint = OutPoint::new(
        "d9334caed6503ebc710d13a5f663f03bec531026d2bc786befdfdb8ef5aad721"
            .parse::<Txid>()
            .unwrap(),
        1,
    );

    let txin = TxIn {
        previous_output: outpoint,
        script_sig: ScriptBuf::new(),
        sequence: Sequence::MAX,
        witness: Witness::new(),
    };

    let revocation_pubkey = pubkey_from_private_key(&[0x01; 32]);
    let broadcaster_delayed_payment_key = pubkey_from_private_key(&[0x02; 32]);
    let local_htlc_pubkey = pubkey_from_private_key(&[0x03; 32]);
    let to_local_delayed_pubkey = pubkey_from_private_key(&[0x02; 32]);
    let remote_pubkey = pubkey_from_private_key(&[0x02; 32]);

    let contest_delay: i64 = 144;
    let cltv_expiry: u32 = 300;
    let htlc_amount: u64 = 404_000;

    let transaction = build_htlc_timeout_transaction(
        txin,
        &revocation_pubkey,
        &broadcaster_delayed_payment_key,
        contest_delay,
        cltv_expiry,
        htlc_amount,
    );

    let their_solution = transaction.compute_txid().to_string();

    println!("their solution: {}", their_solution);

    let acceptable_solutions =
        ["5899cdd0e418b516afa2552611aad22f974ac17e9892f5828e2e55f18b2d7899".to_string()];

    assert!(acceptable_solutions.contains(&their_solution));
}
