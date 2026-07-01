//! ## Digital Signatures:
//! A digital signature is a cryptographic proof that ta message was approved by the holder of
//! private key.
//!
//!
//! Sign a message using private key.
//! Verify using public key.
//!
//! It solves three major problem.
//!
//! 1. Who approved this?
//! 2. Was the message changed?
//! 3. Can the signer deny signing it later?
//!
//! ### Common digital signature algorithms
//!
//!     RSA signature
//!     DSA
//!     ECDSA
//!     EdDSA
//!     BLS signature
//!
//! Bitcoin uses ECDSA over secp256k1
//! Ethereum uses ECDSA over secp256k1
//! Ethereum validators : BLS signatures
//! Solana : Ed25519 / EdDSA-style signatures.

pub enum SignatureAlgorithms {
    /// # RSA signatures
    /// RSA signatures use the RSA private key to sign and the RSA public key to verify.
    ///
    /// RSA-PSS should be used.
    ///
    /// Advantages:
    ///     
    ///     Well studied
    ///     widely supported
    ///     good for certificates and PKI
    ///
    /// Disadvantages:
    ///
    ///     large keys
    ///     large signatures
    ///     slower than ECC-based signatures
    ///     not ideal for blockchain transaction size
    RSASignature,
    /// # Digital Signature algorithm
    /// uses random per-signature nonce
    DSA,
    /// # Elliptical Curve Digital Signature Algorithm
    ///
    /// Advantages:
    ///
    ///     smaller keys than RSA
    ///     Smaller signatures than RSA
    ///     Efficient
    ///     Widely used in blockchain
    ///
    /// Disadvantages:
    ///
    ///     very sensitive to nonce mistakes
    ///     harder math than RSA
    ///     signature malleability issue if not handled properly
    ///     not quantum-safe
    ECDSA,

    /// # Edwards-curve Digital Signature Algorithm
    ///
    /// Common version is Ed25519.
    ///
    /// Used by solana, some newer blockchain systems, SSH keys, modern cryptographic protocols.
    ///
    ///     Fast (signing and verification)
    ///     Simple to implement safely
    ///     deterministic
    ///     resistant to many ECDSA style nonce mistake
    ///     good for modern systems.
    EdDSA,

    /// # Boneh-Lynn-Shacham
    /// they support signature aggregation
    ///
    /// Advantages:
    ///
    ///     signature aggregation
    ///     compact multi-validator proofs
    ///     useful for consensus
    ///     supports threshold/multisig designs
    ///
    /// Disadvantages:
    ///
    ///     more complex cryptography
    ///     pairing based math
    ///     slower individual operations in some context
    ///     requires careful implementation.
    BLS,

    /// # secp256
    /// refers to a family of elliptic curves useed in elliptic curve cryptography.
    Secp256(Secp256),
}

/// # secp256
/// refers to a family of elliptic curves useed in elliptic curve cryptography.
///
/// sec = Standards of Efficient cryptography
/// p   = prime field
/// 256 = 256 bit field size.
///
///
pub enum Secp256 {
    /// ### Important properties:
    /// 256-bit curve
    /// Defined over a prime field
    /// Used for ECDSA signatures
    /// Used by bitcoin and ethereum
    ///
    /// Simple curve : y^2 = x^3 + 7
    secp256k1,
    /// It is also called P-256
    /// P-256
    /// Prime256v1
    /// NIST P-256
    ///
    /// It is widely used in TLS/HTTPS
    /// WebAuthn/passkeys
    /// secure hardware
    /// mobile security
    /// government/enterprise systems
    ///
    /// it is more complex in curve and parameters
    secp256r1,
}
