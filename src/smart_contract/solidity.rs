//! # Solidity
//!
//! Solidity is a programming language used to write smart contracts for the Ethereum Virtual
//! Machine or EVM.
//!
//! ```solidity
//! contract Counter {
//!     uint public count;
//!
//!     function increment() public {
//!         count++;
//!     }
//! }
//! ```
//! the contract can be deployed to Ethereum, Besu private networks, polygon, BSC, Avalanche
//! C-Chain, other EVM-compatible chains.
//!
//! It has state, function, address, bytecode, rules.
//!
//! Example:
//!
//! ```solidity
//! contract Bank {
//!     
//!     mapping(address => uint) public balances;
//!
//!     function deposit() public payable {
//!         balances[msg.sender] += msg.value;
//!     }
//! }
//! ```
//!
//! Once deployed, the contract gets its own blockchain address.
//! ```text
//! oxAbC123
//! ```
//!
//! In Besu:
//!
//!     User sends transaction
//!     Besu node receives it
//!     validators include it in block
//!     EVM executes contract code
//!
//! ### Solidity file structure
//!
//! A basic solidity file usually hash:
//!
//! ```solidity
//! // SPDX-License-Identifier: MIT
//! pragma solidity ^0.8.20;
//!
//! contract MyContract {
//!     // state variables
//!
//!     constructor() {
//!         // constructor logic
//!     }
//!
//!     function myFunction() public {
//!         // function logic
//!     }
//!
//! }
//! ```
//!
//! Common Structure:
//! - SPDX license
//! - pragma version
//! - imports
//! - contract declaration
//! - state variables
//! - events
//! - modifiers
//! - constructor
//! - functions
//!
//! ```solidity
//! // SPDX-License-Identifier: MIT
//! pragma solidity ^0.8.20
//!
//! contract SimpleStorage {
//!
//!     uint public value;
//!     
//!     event ValueChanged(uint newValue);
//!
//!     constructor(uint _initialValue) {
//!         value = _initialValue;
//!     }
//!
//!     function setValue(uint _value) public {
//!         value = _value;
//!         emit ValueChanged(_value);
//!     }
//!
//!     function getValue() public view returns (uint) {
//!         return value;
//!     }
//! }
//! ```
//!
//! ### SPDX license
//! Tells tools and used users what license the source code uses
//!
//! Common licenses are MIT, GPL-3.0, Apache-2.0, UNLICENSED
//!
//! It matters as it provides:
//!
//! - source-code clarity
//! - legal clarity
//! - compiler warning avoidance
//! - block explorer verification
//!
//! ### pragma solidity
//! tells compiler which solidity version can compile the file.
//!
//! - different solidity versions can have different rules
//! - compiler bugs differ by version
//! - security behavior can change
//!
//! ### Contract declaration
//! using `contract` keyword.
//! class like unit deployed on chain
//!
//! - A deployed contract has an address
//! - A deployed contract stores state on-chain
//! - functions may cost gas
//! - state changes require transactions
//!
//! ### State variables
//!
//! - are variables stored permanently in contract storage
//! - It is stored on chain
//! - State variables are expensive compared to local variables because they live in blockchain storage.
//!
//! ### Local variables
//!
//! - exist only inside a function while it is executing
//! ```solidity
//! function add(uint a, uint b) public pure returns (uint) {
//!     uint sum = a + b;
//!     return sum;
//! }
//! ```
//!
//! Here :
//!
//!     a = function parameter
//!     b = function parameter
//!     sum = local variable
//!
//! They are not permanently stored on chain unless assigned to a state variable.
//!
//! ### Value types:
//!
//!     uint  | uint256
//!     int  
//!     bool
//!     address
//!     bytes
//!     bytes32
//!
//! ### Reference types:
//!
//!     string
//!     arrays
//!     structs
//!     mappings
//!
//! ### Functions:
//!
//! ```text
//! function functionName(type parameters) visibility mutability returns (returnTypes) {
//!     // logic
//! }
//! ```
//!
//! Example:
//! ```text
//! function add(uint a, uint b) public pure returns (uint) {
//!     return a + b;
//! }
//! ```
//!
//! functions can :
//!
//! - read state
//! - write state
//! - receive ETH/native currency
//! - emit events
//! - call other contracts
//! - return values
//!
//! Visibility can be :
//!
//! - public   : Can be called inside outside contract
//! - private  : Can only be inside the same contract; doesn't mean hidden from blockchain
//!   observers; only means other contracts can't directly call it; On chain data can still be
//!   inspected.
//! - internal : Can be used inside the same contract and child contracts.
//! - external : Can only be called outside the contract; often used for user-facing functions.
//!
//!
//! State mutability:
//!
//! - view    : reads state but doesn't modify it; read (state variable, block data, msg.sender);
//! - pure    : Doesn't read or write state;
//! - payable : can receive native currency; on Ethereum native is ETH; on Besu private network, it
//!   is the native currency of that chain;
//!
//! Without payable, a function can't receive native currency.
//!
//!
//! ### Constructor:
//!
//! - Runs only once when the contract is deployed.
//! - not callable after deployment
//! - used for initial setup
//!
//! Common use cases:
//!
//! - set owner
//! - set initial supply
//! - set token name
//! - set config values
//!
//! ### `msg.sender`
//! - is the address that called the current function
//! ```solidity
//! address public owner;
//!
//! constructor() {
//!     owner = msg.sender;
//! }
//! ```
//! - If the user calls the contract directly : user address
//! - if another contract calls this contract : calling contract's address
//!
//! ```solidity
//! function onlyOwnerAction() public {
//!     require(msg.sender == owner, "Not owner");
//! }
//! ```
//!
//! ### `msg.value`
//! - is the amount of native currency sent with the transaction.
//! ```solidity
//! function deposit() public payable {
//!     balances[msg.sender] += msg.value;
//! }
//! ```
//!
//! if user sends 1ETH
//!
//! msg.value = 1 ether
//!
//! in solidity
//!
//! ```solidity
//! require(msg.value > 0, "Send some value");
//! ```
//!
//! Important
//!
//! msg.value only exists meaningfully in payable functions
//!
//! ### `require`
//! checks a condition and reverts if false
//!
//! if condition is false:
//!
//! - transaction reverts
//! - state changes are undone
//! - error message is returned
//!
//! Common use cases:
//!
//! - access control
//! - input validation
//! - balance checks
//! - state checks
//!
//! ### revert
//! - manually stops execution and reverts state changes.
//! ```solidity
//! function withdraw(uint amount) public {
//!     if (amount == 0) {
//!         revert("Amount can't be zero");
//!     }
//! }
//! ```
//!
//! require is basically a convenient way to do:
//!
//! ```solidity
//! if (!condition) {
//!     revert("error");
//! }
//! ```
//!
//! Modern solidity often uses custom errors for gas savings:
//!
//! ```solidity
//! error NotOwner();
//!
//! function onlyOwnerAction() public {
//!     if (msg.sender != owner) {
//!         revert NotOwner();
//!     }
//! }
//! ```
//!
//!
//! require = common validation shortcut
//! revert = explicit failure
//!
//! ### assert
//! checks conditions that should never be false
//!
//! ```solidity
//! assert(totalSupply >= balances[msg.sender]);
//! ```
//!
//! - use assert for internal invariants.
//! - did not use assert for normal input validation
//!
//! require -> user/input/business rule checks
//! assert -> internal impossible-state checks
//!
//!
//! ### Events
//!
//! ```solidity
//! event ValueChanged(address indexed user, uint newValue);
//! ```
//!
//!
//! events are useful:
//!
//! - frontend/app can listen to them
//! - off-chain services can index them
//! - they create transaction logs
//! - cheaper than storing unnecessary history
//!
//! ```solidity
//! contract Counter {
//!     uint public count;
//!
//!     event Incremented(address indexed user, uint newCount);
//!
//!     function increment() public {
//!         count++;
//!         emit Incremented(msg.sender, count);
//!     }
//! }
//! ```
//!
//! - Events are not readable by smart contracts directly.
//! - They are mainly for off-chain systems.
//!
//! In Besu:
//! - Besu stores event logs in transaction receipts.
//! - Apps can query then using JSON-RPC.
//!
//!
//! ## Deployment
//! - Solidity source code
//! - compile
//! - ABI + bytecode
//! - send deployment transaction
//! - Besu/EVM executes constructor
//! - contract address created
//!
//!
//! Important output from compilation
//!
//! - ABI : means application binary interface; tells client how to call contract functions;
//!   includes functions name, inputs, outputs, events, errors
//! - Bytecode : is the low-level EVM code deployed to the blockchain; contains contract bytecode,
//!   constructor arguments, gas, sender signature

/// Solidity
#[derive(Debug, Default)]
pub struct Solidity;
