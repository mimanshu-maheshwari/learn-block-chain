//! # Hash functions:
//! A hash function is a function that takes input data of any size and produces a fixed size
//! output.
//!
//! The output is called
//!     
//!     Hash
//!     digest
//!     message digest
//!     fingerprint
//!     checksum, in non security context
//!
//! hashing is not encryption
//! Hashing is one way
//! You can't decrypt a hash back into the original input
//!
//! A hash has a deterministic output.
//! For same input output will also be some
//!
//! Will have fixed size output.
//!
//! One way property, once hashed can't be reversed.
//!
//! Pre image resistance, given a hash output, it should be difficult to find the original content
//! that produces that hash.
//! Second pre image resistance: given one input, it should be hard to find a different input with
//! the same hash.
//!
//! If a transaction has a hash, an attacker should not be able to replace it with different
//! transaction having the same hash.
//!
//! Collision resistance.
//!
//! Avalanche effect
//! Tiny changes in the input should causes a completely diffferent looking hash.
//!
//! Example SHA-256, Keccak-256
//!
//! Address generation:
//! Ethereum address generation using hashing:
//!
//! private key -> public key -> keccak-256(public key) -> last 20 bytees -> address
//!
//! Bitcoin uses hashing too, including SHA-256 and RIPEMD-160 in its address generation process.
//!
//! Common hashing algorithm:
//!
//!     MD5
//!     SHA-1
//!     SHA-2
//!     SHA-256
//!     SHA-512
//!     SHA-3
//!     keccak-256
//!     RIPEMD-160
//!     BLAKE2
//!     BLAKE3
//!

/// # Message Digest Algorithm 5
/// It produces a 128-bit hash.
///
/// MD5 is insecure as collisions can be created.
pub struct MD5;

/// # Secure hashing algorithm 1
/// 160-bit hash
///
/// Insecure because of collision attack
pub struct SHA_1;

/// # Family of Secure hashing algorithm
///
/// It is still considered very secure.
///
/// SHA-2 uses a Merkle-Damgård style construction.
pub enum SHA_2 {
    SHA_224,
    /// produces a 256-bit hash.
    ///
    ///     Diterministic
    ///     fixed size
    ///     preimage resistance
    ///     seecond preimage resistance
    ///     collision resistance
    ///     avalache effect
    SHA_256,
    SHA_384,
    /// produces a 512-bit hash
    ///
    /// Faster for 64 bit processors as it is designed around 64 bit operations.
    SHA_512,
}

/// SHA-3 uses a sponge construction.
///
/// It absorbs input data,
/// mixes it internally
/// then squeezes out hash output.
pub enum SHA_3 {
    SHA3_224,
    SHA3_256,
    SHA3_384,
    SHA3_512,
}

pub enum Keccak {
    /// 32 bytes (256 bits)
    Keccak_256,
}

/// 160 bit hash function
pub struct RIPEMD_160;

///
///     Fast
///     Secure
///     supports keyed hashing
///     supports salting/personalization
///     efficient in software
pub enum BLAKE2 {
    /// optimized for 64-bit platforms
    BLAKE2b,
    /// optimized for 8-bit to 32 bit platforms
    BLAKE2s,
}

/// Newer hash function based on BLAKE2 and the Bao tree mode.
///
/// Designed to be fast and parallelizable .
///
///     very fast
///     parallel-friendly
///     support extendable output
///     can work as hash, keyed has, or key derivation function
///
/// BLAKE3 usees a tree structure internally, which allows it to hash a large files efficiently in parallel
///
pub struct BLAKE3;
