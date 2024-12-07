#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::internal;

use bitcoin::amount::Amount;
use bitcoin::blockdata::opcodes::all as opcodes;
use bitcoin::hash_types::Txid;
use bitcoin::hashes::ripemd160::Hash as Ripemd160;
use sha2::{Sha256, Digest};
use bitcoin::hashes::Hash;
use bitcoin::hashes::Hash as TraitImport;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::script::{ScriptBuf, ScriptHash};
use bitcoin::secp256k1;
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::secp256k1::Message;
use bitcoin::secp256k1::PublicKey as Secp256k1PublicKey;
use bitcoin::secp256k1::Scalar;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::sighash::EcdsaSighashType;
use bitcoin::sighash::SighashCache;
use bitcoin::transaction::Version;
use bitcoin::{Block, OutPoint, PublicKey, Sequence, Transaction, TxIn, TxOut, Witness};
use bitcoin::{PubkeyHash, WPubkeyHash};
use internal::bitcoind_client::BitcoindClient;
use internal::builder::Builder;
use internal::channel_manager::ChannelManager;
use internal::helper::{pubkey_multiplication_tweak, sha256_hash};
use rand::{thread_rng, Rng};

pub fn p2pkh(pubkey: &Secp256k1PublicKey) -> ScriptBuf {
    Builder::new()
        .push_opcode(opcodes::OP_DUP)
        .push_opcode(opcodes::OP_HASH160)
        .push_slice(&PubkeyHash::hash(&pubkey.serialize()))
        .push_opcode(opcodes::OP_EQUALVERIFY)
        .push_opcode(opcodes::OP_CHECKSIG)
        .into_script()
}

pub fn two_of_two_multisig_redeem_script(
    pubkey1: &Secp256k1PublicKey,
    pubkey2: &Secp256k1PublicKey,
) -> ScriptBuf {
    Builder::new()
        .push_int(2)
        .push_slice(pubkey1.serialize())
        .push_slice(pubkey2.serialize())
        .push_int(2)
        .push_opcode(opcodes::OP_CHECKMULTISIG)
        .into_script()
}

pub fn two_of_two_multisig(alice_pubkey: &PublicKey, bob_pubkey: &PublicKey) -> ScriptBuf {
    Builder::new()
        .push_int(2)
        .push_key(alice_pubkey)
        .push_key(bob_pubkey)
        .push_int(2)
        .push_opcode(opcodes::OP_CHECKMULTISIG)
        .into_script()
}

pub fn p2sh(script_hash: &ScriptHash) -> ScriptBuf {
    Builder::new()
        .push_opcode(opcodes::OP_HASH160)
        .push_script_hash(script_hash)
        .push_opcode(opcodes::OP_EQUAL)
        .into_script()
}

pub fn cltv_p2pkh(pubkey: &Secp256k1PublicKey, height_or_timestamp: i64) -> ScriptBuf {
    Builder::new()
        .push_int(height_or_timestamp)
        .push_opcode(opcodes::OP_CLTV)
        .push_opcode(opcodes::OP_DROP)
        .push_opcode(opcodes::OP_DUP)
        .push_opcode(opcodes::OP_HASH160)
        .push_slice(&PubkeyHash::hash(&pubkey.serialize()))
        .push_opcode(opcodes::OP_EQUALVERIFY)
        .push_opcode(opcodes::OP_CHECKSIG)
        .into_script()
}

pub fn csv_p2pkh(pubkey: &Secp256k1PublicKey, height_or_timestamp: i64) -> ScriptBuf {
    Builder::new()
        .push_int(height_or_timestamp)
        .push_opcode(opcodes::OP_CSV)
        .push_opcode(opcodes::OP_DROP)
        .push_opcode(opcodes::OP_DUP)
        .push_opcode(opcodes::OP_HASH160)
        .push_slice(&PubkeyHash::hash(&pubkey.serialize()))
        .push_opcode(opcodes::OP_EQUALVERIFY)
        .push_opcode(opcodes::OP_CHECKSIG)
        .into_script()
}

