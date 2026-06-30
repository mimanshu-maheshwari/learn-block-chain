//! # dApp
//! Decentralized app usually has several layers
//!     
//!     1. Frontend
//!     2. Wallet
//!     3. RPC provider
//!     4. Smart contract
//!     5. Blockchain nodes
//!     6. Indexers
//!     7. Storage
//!     8. Oracles
//!
//! A typical dApp flow:
//! ```text
//! User opens website:
//!     -> connects wallet
//!     -> frontend reads blockchain through RPC
//!     -> User signs transaction
//!     -> wallet broadcasts transactions
//!     -> transaction reaches mempool
//!     -> validator includes it in block
//!     -> Frontend watches for confirmation
//! ```
//!
//! Important points:
//!     
//!     Frontend may be normal webcode
//!     Smart contract is Decentralized
//!     RPC provider may or may not be decentralized

#[derive(Debug, Default)]
pub struct DApp;
