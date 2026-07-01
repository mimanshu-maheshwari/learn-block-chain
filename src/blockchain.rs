//! ## What is blockchain?
//! At simplest, it is a linked list of blocks
//!
//! Genesis block -> block1 -> block2 -> block3 ... -> blockn
//!
//! The first block is called genesis block
//!
//! Some time two valid blocks are generated on same time.
//! This is called as fork
//! Consensus decides which branch becomes canonical
//!
//! It can be viewed as a state machine
//!
//! Old state + Transactions = New state
//!
//! *Blockchain is a replicated state machine where nodes agree on the order of transactions*
//!
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
//! # Layer 1 Blockchain:
//! Layer 1 is the base blockchain.
//! Examples:
//!
//!     1. Bitcoin
//!     2. Ethereum
//!     3. Solana
//!     4. Avalanche
//!     5. Cosmos Chain
//!
//! # Layer 2 Blockchain
//! are scailing system built on top of layer 1 blockchain.
//!
//! Examples:
//!
//!     1. Rollups
//!     2. State channels
//!     3. Plasma-style systems
//!     4. Sidechains
//!     5. Validiums
//!
//! Main goal is:
//!
//!     1. More throughput
//!     2. Lower fees
//!     3. Still inherit some security from L1

#[derive(Debug, Default)]
pub struct Blockchain {}

/// Types of block chain
///
/// Private/public blockchain means who can see/read the ledger.
///
/// Permissioned/permissionless means who can join, submit, or validate
///
#[derive(Debug, Default)]
pub enum BlockchainType {
    #[default]
    /// public read, private validator/executions
    Hybrid,

    /// Layer 0:
    /// Infrastructure for connecting blockchains.
    /// Example: Cosmos IBC, Polkadot-style interoperability, cross-chain messaging.
    L0,

    /// Layer 1:
    /// The base blockchain.
    /// Example: Bitcoin, Ethereum, Solana.
    /// Function: consensus, security, settlement, state.
    ///Layer 1 was not enough for mass adoption because it is expensive and limited in throughput.
    L1,

    /// Layer 2:
    /// Scaling layer on top of Layer 1.
    /// Example: Arbitrum, Optimism, Base, zkSync.
    /// Function: cheaper and faster transactions.
    /// Layer 2 was created to scale Layer 1 by processing transactions off-chain or on a separate rollup, then settling/proving results back to L1.
    L2,

    /// Layer 3:
    /// Application-specific layer on top of Layer 2.
    /// Example: gaming rollup, DeFi appchain, custom enterprise chain.
    /// Function: customization, app-specific scaling, custom rules.
    /// Layer 3 was created because some apps need more than scaling — they need their own custom rules, gas token, privacy model, governance, and dedicated block space.
    L3,

    /// small chains connected through bridge for cheaper executions
    /// not as secure as main chain
    Sidechain,

    /// app specific validators
    AppChain,

    Permissionless(PermissionlessKind),

    /// Only provided participants can validate or write certain data.
    /// This is common in enterprise settings.
    /// permissioned chains optimize for control, performance, compliance, and known participants.
    Permissioned(PermissionedKind),
}

#[derive(Debug, Default)]
pub enum PermissionlessKind {
    /// Anyone can read public blockchain
    ///
    /// Anyone can :
    ///
    ///     1. read
    ///     2. send transactions
    ///     3. Run a node
    ///     4. Participate according to protocol rules.
    ///
    /// Examples:
    ///
    ///     1. Bitcoin
    ///     2. Ethereum
    ///
    /// Public blockchain optimize for open participation and censorship resistance.
    ///
    /// Use cases:
    ///
    ///     1. Open money
    ///     2. Public asset ownership
    ///     3. DeFi
    ///     4. NFTs
    ///     5. public smart contracts
    ///     6. censorship resistance
    ///     7. global settlement
    ///
    /// Advantages:
    ///
    ///     1. Highly Open
    ///     2. Strong transparency
    ///     3. High cencorship resistance
    ///     4. Large public network effects
    ///     5. Anyone can verify state
    ///
    /// Disadvantages:
    ///
    ///     1. Low privacy by default.
    ///     2. Fees can fluctuate.
    ///     3. Performance is limited compared to centralized databases.
    ///     4. Governance can be slow.
    ///     5. Data is visible to everyone.
    #[default]
    Public,
}

#[derive(Debug, Default)]
pub enum PermissionedKind {
    /// Anyone can read the ledger.
    /// Anyone can observe the system.
    /// But only approved entities can validate or produce blocks.
    ///
    /// public visibility + controlled validators.
    ///
    ///     1. regulated asset network
    ///     2. public audit ledgers
    ///     3. government transparency systems
    ///     4. industry networks where public visibility is useful
    ///
    /// Advantages:
    ///
    ///     1. More transparent than private chain
    ///     2. Usually faster than fully permissionless chains
    ///     3. Validator accountability is easier.
    ///     4. Governance is more controlled.
    ///
    /// Disadvantages:
    ///
    ///     1. Less decentralized
    ///     2. Validator cencorship is easier
    ///     3. Users must trust the validator governance model.
    ///
    #[default]
    PublicPermissioned,

    /// Access is restricted usually inside companies or consortiums.
    /// Private chains optimize for control, performance, compliance, and known participants.
    /// This is common in enterprice
    /// Examples of technologies used for this style are:
    ///
    ///     1. Hyperledger Besu
    ///     2. Hyperledger Fabric
    ///     3. Quorum-style networks
    ///     4. Corda like enterprice ledgers
    ///
    /// Here only approved,
    ///
    ///     1. users can access the networks.
    ///     2. nodes can connect.
    ///     3. accounts may transact.
    ///     4. validators can produce blocks.
    ///
    /// Example use cases:
    ///
    ///     1. bank settlement networks
    ///     2. supply chain tracking.
    ///     3. trade finance.
    ///     4. inter-company reconciliation.
    ///     5. enterprise asset tokenization
    ///     6. private consortium ledgers
    ///
    /// Advantages:
    ///
    ///     1. Higher privacy
    ///     2. Higher throughput
    ///     3. lower transaction cost
    ///     4. Known participants
    ///     5. Easier compliance
    ///     6. Custom governance
    ///
    /// Disadvantages:
    ///
    ///     1. less decentralized
    ///     2. More trust in administrators
    ///     3. smaller validator set
    ///     4. weaker censorship resistance
    ///     5. may not need blockchain if one central database is enough
    ///
    PrivatePermissioned,

    /// ## Consortium or federated blockchain
    /// A consortium blockchain is a specific kind of blockchain where multiple organizations
    /// jointly operate the network.
    ///
    /// Example:
    ///
    ///     1. 5 banks create a settlement network.
    ///     2. each bank runs validator nodes.
    ///     3. rules are agreed by the consortium.
    ///
    /// The validator set may look like:
    ///
    ///     Bank A validator
    ///     Bank B validator
    ///     Bank C validator
    ///     Bank D validator
    ///     Regulator observer node
    ///
    /// Good for :
    ///
    ///     1. Banking
    ///     2. insurance
    ///     3. logistics
    ///     4. suply chain
    ///     5. healthcare records
    ///     6. government-industry systems
    ///
    /// Advantages:
    ///
    ///     1. Shared governance
    ///     2. Better trust distribution than one central database
    ///     3. more privacy than public blockchain
    ///     4. known validator identities.
    ///
    /// Disadvantages:
    ///
    ///     1. Governance can become political
    ///     2. Adding/removing member requires process.
    ///     3. Users must trust the consortium rules
    ///     4. not as open as public networks.
    Consortium,
}
