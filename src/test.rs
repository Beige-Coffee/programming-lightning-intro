use crate::internal::bitcoind_client::BitcoindClient;
use crate::internal::channel_manager::ChannelManager;
use crate::{
    block_connected, build_htlc_offerer_witness_script, build_htlc_receiver_witness_script,
    channel_closed, cltv_p2pkh, csv_p2pkh, generate_revocation_pubkey,
    handle_funding_generation_ready, p2pkh, p2sh, payment_channel_funding_output, spend_multisig,
    spend_refund, two_of_three_multisig_redeem_script, two_of_two_multisig, build_timelocked_transaction
};

use crate::internal::helper::{pubkey_multiplication_tweak, sha256_hash};
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

#[test]
fn test_handle_funding_generation_ready() {
    let bitcoind_client = BitcoindClient::new();
    let channel_manager = ChannelManager::new();
    let temporary_channel_id: [u8; 32] = [3; 32];
    let counterparty_node_id = pubkey_from_private_key(&[0x01; 32]);
    let channel_value_satoshis = Amount::from_sat(1000000);
    let output_script = ScriptBuf::new();
    let user_channel_id = 0;

    let result = std::panic::catch_unwind(|| {
        handle_funding_generation_ready(
            &channel_manager,
            &bitcoind_client,
            &temporary_channel_id,
            &counterparty_node_id,
            channel_value_satoshis,
            output_script,
            user_channel_id,
        )
    });

    match result {
        Ok(_) => {
            let last_funding_tx_gen = channel_manager.last_funding_tx_gen.lock().unwrap();
            assert!(last_funding_tx_gen.is_some());

            let (temp_cid, node_id, tx_hex) = last_funding_tx_gen.clone().unwrap();
            assert_eq!(temp_cid, temporary_channel_id);
            assert_eq!(node_id, counterparty_node_id);
            assert_eq!(tx_hex, "signedtxhex".to_string());
        }
        Err(e) => {
            if let Ok(string) = e.downcast::<String>() {
                println!("{}", string);
            }
        }
    }
}

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
    let alice_pubkey = pubkey_from_private_key(&[0x01; 32]);
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
    let alice_pubkey = pubkey_from_private_key(&[0x01; 32]);
    let bob_pubkey = pubkey_from_private_key(&[0x02; 32]);
    let height = 1000000;

    let result = std::panic::catch_unwind(|| {
        payment_channel_funding_output(&alice_pubkey, &bob_pubkey, height)
    });

    match result {
        Ok(script) => {
            let their_solution = format!("{}", script.script_hash());
            let acceptable_solutions = [
                "4a6a4f1fd56fd8684acce69eb49f7ae2b469b077".to_string(),
                "0479d03e595c9c0242a4e46a2b67564617eaa993".to_string(),
                "33ed58fc90a6d3687c71e3ce362014347ea2c964".to_string(),
                "6022dbe26d95d361a0369d355ad12a1b193befa8".to_string(),
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
    let alice_pubkey = pubkey_from_private_key(&[0x01; 32]);
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
    let alice_pubkey = pubkey_from_private_key(&[0x01; 32]);
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
fn test_two_of_two_multisig() {
    let alice_pubkey = pubkey_from_private_key(&[0x01; 32]);
    let bob_pubkey = pubkey_from_private_key(&[0x02; 32]);
    let result = std::panic::catch_unwind(|| two_of_two_multisig(&alice_pubkey, &bob_pubkey));

    match result {
        Ok(script) => {
            let their_solution = format!("{}", script.script_hash());
            let acceptable_solutions = [
                "e5c7b40d1f14542cc2c7a15819de088a33c8f7ba".to_string(),
                "d8fecda80c30e89a9e7f0964ee79ce055288bc1c".to_string(),
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
fn test_two_of_three_multisig_redeem_script() {
    let pubkey1 = pubkey_from_private_key(&[0x01; 32]);
    let pubkey2 = pubkey_from_private_key(&[0x02; 32]);
    let pubkey3 = pubkey_from_private_key(&[0x03; 32]);
    let result = std::panic::catch_unwind(|| {
        two_of_three_multisig_redeem_script(&pubkey1, &pubkey2, &pubkey3)
    });

    match result {
        Ok(script) => {
            let their_solution = format!("{}", script.script_hash());
            let acceptable_solutions = [
                "ae79902ae33900b679c76ced8576362e4abb15e8".to_string(), //123
                "1dd8aaf8dacff4b11fe1d0ee75ca4a5a6c5922d5".to_string(), //132
                "9d1bb190ab3cab8cb38645be2c7d34aee2200792".to_string(), //231
                "25e33caeb2c30b28710ee5e63ca04f88ad47a6d7".to_string(), //213
                "521dc8bd33010dce2dc2498bf2d443c2f4568864".to_string(), //321
                "303bc7ac756ba94a75bf2e211a5575f203f27d0c".to_string(), //312
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
    let alice_pubkey = pubkey_from_private_key(&[0x01; 32]);

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
    let remote_htlc_pubkey = pubkey_from_private_key(&[0x01; 32]);
    let local_htlc_pubkey = pubkey_from_private_key(&[0x02; 32]);
    let revocation_key = secp256k1_pubkey_from_private_key(&[0x02; 32]);
    let payment_hash160 = HASH160_DUMMY;
    let revocation_hash160 = PubkeyHash::hash(&revocation_key.serialize());
    let cltv_expiry: i64 = 1000000000;

    let result = std::panic::catch_unwind(|| {
        build_htlc_receiver_witness_script(
            &revocation_hash160,
            &remote_htlc_pubkey,
            &local_htlc_pubkey,
            &payment_hash160,
            cltv_expiry,
        )
    });

    match result {
        Ok(script) => {
            let their_solution = format!("{}", script.script_hash());
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
    let remote_htlc_pubkey = pubkey_from_private_key(&[0x01; 32]);
    let local_htlc_pubkey = pubkey_from_private_key(&[0x02; 32]);
    let revocation_key = secp256k1_pubkey_from_private_key(&[0x02; 32]);
    let payment_hash160 = HASH160_DUMMY;
    let revocation_hash160 = PubkeyHash::hash(&revocation_key.serialize());

    let result = std::panic::catch_unwind(|| {
        build_htlc_offerer_witness_script(
            &revocation_hash160,
            &remote_htlc_pubkey,
            &local_htlc_pubkey,
            &payment_hash160,
        )
    });

    match result {
        Ok(script) => {
            let their_solution = format!("{}", script.script_hash());
            let acceptable_solutions = [
                "0c758acc3320bc585d3cf5ef0416f28430d2398e".to_string(), //123
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

    let pubkey = pubkey_from_private_key(&[0x01; 32]);

    let block_height: u32 = 1000000;

    let csv_delay: i64 = 144;

    let amount = Amount::from_sat(100000);

    let transaction =
        build_timelocked_transaction(txins,
                                    &pubkey,
                                    block_height,
                                    csv_delay,
                                    amount);

    let actual = transaction.compute_txid().to_string();

    let expected = "bcd82f4d3ae92ae10ae51ba24693028560fed42eb3d8456d2cc88867351b71df";

    assert_eq!(
        actual, expected,
        "Transaction ID doesn't match expected value"
    );
}

pub fn secp256k1_pubkey_from_private_key(private_key: &[u8; 32]) -> Secp256k1PublicKey {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(private_key).unwrap();
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
    public_key
}

fn pubkey_from_private_key(private_key: &[u8; 32]) -> PublicKey {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(private_key).unwrap();
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
    PublicKey::new(public_key)
}
