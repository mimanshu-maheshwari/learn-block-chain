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
//!
//! ## Reading from contract :
//! - calling a function that doesn't change state
//!
//! ```solidity
//! function getValue() public view returns (uint) {
//!     return value;
//! }
//! ```
//!
//! Reading usually uses an `eth_call`
//!
//! - doesn't create transaction
//! - doesn't change blockchain state
//! - doesn't require mining/finality
//! - doesn't cost gas for local caller
//!
//! ## Writing to a contract :
//! - calling a function that changes state.
//! ```solidity
//! function setValue(uint _value) public {
//!     value = _value;
//! }
//! ```
//!
//! - Writing requires a signed transaction
//! - costs gas
//! - changes blockchain state
//! - must be included in a block
//! - takes time to finalize
//!
//! - set a value
//! - transfer token
//! - mint NFT
//! - update mapping
//! - deposit ETH/native currency
//! - emit event as part of transaction
//!
//! in Besu QBFT:
//!
//! write transaction → included in block → block finalized immediately after validator agreement
//!
//! # Advance concepts in solidity:
//!
//! ## Inheritance
//! One contract reuse code from another contract
//!
//! ```solidity
//! // SPDX-License-Identifier: MIT
//! pragma solidity ^0.8.20;
//!
//! contract Ownable {
//!     address public user;
//!
//!     constructor() {
//!         owner = msg.sender;
//!     }
//!
//!     modifier onlyOwner() {
//!         require(msg.sender == owner, "Not Owner");
//!         ...
//!     }
//! }
//!
//! contract Vault is Ownable {
//!     uint public value;
//!
//!     function setValue(uint _value) public onlyOwner {
//!         value = _value;
//!     }
//! }
//!
//! ```
//!
//! ## Interfaces
//! ## Abstract contract
//! ## Libraries
//! ## Modifiers
//! - is a reusable pre/post logic for functions
//!
//! Most common use:
//!
//! - access control
//! - input validation
//! - state checks
//!
//! ```solidity
//! contract OwnableExample {
//!     address public owner;
//!
//!     constructor() {
//!         owner = msg.sender;
//!     }
//!     
//!     modifier onlyOwner() {
//!         require(msg.sender == owner, "Not Owner");
//!         _;
//!     }
//!
//!     function adminAction() public onlyOwner {
//!         // only owner can call
//!     }
//! }
//! ```
//!
//! `_` is where the function call get inserted.
//!
//! ## Custom Errors:
//! are gas efficient error types
//!
//! older style:
//!
//! ```solidity
//! require(msg.sender == owner, "Not owner");
//! ```
//!
//! Custom error style:
//! ```solidity
//! error NotOwner(address caller);
//! contract Example {
//!     address public owner;
//!
//!     contract Example {
//!         owner = msg.sender;
//!     }
//!
//!     function adminAction() public {
//!         if (msg.sender != owner) {
//!             revert NotOwner(msg.sender);
//!         }
//!     }
//! }
//! ```
//!
//! - Cheaper than long revert strings
//! - structured error information
//! - clearer debugging
//!
//! Prefer using custom errors when gas matters.
//! Use require strings while learning if readability matters.
//!
//! ### Events and indexed parameters
//! ```solidity
//! event Transfer(address indexed from, address indexed to, uint amount);
//! ```
//!
//! - indexed means the field can be searched/filterable in logs.
//! - off-chain systems can filter logs by indexed values.
//!
//! Example:
//!
//! - Find all Transfer events where from = Alice
//! - Find all Transfer events where to = Bob
//!
//! - Only 3 parameters can be indexed in normal events.
//!
//! besu stores logs inside transaction receipts.
//! Java/Rust services can query logs using `eth_getLogs`.
//!
//! ### Fallback functions
//! The fallback function runs when:
//! - a called function doesn't exist
//! - Or call data does not match any function
//!
//! ```solidity
//! fallback() external payable {
//!     // fallback logic
//! }
//! ```
//!
//! Used for
//!
//!     - receiving unexpected calls
//!     - proxy contracts
//!     - low-level routing
//!     - logging unknown calls
//!
//! Example:
//! ```solidity
//! contract FallbackExample {
//!     event FallbackCalled(address sender, uint value, bytes data);
//!     fallback() external payable {
//!         emit FallbackCalled(msg.sender, msg.value, msg.data);
//!     }
//! }
//! ```
//!
//! ### Receive Function
//! runs when native currency is sent with empty call-data
//! ```solidity
//! receive() external payable {
//!     // receive native currency
//! }
//! ```
//!
//! ### Role:
//! - ADMIN_ROLE
//! - MINTER_ROLE
//! - PAUSER_ROLE
//! - UPGRADER_ROLE
//! - AUDITOR_ROLE
//!
//! ```solidity
//! contract RoleExample {
//!     mapping(bytes32 => mapping(address => bool)) public hasRole;
//!
//!     bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");
//!     bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
//!
//!     modifier onlyRole(bytes32 role) {
//!         require(hasRole[role][msg.sender], "Missing role");
//!         _;
//!     }
//!
//!     constructor() {
//!         hasRole[ADMIN_ROLE][msg.sender] = true;
//!         hasRole[MINTER_ROLE][msg.sender] = true;
//!     }
//!
//!     function grantRole(bytes32 role, address account) external onlyRole(ADMIN_ROLE) {
//!         hasRole[role][account] = true;
//!     }
//!
//!     function mint(address to, uint amount) external onlyRole(MINTER_ROLE) {
//!         // mint logic
//!     }
//! }
//! ```
//!
//! - different organizations need different permissions
//! - enterprise workflows need separation of duties
//! - validators/operators/admins/auditors are separate
//!
//! ### Reentrancy:
//! happens when a contract calls an external contract, and that contract calls back before the
//! first function finishes.
//!
//!
//!```solidity
//!contract VulnerableVault {
//!    mapping(address => uint) public balances;
//!
//!    function deposit() external payable {
//!        balances[msg.sender] += msg.value;
//!    }
//!
//!    function withdraw() external {
//!        uint amount = balances[msg.sender];
//!        require(amount > 0, "No balance");
//!
//!        (bool success, ) = payable(msg.sender).call{value: amount}("");
//!        require(success, "Transfer failed");
//!
//!        balances[msg.sender] = 0;
//!    }
//!}
//!```
//!
//! ### Checks-effects-interactions pattern
//! prevents many reentrancy bugs:
//!
//! - checks : validate conditions
//! - effects : update internal state
//! - interactions : call external contracts/send value
//!
//! ```solidity
//! function withdraw() external {
//!     uint amount = balances[msg.sender];
//!
//!     // checks
//!     require(amount > 0, "No balance");
//!
//!     // effects
//!     balances[msg.sender] = 0;
//!
//!     // interactions
//!     (bool success, ) = payable(msg.sender).call{value: amount}("");
//!     require(success, "Transfer failed");
//! }
//! ```
//!
//! ## Upgradable contracts concept
//! Upgradable contracts solve this by separating:
//!
//!     storage
//!     logic
//!
//! High level idea:
//!
//! - Proxy contract = stable address and storage
//! - Implementation contract = logic/code
//!
//! Users interact with proxy:
//!
//! - User → Proxy → delegate call → implementation
//!
//! When upgrading
//!
//! - Proxy keeps same address and storage
//! - Admin changes implementation address
//! - new Logic is used
//!
//! Important risk
//!
//! - Upgradability adds centralization and security risk
//! - bad upgrade can steal funds and corrupt storage
//!
//! - Upgradeable contracts can be useful in enterprise/private Besu networks, but governance around
//!   upgrades must be very clear.
//!
//! ### Proxy pattern concept
//! A proxy is a contract that forward calls to another contract.
//!
//! Core idea:
//!
//! - Proxy receives call
//! - Proxy uses delegatecall to implementation
//! - Implementation code runs using proxy storage
//!
//! ```solidity
//! fallback() external payable {
//!     address impl = implementation
//!
//!     assembly {
//!         calldatacopy(0, 0, calldatasize())
//!
//!         let result := delegatecall(
//!             gas(),
//!             impl,
//!             0,
//!             calldatasize(),
//!             0,
//!             0
//!         )
//!
//!         returndatacopy(0, 0, returndatasize())
//!
//!         switch result
//!         case 0 { revert(0, returndatasize()) }
//!         default { return(0, returndatasize()) }
//!     }
//! }
//! ```
//!
//! - Proxy has address and storage.
//! - Implementation has logic.
//! - delegatecall connects them.
//!
//! - Proxy has address and storage.
//! - Implementation has logic.
//! - delegatecall connects them
//!
//! common proxy types:
//!
//! - Transparent proxy
//! - UUPS proxy
//! - Beacon proxy
//! - Diamond proxy
//!
//!

