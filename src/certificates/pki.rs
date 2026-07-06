//! # PKI
//! stands for public key infrastructure
//!
//! It is the trust system that connects public keys to real-world identities using certificats.

use crate::certificates::Certificate;
pub struct PKI {
    /// A certificate authority or CA is an organization that issues certificates.
    /// Its job is to verify the identity and sign certificates.
    ///
    /// Example:
    ///
    ///     DigiCert
    ///     GlobalSign
    ///     Sectigo
    ///     Let's Encrypt
    ///     Google Trust Services
    ///
    /// Browsers and operating systems trust selected CAs by default
    certificates_authorities: Vec<String>,
    /// *Root CA*
    /// is the top level trusted authority. Its certificate is self-signed. Browsers or OS has a
    /// build in list of trusted root CAs
    ///
    /// This list is called Trust store, root store
    ///
    /// Root CA private keys are extremely sensitive. If compromised, many certificates could become
    /// untrustworthy.
    root_cas: Vec<String>,
    /// an Intermidiate CA is a CA whose certificate is signed by a root CA or another intermediate
    /// CA.
    ///
    /// Root CA signs Intermediate CA. Intermediate CA signs website certificate.
    ///
    /// Protect root private keys, Delegate certificate issuance, limit damage if an intermediate is
    /// compromised, organize certificate operations.
    ///
    intermediate_cas: Vec<String>,
    /// End-Entity certificate
    /// is the final certificate used by a website, server, user or device.
    /// For HTTPS, this is the website certificate.
    ///
    /// It contains:
    ///
    ///     domain name
    ///     public key,
    ///     validity period
    ///     issuer
    ///     signature
    ///
    /// It is called end entity as it doesn't sign other certificates.
    ///
    /// It is used for:
    ///
    /// - server authentication
    /// - TLS/HTTPS
    /// - client authentication
    /// - code signing
    /// - email signing/encryption
    end_entity_certificates: Vec<Certificate>,
    /// Trust chain
    /// links an end-entity certificate back to a trusted root ca.
    /// If every signature in the chain is valid and the root CA is trusted, the certificate can be
    /// trusted.
    /// Browser checks:
    ///
    /// - is the chain valid?
    /// - is the root trusted?
    /// - is the domain name correct?
    /// - is the certificate expired?
    /// - was it revoked?
    /// - is the signature valid?
    trust_stores: Vec<String>,
    certificate_validation_rules: Vec<String>,
    revocation_systems: Vec<String>,
}

/// # X.509
/// is the standard format used ifor most digital certificates.
pub struct XDot509 {
    version: String,
    serial_number: String,
    signature_algorithm: String,
    issuer: String,
    validity_period: f64,
    subject: String,
    subject_public_key: Vec<u8>,
    extensions: Vec<XDot509Extensions>,
    issuer_signature: Vec<u8>,
}
pub struct XDot509Extensions {
    /// for websites, the most important identity field is usually
    /// Subject Alternate name or SAN
    subject_alternative_name: String,
    key_usage: String,
    extended_key_usage: Vec<String>,
    basic_constraints: String,
    certificate_policies: Vec<String>,
    crl_distribution_points: Vec<String>,
    authority_information_access: Vec<String>,
}
