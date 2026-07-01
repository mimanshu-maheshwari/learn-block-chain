//! # Mnemonics
//! Mnemonic phrase is a human readable list of words used to backup and recover a crypto wallet.
//!
//! It is also called :
//!
//! seed phrase
//! recovery phrase
//! backup phrase
//! mnemonics seed
//!
//! A mnemonic phrase can regenerate your walled keys.
//! Whoever has your mnemonic can control your walled.
//!
//! Seed phrase vs private key
//!
//! These are related but not the same
//!
//! Private key controls one blockchain account/address.
//!
//! Seed phrase can generate many private keys and many addresses.
//!
//! Example:
//!
//!     One seed phrase can generate:
//!     Ethereum account 1
//!     Ethereum account 2
//!     Bitcoin account 1
//!     Bitcoin account 2
//!
//! Never share your seed phrase
//! Never paste it on unknown websites
//! Never store it in plain text online

#[derive(Default, Debug)]
pub struct Mnemonic;

/// # BIP-39
/// It is the standard that defines how mnemonic phrases are created from entropy.
///
/// BIP : Bitcoin improvement proposal.
///
/// It defines:
///
///     How to generate mnemonic words?
///     How to use a word list?
///     how to checksum works?
///     how to convert mnemonic into seed?
///
/// Common mnemonic words count:
/// 12, 15, 18, 21, 24
///
/// Most common are : 12, 24
///
/// A 12 word mnemonic comes from:
///
/// 128 bits entropy + 4 bits checksum = 132 bits total
/// Since each word represents 11 bits:
/// 132 / 11 = 12 words
///
pub trait BIP_39 {}

/// # Checksum
/// is extra information added to detect errors
///
/// in BIP-39
/// part of entropy is hashed, part of hash is used as checksum.
/// checksum bits are added to entropy bits
/// then the full bit sequence is split into word indexes
///
/// detects typing mistakes
/// detects invalid mnemonic phrases
/// helps wallets reject incorrect recovery phrases
pub struct Checksum;

/**
# Seed Generation:
The mnemonic phrase is converted into a seed
BIP-39 uses a key derivation function called "PBKDF2-SHA512"
Input :
Mnemonic phrase
optional passphrase

output:

    512 bit seed

Mnemonics and seeds are not same

Optional passphrase
Sometimes called the 25th word

But it is not literally one word. IT can be any passphrase
if you use a passphrase and forget it. Wallet may be unrecoverable.
*/
pub struct Seed;

/**
* # Hierarchical Deterministic wallets:
* Keys are organized like a tree.
* same seed always generates same keys.
*
* One backup can recover many addresses
* wallet can generate new addresses automatically
* same seed gives same address sequence
* supports multiple accounts and chains
*/
pub struct HD_Wallets;
