#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal::bitcoind_client::BitcoindClient;
use crate::internal::channel_manager::ChannelManager;
use crate::ch1_intro_htlcs::exercises::{
    block_connected, build_htlc_offerer_witness_script, build_htlc_receiver_witness_script,
    channel_closed, cltv_p2pkh, csv_p2pkh, generate_revocation_pubkey,
    p2pkh, p2sh, payment_channel_funding_output, spend_multisig,generate_p2wsh_signature,
    spend_refund, two_of_two_multisig_redeem_script, build_timelocked_transaction, build_multisig_transaction
}; // handle_funding_generation_ready
use crate::internal::helper::{pubkey_multiplication_tweak, sha256_hash, secp256k1_private_key};
use bitcoin::sighash::EcdsaSighashType;
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::amount::Amount;
use bitcoin::hash_types::Txid;
use bitcoin::hashes::hex::FromHex;
use bitcoin::hashes::Hash;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::script::{ScriptBuf, ScriptHash};
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::secp256k1::PublicKey as Secp256k1PublicKey;
use bitcoin::secp256k1::Scalar;
use bitcoin::secp256k1::{self, Secp256k1};
use bitcoin::transaction::Version;
use bitcoin::PubkeyHash;
use bitcoin::{OutPoint, PublicKey, Sequence, Transaction, TxIn, Witness};

/// hash160 of the empty string
const HASH160_DUMMY: [u8; 20] = [
    0xb4, 0x72, 0xa2, 0x66, 0xd0, 0xbd, 0x89, 0xc1, 0x37, 0x06, 0xa4, 0x13, 0x2c, 0xcf, 0xb1, 0x6f,
    0x7c, 0x3b, 0x9f, 0xcb,
];

//#[test]
//fn test_handle_funding_generation_ready() {
//let bitcoind_client = BitcoindClient::new();
//let channel_manager = ChannelManager::new();
//let temporary_channel_id: [u8; 32] = [3; 32];
//let counterparty_node_id = pubkey_from_private_key(&[0x01; 32]);
//let channel_value_satoshis = Amount::from_sat(1000000);
//let output_script = ScriptBuf::new();
//let user_channel_id = 0;

//let result = std::panic::catch_unwind(|| {
//handle_funding_generation_ready(
//&channel_manager,
//&bitcoind_client,
//&temporary_channel_id,
//&counterparty_node_id,
//channel_value_satoshis,
//output_script,
//user_channel_id,
//)
//});

//match result {
//Ok(_) => {
//let last_funding_tx_gen = channel_manager.last_funding_tx_gen.lock().unwrap();
//assert!(last_funding_tx_gen.is_some());
//
//let (temp_cid, node_id, tx_hex) = last_funding_tx_gen.clone().unwrap();
//assert_eq!(temp_cid, temporary_channel_id);
//assert_eq!(node_id, counterparty_node_id);
//assert_eq!(tx_hex, "signedtxhex".to_string());
//}
//Err(e) => {
//if let Ok(string) = e.downcast::<String>() {
//println!("{}", string);
//}
//}
//}
//}

