//! To use test net to test out sending/testing wallet.
//!
//! Know which networks are available
//! how to get testnet ETH to play around with.
//! not recommended to reuse main-net accounts on test-nets
//!
//! ETH on test-nets is supposed to have no real value;
//! need ETH to actually interact with ethereum (event on test-nets), most people get testnet ETH for free from faucets.
//! most faucets are web-apps where you can input an address which you request ETH to be sent to.
//!
//! Two public test-nets that client developers are currently maintaining
//!
//!     1. Sepolia
//!     2. Hoodi
//!
//! Gas computation:
//!     
//!     1. 1 GWEI = 1_000_000_000 WEI
//!     1. 1 ETH = 1_000_000_000 GWEI

/// Sepolia is the recommended default test-net for application development.
///
/// The sepolia network uses permissioned validator set controlled by client & testing teams
pub struct Sepolia;

/// testnet for testing validating and staking.
///
/// Test protocol upgrades
pub struct Hoodi;
