use crate::transaction::Transaction;

/// There are two major state models
///     1. UTXO
///     2. Account
/// Bitcoin uses UTXO model
/// Etherium uses account model
pub trait StateModel {
    type Transaction;
    type State;
}

/// UTXO state model
/// (Unspent transaction output)
///
/// Explanation:
///
/// Imagine money as physical coins
///
///     If Alice has 10 BTC, it may be represented as multiple unspent outputs:
///     ```
///     UTXO 1 = 3 BTC
///     UTXO 2 = 7 BTC
///     ```
///
///     if Alice wants to send 5 bitcoins to bob she sends the 7 BTC UTXO i.e. UTXO 2
///
///     ```
///     input
///         7 BTC UTXO from alice
///     output:
///         5 BTC to bob
///         2 BTC change back to alice
///     ```
///
/// So bitcouin doeesn't store the output directly but only the unspent output
///
/// Alice balance will be calculated summing all the UTXOs she can spend
/// Advantages:
///     1. Simple validation
///     2. Good Parallelism
///     3. Clear transaction history
///     4. Strong fit for payments
///
/// Disadvantages:
///     1. harder for complex smart contracts
///     2. State is less intuitive for app developers
#[derive(Debug, Default)]
pub struct UTXOState {}
impl StateModel for UTXOState {
    type Transaction = Transaction;
    type State = Self;
}

#[derive(Debug, Default)]
pub enum AccountStateType {
    /// Controlled by a private key
    #[default]
    ExtenrallyOwnedAccount,
    /// Smart contract code
    ContractAccount {
        // TODO: what is type?
        /// example ERC20 byte code
        code: Vec<u8>,
        // TODO: what is type?
        /// Token balance, allowance, etc
        storage: u32,
    },
}

/// Etherium state is like big key value database
/// address -> account data
/// Advantages :
///     1. Natural for smart contracts
///     2. Easy to understand balances
///     3. Good for application development
/// Disadvantages:
///     1. Global state can become large
///     2. Parallel execution is harder
///     3. More complex state management
///
/// ### Trie like data structures in Etherium-like blockchains
///
#[derive(Debug, Default)]
pub struct AccountState {
    balance: f64,
    nonce: u32,
    kind: AccountStateType,
}

impl StateModel for AccountState {
    type Transaction = Transaction;
    type State = Self;
}
