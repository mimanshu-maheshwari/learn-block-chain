//! Symmetric key cryptography is a type of cryptography where the same key is used for both
//! encryption and decryption
//!
//! ### Block cipher
//! encrypts data in a fixed-size chunk called blocks.
//!
//! AES block size = 128 bits = 16 bytes
//!
//! Common block cipher are
//!
//!     DES
//!     3DES
//!     AES
//!
//! A block cipher only tells us how to encrypt one block.
//! But real messages are usually longer than one block.
//! So we need modes of operation.
//!     
//!     ECB Electronic codebook
//!     CBC Chipher Block Chaining
//!     CTR Counter Mode
//!     GCM Galois/Counter Mode
//!
//! These modese define how multiple blocks are encrypted together.
//!
//! AES = block cipher
//! AES-GCM = AES in GCM mode.
//! AES-CBC = AES in CBC mode.
//!
//! ### Stream cipher
//! encrypts data one bit or byte at a time.
//!
//! Instead of fixed-size blocks, it creates a stream of pseudo-random data called keystream.
//!
//! Then teh plaintext is combined with that keystream
//!
//! Common streaem ciphers:
//!
//!     ChaCha20
//!     Salsa20
//!     RC4, but RC4 is now considered insecure
//!
//! Advantages:
//!
//!     1. Very fast
//!     2. Efficient for large data
//!     3. Smaller keys compared to RSA
//!
//! Disadvantages:
//!     
//!     1. Key sharing problem
//!     2. Does not scale well for many people
//!     3. If key leaks, everything protected by it is exposed.
//!     4. No build in identity proof
//!
//! Usage:
//!
//!     1. HTTPS/TLS
//!     2. Wi-Fi encryption
//!     3. VPNs
//!     4. disk encryption
//!     5. database encryption
//!     6. messaging apps
//!     7. password managers
//!     8. cloud storage encryption
//!
//! For blockchain it is used for :
//!
//!     wallet encryption
//!     keystore files
//!     private key storage
//!     encrypted backups
//!     encrypted messaging
//!     secure RPC connections
//!     TLS/HTTPS
//!     private blockchain communication

/// block size 64 bits
/// key size 56 effective bits
/// Insecure/withdrawn
///
/// Working:
///
///     1. Take 64-bit plaintext block
///     2. Apply initial permutation
///     3. Split block into left half and right half
///     4. Run 16 rounds of transformation
///     5. Swap/combine halves
///     6. Apply final premutaion
///     7. Output 64-bit ciphertext block.
///
#[derive(Debug, Default)]
pub struct DES;

/// block size 64 bits
/// key sie 112 or 168 normal bits
/// Depricated
#[derive(Debug, Default)]
pub struct ThreeDES;

/// block size 128 bit
/// key size 128, 192, 256 bits
/// modern standerd
/// Advanced Encryption Standard
///
/// based on Rijndael cipher family.
///
/// Used in
///     
///     1. HTTPS/TLS
///     2. Wi-Fi security
///     3. VPNs
///     4. Disk encryption
///     5. wallet keystore encryption
///     6. password managers
///     7. Cloud storage encryption
///
/// Basic working:
///
///     Substitues bytes
///     shifts rows
///     mixes colums
///     adds round keys
///
/// Operates on a 4 x 4 matrix called the state
///
/// 128 bit block = 16 bytes
///
/// Each AES round uses operations like SubBytes, ShiftRows, MixColumns, AddRoundKey
///
/// Public blockchains do not usually encrypt public transaction data using DES/AES because data
/// must be visible for verification.
///
/// But it is used for
///
///     Wallet keystore encryption
///     private key backup encryption
///     encrypted local storage
///     encrypted node keys
///     secure API/RPC communication through TLS
///     encrypted databases
#[derive(Debug, Default)]
pub struct AES;
