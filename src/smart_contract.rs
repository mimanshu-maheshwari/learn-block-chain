//!
//! # Smart Contracts:
//! A smart contract is a self executing program that automates the action required in a blockchain
//! transaction. Once done, these transaction are traceable and can't be undone.
//!
//! Allows for secure transaction and agreements between anonymous partiese
//! without needing a central authority or legal system.
//!
//! Advantages:
//!
//!     1. Efficent
//!     2. Accurate
//!     3. Immutable
//!
//! Disadvantages:
//!
//!     1. Permanent
//!     2. Human factor
//!     3. Loop Holes
//!
//! A smart contract will have
//!
//!     1. data: state variables
//!     2. Functions: what can be done
//!     3. events: messages in and out
//!     4. modifiers: special rules for specific users
//!
//! ## Deployment of smart contracts
//! Smart contract deployment is just a special transaction
//!
//! In ethereum like systems deployment looks like this:
//!     
//!     1. Write contract source code. (Solidity)
//!     2. Compile source code to byte code. (EVM byte code)
//!     3. Create deployment transaction
//!     4. Sign transaction with private key.
//!     5. Broadcast transaction to network.
//!     6. Validator includes transaction in block.
//!     7. Contract byte code is stored on chain.
//!     8. Contract gets an address.
//!     9. User apps interacts with the address.
//!
//! Solidity Code:
//! ```salidity
//! contract Counter {
//!     uint256 public count;
//!
//!     function increment() {
//!         count += 1;
//!     }
//! }
//! ```
//!
//! This gets compiled into byte code.
//! Once included in block the block chain stores the address in new address.
//!
//! After that user can call:
//! ```solidity
//! Counter.increment()
//! ```
//! the contract address becomes permanent identity of that deployed contract.
//!
//! ## Contract address creation
//!
//! In ethereum style systems, contract addresses are derived deterministically.
//!
//! Usually the address depends on
//!
//!     deployer address
//!     deployer nonce
//!
//! There is also a method called CREATE2, where the address can be predicted using:
//!     
//!     deployer address
//!     salt
//!     contract bytecode hash
//!
//! this allows developers to know contract address before deployment

/// Smart contract is a code stored on blockchain
///
/// It can hold assets and define rules.
///
/// Example:
///
///     1. A token contract track balances.
///     2. Lending contract manages deposits and loans
///     3. A market place contract handles buying and selling.
///
/// They are not smart in AI sense. They are in deterministic programs.
/// Given the same input and same state, every node must produce same output.
///
/// Smart contracts can behave randomly unless randomness is provided through
/// a more secure external mechanisim
///
/// They also can't call normal web API's by themselves.
/// That is why blockchain uses oracles.
#[derive(Debug, Default)]
pub struct SmartContract {
    /// ## Contract address creation
    ///
    /// In ethereum style systems, contract addresses are derived deterministically.
    ///
    /// Usually the address depends on
    ///
    ///     deployer address
    ///     deployer nonce
    ///
    /// There is also a method called CREATE2, where the address can be predicted using:
    ///     
    ///     deployer address
    ///     salt
    ///     contract bytecode hash
    ///
    /// this allows developers to know contract address before deployment
    pub contract_address: String,
}
