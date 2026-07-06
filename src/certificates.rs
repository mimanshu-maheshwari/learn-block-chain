//! # Certificates and PKI
//! A digital certificate is a digital document that binds a public key to an identity.
//!
//! A certificate usually contains:
//!
//! - subject name
//! - subject public key
//! - issuer name
//! - validity period
//! - serial number
//! - signature algorithm
//! - issuer's digital signature

/// Public keys don't alone prove identity, A trusted authority confirms that this public key
/// belongs to this identity.
pub struct Certificate {
    subject_name: String,
    subject_public_key: Vec<u8>,
    issuer_name: String,
    serial_number: usize,
    signature_algorithm: String,
    issuer_digital_signature: Vec<u8>,
}

mod pki;
