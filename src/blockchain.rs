/// At simplest, it is a linked list of blocks
/// Genesis block -> block1 -> block2 -> block3 ... -> blockn
/// The first block is called genesis block
///
/// Some time two valid blocks are generated on same time.
/// This is called as fork
/// Consensus decides which branch becomes canonical
///
/// It can be viewed as a state machine
/// Old state + Transactions = New state
///
/// *Blockchain is a replicated state machine where nodes agree on the order of transactions*
#[derive(Debug)]
pub struct Blockchain {}
impl Blockchain {}
