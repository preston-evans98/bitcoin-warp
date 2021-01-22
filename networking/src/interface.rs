use shared::{u256, Block, BlockHeader, EncapsulatedAddr, Transaction};
use std::collections::HashSet;
/// NetworkRequest provides the inbound interface to the high level 'the rest of the network' abstraction.
pub enum NetworkRequest {
    /// Requests peer information
    Peers,
    /// Requests all blocks with provided hashes
    BlocksByHash(HashSet<u256>),
    // /// Requests a single block when the requester believes itself to be in sync.
    // /// Allows us to use cmpctblck to save bandwidth
    // NextBlock,
    /// Requests all transactions with provided hashes
    TransactionsByHash(HashSet<u256>),
    /// Requests headers starting with the first header in the vec. If max_responses is not provided, the Service will attempt to return every header up to the current tip.
    ///
    /// last_known_headers should be ordered from newest to oldest (i.e. from now toward Genesis block) if it contains more than one item
    Headers {
        last_known_headers: Vec<u256>,
        max_responses: Option<usize>,
    },
    /// Pushes a transaction to the network using unsolicited Tx Message
    PushTransaction(Transaction),
    /// Advertises to peers that each transaction is available using an inv message containing the Tx Hash.
    AdvertiseTransactions(HashSet<u256>),
    /// Advertises to peers that a block is available using an inv message containing its hash
    AdvertiseBlock(HashSet<u256>),
    /// Request a peer's view of the mempool. By default, the Service should aggregate responses from a small subset of peers.
    Mempool,
}

/// NetworkResponse provides the possible responses of the 'rest of the network' abstraction to a ['NetworkRequest'](crate::NetworkRequest)
pub enum NetworkResponse {
    // Returns a list of encapsulated addresses.
    Peers(Vec<EncapsulatedAddr>),
    /// Success announces that the request completed succesfully but did not need to return any data
    /// Used in response to Advertise Requests
    Success,
    /// A list of blocks
    Blocks(Vec<Block>),
    /// A list of Headers
    Headers(Vec<BlockHeader>),
    /// A list of Transactions
    Transactions(Vec<Transaction>),
}
