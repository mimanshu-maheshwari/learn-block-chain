//! # Indexer
//! Reading blockchain can be slow.
//!
//! For example you want to show:
//!
//! 1. All NFT's owned by this user.
//! 2. All transfer for this token.
//! 3. All trades on this DEX (Decentralized EXchange).
//!
//! You usually don't query the blockchain for everything.
//!
//! Instead indexers read blockchain event and store them in query-friendly databases.
//!
//! Examples of Indexer like systems are:
//!
//!     1. The Graph
//!     2. Custom event indexers
//!     3. Block exploeres
//!     4. Analytic pipelines
//!
//! Mental model:
//!
//!     1. Block chain is source of truth.
//!     2. Indexers are fast query-friendly search layers.

#[derive(Debug, Default)]
pub struct Indexer;
