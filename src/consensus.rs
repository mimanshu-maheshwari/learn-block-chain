//! ## Consensus
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
#[derive(Debug, Default)]
pub struct BFT;