pub fn timelocked_p2pkh(pubkey: &Secp256k1PublicKey, height_or_timestamp: i64) -> ScriptBuf {
    Builder::new()
        .push_int(height_or_timestamp)
        .push_opcode(opcodes::OP_CSV)
        .push_opcode(opcodes::OP_DROP)
        .push_opcode(opcodes::OP_DUP)
        .push_opcode(opcodes::OP_HASH160)
        .push_slice(&PubkeyHash::hash(&pubkey.serialize()))
        .push_opcode(opcodes::OP_EQUALVERIFY)
        .push_opcode(opcodes::OP_CHECKSIG)
        .into_script()
}

pub fn payment_channel_funding_output(
    alice_pubkey: &Secp256k1PublicKey,
    bob_pubkey: &Secp256k1PublicKey,
    blocks_or_timestamp: i64,
) -> ScriptBuf {
    Builder::new()
        .push_opcode(opcodes::OP_IF)
        .push_script(two_of_two_multisig_redeem_script(alice_pubkey, bob_pubkey))
        .push_opcode(opcodes::OP_ELSE)
        .push_script(csv_p2pkh(alice_pubkey, blocks_or_timestamp))
        .push_opcode(opcodes::OP_ENDIF)
        .into_script()
}

pub fn build_funding_transaction(
    txins: Vec<TxIn>,
    alice_pubkey: &Secp256k1PublicKey,
    bob_pubkey: &Secp256k1PublicKey,
    amount: u64,
) -> Transaction {
    
    let output_script = two_of_two_multisig_redeem_script(alice_pubkey, bob_pubkey);

    let txout = build_output(amount, output_script.to_p2wsh());

    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(
        version,
        locktime,
        txins,
        vec![txout],
    );

    tx
}

pub fn build_htlc_commitment_transaction(
    funding_txin: TxIn,
    revocation_pubkey: &Secp256k1PublicKey,
    remote_htlc_pubkey: &Secp256k1PublicKey,
    local_htlc_pubkey: &Secp256k1PublicKey,
    to_local_delayed_pubkey: &Secp256k1PublicKey,
    remote_pubkey: Secp256k1PublicKey,
    to_self_delay: i64,
    payment_hash160: &[u8; 20],
    htlc_amount: u64,
    local_amount: u64,
    remote_amount: u64,
) -> Transaction {
    let htlc_offerer_script = build_htlc_offerer_witness_script(
        revocation_pubkey,
        remote_htlc_pubkey,
        local_htlc_pubkey,
        payment_hash160,
    );

    let to_local_script =
        to_local(revocation_pubkey, to_local_delayed_pubkey, to_self_delay);

    let to_remote_script = p2wpkh_output_script(remote_pubkey);

    let htlc_output = build_output(htlc_amount, htlc_offerer_script.to_p2wsh());

    let local_output = build_output(local_amount, to_local_script.to_p2wsh());

    let remote_output = build_output(remote_amount, to_remote_script);

    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(version,
                               locktime,
                               vec![funding_txin],
                            vec![local_output, remote_output, htlc_output]);

    tx
}

pub fn build_htlc_timeout_transaction(
    funding_txin: TxIn,
    revocation_pubkey: &Secp256k1PublicKey,
    broadcaster_delayed_payment_key: &Secp256k1PublicKey,
    contest_delay: i64,
    cltv_expiry: u32,
    htlc_amount: u64,
) -> Transaction {
    let htlc_timeout_script = to_local(
        revocation_pubkey,
        broadcaster_delayed_payment_key,
        contest_delay,
    );

    let htlc_output = build_output(htlc_amount, htlc_timeout_script.to_p2wsh());

    let version = Version::TWO;
    let locktime = LockTime::from_consensus(cltv_expiry);

    let tx = build_transaction(
                version,
                locktime,
                vec![funding_txin],
                vec![htlc_output]);

    tx
}

pub fn to_local(
    revocation_key: &Secp256k1PublicKey,
    to_local_delayed_pubkey: &Secp256k1PublicKey,
    to_self_delay: i64,
) -> ScriptBuf {
    Builder::new()
        .push_opcode(opcodes::OP_IF)
        .push_slice(revocation_key.serialize())
        .push_opcode(opcodes::OP_ELSE)
        .push_int(to_self_delay)
        .push_opcode(opcodes::OP_CSV)
        .push_opcode(opcodes::OP_DROP)
        .push_slice(to_local_delayed_pubkey.serialize())
        .push_opcode(opcodes::OP_ENDIF)
        .push_opcode(opcodes::OP_CHECKSIG)
        .into_script()
}

