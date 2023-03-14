use crate::internal::bitcoind_client::BitcoindClient;
use crate::internal::channel_manager::ChannelManager;
use crate::{
    cltv_p2pkh, 
    csv_p2pkh, 
    p2pkh, 
    payment_channel_funding_output, 
    two_of_two_multisig, 
    block_connected,
    spend_multisig,
    spend_refund,
    channel_closed,
    handle_funding_generation_ready
};
use bitcoin::hashes::hex::FromHex;
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::secp256k1::{self, Secp256k1};
use bitcoin::{PublicKey, Script, Transaction, PackedLockTime, OutPoint, TxIn, Witness, Sequence};

#[test]
fn test_handle_funding_generation_ready() {
    let bitcoind_client = BitcoindClient::new();
    let channel_manager = ChannelManager::new();
    let temporary_channel_id: [u8; 32] = [3; 32];
    let counterparty_node_id = pubkey_from_private_key(&[0x01; 32]);
    let channel_value_satoshis = 1000000;
    let output_script = Script::new();
    let user_channel_id = 0;
    
    let result = std::panic::catch_unwind(|| {

        handle_funding_generation_ready(
            &channel_manager, 
            &bitcoind_client, 
            &temporary_channel_id, 
            &counterparty_node_id, 
            channel_value_satoshis, 
            output_script, 
            user_channel_id
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
        },
        Err(e) => {
            if let Ok(string) = e.downcast::<String>() {
                println!("{}", string);
            }
        },
    }
}

#[test]
fn test_channel_closed() {
    let correct_outpoint = OutPoint::new(bitcoin::hash_types::Txid::from_hex("d9334caed6503ebc710d13a5f663f03bec531026d2bc786befdfdb8ef5aad721").unwrap(), 0);
    let wrong_txid_outpoint = OutPoint::new(bitcoin::hash_types::Txid::from_hex("d9334caed6503ebc710d13a5f663f03bec531026d2bc786befdfdb8ef5aad722").unwrap(), 0);
    let wrong_vout_outpoint = OutPoint::new(bitcoin::hash_types::Txid::from_hex("d9334caed6503ebc710d13a5f663f03bec531026d2bc786befdfdb8ef5aad721").unwrap(), 1);

    let correct_block = bitcoin::Block {
        header: bitcoin::blockdata::constants::genesis_block(bitcoin::Network::Bitcoin).header,
        txdata: vec![Transaction {
            version: 1,
            lock_time: PackedLockTime(0),
            input: vec![TxIn {
                previous_output: correct_outpoint,
                script_sig: Script::new(),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            }],
            output: vec![],
        }],
    };

    let wrong_vout_block = bitcoin::Block {
        header: bitcoin::blockdata::constants::genesis_block(bitcoin::Network::Bitcoin).header,
        txdata: vec![Transaction {
            version: 1,
            lock_time: PackedLockTime(0),
            input: vec![TxIn {
                previous_output: wrong_vout_outpoint,
                script_sig: Script::new(),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            }],
            output: vec![],
        }],
    };

    let wrong_txid_block = bitcoin::Block {
        header: bitcoin::blockdata::constants::genesis_block(bitcoin::Network::Bitcoin).header,
        txdata: vec![Transaction {
            version: 1,
            lock_time: PackedLockTime(0),
            input: vec![TxIn {
                previous_output: wrong_txid_outpoint,
                script_sig: Script::new(),
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

    let result = std::panic::catch_unwind(|| {
        spend_refund(&alice_pubkey, alice_signature)
    });

    match result {
        Ok(script) => {
            let their_solution = format!("{}", script.script_hash());
            let acceptable_solutions = [
                "39d56399730f3cb47113f23e6b7a1f7d395af5ea".to_string(),
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
fn test_spend_multisig() {
    let alice_signature = Signature::from_compact(&[0x01; 64]).unwrap();
    let bob_signature = Signature::from_compact(&[0x02; 64]).unwrap();


    let result = std::panic::catch_unwind(|| {
        spend_multisig(alice_signature, bob_signature)
    });

    match result {
        Ok(script) => {
            let their_solution = format!("{}", script.script_hash());
            let acceptable_solutions = [
                "4b1f23900980da686c4d73cc333e0f5509323701".to_string(),
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
fn test_block_connected() {
    let script = Script::from(vec![0x01, 0x02, 0x03]);
    let amount = 1000000;

    let correct_block = bitcoin::Block {
        header: bitcoin::blockdata::constants::genesis_block(bitcoin::Network::Bitcoin).header,
        txdata: vec![Transaction {
            version: 1,
            lock_time: PackedLockTime(0),
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
            version: 1,
            lock_time: PackedLockTime(0),
            input: vec![],
            output: vec![bitcoin::TxOut {
                value: amount - 1,
                script_pubkey: script.clone(),
            }],
        }],
    };

    let wrong_script_block = bitcoin::Block {
        header: bitcoin::blockdata::constants::genesis_block(bitcoin::Network::Bitcoin).header,
        txdata: vec![Transaction {
            version: 1,
            lock_time: PackedLockTime(0),
            input: vec![],
            output: vec![bitcoin::TxOut {
                value: amount,
                script_pubkey:Script::from(vec![0x01, 0x02, 0x04]),
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
                "6022dbe26d95d361a0369d355ad12a1b193befa8".to_string()
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
                "d8fecda80c30e89a9e7f0964ee79ce055288bc1c".to_string()
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

fn pubkey_from_private_key(private_key: &[u8; 32]) -> PublicKey {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(private_key).unwrap();
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
    PublicKey::new(public_key)
}
