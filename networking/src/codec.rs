use crate::Message;
use bytes::{BufMut, BytesMut};
// // impl
use crate::block_header::BlockHeader;
use crate::header::Header;
use shared::CompactInt;
use shared::Serializable;

pub struct Codec {
    magic: u32,
}

impl tokio_util::codec::Encoder<Message> for Codec {
    type Error = std::io::Error;

    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let cmd: &[u8; 12] = match item {
            Message::Version { .. } => b"version\0\0\0\0\0",
            Message::Verack { .. } => b"verack\0\0\0\0\0\0",
            Message::GetBlocks { .. } => b"getblocks\0\0\0",
            Message::GetData { .. } => b"getdata\0\0\0\0\0",
            Message::Block { .. } => b"block\0\0\0\0\0\0\0",
            Message::GetHeaders { .. } => b"getheaders\0\0",
            Message::BlockTxn { .. } => b"blocktxn\0\0\0\0",
            Message::CompactBlock { .. } => b"cmpctblock\0\0",
            Message::Headers { .. } => b"headers\0\0\0\0\0",
            Message::Inv { .. } => b"inv\0\0\0\0\0\0\0\0\0",
            Message::MemPool { .. } => b"mempool\0\0\0\0\0",
            Message::MerkleBlock { .. } => b"merkleblock\0",
            Message::SendCompact { .. } => b"sendcmpct\0\0\0",
            Message::GetBlockTxn { .. } => b"getblocktxn\0",
            Message::NotFound { .. } => b"notfound\0\0\0\0",
            Message::Tx { .. } => b"tx\0\0\0\0\0\0\0\0\0\0",
            Message::Addr { .. } => b"addr\0\0\0\0\0\0\0\0",
            // Message::Alert { .. } => b"alert\0\0\0\0\0\0\0",
            Message::FeeFilter { .. } => b"feefilter\0\0\0",
            Message::FilterAdd { .. } => b"filteradd\0\0\0",
            Message::FilterClear { .. } => b"filterclear\0",
            Message::FilterLoad { .. } => b"filterload\0\0",
            Message::GetAddr { .. } => b"getaddr\0\0\0\0\0",
            Message::Ping { .. } => b"ping\0\0\0\0\0\0\0\0",
            Message::Pong { .. } => b"pong\0\0\0\0\0\0\0\0",
            Message::Reject { .. } => b"reject\0\0\0\0\0\0",
            Message::SendHeaders { .. } => b"sendheaders\0",
        };

        // Preallocate vector
        let payload_size = self.get_serialized_size(&item);
        dst.reserve(payload_size + Header::len());
        let initial_offset = dst.len();
        let mut target = dst.writer();

        // write known header fields and zero checksum
        self.magic.serialize(&mut target)?;
        cmd.serialize(&mut target)?;
        (payload_size as u32).serialize(&mut target)?;
        let start_checksum = initial_offset + 20;
        let start_payload = start_checksum + 4;
        [0u8; 4].serialize(&mut target)?;

        // Serialize the message. Some messages will likely need non-default serialization
        match item {
            _ => item.serialize(&mut target)?,
        }

        // Fill in the checksum
        let checksum = warp_crypto::sha256d(&dst[start_payload..(start_payload + payload_size)]);
        dst[start_checksum..start_payload].copy_from_slice(&checksum[0..4]);
        Ok(())
    }
}
// impl Decoder for Codec {}

