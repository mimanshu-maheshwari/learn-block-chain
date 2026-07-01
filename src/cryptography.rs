//! # Cryptography
//!
//! Cryptography is the study and practice of techniques used to secure communication and data
//! against adversaries.
//!
//! Cryptography is a science of protecting information using mathematical techniques.
//!
//! It is use of mathematics to secure data, verify identies, prove authenticity, and prevent
//! unauthorized access or tampering.
//!
//! Allows blockchains to prove ownership , verify transactions, link blocks securely, and detect tampering.
//!
//! Blockchain cryptography is mainly used for :
//!
//!     1. proving ownership.
//!     2. Signing transactions.
//!     3. Creating wallet adresses.
//!     4. Linking blocks together.
//!     5. Detecting tampering.
//!     6. Verifying data efficiently.
//!
//! Cryptography provides four major guarentees.
//!
//!     1. Confidentiality : Only person intended can read teh data.
//!     2. Integrity : You can detect if data was changed.
//!     3. Authentication : You can verify who created or approved something.
//!     4. Non-repudiation : the sender cannot easily deny that they singed something
//!
//! ## Key temrs:
//! ### Plain text:
//! is the original readable data before encryption.
//!
//! ### Ciphertext:
//! is encrypted unreadable data.
//!
//! It should look random
//!
//! If encryption is strong then attackers should not be able to recover the plain text without the
//! correct key.
//!
//! ### Encryption:
//! is the process of converting plaintext into ciphertext
//!
//! ### Decryption:
//! is the process of converting ciphertext into plaintext
//!
//! ### Key
//! is a secret or semi-secret data used by a cryptographic algorithm
//!
//! There are multiple types of keys:
//!
//!     1. symmetric key
//!     2. Private key
//!     3. public key
//!     4. session key
//!     5. master key
//!     6. derived key
//!
//! In case of blockchain
//!
//!     Private key signs transactions
//!     Public key verifies signatures
//!     Address is derived from the public key
//!
//! ### Cipher
//! is a mthod for encrypting and decrypting data.
//!
//! Examples:
//!
//!     AES
//!     DES
//!     3DES
//!     ChaCha20
//!
//! A cipher defines how data is transformed
//!
//! The cipher can be public. The key must be protected.
//!
//! ### Entropy
//! means unpredictability or randomness.
//!
//! high entropy means hard to guess.
//!
//! Blockchain example:
//!
//! Private keys must be generated with strong randomness.
//! A weak random generator can create guessable private keys.
//!
//! ### Randomness
//! is the generation of unpredictable values.
//!
//! Cryptography needs secure randomness for
//!
//!     private keys
//!     session keys
//!     nonces
//!     IVs
//!     salts
//!     mnemonics
//!     ECDSA signing nonces
//! ### Nonce
//! number used oncee
//!
//! It is a value that should not be reused in the same context.
//!
//! User for:
//!     
//!     preventing replay
//!     ensuring uniqueness
//!     encryption modes
//!     digital signature
//!     blockchain transactions
//!
//! ### IV
//! means Initialization vector.
//!
//! It is a value used with some encryption modes to make encryption unique even when the same
//! plaintext is encrypted multiple times.
//!
//! Same plain text and key can lean patterns. Using an IV makes the cipher text unpredictable.
//!
//! IV usually doesn't need to be a secret.
//! IV must be unique/unpredictable depending on the encryption mode.
//!
//!
//! ### Padding
//!
//! if message is not exactly multiple of 16 bytes, padding is added.
//!
//! block ciphers need full blocks. Incorrect padding can usee security issues.
//! Some padding schemes are safer than others.
//!
//! PKC5#7 for block cipher modes
//! OAEP for RSA encryption
//! PSS for RSA signatures
//!

#[derive(Debug, Default)]
pub enum CryptographyKind {
    Symmetric,
    #[default]
    Asymmetric,
}

pub mod asymmetric;
pub mod symmetric;
