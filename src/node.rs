//! Node is software that participates in the network
//!
//! There are defferent kind of nodes
//!     1. Full node`
//!

#[derive(Debug, Default)]
pub enum Node {
    /// Stores historicals state.
    ///
    /// An archive node can answer questions like
    ///     1. What was Alice's balance at block 8_000_000?
    /// Archive node requires much more storage
    Archive,
    /// Boot node is a known node that helps to find new nodees in the network
    ///
    /// it doesn't control network it just helps in initial discovery
    Boot,
    /// Full node verifies the block chain. It stores enough data to check
    ///     1. All blocks are valid?
    ///     2. Are transactions valid?
    ///     3. Is the current state correct?
    /// A full node doesn't blindly trust others
    /// A normal full node may know the historical and current state but not every past state at
    /// every block
    Full,
    /// Doesn't store everything
    ///
    /// Might store block headers and requests proof from full node.
    /// relies on cryptographic proof rather than downloading the full node
    Light,
    /// RPC node exposes api for wallets and application.
    ///
    /// For example when meta mask shows you your balance it ask an RPC endpoint
    ///
    /// Common methods are:
    ///     1. get_balance
    ///     2. send_raw_transaction
    ///     3. call
    ///     4. get_logs
    ///     5. get_transaction_receipt
    ///
    /// It is a bridge between normal apps and block chain
    RPC,
    Sequencer,
    /// Also known as minors
    /// The node particiapte in block production
    ///
    /// In proof of work system minors produce blocks by spending computation.
    ///
    /// In proof of stake systems, validators produce or attest to blocks by staking tokens
    #[default]
    Validator,
}
