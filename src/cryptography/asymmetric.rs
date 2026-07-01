//! Asymmetric cryptography is a cryptographic system that uses two different but mathematically
//! related keys:
//!
//!     1. Public key
//!     2. Private key
//!
//! The public key can be shared with everyone.
//!
//! the private key must be kept secret.
//!
//! Asymmetric cryptography is a system where one key is public and other key is private, allowing
//! secure encryption, decryption, signing, and verification without sharing the private key.
//!
//! ### Public key
//!
//! - encrypting data for the private key owner
//! - verifying digital signatures
//! - deriving blockchain addresses
//! - identifying an account
//!
//! ### Priavte key:
//!
//! - decrypting messages encrypted by private key
//! - signing messages or transactions
//! - proving ownership
//! - controlling blockchain assets
//!
//! Two main uses:
//!
//!     1. Encryption
//!     2. Signing
//!
//! Advantages:
//!
//!     1. Solves key sharing problem
//!     2. Enables digital signatures
//!     3. Usefull for open networks
//!     4. Supports identity systems
//!
//! Disadvantages:
//!
//!     1. Slower than symmetric cryptography
//!     2. More comples
//!     3. Private key loss is catastrophic
//!     4. Vulnerable to poor randomness
//!     5. Public key authenticity problem
//!
//! Transaction flow:
//!
//!     1. User creates transaction
//!     2. Wallet hashes the transaction
//!     3. Private key signs the transaction hash
//!     4. Signature is attached to transaction
//!     5. Network verifies signature using public key/signature recovery
//!     6. If valid, transaction is accepted

/// # Rivest, Shamir, Adleman (RSA)
/// RSA is an asymmetric cryptographic algorithm based on the difficulty of factoring large numbers.
///
/// It is easy to multiply two large prime numbers.
/// But given only the result, it is extremely hard to find the original prime numbers.
///
/// p * q = n
///
/// Given n, find p and q is hard
///
/// p and q are extremely large, usually hundreds or thousands of bits.
///
/// ### Modulo arithmetic basics
///
/// Modulo means remainder after division.
/// In RSA, calculations are done "mod n".
///
/// That means numbers wrap around after reaching n.
///
/// ciphertext = message ^ e mod n
/// message = ciphertext ^ d mod n
///
/// n = modulus
/// e = public exponent
/// d = private exponent
///
/// public key contains (n, e)
///
/// where n = modulus, e = public exponent
///
/// common value of e in real systems = 65537 as it is efficient, large enough to avoid some old
/// weaknesses and become a standard public exponent
///
/// Private key = (n, d)
///
/// n = modulus, d = private exponent
/// decrypting messages encrypted with the public key creating RSA digital signatures
///
/// p, q, adn d must remain secret.
///
/// Idea:
///
/// ```text
/// 1. Choose tow large prime numbers: p and q.
/// 2. Compute n = p * q
/// 3. Compute φ(n) = (p - 1) × (q - 1)
/// 4. Choose public exponent e
/// 5. Compute private exponent d
/// 6. PUblic key = (n, e)
/// 7. Private key = (n, d)
/// ```
/// φ(n)  is called Euler's totient function.
///
/// The private exponent d is chosen so that
///
/// e × d ≡ 1 mod φ(n)
///
/// meaning d is modulo inverse of e
///
/// ### Encryption:
/// ciphertext = message ^ e mod n
///
/// ### Decryption
/// message = ciphertext ^ d mod n
///
///
/// Real RSA uses padding schemes:
///
///     1. OAEP (Optimal asymmetric Encryption Padding) for encryption
///     2. PSS (Probabilistic Signature Scheme) for signatures
///
/// Advantages:
///
///     1. Simple public/private key model
///     2. Widely used and well studied.
///     3. Useful for encryption and signatures.
///     4. Strong when used correctly
///
/// Disadvantages:
///
///     1. Slower than symmetric
///     2. Large key sizes
///     3. Padding mistakes are dangerous
///     4. key generation must be strong
///     5. Not commonly used in block chain wallets
///
/// They use elliptic curve cryptography, mainly:
///
/// ECC
/// ECDSA
/// secp256k1
///
/// ECC gives strong security with smaller keys and signatures.
#[derive(Debug, Default)]
pub struct RSA {}

pub mod ecc;
