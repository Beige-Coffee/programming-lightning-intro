#!/bin/bash

# Ensure the Rust toolchain is set to stable
rustup default stable

already_running="$(bitcoin-cli -regtest -rpcuser=bitcoind -rpcpassword=bitcoind getblockchaininfo)"

echo $already_running

if [[ "$already_running" =~ "blocks" ]]; then
  echo "bitcoind already running"
else
  # start bitcoind with the provided configuration file
  bitcoind -conf=$(pwd)/bitcoin.conf 
  sleep 3
    # create a dummy wallet in bitcoin core for us to mine blocks to and use as a source of funds for funding our addresses
  bitcoin-cli -regtest -rpcuser=bitcoind -rpcpassword=bitcoind createwallet "pl"
  # mine some blocks so we have bitcoin to use
  bitcoin-cli -regtest -rpcuser=bitcoind -rpcpassword=bitcoind generatetoaddress 151 $(bitcoin-cli -regtest -rpcuser=bitcoind -rpcpassword=bitcoind getnewaddress "" "bech32")

for i in {1..100}
do
  bitcoin-cli -regtest -rpcuser=bitcoind -rpcpassword=bitcoind sendtoaddress "$(bitcoin-cli -regtest -rpcuser=bitcoind -rpcpassword=bitcoind getnewaddress)" 0.05
done
bitcoin-cli -regtest -rpcuser=bitcoind -rpcpassword=bitcoind generatetoaddress 1 $(bitcoin-cli -regtest -rpcuser=bitcoind -rpcpassword=bitcoind getnewaddress "" "bech32")
fi