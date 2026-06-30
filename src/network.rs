//! ## Peer to Peer networking
//!
//! Block chain networks are usually peer-to-peer (P2P)
//!
//! There is not central server that all nodes depends on.
//!
//! Instead:
//!     
//!     Node A connects to node B, C, D
//!     Node B connects to node D, F, G,
//!     Node C connects to node F, E, J
//!     ...
//!
//! Together they form a mesh network
//!
//!
//! ### Peer to peer discovery
//! A node needs to find other nodes.
//!
//! It can do it using:
//!     
//!     bootnodes
//!     DNS discovery
//!     peer exchange
//!     discovery protocols
//!     static peer lists
//!
//! ### Gossip Protocol
//! Block chain often uses gossip type communication.
//!
//! If one node receives a new transaction it tells some peers.
//!
//! Those peers tell their peers
//!
//! Eventually transaction spread across the network
//!
//! Example:
//! ```text
//! Alice's wallet:
//!     Node1 ->
//!         Node 2 ->
//!             Node 3 ->
//!             Node 4 ->
//!         Node 4 ->
//!             Node 5 ->
//!             Node 7 ->
//! ```
//! This is similar to how rumers spread in the croud
//!
//! ### Transaction propogation:
//! When a node receives a transaction, it checks basic validity:
//!     
//!     Is the signature valid?
//!     Is nonce correct?
//!     Can the sender pay?
//!     Is the transaction format valid?
//!     Is the gas limit acceptable?
//!
//! If all is valid it adds it to mempool and broadcasts it to other nodes.
//!
//! ### Block propogation
//! When validator creates a block, It will broadcasts the block to peers.
//! Each receiving node verifies
//!     Is the block structure valid?
//!     Does it point to known parent?
//!     Are all transactions valid?
//!     Does executing them produce the claimed state root?
//!     Is the validator/minor authorized?
//!     Does the block satisfies consensus rules?
//!
//! if all valid then node accepts block and forwards it.

use crate::{block::Block, state::StateModel, transaction::Transaction};

pub trait Networking {
    fn p2p_discovery();
    fn gossip();
    fn propogate_block<State: StateModel>(block: &Block<State>);
    fn propogate_transaction(transaction: &Transaction);
}
