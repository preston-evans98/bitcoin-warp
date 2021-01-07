use crate::message_header::MessageHeader;
use crate::types::*;
use crate::Message;
use bytes::{BufMut, BytesMut};
use shared::BlockHeader;
use shared::EncapsulatedAddr;
use shared::Serializable;
use shared::Transaction;
use shared::{u256, CompactInt, Deserializable, DeserializationError, InventoryData};
use tracing::{self, debug, trace};
#[derive(Debug)]
pub struct Codec {
    magic: u32,
    state: DecoderState,
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
        dst.reserve(payload_size + MessageHeader::len());
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

#[derive(Debug)]
enum DecoderState {
    Header,
    Body { header: MessageHeader },
}

impl tokio_util::codec::Decoder for Codec {
    type Error = shared::DeserializationError;

    type Item = Message;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.state {
            DecoderState::Header => {
                if src.len() < MessageHeader::len() {
                    return Ok(None);
                }

                let reader = src.split_to(MessageHeader::len());
                let mut reader = std::io::Cursor::new(reader);

                let header = MessageHeader::deserialize(&mut reader, self.magic)?;
                self.set_decoder_state(DecoderState::Body { header });

                // Recursively decode body
                self.decode(src)
            }

            DecoderState::Body { ref header } => {
                if src.len() < header.get_payload_size() {
                    return Ok(None);
                }

                let reader = src.split_to(header.get_payload_size());
                let mut reader = std::io::Cursor::new(reader);

                let contents = self.deserialize(&mut reader)?;
                Ok(Some(contents))
            }
        }
    }

    // fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
    //     match self.decode(buf)? {
    //         Some(frame) => Ok(Some(frame)),
    //         None => {
    //             if buf.is_empty() {
    //                 Ok(None)
    //             } else {
    //                 Err(std::io::Error::new(std::io::ErrorKind::Other, "bytes remaining on stream").into())
    //             }
    //         }
    //     }
    // }

    // fn framed<T: AsyncRead + AsyncWrite + Sized>(self, io: T) -> tokio_util::codec::Framed<T, Self>
    // where
    //     Self: Sized,
    // {
    //     tokio_util::codec::Framed::new(std::io, self)
    // }
}

impl Codec {
    pub fn new(magic: u32) -> Codec {
        Codec {
            magic,
            state: DecoderState::Header,
        }
    }
    fn set_decoder_state(&mut self, state: DecoderState) {
        self.state = state;
    }

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
                extra_data: _,
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

