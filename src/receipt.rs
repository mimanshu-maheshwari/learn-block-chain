//! When transaction executes it generates a receipt.

#[derive(Debug, Default)]
pub struct Receipt {
    /// success or failure
    pub status: String,
    pub gas_useed: u32,
    // TODO: what is the type of event?
    /// logs or events emitted
    ///
    /// When a contract emmit event then logs are generated in receipt
    ///
    /// Apps use logs to track block chain activity
    ///
    /// for example a wallet may not scan every contract storage slot. Instead it watches Transfer
    /// events
    ///
    /// Some block chains uses bloom filters to check if logs exists or not in a block
    /// it can say
    ///     1. definately not present
    ///     2. maybe present
    pub event_logs: Vec<String>,
    pub contract_address_created: String,
}