pub fn build_to_remote_script(remotepubkey: &PublicKey) -> ScriptBuf {
    ScriptBuf::new_p2wpkh(&remotepubkey.wpubkey_hash().unwrap())
}

pub fn block_connected(
    funding_output: ScriptBuf,
    channel_amount_sats: Amount,
    block: Block,
) -> bool {
    todo!()
}

pub fn spend_multisig(alice_signature: Signature, bob_signature: Signature) -> ScriptBuf {
    todo!()
}

pub fn spend_refund(alice_pubkey: &Secp256k1PublicKey, alice_signature: Signature) -> ScriptBuf {
    todo!()
}

pub fn generate_revocation_pubkey(
    countersignatory_basepoint: Secp256k1PublicKey,
    per_commitment_point: Secp256k1PublicKey,
) -> Secp256k1PublicKey {
    let rev_append_commit_hash_key =
        sha256_hash(&countersignatory_basepoint, &per_commitment_point);

    let commit_append_rev_hash_key =
        sha256_hash(&per_commitment_point, &countersignatory_basepoint);

    let countersignatory_contrib =
        pubkey_multiplication_tweak(countersignatory_basepoint, rev_append_commit_hash_key);

    let broadcaster_contrib =
        pubkey_multiplication_tweak(per_commitment_point, commit_append_rev_hash_key);

    let pk = countersignatory_contrib.combine(&broadcaster_contrib)
        .expect("Addition only fails if the tweak is the inverse of the key. This is not possible when the tweak commits to the key.");

    pk
}

pub fn build_htlc_offerer_witness_script(
    revocation_pubkey: &Secp256k1PublicKey,
    remote_htlc_pubkey: &Secp256k1PublicKey,
    local_htlc_pubkey: &Secp256k1PublicKey,
    payment_hash160: &[u8; 20],
) -> ScriptBuf {
    Builder::new()
        .push_opcode(opcodes::OP_DUP)
        .push_opcode(opcodes::OP_HASH160)
        .push_slice(&PubkeyHash::hash(&revocation_pubkey.serialize()))
        .push_opcode(opcodes::OP_EQUAL)
        .push_opcode(opcodes::OP_IF)
        .push_opcode(opcodes::OP_CHECKSIG)
        .push_opcode(opcodes::OP_ELSE)
        .push_slice(&remote_htlc_pubkey.serialize())
        .push_opcode(opcodes::OP_SWAP)
        .push_opcode(opcodes::OP_SIZE)
        .push_int(32)
        .push_opcode(opcodes::OP_EQUAL)
        .push_opcode(opcodes::OP_NOTIF)
        .push_opcode(opcodes::OP_DROP)
        .push_int(2)
        .push_opcode(opcodes::OP_SWAP)
        .push_slice(&local_htlc_pubkey.serialize())
        .push_int(2)
        .push_opcode(opcodes::OP_CHECKMULTISIG)
        .push_opcode(opcodes::OP_ELSE)
        .push_opcode(opcodes::OP_HASH160)
        .push_slice(payment_hash160)
        .push_opcode(opcodes::OP_EQUALVERIFY)
        .push_opcode(opcodes::OP_CHECKSIG)
        .push_opcode(opcodes::OP_ENDIF)
        .push_opcode(opcodes::OP_ENDIF)
        .into_script()
}

pub fn build_htlc_timeout_witness_script(
    revocation_key: &Secp256k1PublicKey,
    contest_delay: i64,
    broadcaster_delayed_payment_key: &Secp256k1PublicKey,
) -> ScriptBuf {
    Builder::new()
        .push_opcode(opcodes::OP_IF)
        .push_slice(&revocation_key.serialize())
        .push_opcode(opcodes::OP_ELSE)
        .push_int(contest_delay)
        .push_opcode(opcodes::OP_CSV)
        .push_opcode(opcodes::OP_DROP)
        .push_slice(&broadcaster_delayed_payment_key.serialize())
        .push_opcode(opcodes::OP_ENDIF)
        .push_opcode(opcodes::OP_CHECKSIG)
        .into_script()
}

