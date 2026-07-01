//! # Oracles:
//! Blockchain can't directly know external facts.
//!
//! A smart contract can't automatically know:
//!
//!     1. Current ETH/USD rate.
//!     2. Weather in Delhi
//!     3. Football match result.
//!     4. Bank account balance.
//!     5. Random number from real world.
//!
//! An Oracle brings external data on chain.
//!
//! An oracle design is very important as a bad oracle can break DeFi (Decentralized Finance) protocol

#[derive(Debug, Default)]
pub struct Oracles;