#[test]
fn test_channel_closed() {
    let correct_outpoint = OutPoint::new(
        "d9334caed6503ebc710d13a5f663f03bec531026d2bc786befdfdb8ef5aad721"
            .parse::<Txid>()
            .unwrap(),
        0,
    );
    let wrong_txid_outpoint = OutPoint::new(
        "d9334caed6503ebc710d13a5f663f03bec531026d2bc786befdfdb8ef5aad722"
            .parse::<Txid>()
            .unwrap(),
        0,
    );
    let wrong_vout_outpoint = OutPoint::new(
        "d9334caed6503ebc710d13a5f663f03bec531026d2bc786befdfdb8ef5aad721"
            .parse::<Txid>()
            .unwrap(),
        1,
    );

    let correct_block = bitcoin::Block {
        header: bitcoin::blockdata::constants::genesis_block(bitcoin::Network::Bitcoin).header,
        txdata: vec![Transaction {
            version: Version::ONE,
            lock_time: LockTime::ZERO,
            input: vec![TxIn {
                previous_output: correct_outpoint,
                script_sig: ScriptBuf::new(),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            }],
            output: vec![],
        }],
    };

    let wrong_vout_block = bitcoin::Block {
        header: bitcoin::blockdata::constants::genesis_block(bitcoin::Network::Bitcoin).header,
        txdata: vec![Transaction {
            version: Version::ONE,
            lock_time: LockTime::ZERO,
            input: vec![TxIn {
                previous_output: wrong_vout_outpoint,
                script_sig: ScriptBuf::new(),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            }],
            output: vec![],
        }],
    };

    let wrong_txid_block = bitcoin::Block {
        header: bitcoin::blockdata::constants::genesis_block(bitcoin::Network::Bitcoin).header,
        txdata: vec![Transaction {
            version: Version::ONE,
            lock_time: LockTime::ZERO,
            input: vec![TxIn {
                previous_output: wrong_txid_outpoint,
                script_sig: ScriptBuf::new(),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            }],
            output: vec![],
        }],
    };

    let result = std::panic::catch_unwind(|| {
        let correct = channel_closed(correct_outpoint, correct_block);
        let wrong_txid = channel_closed(correct_outpoint, wrong_txid_block);
        let wrong_vout = channel_closed(correct_outpoint, wrong_vout_block);
        (correct, wrong_txid, wrong_vout)
    });

    match result {
        Ok((correct, wrong_txid, wrong_vout)) => {
            assert!(correct);
            assert!(!wrong_txid);
            assert!(!wrong_vout);
        }
        Err(e) => {
            if let Ok(string) = e.downcast::<String>() {
                println!("{}", string);
            }
        }
    }
}

#[test]
fn test_spend_refund() {
    let alice_pubkey = secp256k1_pubkey_from_private_key(&[0x01; 32]);
    let alice_signature = Signature::from_compact(&[0x01; 64]).unwrap();

    let result = std::panic::catch_unwind(|| spend_refund(&alice_pubkey, alice_signature));
    match result {
        Ok(script) => {
            let their_solution = format!("{}", script.script_hash());
            let acceptable_solutions = ["da169b57ccc280318504cd637e46bc01e2630e42".to_string()];
            assert!(acceptable_solutions.contains(&their_solution))
        }
        Err(e) => {
            if let Ok(string) = e.downcast::<String>() {
                println!("{}", string);
            }
        }
    }
}

#[test]
fn test_spend_multisig() {
    let alice_signature = Signature::from_compact(&[0x01; 64]).unwrap();
    let bob_signature = Signature::from_compact(&[0x02; 64]).unwrap();

    let result = std::panic::catch_unwind(|| spend_multisig(alice_signature, bob_signature));

    match result {
        Ok(script) => {
            let their_solution = format!("{}", script.script_hash());
            let acceptable_solutions = ["678cb7114e47b48bc4bd2cc0aace68001f8208c2".to_string()];
            assert!(acceptable_solutions.contains(&their_solution))
        }
        Err(e) => {
            if let Ok(string) = e.downcast::<String>() {
                println!("{}", string);
            }
        }
    }
}

