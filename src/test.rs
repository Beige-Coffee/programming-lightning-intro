use crate::{cltv_p2pkh, csv_p2pkh, p2pkh, payment_channel_funding_output, two_of_two_multisig};
use bitcoin::secp256k1::{self, Secp256k1};
use bitcoin::PublicKey;

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
            assert_eq!(
                format!("{}", script.script_hash()),
                "97a7bdcde0c83d51060bf048b4bd3ff97de81ccd".to_string()
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
            assert_eq!(
                format!("{}", script.script_hash()),
                "d8fecda80c30e89a9e7f0964ee79ce055288bc1c".to_string()
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
