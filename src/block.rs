//! # Block
//! A block in a blockchain is a digital container that permanently stores batches of valid transactions and data.
//! It acts as a single page in a giant, distributed ledger.
//! Once a block is filled with data, it is cryptographically sealed and permanently connected to the previous block, creating an unchangeable chain.

use crate::{state::StateModel, transaction::Transaction};
use std::marker::PhantomData;

/// Block is a batch of transactions
#[derive(Debug, Default)]
pub struct Block<State: StateModel> {
    pub header: Header,
    pub body: Body,
    pub state_kind: PhantomData<State>,
}

/// The  block header stores root such as
///
///     1. state root
///     2. transactions root
///     3. receipt root
///
/// These roots are compact finger print of large data structure.
///
#[derive(Debug, Default)]
pub struct Header {
    /// It is a hash pointer which stores the pointer to previous block and its hash for validation
    /// if the hash stored doesn't match the hash of block, then the chain is tempered
    pub previous_block_hash: [u8; 32],
    // FIXME: should be a timestamp
    pub timestamp: String,
    // TODO: what is type?
    /// Merkle tree is the tree of hashes
    /// It is used to validate the transactions present in current block
    ///
    /// Example:
    /// ```text
    /// lets say we have 4 transactions t1, t2, t3, t4
    /// we hash all the transactions
    /// h1 = hash(t1)
    /// h2 = hash(t2)
    /// h3 = hash(t3)
    /// h4 = hash(t4)
    /// then we hash joint hashes
    /// h12 = hash(h1 + h2)
    /// h34 = hash(h3 + h4)
    /// then we again hash to get root hash
    /// merkle_root = hash(h12 + h34)
    /// ```
    ///
    /// Through this merkle tree we can valiate easily if the transactions are valid or not
    pub merkle_root: String,
    // TODO: what is type?
    /// Block chain stores history but users generally care about the current state
    /// Cryptographic commitment to entire world state.
    ///
    /// It means if any account balance or contract storage changes, the state root changes.
    ///
    /// This allows nodes to verify they are working with the same state.
    pub state_root: String,
    // TODO: what is type?
    pub transaction_root: String,
    // TODO: what is type?
    pub receipts_root: String,
    // TODO: what is type?
    pub validator_info: String,
    // TODO: what is type?
    pub block_number: u128,
    // TODO: what is type?
    pub consensus_fields: String,
}

/// Body stores list of `crate::transaction::Transaction`
#[derive(Debug, Default)]
pub struct Body {
    pub transactions: Vec<Transaction>,
}