#[test]
fn test_block_connected() {
    let script = ScriptBuf::from(vec![0x01, 0x02, 0x03]);
    let amount = Amount::from_sat(1000000);
    let wrong_amount = Amount::from_sat(1000000 - 1);

    let correct_block = bitcoin::Block {
        header: bitcoin::blockdata::constants::genesis_block(bitcoin::Network::Bitcoin).header,
        txdata: vec![Transaction {
            version: Version::ONE,
            lock_time: LockTime::ZERO,
            input: vec![],
            output: vec![bitcoin::TxOut {
                value: amount,
                script_pubkey: script.clone(),
            }],
        }],
    };

    let wrong_amount_block = bitcoin::Block {
        header: bitcoin::blockdata::constants::genesis_block(bitcoin::Network::Bitcoin).header,
        txdata: vec![Transaction {
            version: Version::ONE,
            lock_time: LockTime::ZERO,
            input: vec![],
            output: vec![bitcoin::TxOut {
                value: wrong_amount,
                script_pubkey: script.clone(),
            }],
        }],
    };

    let wrong_script_block = bitcoin::Block {
        header: bitcoin::blockdata::constants::genesis_block(bitcoin::Network::Bitcoin).header,
        txdata: vec![Transaction {
            version: Version::ONE,
            lock_time: LockTime::ZERO,
            input: vec![],
            output: vec![bitcoin::TxOut {
                value: amount,
                script_pubkey: ScriptBuf::from(vec![0x01, 0x02, 0x04]),
            }],
        }],
    };

    let result = std::panic::catch_unwind(|| {
        let correct = block_connected(script.clone(), amount, correct_block);
        let wrong_amount = block_connected(script.clone(), amount, wrong_amount_block);
        let wrong_script = block_connected(script, amount, wrong_script_block);
        (correct, wrong_amount, wrong_script)
    });

    match result {
        Ok((correct, wrong_amount, wrong_script)) => {
            assert!(correct);
            assert!(!wrong_amount);
            assert!(!wrong_script);
        }
        Err(e) => {
            if let Ok(string) = e.downcast::<String>() {
                println!("{}", string);
            }
        }
    }
}

#[test]
fn test_payment_channel_funding_output() {
    let alice_pubkey = secp256k1_pubkey_from_private_key(&[0x01; 32]);
    let bob_pubkey = secp256k1_pubkey_from_private_key(&[0x02; 32]);
    let height = 1000000;

    let result = std::panic::catch_unwind(|| {
        payment_channel_funding_output(&bob_pubkey, &alice_pubkey, height)
    });

    match result {
        Ok(script) => {
            let their_solution = format!("{}", script.script_hash());
            println!("their solution: {}", their_solution);
            let acceptable_solutions = [
                "4a6a4f1fd56fd8684acce69eb49f7ae2b469b077".to_string(),
                "4b9ea4b7fba619a732e98fd54e913d560dcbd690".to_string(),
            ];
            assert!(acceptable_solutions.contains(&their_solution))
        }
        Err(e) => {
            if let Ok(string) = e.downcast::<String>() {
                println!("{}", string);
            }
        }
    }
}

#[test]
fn test_csv_p2pkh() {
    let alice_pubkey = secp256k1_pubkey_from_private_key(&[0x01; 32]);
    let timestamp: i64 = 1000000000;

    let result = std::panic::catch_unwind(|| csv_p2pkh(&alice_pubkey, timestamp));
    match result {
        Ok(script) => {
            assert_eq!(
                format!("{}", script.script_hash()),
                "9c727802a2cd91beb1f1a0a5fd8b391f61c76343".to_string()
            )
        }
        Err(e) => {
            if let Ok(string) = e.downcast::<String>() {
                println!("{}", string);
            }
        }
    }
}

#[test]
fn test_cltv_p2pkh() {
    let alice_pubkey = secp256k1_pubkey_from_private_key(&[0x01; 32]);
    let height: i64 = 1000000;

    let result = std::panic::catch_unwind(|| cltv_p2pkh(&alice_pubkey, height));
    match result {
        Ok(script) => {
            assert_eq!(
                format!("{}", script.script_hash()),
                "0ab02ee78c2adeca048d6c19287a84e171fee58d".to_string()
            )
        }
        Err(e) => {
            if let Ok(string) = e.downcast::<String>() {
                println!("{}", string);
            }
        }
    }
}

#[test]
fn test_two_of_two_multisig_redeem_script() {
    let alice_pubkey = secp256k1_pubkey_from_private_key(&[0x01; 32]);
    let bob_pubkey = secp256k1_pubkey_from_private_key(&[0x02; 32]);
    let result = std::panic::catch_unwind(|| two_of_two_multisig_redeem_script(&alice_pubkey, &bob_pubkey));

    match result {
        Ok(script) => {
            let their_solution = format!("{}", script.script_hash());
            println!("their solution: {}", their_solution);
            let acceptable_solutions = [
                "d8fecda80c30e89a9e7f0964ee79ce055288bc1c".to_string(),
                "e5c7b40d1f14542cc2c7a15819de088a33c8f7ba".to_string(),
            ];

            assert!(acceptable_solutions.contains(&their_solution))
        }
        Err(e) => {
            if let Ok(string) = e.downcast::<String>() {
                println!("{}", string);
            }
        }
    }
}

