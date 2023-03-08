#  Environment Setup

For most of the exercises we will need bitcoind and an electrum server running.  The binaries, configurations, and some helpful aliases are all available as part of the replit environment. 

👉 Run this in your console to get your environment set up:
```
./start.sh
```

If you're curious you can take a look to see what the script is doing but it's pretty straight forward if you're familiar with bitcoind but it:

- It checks to see if bitcoind is already running and if not it automatically starts bitcoind on regtest
  
- It creates a bitcoin core wallet and then mines some blocks so we have bitcoin to use

When you see all of the block hases that have been mined you're all set to move on to the next section!

```
 [
  "0dcd0833b53ae3cbe77aa6f2edb9d4a4acd13fac79c26c4f04f6173e2211a487",
  "78e02951603e12f264753dd86b11879f5819d034e5c09614cefea6934614245f",
  "65d5cebede17dc971866eb7a8506dfa7230adbe089b37d0cfe9c6645027d74c4",
  "62069ab6a4efe2466970ab33bd95fca320bc564b90cc425a9cc1984e9ee2bab9",
  "19da0b314fa3e1fe65ab3eec1ecda3e4b7061d444f1ac739ddd91656bfa5b8d1",
  "3f5bd8eafa7d8533f262434c8d4390205e77d29a177f70205f5500ab77ae6bd0",
  "5aa8ad9d912d033cb96c7131f6ef4b73f2759f337ee4b0049fadc21d9bfdce16",
  "1f00930bdad57d2fb2591f2b97d7d225f3b5d78f368b1b8dab7a426dc78950a5",
  "5a92b9172896f224349156b538c57034ec74770bfd83ace92aa6f5d18b933d3a",
  "1d039ce163e12d4586a6b419cfe020efcd1fdc08f5bc5ed2ff142bca4aa3e565",
  "713141b381d01873a3da97d772b7716449ee7181169a52a464c11813ac0c3a83",
  "1847e3090755763626b05d590465bb675921ce3d314d20a7cf48a2da5353fde2",
  "2b63c114311a30e02464e230cd3db02e0eb305bc38de9dd0a79c163a0c780ede"
]
```