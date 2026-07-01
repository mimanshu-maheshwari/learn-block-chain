//! # Rollups
//! Rollup execute transaction off-chain on a seperate layeer, then post compressed data proofs to l1
//!
//! Two major types of Rollups:
//!
//!     1. Optimistic Rollup
//!     2. ZK rollup
//!
//! The key difference is how they prove the transactions are correct.

/// L2 executes many transactions
/// L2 compresses result
/// L2 posts proof/data to L1
/// L1 secures settlement
#[derive(Debug, Default)]
pub enum Rollup {
    /// Optimistic rollups assume transactions are valid by default unless challenged.
    ///
    /// Flow:
    ///
    ///     1. Users send transactions to the rollups
    ///     2. Rollups execute transactions off-chain
    ///     3. Rollups post transaction data/result to layer 1
    ///     4. Layer 1 accepts the reseults optimistically.
    ///     5. Anyone can challenge a result during a challenge window
    ///     6. If fraud is proven the bad result is rejected.
    ///
    /// Examples:
    ///
    ///     1. Optimism
    ///     2. Arbitium
    ///     3. Base
    ///     4. Mantle-style optimistic system.
    ///
    /// Advantages:
    ///
    ///     1. Easier EVM compatibility.
    ///     2. Good for general smart contracts.
    ///     3. Mature ecosystem.
    ///     4. Lower computation cost than L1
    ///
    /// Disadvantages:
    ///
    ///     Withdrawals to l1 can be slow.
    ///     Fraud challenge period is neede.
    ///     Security depends on at least one honest challenger/watcher.
    ///
    /// The classic issue is withdrawals delay.
    ///
    /// Because system needs time to fraud challengs, moving assets from the rollup back to ethereum
    /// can take days
    #[default]
    Optimistic,
    /// Zero Knowledge rollups or validity rollups
    /// ZK rollups post cryptographic validity proofs.
    ///
    /// In rollups the main point is not always privacy. The main point is validity proof.
    ///
    /// Instead of assuming correctness ZK rollups generate a proof that state transition is correct.
    ///
    ///
    /// Flow:
    ///
    ///     1. Users send transactions to the rollup
    ///     2. Rollup execute transaction off-chain
    ///     3. Rollup creawte a cryptographic validity proof
    ///     4. Proof is submitted to layer one
    ///     5. Layer 1 verifies the proof
    ///     6. If the proof is valid the batch is accepted.
    ///
    /// Examples:
    ///
    ///     zkSync
    ///     Starknet
    ///     Ploygon zkEVM
    ///     Scroll
    ///     Linea
    ///     Taiko-style zkEVN system
    ///
    /// Advantages:
    ///
    ///     Faster finality
    ///     No long fraud challenge window
    ///     Strong crytographic correctness
    ///     Good for scalling
    ///     Potential privacy usee cases
    ///
    /// Disadvantages:
    ///
    ///     Proof generation is comples
    ///     Harder EVM compatibility, though improving
    ///     More advanced cryptography
    ///     Can be expensive to build
    ZK,
}