pub fn build_htlc_receiver_witness_script(
    revocation_pubkey: &Secp256k1PublicKey,
    remote_htlc_pubkey: &Secp256k1PublicKey,
    local_htlc_pubkey: &Secp256k1PublicKey,
    payment_hash160: &[u8; 20],
    cltv_expiry: i64,
) -> ScriptBuf {
    todo!()
}

pub fn channel_closed(funding_outpoint: OutPoint, block: Block) -> bool {
    todo!()
}

//pub fn handle_funding_generation_ready(
//channel_manager: &ChannelManager,
//bitcoind_client: &BitcoindClient,
//temporary_channel_id: &[u8; 32],
//counterparty_node_id: &PublicKey,
//channel_value_satoshis: Amount,
//output_script: ScriptBuf,
//user_channel_id: u128,
//) {
//let raw_tx = bitcoind_client.create_raw_transaction(vec![TxOut {
//value: channel_value_satoshis,
//script_pubkey: output_script,
//}]);
//
//let funded_tx = bitcoind_client.fund_raw_transaction(raw_tx);
//
//let signed_tx = bitcoind_client.sign_raw_transaction_with_wallet(funded_tx);
//
//channel_manager.funding_transaction_generated(
//temporary_channel_id,
//counterparty_node_id,
//signed_tx,
//);
//}

pub fn build_output(amount: u64, output_script: ScriptBuf) -> TxOut {
    
    TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: output_script,
    }
}

pub fn build_timelocked_transaction(
    txins: Vec<TxIn>,
    pubkey: &Secp256k1PublicKey,
    block_height: u32,
    csv_delay: i64,
    amount: u64,
) -> Transaction {
    let output_script = csv_p2pkh(pubkey, csv_delay);
    let txout = build_output(amount, output_script.to_p2wsh());

    Transaction {
        version: Version::ONE,
        lock_time: LockTime::from_consensus(block_height),
        input: txins,
        output: vec![txout],
    }
}

pub fn generate_p2wsh_signature(
    transaction: Transaction,
    input_idx: usize,
    witness_script: &ScriptBuf,
    value: u64,
    sighash_type: EcdsaSighashType,
    private_key: secp256k1::SecretKey,
) -> Signature {
    let secp = Secp256k1::new();

    let message =
        generate_p2wsh_message(transaction, input_idx, witness_script, value, sighash_type);
    let signature = secp.sign_ecdsa(&message, &private_key);

    signature
}

fn generate_p2wsh_message(
    transaction: Transaction,
    input_idx: usize,
    witness_script: &ScriptBuf,
    value: u64,
    sighash_type: EcdsaSighashType,
) -> Message {
    let secp = Secp256k1::new();

    let mut cache = SighashCache::new(&transaction);

    let amount = Amount::from_sat(value);

    let sighash = cache
        .p2wsh_signature_hash(input_idx, &witness_script, amount, sighash_type)
        .unwrap();

    let message = Message::from_digest_slice(&sighash[..]).unwrap();

    message
}

pub fn build_p2wpkh_transaction(txin: TxIn, pubkey: &PublicKey, amount: u64) -> Transaction {
    let script = build_to_remote_script(&pubkey);

    let output = build_output(amount, script);

    Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![txin],
        output: vec![output],
    }
}

pub fn build_multisig_transaction(
    tx_ins: Vec<TxIn>,
    pubkey1: &Secp256k1PublicKey,
    pubkey2: &Secp256k1PublicKey,
    amount: u64,
) -> Transaction {
    let output_script = two_of_two_multisig_redeem_script(pubkey1, pubkey2);

    let txout = build_output(amount, output_script.to_p2wsh());

    let locktime = LockTime::ZERO;

    let version = Version::ONE;

    build_transaction(version, locktime, tx_ins, vec![txout])
}

