//! ## Consensus
//!
//! consensus means agreement
//!
//! Consensus is the rule system that lets distributed node agree on canonical chain.
//! The core questions are:
//!
//!     Who gets to produce the next block?
//!     How do others verify it?
//!     What happens if two blocks conflict?
//!     What is the transaction final?
//!
//! Major Consensus families:
//!
//!     Proof of work
//!     Proof of Stake
//!     BFT style consensus
//!     Proof of authority
//!
//!
//! ### Forks and finality
//! Sometimes two valid blocks appear at the same height
//! ```text
//! Block 100
//!     Block 101A
//!     Block 101B
//! ```
//!
//! this can happen because of network delay.
//!
//! Different ndoes will see different blocks first.
//!
//! Eventually protocol chooses one branch
//!
//! In proof of work the chain with most accumulated work usually wins
//! In proof of stake/BFT system, the validators vote accordingly to protocol rules
//!

pub trait Consensus {}

/// Used by bitcoin
///
/// Minors compete to solve a computational puzzles
/// The puzzle is roughly:
///     Find a block hash below a target value
///
/// This requiers repeated hashing
///
/// The winner gets to propose a next block
///
/// Security comes from the cost of computation
///
/// To attack the network the attacker need enormous hashing power
///
/// Advantages
///     
///     1. Simple security model
///     2. Battle tested.
///     3. No need to identify validators
///
/// Disadvantages:
///     1. Energy intensive
///     2. Lower throughput
///     3. Probablistic finality
///
#[derive(Debug, Default)]
pub struct ProofOfWork;

/// Used by many mordern chain
///
/// Validators lock up, or "stake" tokens
///
/// The protocol selects validators to propose or vote on blocks.
///
/// if validators behave honestly, they earn rewards.
///
/// if they behave maliciously, they can loose stake.
/// the penalty is called slashing
///
/// Advantages:
///
///     1. Energy efficient
///     2. Can support faster finality
///     3. Economic penalties for bad behavious
///
/// Disadvantages:
///     
///     1. More complex
///     2. Wealth/stake concentration concerns
///     3. Validator coordination complexity
#[derive(Debug, Default)]
pub struct ProofOfStake;

/// Byzantine Fault Tolerant
///
/// It deals with the problem of nodes being faulty or malicious
///
///
/// BFT system ususally involves validator voting through multiple rounds.
///
/// they can provide strong finality
///
/// A simplified model is:
///     
///     Propose block
///     validator vote
///     If enough votes are collected block is finalized
///
/// > Often, “enough” means more than two-thirds of voting power.
///
/// Advantages:
///
///     1. Fast finality
///     2. Good for permissioned or validator based system
///
/// Disadvantages:
///     1. Usually harder to scale to huge validator set.
///     2. More communication overhead
#[derive(Debug, Default)]
pub struct BFT;

/// Used in private or Consortium chain
/// Known validators produce blocks
///
/// Validators are trusted authorities rather than minors/stakers.
///
/// Advantages:
///
///     1. Fast
///     2. Cheap
///     3. Simple
///     4. Good for private/enterprice network
///
/// Disadvantages:
///
///     1. Less decentralized
///     2. Trust assumptions are stronger
///
#[derive(Debug, Default)]
pub struct ProofOfAuthority;

/// _**Finality Means**_ : How sure are we that the transactions will not be reversed?
/// There are two broad types
///
#[derive(Debug, Default)]
pub enum Finality {
    /// Deterministic/economic finality
    /// BFT style finality
    /// it is much stronger. Once finalized reverting would require sever protocol violations
    #[default]
    Deterministic,
    /// Bitcoin is probablistic finality. The more blocks after your transactions the harder it is
    /// to reverse
    Probablistic,
}

/// Block proposer
/// is the participant selected to create teh enxt block.
///
/// the propser chooses which transaction to include
/// transaction order
/// block metadata
/// parent block
pub struct BlockProposer;

/// Fork happens when the blockchain temporarily or permanently splits into different branches.
///
/// This can happen when
/// - Two minors find block at nearly same time.
/// - Network delay causes nodes to see different block first
/// - validators disagree
/// - software rules change
/// - malicious actor create conflicting blocks
///
/// There are two broad types:
///
///     1. temporary fork; happens because of timing; eventually network chooses one batch;
///     2. Protocol fork; happens when blockchain rules change;
///
///         2.1. Soft fork
///         2.2. Hard fork
pub struct Fork;
