//! `Mempool` is a temporary waiting area for transactions.
//!
//! When you submit a transaction it doesn't directly goes into the block chain
//! it usually goes here first
//!
//! User wallet -> RPC node -> mempool -> validator/minor -> block
//! The mempool contains pending transactions
//! Validators choose transactions from this pool prioritizing higher fees

#[derive(Debug, Default)]
pub struct Mempool;
