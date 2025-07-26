#!/bin/bash

# Ensure the Rust toolchain is set to stable
rustup default stable

# Persistent data directory for Bitcoin Core
BITCOIN_DATA_DIR=/home/runner/workspace/.bitcoin/
# Ensure data directory exists
mkdir -p $BITCOIN_DATA_DIR

# Remove stale lock files (if any)
rm -f $BITCOIN_DATA_DIR/regtest/.lock

# Start bitcoind if not already running
already_running=$(bitcoin-cli -datadir=$BITCOIN_DATA_DIR -regtest -rpcuser=bitcoind -rpcpassword=bitcoind getblockchaininfo 2>/dev/null)
if [[ "$already_running" =~ "blocks" ]]; then
  echo "bitcoind already running."
else
  echo "Starting bitcoind..."
  bitcoind -regtest -conf=$(pwd)/bitcoin.conf -datadir=$BITCOIN_DATA_DIR -reindex &
  sleep 5
fi

# Wait for bitcoind to initialize
echo "Waiting for bitcoind to finish initializing..."
while true; do
  status=$(bitcoin-cli -datadir=$BITCOIN_DATA_DIR -regtest -rpcuser=bitcoind -rpcpassword=bitcoind getblockchaininfo 2>&1)
  if [[ "$status" =~ "blocks" ]]; then
    echo "bitcoind is ready."
    break
  elif [[ "$status" =~ "Loading" ]]; then
    echo "$status"
  else
    echo "Waiting for bitcoind to initialize... (status: $status)"
  fi
  sleep 2
done

# Check if wallet "pl" exists and load/create accordingly
wallet_exists=$(bitcoin-cli -datadir=$BITCOIN_DATA_DIR -regtest -rpcuser=bitcoind -rpcpassword=bitcoind listwalletdir | grep -o "pl")
wallet_loaded=$(bitcoin-cli -datadir=$BITCOIN_DATA_DIR -regtest -rpcuser=bitcoind -rpcpassword=bitcoind listwallets | grep -o "pl")

if [[ -z "$wallet_exists" ]]; then
  echo "Creating wallet 'pl'..."
  bitcoin-cli -datadir=$BITCOIN_DATA_DIR -regtest -rpcuser=bitcoind -rpcpassword=bitcoind createwallet "pl"
elif [[ -z "$wallet_loaded" ]]; then
  echo "Loading wallet 'pl'..."
  bitcoin-cli -datadir=$BITCOIN_DATA_DIR -regtest -rpcuser=bitcoind -rpcpassword=bitcoind loadwallet "pl"
else
  echo "Wallet 'pl' is already loaded."
fi

# Check current block count
block_count=$(bitcoin-cli -datadir=$BITCOIN_DATA_DIR -regtest -rpcuser=bitcoind -rpcpassword=bitcoind getblockcount)

if (( block_count < 150 )); then
  blocks_to_mine=$((150 - block_count))
  echo "Mining $blocks_to_mine blocks to reach 150..."
  bitcoin-cli -datadir=$BITCOIN_DATA_DIR -regtest -rpcuser=bitcoind -rpcpassword=bitcoind generatetoaddress $blocks_to_mine $(bitcoin-cli -datadir=$BITCOIN_DATA_DIR -regtest -rpcuser=bitcoind -rpcpassword=bitcoind getnewaddress "" "bech32")

  echo "Distributing funds to random addresses we control..."
  for i in {1..75}; do
    bitcoin-cli -datadir=$BITCOIN_DATA_DIR -regtest -rpcuser=bitcoind -rpcpassword=bitcoind sendtoaddress "$(bitcoin-cli -datadir=$BITCOIN_DATA_DIR -regtest -rpcuser=bitcoind -rpcpassword=bitcoind getnewaddress)" 0.05
  done

  echo "Mining 1 additional block..."
  bitcoin-cli -datadir=$BITCOIN_DATA_DIR -regtest -rpcuser=bitcoind -rpcpassword=bitcoind generatetoaddress 1 $(bitcoin-cli -datadir=$BITCOIN_DATA_DIR -regtest -rpcuser=bitcoind -rpcpassword=bitcoind getnewaddress "" "bech32")
else
  echo "Blockchain already has $block_count blocks. No additional mining needed."
fi
