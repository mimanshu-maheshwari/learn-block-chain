//! # Rollups
//! Rollup execute transaction off-chain on a seperate layeer, then post compressed data proofs to l1
//!
//! Two major types of Rollups:
//!
//!     1. Optimistic Rollup
//!     2. ZK rollup

/// L2 executes many transactions
/// L2 compresses result
/// L2 posts proof/data to L1
/// L1 secures settlement
#[derive(Debug, Default)]
pub enum Rollup {
    /// Optimistic rollups assume transactions are valid unless challenged.
    #[default]
    Optimistic,
    /// ZK rollups post cryptographic validity proofs.
    ZK,
}
