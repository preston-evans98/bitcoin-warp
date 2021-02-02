use crate::message_header::MessageHeader;
use crate::types::*;
use crate::Message;
use byteorder::WriteBytesExt;
use bytes::{Buf, BufMut, BytesMut};
use shared::EncapsulatedAddr;
use shared::Serializable;
use shared::Transaction;
use shared::{u256, BlockHeader, CompactInt, Deserializable, DeserializationError, InventoryData};
use tracing::{self, debug, trace};
/// A [Codec](https://tokio-rs.github.io/tokio/doc/tokio_util/codec/index.html) converting a raw TcpStream into a Sink + Stream of Bitcoin Wire Protocol [`Message`s](crate::Message).
///
/// This struct handles the serialization and sending of [`Message`s](crate::Message). Callers simply construct a [Framed](https://tokio-rs.github.io/tokio/doc/tokio_util/codec/struct.Framed.html)
/// instance containing the codec and the TcpStream.
/// ```ignore
/// // Note: This example does not compile outside the context of an async runtime.
/// let connection = tokio::net::TcpStream::connect("127.0.0.1:8333".into()).await?;
/// let codec = networking::BitcoinCodec::new(0);
/// let connection = Framed::new(connection, codec);
///
/// // Create and send a message.
/// let msg = Message::Verack{};
/// connection.send(msg).await?;
/// // Await the response.
/// let response: Message  = connection.next().await?;
/// ```

#[derive(Debug)]
pub struct Codec {
    magic: u32,
    state: DecoderState,
}

impl tokio_util::codec::Encoder<Message> for Codec {
    type Error = std::io::Error;

    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let cmd = item.command();
        let cmd: &[u8; 12] = cmd.bytes();

        // Reserve space
        let payload_size = Codec::get_serialized_size(&item);
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

        // Serialize the message body. Some messages need non-default serialization, so use a custom Codec method
        self.serialize_body(&item, &mut target)?;

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

                let mut reader = src.split_to(MessageHeader::len());

                let header = MessageHeader::deserialize(&mut reader, self.magic)?;
                self.set_decoder_state(DecoderState::Body { header });

