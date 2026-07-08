//!
//! # Smart Contracts:
//! A smart contract is a self executing program that automates the action required in a blockchain
//! transaction. Once done, these transaction are traceable and can't be undone.
//!
//! Allows for secure transaction and agreements between anonymous parties
//! without needing a central authority or legal system.
//!
//! Advantages:
//!
//!     1. Efficient
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

use crate::{node::Node, oracles::Oracles};

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
/// a more secure external mechanism
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

mod solidity;

/// ### NFT:
/// Non Fungible Tokens
pub struct NFT;

/// ### IPFS:
/// InterPlanetary File System
/// ipfs:// URI points to IPFS content.
///
/// IPFS doesn't automatically guarantee that your file will stay available forever.
/// Someone must keep hosting/pinning the content.
/// Pinning means telling IPFS node or pinning service to keep a copy of the content.
pub struct IPFS {
    /// Content identifier
    /// Fingerprint/address of content
    cid: String,
}
/// A bridge is a system that connects two blockchains so that assets or messages can move between them.
/// They can move tokens, NFTs, messages, state proofs, contract instructions, cross-chain data
///
/// Cross chain bridges are commonly described as applications or protocols that enable transactions
/// and assets transfers between otherwise separate blockchain networks.
///
/// They can solve interoperability problem.
///
/// Assets usually do not literally travel from one chain to another. Instead a bridge does one of
/// these:
///
/// - locks asset on source chain and mints representation on destination chain.
/// - burns representation on destination chain and releases original asset on source chain.
/// - burns asset on source chain and mints asset on destination chain
/// - use liquidity pools to pay user on destination chain.
///
/// ### Basic asset bridge : lock and mint
///
/// This is the easiest bridge model.
/// - Move token from chain A to chain B.
///
/// FLow:
///
///     1. User deposits TOKEN into bridge contract on Chain A.
///     2. Chain A bridge locks TOKEN.
///     3. Chain A bridge emits event.
///     4. Relayer/validator observes event.
///     5. Proof/signature/message is sent to Chain B
///     6. Chain B bridge verifies it.
///     7. Chain B bridge mints wrapped TOKEN to user.
///
/// ### Reverse direction : burn and release
///
///     1. User burns wrapped TOKEN on Chain B.
///     2. Chain B bridge emits burn event.
///     3. Relayer/validator observes burn.
///     4. Proof/message is sent to chain A.
///     5. Chain A bridge verifies it.
///     6. Chain A sort release original TOKEN.
pub struct Bridges {
    /// chain where the action starts
    source_chain: String,
    // chain where the result appears
    destination_chain: String,
    /// smart contract that handles bridge logic
    /// - lock tokens,
    /// - burn tokens,
    /// - emit bridge event,
    /// - record nonce,
    /// - prevent replay,
    source_bridge_contract: SmartContract,
    /// - verify message/proof
    /// - mint wrapped tokens
    /// - release tokens
    /// - execute cross-chain message
    /// - mark message as processed
    destination_bridge_contract: SmartContract,
    /// A relayer watches one chain and submits information to another chain.
    ///
    /// Example:
    ///
    ///     Relayer sees lock event on Chain A.
    ///     Relayer submits proof/message to Chain B.
    ///
    /// Relayers are usually not supposed to be trusted blindly.
    ///
    /// A good bridge design ask:
    /// Can the destination chain verify what the relayer says?
    relayer: String,
    /// some bridges use a group of signers
    /// They observe events on the source chain and sign messages for the destination chain.
    /// The destination bridge contract accepts the message if enough signatures are valid.
    validator: Node,
    oracle: Oracles,
    /// More trust minimized bridges verify cryptographic proofs.
    /// block headers, merkle proofs, state proofs, finality proofs, zero knowledge proofs, light
    /// client proofs
    proof_verifier: String,
    wrapped_token_contract: SmartContract,
    liquidity_pool: String,
    frontend_api: String,
}
