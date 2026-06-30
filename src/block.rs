use crate::{state::StateModel, transaction::Transaction};

/// Block is a batch of transactions
#[derive(Debug, Default)]
pub struct Block<State: StateModel> {
    header: Header<State>,
    body: Body,
}

#[derive(Debug, Default)]
struct Header<State: StateModel> {
    /// It is a hash pointer which stores the pointer to previous block and its hash for validation
    /// if the hash stored doesn't match the hash of block, then the chain is tempered
    previous_block_hash: [u8; 32],
    // TODO: what is type?
    timestamp: String,
    // TODO: what is type?
    /// Merkle tree is the tree of hashes
    /// It is used to validate the transactions present in current block
    ///
    /// Example:
    /// lets say we have 4 transactions t1, t2, t3, t4
    /// we hash all the transactions
    /// h1 = hash(t1)
    /// h2 = hash(t2)
    /// h3 = hash(t3)
    /// h4 = hash(t4)
    ///
    /// then we hash joint hashes
    /// h12 = hash(h1 + h2)
    /// h34 = hash(h3 + h4)
    ///
    /// then we again hash to get root hash
    /// merkle_root = hash(h12 + h34)
    ///
    /// Through this merkle tree we can valiate easily if the transactions are valid or not
    merkle_root: String,
    // TODO: what is type?
    /// Block chain stores history but users generally care about the current state
    state_root: State,
    // TODO: what is type?
    transaction_root: String,
    // TODO: what is type?
    receipts_root: String,
    // TODO: what is type?
    validator_info: String,
    // TODO: what is type?
    block_number: u128,
    // TODO: what is type?
    consensus_fields: String,
}

#[derive(Debug, Default)]
struct Body {
    transactions: Vec<Transaction>,
}