pub fn build_csv_input(txid: String, vout: u32, block_delay: u32) -> TxIn {
    let sequence = Sequence(block_delay);

    let tx_in = build_unsigned_input(txid, vout, sequence);

    tx_in
}

pub fn build_unsigned_input(txid: String, vout: u32, sequence: Sequence) -> TxIn {
    let tx_id = txid_from_string(txid);

    TxIn {
        previous_output: OutPoint {
            txid: tx_id,
            vout: vout,
        },
        sequence: sequence,
        script_sig: ScriptBuf::new(),
        witness: Witness::new(),
    }
}

fn txid_from_string(txid: String) -> Txid {
    // Get an unspent output to spend
    let mut tx_id_bytes = hex::decode(txid).expect("Valid hex string");
    tx_id_bytes.reverse();
    let input_txid = Txid::from_byte_array(tx_id_bytes.try_into().expect("Expected 32 bytes"));

    input_txid
}

pub fn build_transaction(
    version: Version,
    locktime: LockTime,
    tx_ins: Vec<TxIn>,
    tx_outs: Vec<TxOut>,
) -> Transaction {
    Transaction {
        version: version,
        lock_time: locktime,
        input: tx_ins,
        output: tx_outs,
    }
}

pub fn generate_multisig_signature(
    transaction: Transaction,
    input_idx: usize,
    witness_script: &ScriptBuf,
    value: u64,
    private_key: secp256k1::SecretKey,
) -> Signature {
    
    let secp = Secp256k1::new();

    // define sighash type
    let sighash_type = EcdsaSighashType::All;

    // generate message
    let message =
        generate_p2wsh_message(transaction, input_idx, witness_script, value, sighash_type);

    // generate signature
    let signature = secp.sign_ecdsa(&message, &private_key);

    signature
}

pub fn generate_p2wsh_multisig_witness(
    signature1: Vec<u8>,
    signature2: Vec<u8>,
    redeem_script: &ScriptBuf,
    mut witness: Witness,
    ) -> Witness {

    // First push empty element for NULLDUMMY compliance
    witness.push(Vec::new());

    // Determine signature order based on pubkey comparison
    let sig1_first = signature1 < signature2;

    // Push signatures in correct order
    if sig1_first {
        witness.push(signature1);
        witness.push(signature2);
    } else {
        witness.push(signature2);
        witness.push(signature1);
    }

    witness.push(redeem_script.clone().into_bytes());

    witness
}

pub fn build_refund_transaction(
    funding_txin: TxIn,
    alice_pubkey: Secp256k1PublicKey,
    bob_pubkey: Secp256k1PublicKey,
    alice_balance: u64,
    bob_balance: u64
) -> Transaction {

    let alice_script = p2wpkh_output_script(alice_pubkey);

    let bob_script = p2wpkh_output_script(bob_pubkey);

    let alice_output = build_output(alice_balance, alice_script);

    let bob_output = build_output(bob_balance, bob_script);

    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(version,
                      locktime,
                      vec![funding_txin],
                      vec![alice_output, bob_output]);
    tx
}

pub fn build_commitment_transaction(
    funding_txin: TxIn,
    revocation_pubkey: &Secp256k1PublicKey,
    to_local_delayed_pubkey: &Secp256k1PublicKey,
    remote_pubkey: Secp256k1PublicKey,
    to_self_delay: i64,
    local_amount: u64,
    remote_amount: u64,
) -> Transaction {

    let to_local_script =
        to_local(revocation_pubkey, to_local_delayed_pubkey, to_self_delay);

    let to_remote_script = p2wpkh_output_script(remote_pubkey);

    let local_output = build_output(local_amount, to_local_script.to_p2wsh());

    let remote_output = build_output(remote_amount, to_remote_script);

    let version = Version::TWO;
    let locktime = LockTime::ZERO;

    let tx = build_transaction(version,
                      locktime,
                      vec![funding_txin],
                      vec![local_output, remote_output]);
    tx
    }

pub fn p2wpkh_output_script(public_key: Secp256k1PublicKey) -> ScriptBuf {
    let pubkey = PublicKey::new(public_key);
    ScriptBuf::new_p2wpkh(&pubkey.wpubkey_hash().unwrap())
}