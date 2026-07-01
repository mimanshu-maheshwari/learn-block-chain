//! # ECC : Elliptic Curve Cryptography
//!
//! It is an asymmetric cryptography system based on elliptic curve mathematics.
//!
//! ECC = public/private key cryptography using elliptic curve.
//!
//! Used for key generation, digital signatures, key exchange, blockchain wallets, TLS/HTTPS,
//! hardware security and more.
//!
//! ECC is a public-key cryptography method where private keys are numberes, public keys are points
//! on an elliptic curve, and security comese from difficulty of reversing elliptic curve scalar
//! multiplication.
//!
//! ## Elliptic curve basics
//! An elliptic curve is a mathematical curve usually written like this:
//!
//!     $$y^2 = x^2 + ax + b$$
//!
//! for cryptography we don't usee normal infinite real number curves. We use elliptic curves over
//! finite fields.
//!
//! Important ECC terms:
//!
//!     curve equation
//!     point
//!     generator point
//!     private key
//!     public key
//!     point addition
//!     scalar multiplication
//!     finite field
//!     order
//!
//! ### Finite fields:
//! A finte field is a limited set of numbers wehre arithmetic wraps around using modulo
//!
//! In ECC, instead ofusing normal numbers, we use arithmetic modulo a large prime number : mod p
//!
//! So the curve becomes
//!
//!     $$y^2 = x^2 + ax + b mod p$$
//!
//! Why use ?
//!
//!     computer can store finite values
//!     operations are exact
//!     security can be based on hard number problems
//!     the curve has a fixed number of valid points
//!
//! blockchain example :
//!
//!     Bitcoin and ethereum use secp256K1
//!     Its field prime is very large
//!     all point math happens module that prime
//!
//! ### Elliptic curve discrete logarithm problem
//! Given G, Q where Q = K * G
//!
//! It is hard to find K
//! This is called ECDLP
//!
//! RSA 2048-bit ≈ ECC 224/256-bit security level range
//!
//! RSA 3072-bit ≈ ECC 256-bit security level range
//!
//! Advantages:
//!
//!     1. Smaller keys
//!     2. Smaller signatures
//!     3. Efficient Verification and signing
//!     4. Good for blockchain ownership
//!     5. Strong security
//!
//! Disadvantages:
//!
//!     1. harder to understand
//!     2. Implementation mistakes are dangerous
//!     3. ECDSA nounce reuse can leak private key
//!     4. Not quantum-safe