///
/// # Solidity
/// Complete beginner contract :
///
/// ```solidity
/// // SPDX-License-Identifier: MIT
/// pragma solidity ^0.8.20
///
/// contract SimpleStorage {
///     address public owner;
///     uint public value;
///
///     event valueChanged(address indexed changedBy, uint oldValue, uint newValue);
///
///     constructor(uint _initialValue) {
///         owner = msg.sender;
///         value = _initialValue;
///     }
///
///     function setValue(uint _newValue) public {
///         require(msg.sender == owner, "Only owner can set value");
///         uint oldValue = value;
///         value = _newValue;
///         
///         emit valueChanged(msg.sender, oldValue, _newValue);
///     }
///
///     function getValue() public view returns (uint) {
///         return value;
///     }
///
///     function add(uint a, uint b) public pure returns (uint) {
///         return a + b;
///     }
/// }
/// ```
///
/// Payable example
/// ```solidity
/// // SPDX-License-Identifier: MIT
/// pragma solidity ^0.8.20;
///
/// contract SimpleVault {
///     mapping(address => uint) public balances;
///
///     event Deposited(address indexed user, uint amount);
///
///     function deposit() public payable {
///         require(msg.value > 0, "No value sent");
///         balances[msg.sender] += msg.value;
///         emit Deposited(msg.sender, msg.value);
///     }
///
///     function getMyBalance() public view returns (uint) {
///         return balances[msg.sender];
///     }
/// }
/// ```
///
/// ```solidity
/// // SPDX-License-Identifier: MIT
/// pragma solidity ^0.8.20;
///
/// interface IVault {
///     function deposit() external payable;
///     function withdraw(uint amount) external;
/// }
///
/// library AmountLib {
///     function isValid(uint amount) internal pure returns (bool) {
///         return amount > 0;
///     }
/// }
///
/// abstract contract BaseVault {
///     event Deposited(address indexed user, uint amount);
///     event Withdrawn(address indexed user, uint amount);
/// }
///
/// contract Vault is IVault, BaseVault {
///     using AmountLib for uint;
///
///     address public owner;
///     mapping(address => uint) public balances;
///
///     error NotOwner();
///     error InvalidAmount();
///     error InsufficientBalance();
///
///     modifier onlyOwner() {
///         if (msg.sender != owner) revert NotOwner();
///         _;
///     }
///
///     constructor() {
///         owner = msg.sender;
///     }
///
///     function deposit() external payable {
///         if (!msg.value.isValid()) revert InvalidAmount();
///
///         balances[msg.sender] += msg.value;
///
///         emit Deposited(msg.sender, msg.value);
///     }
///
///     function withdraw(uint amount) external {
///         if (!amount.isValid()) revert InvalidAmount();
///         if (balances[msg.sender] < amount) revert InsufficientBalance();
///
///         // effects first
///         balances[msg.sender] -= amount;
///
///         // interaction last
///         (bool success, ) = payable(msg.sender).call{value: amount}("");
///         require(success, "Native transfer failed");
///
///         emit Withdrawn(msg.sender, amount);
///     }
/// }
/// ```
#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct Solidity {
    /// - adding numbers costs gas
    /// - reading storage costs gas
    /// - writing storage costs more gas
    /// - deploying contract bytecode costs gas
    /// - calling another contract costs gas
    /// - emitting events costs gas
    gas: f64,
}