#[test]
fn test_p2pkh() {
    let alice_pubkey = secp256k1_pubkey_from_private_key(&[0x01; 32]);

    let result = std::panic::catch_unwind(|| p2pkh(&alice_pubkey));
    match result {
        Ok(script) => {
            assert_eq!(
                format!("{}", script.script_hash()),
                "832e012d4cd5f23df82efd34e473345a2f8aa4fb".to_string()
            )
        }
        Err(e) => {
            if let Ok(string) = e.downcast::<String>() {
                println!("{}", string);
            }
        }
    }
}

#[test]
fn test_p2sh() {
    let script_hash = ScriptHash::from_slice(&HASH160_DUMMY).unwrap();

    let result = std::panic::catch_unwind(|| p2sh(&script_hash));
    match result {
        Ok(script) => {
            assert_eq!(
                format!("{}", script.script_hash()),
                "92a04bc86e23f169691bd6926d11853cc61e1852".to_string()
            )
        }
        Err(e) => {
            if let Ok(string) = e.downcast::<String>() {
                println!("{}", string);
            }
        }
    }
}

#[test]
fn test_build_htlc_offerer_witness_script() {
    let remote_htlc_pubkey = secp256k1_pubkey_from_private_key(&[0x01; 32]);
    let local_htlc_pubkey = secp256k1_pubkey_from_private_key(&[0x02; 32]);
    let revocation_key = secp256k1_pubkey_from_private_key(&[0x02; 32]);
    let payment_hash160 = HASH160_DUMMY;
    let cltv_expiry: i64 = 1000000000;

    let result = std::panic::catch_unwind(|| {
        build_htlc_receiver_witness_script(
            &revocation_key,
            &remote_htlc_pubkey,
            &local_htlc_pubkey,
            &payment_hash160,
            cltv_expiry,
        )
    });

    match result {
        Ok(script) => {
            let their_solution = format!("{}", script.script_hash());
            println!("their solution: {}", their_solution);
            let acceptable_solutions = [
                "db3d8b6f8b95e2bc994dd29941b2431dd9c1d75e".to_string(), //123
            ];

            assert!(acceptable_solutions.contains(&their_solution))
        }
        Err(e) => {
            if let Ok(string) = e.downcast::<String>() {
                println!("{}", string);
            }
        }
    }
}

#[test]
fn test_build_htlc_receiver_witness_script() {
    let remote_htlc_pubkey = secp256k1_pubkey_from_private_key(&[0x01; 32]);
    let local_htlc_pubkey = secp256k1_pubkey_from_private_key(&[0x02; 32]);
    let revocation_key = secp256k1_pubkey_from_private_key(&[0x02; 32]);
    let payment_hash160 = HASH160_DUMMY;

    let result = std::panic::catch_unwind(|| {
        build_htlc_offerer_witness_script(
            &revocation_key,
            &remote_htlc_pubkey,
            &local_htlc_pubkey,
            &payment_hash160,
        )
    });

    match result {
        Ok(script) => {
            let their_solution = format!("{}", script.script_hash());
            println!("their solution: {}", their_solution);
            let acceptable_solutions = [
                "8bef95533bbe782c3ad32343aeae56006b2096a6".to_string(), //123
            ];

            assert!(acceptable_solutions.contains(&their_solution))
        }
        Err(e) => {
            if let Ok(string) = e.downcast::<String>() {
                println!("{}", string);
            }
        }
    }
}

