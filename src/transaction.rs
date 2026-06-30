/// Transaction is a signed instruction from user
#[derive(Debug, Default)]
pub struct Transaction {
    // TODO: what is type?
    /// sender address
    from: String,
    // TODO: what is type?
    /// receiver address or smart contract addres
    to: String,
    /// native currency amount
    value: f64,
    // TODO: what is type?
    /// function call or contract byte-code
    data: Option<Vec<u8>>,
    /// sender transaction counter
    nonce: u32,
    /// max computation sender allows
    gas: u32,
    /// payment to validator
    fee: f64,
    // TODO: what is type?
    /// Proof that sender authorized this
    ///
    /// a user doesn't login with user or password.
    /// User signs with private key and public key/address verifies it.
    /// If you control the private key then you control the asset
    signature: [u8; 32],
}

impl Transaction {}
