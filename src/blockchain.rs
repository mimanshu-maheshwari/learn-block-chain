//! ## What is blockchain?
//! At simplest, it is a linked list of blocks
//! Genesis block -> block1 -> block2 -> block3 ... -> blockn
//! The first block is called genesis block
//!
//! Some time two valid blocks are generated on same time.
//! This is called as fork
//! Consensus decides which branch becomes canonical
//!
//! It can be viewed as a state machine
//! Old state + Transactions = New state
//!
//! *Blockchain is a replicated state machine where nodes agree on the order of transactions*
//! ## Defination:
//! Blockchain is a decentralized Computation and information sharing platform that enables
//! multiple authoritative domains who do not trust each other, to co-operate, co-ordinate and
//! collaborate in rational decision making
//!
//! ## Deployment of whole blockchain
//! Deploying a blockchain means launching your own chain and this involved:
//!
//!     1. Choosing consensus
//!     2. creating genesis block
//!     3. configuring chain id
//!     4. setting initial accounts/balances
//!     5. setting validator set
//!     6. Running bootnodes
//!     7. Running RPC nodes
//!     8. Running validator nodes
//!     9. setting block time and gas rules
//!     10. setting networking config
//!     11. monitoring nodes
//!     12. providing explorers and wallets
//!
//! A simple private block chain setup:
//!
//!     1. define chain config
//!     2. create genesis file
//!     3. start bootnode
//!     4. start validator nodes
//!     5. start rpc node
//!     6. connect peers
//!     7. deploy contracts
//!     8. build apps on top
//!

#[derive(Debug, Default)]
pub struct Blockchain {}
