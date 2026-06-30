//! Transaction is a signed instruction from user
//!
//! transaction lifecycle
//!     1. user creates transaction
//!     2. User signs transaction
//!     3. Wallet sends it to a node.
//!     4. Node validates basic rules.
//!     5. Node brodcasts it to peers.
//!     6. Transaction enters mempool
//!     7. Validators include it in a block.
//!     8. Block is propogated.
//!     9. Other nodes verify block
//!     10. Transaction becomes part of chain

#[derive(Debug, Default)]
pub struct Transaction {
    // TODO: what is type?
    /// sender address
    pub from: String,
    // TODO: what is type?
    /// receiver address or smart contract addres
    pub to: String,
    /// native currency amount
    pub value: f64,
    // TODO: what is type?
    /// function call or contract byte-code
    pub data: Option<Vec<u8>>,
    /// sender transaction counter
    pub nonce: u32,
    /// max computation sender allows
    ///
    /// Gas is the fee modal for computation
    ///
    /// Every operation has a cost
    ///
    /// For example:
    ///
    ///     Adding numbers: cheap
    ///     Writing to storage : expensive
    ///     deploying contract : expensive
    ///
    /// Gas exist to prevent infinite computation
    /// Without gas someone could deploy a contract with and infinite loop and freeze the network.
    ///
    /// If user runs out of gas, the transaction fails, but the user still pays for computation
    /// already consumed.
    pub gas: f64,
    /// payment to validator
    pub fee: f64,
    pub priority_fee: f64,
    // TODO: what is type?
    /// Proof that sender authorized this
    ///
    /// a user doesn't login with user or password.
    /// User signs with private key and public key/address verifies it.
    /// If you control the private key then you control the asset
    pub signature: [u8; 32],
}