    fn deserialize<R: std::io::Read>(
        &mut self,
        src: &mut R,
    ) -> Result<Message, DeserializationError> {
        match self.state {
            DecoderState::Header => {
                unreachable!(
                    "Should never try to decode message body while in 'Header' decoder state"
                );
            }
            DecoderState::Body { ref header } => {
                let msg = match header.get_command() {
                    crate::Command::Addr => {
                        let addrs = Vec::<EncapsulatedAddr>::deserialize(src)?;
                        Message::Addr { addrs }
                    }
                    crate::Command::Version => Message::Version {
                        protocol_version: ProtocolVersion::deserialize(src)?,
                        services: Services::deserialize(src)?,
                        timestamp: u64::deserialize(src)?,
                        receiver_services: Services::deserialize(src)?,
                        receiver: std::net::SocketAddr::deserialize(src)?,
                        transmitter_services: Services::deserialize(src)?,
                        transmitter_ip: std::net::SocketAddr::deserialize(src)?,
                        nonce: Nonce::deserialize(src)?,
                        user_agent: <Vec<u8>>::deserialize(src)?,
                        best_block: u32::deserialize(src)?,
                        relay: bool::deserialize(src)?,
                    },
                    crate::Command::Verack => Message::Verack {},
                    crate::Command::GetBlocks => Message::GetBlocks {
                        protocol_version: ProtocolVersion::deserialize(src)?,
                        block_header_hashes: <Vec<u256>>::deserialize(src)?,
                        stop_hash: u256::deserialize(src)?,
                    },
                    crate::Command::GetData => Message::GetData {
                        inventory: <Vec<InventoryData>>::deserialize(src)?,
                    },
                    crate::Command::Block => Message::Block {
                        block_header: BlockHeader::deserialize(src)?,
                        transactions: <Vec<Transaction>>::deserialize(src)?,
                    },
                    crate::Command::GetHeaders => Message::GetHeaders {
                        protocol_version: ProtocolVersion::deserialize(src)?,
                        block_header_hashes: <Vec<u256>>::deserialize(src)?,
                        stop_hash: u256::deserialize(src)?,
                    },
                    crate::Command::Headers => Message::Headers {
                        headers: <Vec<BlockHeader>>::deserialize(src)?,
                    },
                    crate::Command::Inv => Message::Inv {
                        inventory: <Vec<InventoryData>>::deserialize(src)?,
                    },
                    crate::Command::MemPool => Message::MemPool {},
                    crate::Command::MerkleBlock => Message::MerkleBlock {
                        block_header: BlockHeader::deserialize(src)?,
                        transaction_count: u32::deserialize(src)?,
                        hashes: <Vec<u256>>::deserialize(src)?,
                        flags: <Vec<u8>>::deserialize(src)?,
                    },
                    crate::Command::CmpctBlock => Message::CompactBlock {
                        header: BlockHeader::deserialize(src)?,
                        nonce: Nonce::deserialize(src)?,
                        short_ids: <Vec<u64>>::deserialize(src)?,
                        prefilled_txns: <Vec<PrefilledTransaction>>::deserialize(src)?,
                    },
                    crate::Command::GetBlockTxn => Message::GetBlockTxn {
                        block_hash: <[u8; 32]>::deserialize(src)?,
                        indexes: <Vec<CompactInt>>::deserialize(src)?,
                    },
                    crate::Command::BlockTxn => Message::BlockTxn {
                        block_hash: <[u8; 32]>::deserialize(src)?,
                        txs: <Vec<Transaction>>::deserialize(src)?,
                    },
                    crate::Command::SendCmpct => Message::SendCompact {
                        announce: bool::deserialize(src)?,
                        version: u64::deserialize(src)?,
                    },
                    crate::Command::NotFound => Message::NotFound {
                        inventory_data: <Vec<InventoryData>>::deserialize(src)?,
                    },
                    crate::Command::Tx => Message::Tx {
                        transaction: Transaction::deserialize(src)?,
                    },
                    crate::Command::Alert => {
                        // TODO: Verify that no additional cleanup is required.
                        self.set_decoder_state(DecoderState::Header);
                        return Err(DeserializationError::Parse(format!(
                            "Received Alert message! Alert is insecure and deprecated"
                        )));
                    }
                    crate::Command::FeeFilter => Message::FeeFilter {
                        feerate: u64::deserialize(src)?,
                    },
                    crate::Command::FilterAdd => Message::FilterAdd {
                        elements: <Vec<Vec<u8>>>::deserialize(src)?,
                    },
                    crate::Command::FilterClear => Message::FilterClear {},
                    crate::Command::FilterLoad => Message::FilterLoad {
                        filter: <Vec<u8>>::deserialize(src)?,
                        n_hash_funcs: u32::deserialize(src)?,
                        n_tweak: u32::deserialize(src)?,
                        n_flags: u8::deserialize(src)?,
                    },
                    crate::Command::GetAddr => Message::GetAddr {},
                    crate::Command::Ping => Message::Ping {
                        nonce: Nonce::deserialize(src)?,
                    },
                    crate::Command::Pong => Message::Pong {
                        nonce: Nonce::deserialize(src)?,
                    },
                    crate::Command::Reject => Message::Reject {
                        message: String::deserialize(src)?,
                        code: u8::deserialize(src)?,
                        reason: String::deserialize(src)?,
                        extra_data: <[u8; 32]>::deserialize(src).ok(),
                    },
                    crate::Command::SendHeaders => Message::SendHeaders {},
                };

                trace!("Received {:?}", msg);

                // TODO: Find a more elegant way of checking if the reader is empty
                let mut dummy = [0u8];
                if let Ok(_) = src.read_exact(&mut dummy) {
                    debug!("Had leftover bytes after decoding message. Weird.",);
                }

                self.set_decoder_state(DecoderState::Header);

                Ok(msg)
            }
        }
    }
}