#[test]
fn test_generate_revocation_pubkey() {
    let countersignatory_basepoint = secp256k1_pubkey_from_private_key(&[0x01; 32]);
    let per_commitment_point = secp256k1_pubkey_from_private_key(&[0x02; 32]);

    let revocation_pubkey =
        generate_revocation_pubkey(countersignatory_basepoint, per_commitment_point);

    let actual = revocation_pubkey.to_string();

    let expected = "02f406b2b2ef2372c08d003e5062e4b34929b86f107a36bfbd406f5644419e9ff6";

    assert_eq!(
        actual, expected,
        "Revocation pubkey doesn't match expected value"
    );
}

#[test]
fn test_build_timelocked_transaction() {
    let outpoint = OutPoint::new(
        "d9334caed6503ebc710d13a5f663f03bec531026d2bc786befdfdb8ef5aad721"
            .parse::<Txid>()
            .unwrap(),
        1,
    );

    let txins = vec![TxIn {
                previous_output: outpoint,
                script_sig: ScriptBuf::new(),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            }];

    let pubkey = secp256k1_pubkey_from_private_key(&[0x01; 32]);

    let block_height: u32 = 1000000;

    let csv_delay: i64 = 144;

    let amount: u64 = 100000;

    let transaction =
        build_timelocked_transaction(txins,
                                    &pubkey,
                                    block_height,
                                    csv_delay,
                                    amount);

    let actual = transaction.compute_txid().to_string();

    let expected = "435a6307b715f5e8196bee440597186eba1e797dee71e8aae5ba1d3bc1d41ed8";

    assert_eq!(
        actual, expected,
        "Transaction ID doesn't match expected value"
    );
}

#[test]
fn test_generate_p2wsh_signature() {
    let outpoint = OutPoint::new(
        "d9334caed6503ebc710d13a5f663f03bec531026d2bc786befdfdb8ef5aad721"
            .parse::<Txid>()
            .unwrap(),
        1,
    );

    let txins = vec![TxIn {
                previous_output: outpoint,
                script_sig: ScriptBuf::new(),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            }];

    let private_key = secp256k1_private_key(&[0x01; 32]);
    let pubkey = secp256k1_pubkey_from_private_key(&[0x01; 32]);

    let block_height: u32 = 1000000;

    let csv_delay: i64 = 144;

    let amount: u64 = 100000;

    let transaction =
        build_timelocked_transaction(txins,
                                    &pubkey,
                                    block_height,
                                    csv_delay,
                                    amount);

    let signature = generate_p2wsh_signature(transaction,
                                      0,
                                      &ScriptBuf::new(),
                                      amount,
                                      EcdsaSighashType::All,
                                      private_key);

    let signature_der = signature.serialize_der().to_vec();

    let hex_sig = serialize_hex(&signature_der);

    assert_eq!(
        hex_sig, "473045022100cef19aa2e815da25f6809cb7a3dfa487044c3f48a09618c4e76077f8e8c50409022060bacb75b055cc3c186a9086a3b78f850f35534c7739fc6e9cabae954a0f040a",
        "Signature doesn't match expected value"
    );
}

#[test]
fn test_build_multisig_transaction() {
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

    let pubkey1 = secp256k1_pubkey_from_private_key(&[0x01; 32]);
    let pubkey2 = secp256k1_pubkey_from_private_key(&[0x02; 32]);

    let amount: u64 = 100000;

    let transaction =
        build_multisig_transaction(txin,
                                    &pubkey1,
                                    &pubkey2,
                                    amount);

    let their_solution = transaction.compute_txid().to_string();

    println!("their solution: {}", their_solution);

    let acceptable_solutions = [
        "9deb308457457267f84ffba50289f2907b405c0c4f2e90442ed9a0c569c190cc".to_string(),
        "1f884a66e1a73469c8bb4c90e6099c9697728a6aac51ce02d9aadf18c2535d53".to_string(),
    ];

    assert!(acceptable_solutions.contains(&their_solution));
}

pub fn secp256k1_pubkey_from_private_key(private_key: &[u8; 32]) -> Secp256k1PublicKey {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(private_key).unwrap();
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
    public_key
}

pub fn pubkey_from_private_key(private_key: &[u8; 32]) -> PublicKey {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(private_key).unwrap();
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
    PublicKey::new(public_key)
}