impl Codec {
    fn get_serialized_size(&self, msg: &Message) -> usize {
        match msg {
            Message::Addr { ref addrs } => {
                CompactInt::size(addrs.len()) + addrs.len() * (4 + 8 + 18)
            }
            Message::BlockTxn {
                block_hash: _,
                ref txs,
            } => {
                32 + CompactInt::size(txs.len()) + txs.iter().fold(0, |total, tx| total + tx.len())
            }
            Message::Block {
                block_header: _,
                ref transactions,
            } => {
                BlockHeader::len()
                    + CompactInt::size(transactions.len())
                    + transactions.iter().fold(0, |total, tx| total + tx.len())
            }
            Message::CompactBlock {
                header: _,
                nonce: _,
                ref short_ids,
                ref prefilled_txns,
            } => {
                BlockHeader::len()
                    + 8
                    + CompactInt::size(short_ids.len())
                    + 8 * short_ids.len()
                    + CompactInt::size(prefilled_txns.len())
                    + prefilled_txns.iter().fold(0, |total, pre_tx| {
                        let txn_len = pre_tx.tx.len();
                        total + txn_len + CompactInt::size(txn_len)
                    })
            }
            Message::FeeFilter { .. } => 8,
            Message::FilterAdd { ref elements } => {
                elements.iter().fold(0, |total, elt| {
                    total + elt.len() + CompactInt::size(elt.len())
                }) + CompactInt::size(elements.len())
            }
            Message::FilterClear {} => 0,
            Message::FilterLoad {
                filter,
                n_hash_funcs: _,
                n_tweak: _,
                n_flags: _,
            } => CompactInt::size(filter.len()) + filter.len() + 4 + 4 + 1,
            Message::GetAddr {} => 0,
            Message::GetBlockTxn {
                block_hash: _,
                ref indexes,
            } => {
                32 + indexes.iter().fold(0, |total, index| {
                    total + CompactInt::size(index.value() as usize)
                })
            }
            Message::GetBlocks {
                protocol_version: _,
                ref block_header_hashes,
                stop_hash: _,
            } => {
                4 + CompactInt::size(block_header_hashes.len())
                    + (block_header_hashes.len() * 32)
                    + 32
            }
            Message::GetData { ref inventory } => {
                let mut size = CompactInt::size(inventory.len());
                for inv in inventory.iter() {
                    size += inv.len();
                }
                size
            }
            Message::GetHeaders {
                protocol_version: _,
                ref block_header_hashes,
                stop_hash: _,
            } => {
                4 + CompactInt::size(block_header_hashes.len())
                    + (block_header_hashes.len() * 32)
                    + 32
            }
            Message::Headers { ref headers } => {
                headers.len() * (BlockHeader::len() + 1) + CompactInt::size(headers.len())
            }
            Message::Inv { ref inventory } => {
                let mut size = CompactInt::size(inventory.len());
                for inv in inventory.iter() {
                    size += inv.len();
                }
                size
            }
            Message::MemPool {} => 0,
            Message::MerkleBlock {
                block_header: _,
                transaction_count: _,
                ref hashes,
                ref flags,
            } => {
                BlockHeader::len()
                + 4
                + CompactInt::size(hashes.len())
                + (hashes.len() * 32) //32 bytes for each "hash" as they are u256
                + CompactInt::size(flags.len())
                + flags.len()
            }
            Message::NotFound { ref inventory_data } => {
                let mut size = CompactInt::size(inventory_data.len());
                for inv in inventory_data.iter() {
                    size += inv.len()
                }
                size
            }
            Message::Ping { nonce: _ } => 8,
            Message::Pong { nonce: _ } => 8,
            Message::Reject {
                message: _,
                code: _,
                reason: _,
            } => {
                unimplemented!()
            }
            Message::SendCompact {
                announce: _,
                version: _,
            } => 9,
            Message::SendHeaders {} => 0,
            Message::Tx { ref transaction } => transaction.len(),
            Message::Verack {} => 0,
            Message::Version {
                protocol_version: _,
                services: _,
                timestamp: _,
                receiver_services: _,
                receiver: _,
                transmitter_services: _,
                transmitter_ip: _,
                nonce: _,
                ref user_agent,
                best_block: _,
                relay: _,
            } => 85 + CompactInt::size(user_agent.len()) + user_agent.len(),
        }
    }
}