                // Recursively decode body
                self.decode(src)
            }

            DecoderState::Body { ref header } => {
                if src.len() < header.get_payload_size() {
                    return Ok(None);
                }

                let mut reader = src.split_to(header.get_payload_size());

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
    /// Returns the size of message (excluding the header) after serialization
    fn get_serialized_size(msg: &Message) -> usize {
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
            Message::Block { ref block } => {
                BlockHeader::len()
                    + CompactInt::size(block.transactions().len())
                    + block
                        .transactions()
                        .iter()
                        .fold(0, |total, tx| total + tx.len())
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
                        let txn_len = pre_tx.tx().len();
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
                32 + CompactInt::size(indexes.len())
                    + indexes.iter().fold(0, |total, index| {
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
                (headers.len() * (BlockHeader::len() + 1)) + CompactInt::size(headers.len())
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

    fn serialize_body<W: std::io::Write>(
        &self,
        item: &Message,
        target: &mut W,
    ) -> Result<(), std::io::Error> {
        match item {
            // Custom 'Headers' serialization is necessary to account for extra Transaction count field.
            // Note that transaction count is always zero in a headers message.
            Message::Headers { ref headers } => {
                shared::CompactInt::from(headers.len()).serialize(target)?;
                for item in headers.iter() {
                    item.serialize(target)?;
                    target.write_u8(0)?;
                }
            }
            _ => item.serialize(target)?,
        }
        Ok(())
    }

    fn deserialize(&mut self, mut src: &mut BytesMut) -> Result<Message, DeserializationError> {
        match self.state {
            DecoderState::Header => {
                unreachable!(
                    "Should never try to decode message body while in 'Header' decoder state"
                );
            }
            DecoderState::Body { ref header } => {
                let msg = match header.get_command() {
                    crate::Command::Addr => {
                        let addrs = Vec::<EncapsulatedAddr>::deserialize(&mut src)?;
                        Message::Addr { addrs }
                    }
                    crate::Command::Version => Message::Version {
                        protocol_version: ProtocolVersion::deserialize(&mut src)?,
                        services: Services::deserialize(&mut src)?,
                        timestamp: u64::deserialize(&mut src)?,
                        receiver_services: Services::deserialize(&mut src)?,
                        receiver: std::net::SocketAddr::deserialize(&mut src)?,
                        transmitter_services: Services::deserialize(&mut src)?,
                        transmitter_ip: std::net::SocketAddr::deserialize(&mut src)?,
                        nonce: Nonce::deserialize(&mut src)?,
                        user_agent: String::deserialize(&mut src)?,
                        best_block: u32::deserialize(&mut src)?,
                        relay: bool::deserialize(&mut src)?,
                    },
                    crate::Command::Verack => Message::Verack {},
                    crate::Command::GetBlocks => Message::GetBlocks {
                        protocol_version: ProtocolVersion::deserialize(&mut src)?,
                        block_header_hashes: <Vec<u256>>::deserialize(&mut src)?,
                        stop_hash: u256::deserialize(&mut src)?,
                    },
                    crate::Command::GetData => Message::GetData {
                        inventory: <Vec<InventoryData>>::deserialize(&mut src)?,
                    },
                    crate::Command::Block => Message::Block {
                        block: shared::Block::deserialize(&mut src)?,
                    },
                    crate::Command::GetHeaders => Message::GetHeaders {
                        protocol_version: ProtocolVersion::deserialize(&mut src)?,
                        block_header_hashes: <Vec<u256>>::deserialize(&mut src)?,
                        stop_hash: u256::deserialize(&mut src)?,
                    },
                    crate::Command::Headers => {
                        // Custom deserialization necessary to account for extra
                        // Transaction count field. Note that transaction count is always zero in a headers message.
                        let count = CompactInt::deserialize(&mut src)?;
                        let mut result = Vec::with_capacity(count.value() as usize);
                        for _ in 0..result.len() {
                            result.push(BlockHeader::deserialize(&mut src)?);
                            let _ = u8::deserialize(&mut src)?;
                        }
                        Message::Headers { headers: result }
                    }
                    crate::Command::Inv => Message::Inv {
                        inventory: <Vec<InventoryData>>::deserialize(&mut src)?,
                    },
                    crate::Command::MemPool => Message::MemPool {},
                    crate::Command::MerkleBlock => Message::MerkleBlock {
                        block_header: BlockHeader::deserialize(&mut src)?,
                        transaction_count: u32::deserialize(&mut src)?,
                        hashes: <Vec<u256>>::deserialize(&mut src)?,
                        flags: <Vec<u8>>::deserialize(&mut src)?,
                    },
                    crate::Command::CmpctBlock => Message::CompactBlock {
                        header: BlockHeader::deserialize(&mut src)?,
                        nonce: Nonce::deserialize(&mut src)?,
                        short_ids: <Vec<u64>>::deserialize(&mut src)?,
                        prefilled_txns: <Vec<PrefilledTransaction>>::deserialize(&mut src)?,
                    },
                    crate::Command::GetBlockTxn => Message::GetBlockTxn {
                        block_hash: <[u8; 32]>::deserialize(&mut src)?,
                        indexes: <Vec<CompactInt>>::deserialize(&mut src)?,
                    },
                    crate::Command::BlockTxn => Message::BlockTxn {
                        block_hash: <[u8; 32]>::deserialize(&mut src)?,
                        txs: <Vec<Transaction>>::deserialize(&mut src)?,
                    },
                    crate::Command::SendCmpct => Message::SendCompact {
                        announce: bool::deserialize(&mut src)?,
                        version: u64::deserialize(&mut src)?,
                    },
                    crate::Command::NotFound => Message::NotFound {
                        inventory_data: <Vec<InventoryData>>::deserialize(&mut src)?,
                    },
                    crate::Command::Tx => Message::Tx {
                        transaction: Transaction::deserialize(&mut src)?,
                    },
                    crate::Command::Alert => {
                        // TODO: Verify that no additional cleanup is required.
                        self.set_decoder_state(DecoderState::Header);
                        return Err(DeserializationError::Parse(format!(
                            "Received Alert message! Alert is insecure and deprecated"
                        )));
                    }
                    crate::Command::FeeFilter => Message::FeeFilter {
                        feerate: u64::deserialize(&mut src)?,
                    },
                    crate::Command::FilterAdd => Message::FilterAdd {
                        elements: <Vec<Vec<u8>>>::deserialize(&mut src)?,
                    },
                    crate::Command::FilterClear => Message::FilterClear {},
                    crate::Command::FilterLoad => Message::FilterLoad {
                        filter: <Vec<u8>>::deserialize(&mut src)?,
                        n_hash_funcs: u32::deserialize(&mut src)?,
                        n_tweak: u32::deserialize(&mut src)?,
                        n_flags: u8::deserialize(&mut src)?,
                    },
                    crate::Command::GetAddr => Message::GetAddr {},
                    crate::Command::Ping => Message::Ping {
                        nonce: Nonce::deserialize(&mut src)?,
                    },
                    crate::Command::Pong => Message::Pong {
                        nonce: Nonce::deserialize(&mut src)?,
                    },
                    crate::Command::Reject => Message::Reject {
                        message: String::deserialize(&mut src)?,
                        code: u8::deserialize(&mut src)?,
                        reason: String::deserialize(&mut src)?,
                        extra_data: <[u8; 32]>::deserialize(&mut src).ok(),
                    },
                    crate::Command::SendHeaders => Message::SendHeaders {},
                };

                trace!("Received {:?}", msg);

                if src.remaining() != 0 {
                    debug!("Had leftover bytes after decoding message. Weird.",);
                }

                self.set_decoder_state(DecoderState::Header);

                Ok(msg)
            }
        }
    }
}

#[cfg(test)]
mod message_roundtrip_tests {
    use crate::codec::Codec;
    use crate::{
        Command,
        Message::{self},
        MessageHeader,
    };
    use bytes::{BufMut, BytesMut};
    use shared::Serializable;
    use std::net::SocketAddr;

    use super::DecoderState;

    #[test]
    fn version_roundtrip() {
        let version = Message::version(
            SocketAddr::from(([192, 168, 0, 1], 8333)),
            0,
            SocketAddr::from(([192, 168, 0, 0], 8333)),
            0,
            &config::Config::mainnet(),
        );
        let mut writer = BytesMut::with_capacity(100).writer();

        version
            .serialize(&mut writer)
            .expect("Serialization to bytesmut shouldnt fail");

        let mut out = writer.into_inner();
        let mut codec = Codec {
            magic: config::Config::mainnet().magic(),
            state: DecoderState::Body {
                header: MessageHeader::_test_create(
                    config::Config::mainnet().magic(),
                    Command::Version,
                    out.len() as u32,
                    [0u8; 4],
                ),
            },
        };

        let result = codec.deserialize(&mut out);
        assert!(result.is_ok());
        // let expected = version;
        // let actual = result.unwrap();
        // assert_eq!(format!("{:?}", version), format!("{:?}", result.unwrap()))
    }
}

#[cfg(test)]
mod consensus_deser_tests {

    #[test]
    // Adapted from https://github.com/rust-bitcoin/rust-bitcoin/blob/master/src/blockdata/block.rs
    fn deser_block_test() {
        use bytes::BytesMut;
        use shared::Block;
        // Mainnet block 00000000b0c5a240b2a61d2e75692224efd4cbecdf6eaf4cc2cf477ca7c270e7
        let some_block = hex::decode("010000004ddccd549d28f385ab457e98d1b11ce80bfea2c5ab93015ade4973e400000000bf4473e53794beae34e64fccc471dace6ae544180816f89591894e0f417a914cd74d6e49ffff001d323b3a7b0201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0804ffff001d026e04ffffffff0100f2052a0100000043410446ef0102d1ec5240f0d061a4246c1bdef63fc3dbab7733052fbbf0ecd8f41fc26bf049ebb4f9527f374280259e7cfa99c48b0e3f39c51347a19a5819651503a5ac00000000010000000321f75f3139a013f50f315b23b0c9a2b6eac31e2bec98e5891c924664889942260000000049483045022100cb2c6b346a978ab8c61b18b5e9397755cbd17d6eb2fe0083ef32e067fa6c785a02206ce44e613f31d9a6b0517e46f3db1576e9812cc98d159bfdaf759a5014081b5c01ffffffff79cda0945903627c3da1f85fc95d0b8ee3e76ae0cfdc9a65d09744b1f8fc85430000000049483045022047957cdd957cfd0becd642f6b84d82f49b6cb4c51a91f49246908af7c3cfdf4a022100e96b46621f1bffcf5ea5982f88cef651e9354f5791602369bf5a82a6cd61a62501fffffffffe09f5fe3ffbf5ee97a54eb5e5069e9da6b4856ee86fc52938c2f979b0f38e82000000004847304402204165be9a4cbab8049e1af9723b96199bfd3e85f44c6b4c0177e3962686b26073022028f638da23fc003760861ad481ead4099312c60030d4cb57820ce4d33812a5ce01ffffffff01009d966b01000000434104ea1feff861b51fe3f5f8a3b12d0f4712db80e919548a80839fc47c6a21e66d957e9c5d8cd108c7a2d2324bad71f9905ac0ae7336507d785b17a2c115e427a32fac00000000").unwrap();
        // let some_block = hex::decode("010000004ddccd549d28f385ab457e98d1b11ce80bfea2c5ab93015ade4973e400000000bf4473e53794beae34e64fccc471dace6ae544180816f89591894e0f417a914cd74d6e49ffff001d323b3a7b0201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0804ffff001d026e04ffffffff0100f2052a0100000043410446ef0102d1ec5240f0d061a4246c1bdef63fc3dbab7733052fbbf0ecd8f41fc26bf049ebb4f9527f374280259e7cfa99c48b0e3f39c51347a19a5819651503a5ac00000000010000000321f75f3139a013f50f315b23b0c9a2b6eac31e2bec98e5891c924664889942260000000049483045022100cb2c6b346a978ab8c61b18b5e9397755cbd17d6eb2fe0083ef32e067fa6c785a02206ce44e613f31d9a6b0517e46f3db1576e9812cc98d159bfdaf759a5014081b5c01ffffffff79cda0945903627c3da1f85fc95d0b8ee3e76ae0cfdc9a65d09744b1f8fc85430000000049483045022047957cdd957cfd0becd642f6b84d82f49b6cb4c51a91f49246908af7c3cfdf4a022100e96b46621f1bffcf5ea5982f88cef651e9354f5791602369bf5a82a6cd61a62501fffffffffe09f5fe3ffbf5ee97a54eb5e5069e9da6b4856ee86fc52938c2f979b0f38e82000000004847304402204165be9a4cbab8049e1af9723b96199bfd3e85f44c6b4c0177e3962686b26073022028f638da23fc003760861ad481ead4099312c60030d4cb57820ce4d33812a5ce01ffffffff01009d966b01000000434104ea1feff861b51fe3f5f8a3b12d0f4712db80e919548a80839fc47c6a21e66d957e9c5d8cd108c7a2d2324bad71f9904ac0ae7336507d785b17a2c115e427a32fac00000000").unwrap();
        // let correct_some_block = hex::decode("010000004ddccd549d28f385ab457e98d1b11ce80bfea2c5ab93015ade4973e400000000bf4473e53794beae34e64fccc471dace6ae544180816f89591894e0f417a914cd74d6e49ffff001d323b3a7b0201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0804ffff001d026e04ffffffff0100f2052a0100000043410446ef0102d1ec5240f0d061a4246c1bdef63fc3dbab7733052fbbf0ecd8f41fc26bf049ebb4f9527f374280259e7cfa99c48b0e3f39c51347a19a5819651503a5ac00000000010000000321f75f3139a013f50f315b23b0c9a2b6eac31e2bec98e5891c924664889942260000000049483045022100cb2c6b346a978ab8c61b18b5e9397755cbd17d6eb2fe0083ef32e067fa6c785a02206ce44e613f31d9a6b0517e46f3db1576e9812cc98d159bfdaf759a5014081b5c01ffffffff79cda0945903627c3da1f85fc95d0b8ee3e76ae0cfdc9a65d09744b1f8fc85430000000049483045022047957cdd957cfd0becd642f6b84d82f49b6cb4c51a91f49246908af7c3cfdf4a022100e96b46621f1bffcf5ea5982f88cef651e9354f5791602369bf5a82a6cd61a62501fffffffffe09f5fe3ffbf5ee97a54eb5e5069e9da6b4856ee86fc52938c2f979b0f38e82000000004847304402204165be9a4cbab8049e1af9723b96199bfd3e85f44c6b4c0177e3962686b26073022028f638da23fc003760861ad481ead4099312c60030d4cb57820ce4d33812a5ce01ffffffff01009d966b01000000434104ea1feff861b51fe3f5f8a3b12d0f4712db80e919548a80839fc47c6a21e66d957e9c5d8cd108c7a2d2324bad71f9904ac0ae7336507d785b17a2c115e427a32fac00000000").unwrap();
        let cutoff_block = hex::decode("010000004ddccd549d28f385ab457e98d1b11ce80bfea2c5ab93015ade4973e400000000bf4473e53794beae34e64fccc471dace6ae544180816f89591894e0f417a914cd74d6e49ffff001d323b3a7b0201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0804ffff001d026e04ffffffff0100f2052a0100000043410446ef0102d1ec5240f0d061a4246c1bdef63fc3dbab7733052fbbf0ecd8f41fc26bf049ebb4f9527f374280259e7cfa99c48b0e3f39c51347a19a5819651503a5ac00000000010000000321f75f3139a013f50f315b23b0c9a2b6eac31e2bec98e5891c924664889942260000000049483045022100cb2c6b346a978ab8c61b18b5e9397755cbd17d6eb2fe0083ef32e067fa6c785a02206ce44e613f31d9a6b0517e46f3db1576e9812cc98d159bfdaf759a5014081b5c01ffffffff79cda0945903627c3da1f85fc95d0b8ee3e76ae0cfdc9a65d09744b1f8fc85430000000049483045022047957cdd957cfd0becd642f6b84d82f49b6cb4c51a91f49246908af7c3cfdf4a022100e96b46621f1bffcf5ea5982f88cef651e9354f5791602369bf5a82a6cd61a62501fffffffffe09f5fe3ffbf5ee97a54eb5e5069e9da6b4856ee86fc52938c2f979b0f38e82000000004847304402204165be9a4cbab8049e1af9723b96199bfd3e85f44c6b4c0177e3962686b26073022028f638da23fc003760861ad481ead4099312c60030d4cb57820ce4d33812a5ce01ffffffff01009d966b01000000434104ea1feff861b51fe3f5f8a3b12d0f4712db80e919548a80839fc47c6a21e66d957e9c5d8cd108c7a2d2324bad71f9904ac0ae7336507d785b17a2c115e427a32fac").unwrap();

        let prevhash =
            hex::decode("4ddccd549d28f385ab457e98d1b11ce80bfea2c5ab93015ade4973e400000000")
                .unwrap();
        let merkle =
            hex::decode("bf4473e53794beae34e64fccc471dace6ae544180816f89591894e0f417a914c")
                .unwrap();
        // let work = Uint256([0x100010001u64, 0, 0, 0]);

        let decode: Result<Block, _> = Block::deserialize(&mut BytesMut::from(&some_block[..]));
        let bad_decode: Result<Block, _> =
            Block::deserialize(&mut BytesMut::from(&cutoff_block[..]));

        // assert_eq!(format!("{:?}", decode), "");
        assert!(decode.is_ok());
        assert!(bad_decode.is_err());
        let decode = decode.unwrap();
        // let real_decode = decode.unwrap();
        assert_eq!(decode.header().version(), 1);
        // assert_eq!(serialize(&real_decode.header.prev_blockhash), prevhash);
        // assert_eq!(real_decode.header.merkle_root, real_decode.merkle_root());
        // assert_eq!(serialize(&real_decode.header.merkle_root), merkle);
        // assert_eq!(real_decode.header.time, 1231965655);
        // assert_eq!(real_decode.header.bits, 486604799);
        // assert_eq!(real_decode.header.nonce, 2067413810);
        // assert_eq!(real_decode.header.work(), work);
        // assert_eq!(
        //     real_decode
        //         .header
        //         .validate_pow(&real_decode.header.target())
        //         .unwrap(),
        //     ()
        // );
        // assert_eq!(real_decode.header.difficulty(Network::Bitcoin), 1);
        // // [test] TODO: check the transaction data

        // assert_eq!(real_decode.get_size(), some_block.len());
        // assert_eq!(real_decode.get_weight(), some_block.len() * 4);

        // // should be also ok for a non-witness block as commitment is optional in that case
        // assert!(real_decode.check_witness_commitment());

        // assert_eq!(serialize(&real_decode), some_block);
    }
}
// These are legacy tests copied and pasted the previous implementation.
// Each test was designed to check a particular struct implementing the trait Payload
// (now each of these structs has become a Message variant)
// TODO: Refactor to...
//    - Replace manual creation of each variant with something automated
//    - Consolidate into fewer tests
//    - Remove wrappers over Codec functionality (msg.to_bytes(), msg.serialized_size())
#[cfg(test)]
mod message_size_tests {
    use crate::types::PrefilledTransaction;
    use crate::{
        BitcoinCodec,
        Message::{self, *},
    };
    use bytes::{BufMut, BytesMut};
    use shared::EncapsulatedAddr;
    use shared::{BlockHeader, Transaction};

    impl Message {
        fn to_bytes(&self) -> Result<BytesMut, std::io::Error> {
            // let mut out = Vec::with_capacity(codec.get_serialized_size(&self));
            let codec = BitcoinCodec::new(0);
            let out = BytesMut::with_capacity(BitcoinCodec::get_serialized_size(&self));
            let mut out = out.writer();
            let _ = codec.serialize_body(&self, &mut out)?;
            Ok(out.into_inner())
        }
        fn serialized_size(&self) -> usize {
            BitcoinCodec::get_serialized_size(self)
        }
    }

    #[test]
    fn addr_serial_size() {
        let addr1 = EncapsulatedAddr::new(1, 1, ([192, 168, 0, 1], 8333).into());
        let addr2 = EncapsulatedAddr::new(1, 1, ([192, 168, 0, 1], 8333).into());
        let mut addrs = Vec::with_capacity(2);
        addrs.push(addr1);
        addrs.push(addr2);
        let msg = Addr { addrs };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn blocktxn_serial_size_empty() {
        let txs = Vec::with_capacity(2);
        let msg = BlockTxn {
            block_hash: [1u8; 32],
            txs,
        };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn blocktxn_serial_size_full() {
        let previous_outpoint = shared::TxOutpoint::new(shared::u256::from(1), 438);
        let txin1 = shared::TxInput::new(previous_outpoint, Vec::from([8u8; 21]), 1);
        let txin2 = shared::TxInput::new(
            shared::TxOutpoint::new(shared::u256::new(), 0),
            Vec::new(),
            2,
        );
        let mut txins = Vec::new();
        txins.push(txin1);
        txins.push(txin2);
        let mut outputs = Vec::new();
        let out1 = shared::TxOutput::new(1, Vec::from([3u8; 11]));
        let out2 = shared::TxOutput::new(0, Vec::new());
        outputs.push(out1);
        outputs.push(out2);
        let tx1 = shared::Transaction::new(0, txins, outputs);
        let tx2 = shared::Transaction::new(1, Vec::new(), Vec::new());

        let mut txs = Vec::with_capacity(2);
        txs.push(tx1);
        txs.push(tx2);
        let msg = BlockTxn {
            block_hash: [1u8; 32],
            txs,
        };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn block_serial_size() {
        let previous_outpoint = shared::TxOutpoint::new(shared::u256::from(1), 438);
        let txin1 = shared::TxInput::new(previous_outpoint, Vec::from([8u8; 21]), 1);
        let txin2 = shared::TxInput::new(
            shared::TxOutpoint::new(shared::u256::new(), 0),
            Vec::new(),
            2,
        );
        let mut txins = Vec::new();
        txins.push(txin1);
        txins.push(txin2);
        let mut outputs = Vec::new();
        let out1 = shared::TxOutput::new(1, Vec::from([3u8; 11]));
        let out2 = shared::TxOutput::new(0, Vec::new());
        outputs.push(out1);
        outputs.push(out2);
        let tx1 = shared::Transaction::new(0, txins, outputs);
        let tx2 = shared::Transaction::new(1, Vec::new(), Vec::new());

        let mut txs = Vec::with_capacity(2);
        txs.push(tx1);
        txs.push(tx2);
        let block_header = shared::BlockHeader::new(
            23,
            shared::u256::from(12345678),
            shared::u256::from(9876543),
            2342,
            shared::Nbits::new(shared::u256::from(8719)),
            99,
        );

        let msg = Block {
            block: shared::Block::new(block_header, txs),
        };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn compact_block_serial_size() {
        let previous_outpoint = shared::TxOutpoint::new(shared::u256::from(1), 438);
        let txin1 = shared::TxInput::new(previous_outpoint, Vec::from([8u8; 21]), 1);
        let txin2 = shared::TxInput::new(
            shared::TxOutpoint::new(shared::u256::new(), 0),
            Vec::new(),
            2,
        );
        let mut txins = Vec::new();
        txins.push(txin1);
        txins.push(txin2);
        let mut outputs = Vec::new();
        let out1 = shared::TxOutput::new(1, Vec::from([3u8; 11]));
        let out2 = shared::TxOutput::new(0, Vec::new());
        outputs.push(out1);
        outputs.push(out2);
        let tx1 = PrefilledTransaction::new(
            shared::CompactInt::from(1),
            shared::Transaction::new(0, txins, outputs),
        );
        let tx2 = PrefilledTransaction::new(
            shared::CompactInt::from(1),
            Transaction::new(1, Vec::new(), Vec::new()),
        );

        let mut txs = Vec::with_capacity(2);
        txs.push(tx1);
        txs.push(tx2);
        let header = BlockHeader::new(
            23,
            shared::u256::from(12345678),
            shared::u256::from(9876543),
            2342,
            shared::Nbits::new(shared::u256::from(8719)),
            99,
        );

        let msg = CompactBlock {
            header,
            nonce: 1928712,
            short_ids: Vec::from([8219u64; 7]),
            prefilled_txns: txs,
        };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn feefilter_serial_size() {
        let msg = FeeFilter { feerate: 34567 };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn filteradd_serial_size() {
        let inner1 = Vec::from([1u8; 32]);
        let inner2 = Vec::from([7u8; 11]);
        let outer = Vec::from([inner1, inner2]);
        let msg = FilterAdd { elements: outer };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn filterclear_serial_size() {
        let msg = FilterClear {};
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn filterload_serial_size() {
        let msg = FilterLoad {
            filter: Vec::from([23u8; 22]),
            n_hash_funcs: 0x129381,
            n_tweak: 0xf2391381,
            n_flags: 32,
        };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn get_addr_serial_size() {
        let msg = GetAddr {};
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn get_block_txn_serial_size() {
        use shared::CompactInt;
        let int1 = CompactInt::from(567892322);
        let int2 = CompactInt::from(7892322);
        let int3 = CompactInt::from(0);
        let msg = GetBlockTxn {
            block_hash: [242u8; 32],
            indexes: Vec::from([int1, int2, int3]),
        };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn get_blocks_serial_size() {
        use shared::u256;
        let int1 = u256::from(567892322);
        let int2 = u256::from(7892322);
        let int3 = u256::from(1);
        let msg = GetBlocks {
            protocol_version: 32371,
            block_header_hashes: Vec::from([int1, int2, int3]),
            stop_hash: u256::new(),
        };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn get_data_serial_size() {
        use shared::u256;
        use shared::InventoryData;
        use shared::InventoryType;
        let int1 = u256::from(567892322);
        let int2 = u256::from(7892322);
        let int3 = u256::from(0);
        let t1 = InventoryType::FilteredBlock;
        let t2 = InventoryType::WitnessBlock;
        let t3 = InventoryType::Tx;
        let d1 = InventoryData {
            inventory_type: t1,
            hash: int1,
        };
        let d2 = InventoryData {
            inventory_type: t2,
            hash: int2,
        };
        let d3 = InventoryData {
            inventory_type: t3,
            hash: int3,
        };

        let inventory = Vec::from([d1, d2, d3]);
        let msg = GetData { inventory };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn get_headers_serial_size() {
        use shared::u256;
        let int1 = u256::from(567892322);
        let int2 = u256::from(7892322);
        let int3 = u256::from(1);
        let msg = GetHeaders {
            protocol_version: 32371,
            block_header_hashes: Vec::from([int1, int2, int3]),
            stop_hash: u256::new(),
        };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn headers_serial_size() {
        let h1 = BlockHeader::new(
            23,
            shared::u256::from(12345678),
            shared::u256::from(9876543),
            2342,
            shared::Nbits::new(shared::u256::from(8719)),
            99,
        );
        let h2 = BlockHeader::new(
            0,
            shared::u256::from(2),
            shared::u256::from(88),
            2198321,
            shared::Nbits::new(shared::u256::from(0xf32231)),
            82,
        );

        let headers = Vec::from([h1, h2]);

        let msg = Headers { headers };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn inv_serial_size() {
        use shared::u256;
        use shared::{InventoryData, InventoryType};
        let int1 = u256::from(567892322);
        let int2 = u256::from(7892322);
        let int3 = u256::from(0);
        let t1 = InventoryType::FilteredBlock;
        let t2 = InventoryType::WitnessBlock;
        let t3 = InventoryType::Tx;
        let d1 = InventoryData {
            inventory_type: t1,
            hash: int1,
        };
        let d2 = InventoryData {
            inventory_type: t2,
            hash: int2,
        };
        let d3 = InventoryData {
            inventory_type: t3,
            hash: int3,
        };

        let inventory = Vec::from([d1, d2, d3]);
        let msg = Inv { inventory };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn mempool_serial_size() {
        let msg = MemPool {};
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn merkle_block_serial_size() {
        use shared::u256;
        let int1 = u256::from(567892322);
        let int2 = u256::from(7892322);
        let int3 = u256::from(1);
        let block_header = BlockHeader::new(
            23,
            shared::u256::from(12345678),
            shared::u256::from(9876543),
            2342,
            shared::Nbits::new(u256::from(8719)),
            99,
        );

        let msg = MerkleBlock {
            block_header,
            transaction_count: 113,
            hashes: vec![int1, int2, int3],
            flags: Vec::from([232u8, 11]),
        };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn not_found_serial_size() {
        use shared::u256;
        use shared::{InventoryData, InventoryType};
        let int1 = u256::from(567892322);
        let int2 = u256::from(7892322);
        let int3 = u256::from(0);
        let t1 = InventoryType::FilteredBlock;
        let t2 = InventoryType::WitnessBlock;
        let t3 = InventoryType::Tx;
        let d1 = InventoryData {
            inventory_type: t1,
            hash: int1,
        };
        let d2 = InventoryData {
            inventory_type: t2,
            hash: int2,
        };
        let d3 = InventoryData {
            inventory_type: t3,
            hash: int3,
        };

        let inventory = Vec::from([d1, d2, d3]);
        let msg = NotFound {
            inventory_data: inventory,
        };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn ping_serial_size() {
        let msg = Ping { nonce: 34567 };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn pong_serial_size() {
        let msg = Pong { nonce: 34567 };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn send_compact_serial_size() {
        let msg = SendCompact {
            announce: true,
            version: 32381,
        };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn send_headers_serial_size() {
        let msg = SendHeaders {};
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn tx_serial_size() {
        let previous_outpoint = shared::TxOutpoint::new(shared::u256::from(1), 438);
        let txin1 = shared::TxInput::new(previous_outpoint, Vec::from([8u8; 21]), 1);
        let txin2 = shared::TxInput::new(
            shared::TxOutpoint::new(shared::u256::new(), 0),
            Vec::new(),
            2,
        );
        let mut txins = Vec::new();
        txins.push(txin1);
        txins.push(txin2);
        let mut outputs = Vec::new();
        let out1 = shared::TxOutput::new(1, Vec::from([3u8; 11]));
        let out2 = shared::TxOutput::new(0, Vec::new());
        outputs.push(out1);
        outputs.push(out2);
        let tx = Transaction::new(0, txins, outputs);

        let msg = Tx { transaction: tx };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn verack_serial_size() {
        let msg = Verack {};
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn version_serial_size() {
        let msg = Message::version(
            ([192, 168, 0, 1], 8333).into(),
            2371,
            ([192, 168, 0, 2], 8333).into(),
            0x2329381,
            &config::Config::mainnet(),
        );
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
}
