use crate::{
    message::{BlockTxn, CompactBlock, FilterLoad, GetBlockTxn, Reject, SendCompact, Version},
    types::*,
};
use crate::{
    message::{GetBlocks, GetHeaders},
    Message,
};
use crate::{
    message::{MerkleBlock, Payload},
    message_header::MessageHeader,
};
use byteorder::WriteBytesExt;
use bytes::{Buf, BufMut, BytesMut};
use shared::EncapsulatedAddr;
use shared::Serializable;
use shared::Transaction;
use shared::{BlockHeader, CompactInt, Deserializable, DeserializationError, InventoryData};
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
            Message::Addr(ref addrs) => CompactInt::size(addrs.len()) + addrs.len() * (4 + 8 + 18),
            Message::BlockTxn(block_txn) => block_txn.serialized_size(),
            Message::Block(block) => block.serialized_size(),
            Message::CompactBlock(compact_block) => compact_block.serialized_size(),
            Message::FeeFilter(_) => 8,
            Message::FilterAdd(elements) => {
                elements.iter().fold(0, |total, elt| {
                    total + elt.len() + CompactInt::size(elt.len())
                }) + CompactInt::size(elements.len())
            }
            Message::FilterClear => 0,
            Message::FilterLoad(filter_load) => filter_load.serialized_size(),
            Message::GetAddr => 0,
            Message::GetBlockTxn(get_block_txn) => get_block_txn.serialized_size(),
            Message::GetBlocks(get_blocks) => get_blocks.serialized_size(),
            Message::GetData(inventory) => {
                let mut size = CompactInt::size(inventory.len());
                for inv in inventory.iter() {
                    size += inv.len();
                }
                size
            }
            Message::GetHeaders(get_headers) => get_headers.serialized_size(),
            Message::Headers(headers) => {
                (headers.len() * (BlockHeader::len() + 1)) + CompactInt::size(headers.len())
            }
            Message::Inv(inventory) => {
                let mut size = CompactInt::size(inventory.len());
                for inv in inventory.iter() {
                    size += inv.len();
                }
                size
            }
            Message::MemPool => 0,
            Message::MerkleBlock(merkle_block) => merkle_block.serialized_size(),
            Message::NotFound(inventory_data) => {
                let mut size = CompactInt::size(inventory_data.len());
                for inv in inventory_data.iter() {
                    size += inv.len()
                }
                size
            }
            Message::Ping(_) => 8,
            Message::Pong(_) => 8,
            Message::Reject(reject) => reject.serialized_size(),
            Message::SendCompact(send_compact) => send_compact.serialized_size(),
            Message::SendHeaders => 0,
            Message::Tx(transaction) => transaction.len(),
            Message::Verack => 0,
            Message::Version(version) => version.serialized_size(),
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
            Message::Headers(headers) => {
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
                        Message::Addr(Vec::<EncapsulatedAddr>::deserialize(&mut src)?)
                    }
                    crate::Command::Version => Message::Version(Version::deserialize(&mut src)?),
                    crate::Command::Verack => Message::Verack,
                    crate::Command::GetBlocks => {
                        Message::GetBlocks(GetBlocks::deserialize(&mut src)?)
                    }
                    crate::Command::GetData => {
                        Message::GetData(<Vec<InventoryData>>::deserialize(&mut src)?)
                    }
                    crate::Command::Block => Message::Block(shared::Block::deserialize(&mut src)?),
                    crate::Command::GetHeaders => {
                        Message::GetHeaders(GetHeaders::deserialize(&mut src)?)
                    }
                    crate::Command::Headers => {
                        // Custom deserialization necessary to account for extra
                        // Transaction count field. Note that transaction count is always zero in a headers message.
                        let count = CompactInt::deserialize(&mut src)?;
                        let mut result = Vec::with_capacity(count.value() as usize);
                        for _ in 0..result.len() {
                            result.push(BlockHeader::deserialize(&mut src)?);
                            let _ = u8::deserialize(&mut src)?;
                        }
                        Message::Headers(result)
                    }
                    crate::Command::Inv => {
                        Message::Inv(<Vec<InventoryData>>::deserialize(&mut src)?)
                    }
                    crate::Command::MemPool => Message::MemPool,
                    crate::Command::MerkleBlock => {
                        Message::MerkleBlock(MerkleBlock::deserialize(&mut src)?)
                    }
                    crate::Command::CmpctBlock => {
                        Message::CompactBlock(CompactBlock::deserialize(&mut src)?)
                    }
                    crate::Command::GetBlockTxn => {
                        Message::GetBlockTxn(GetBlockTxn::deserialize(&mut src)?)
                    }
                    crate::Command::BlockTxn => Message::BlockTxn(BlockTxn::deserialize(&mut src)?),
                    crate::Command::SendCmpct => {
                        Message::SendCompact(SendCompact::deserialize(&mut src)?)
                    }
                    crate::Command::NotFound => {
                        Message::NotFound(<Vec<InventoryData>>::deserialize(&mut src)?)
                    }
                    crate::Command::Tx => Message::Tx(Transaction::deserialize(&mut src)?),
                    crate::Command::Alert => {
                        // TODO: Verify that no additional cleanup is required.
                        self.set_decoder_state(DecoderState::Header);
                        return Err(DeserializationError::Parse(format!(
                            "Received Alert message! Alert is insecure and deprecated"
                        )));
                    }
                    crate::Command::FeeFilter => Message::FeeFilter(u64::deserialize(&mut src)?),
                    crate::Command::FilterAdd => {
                        Message::FilterAdd(<Vec<Vec<u8>>>::deserialize(&mut src)?)
                    }
                    crate::Command::FilterClear => Message::FilterClear,
                    crate::Command::FilterLoad => {
                        Message::FilterLoad(FilterLoad::deserialize(&mut src)?)
                    }
                    crate::Command::GetAddr => Message::GetAddr,
                    crate::Command::Ping => Message::Ping(Nonce::deserialize(&mut src)?),
                    crate::Command::Pong => Message::Pong(Nonce::deserialize(&mut src)?),
                    crate::Command::Reject => Message::Reject(Reject::deserialize(&mut src)?),
                    crate::Command::SendHeaders => Message::SendHeaders,
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
        let some_block = hex::decode("010000004ddccd549d28f385ab457e98d1b11ce80bfea2c5ab93015ade4973e400000000bf4473e53794beae34e64fccc471dace6ae544180816f89591894e0f417a914cd74d6e49ffff001d323b3a7b0201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0804ffff001d026e04ffffffff0100f2052a0100000043410446ef0102d1ec5240f0d061a4246c1bdef63fc3dbab7733052fbbf0ecd8f41fc26bf049ebb4f9527f374280259e7cfa99c48b0e3f39c51347a19a5819651503a5ac00000000010000000321f75f3139a013f50f315b23b0c9a2b6eac31e2bec98e5891c924664889942260000000049483045022100cb2c6b346a978ab8c61b18b5e9397755cbd17d6eb2fe0083ef32e067fa6c785a02206ce44e613f31d9a6b0517e46f3db1576e9812cc98d159bfdaf759a5014081b5c01ffffffff79cda0945903627c3da1f85fc95d0b8ee3e76ae0cfdc9a65d09744b1f8fc85430000000049483045022047957cdd957cfd0becd642f6b84d82f49b6cb4c51a91f49246908af7c3cfdf4a022100e96b46621f1bffcf5ea5982f88cef651e9354f5791602369bf5a82a6cd61a62501fffffffffe09f5fe3ffbf5ee97a54eb5e5069e9da6b4856ee86fc52938c2f979b0f38e82000000004847304402204165be9a4cbab8049e1af9723b96199bfd3e85f44c6b4c0177e3962686b26073022028f638da23fc003760861ad481ead4099312c60030d4cb57820ce4d33812a5ce01ffffffff01009d966b01000000434104ea1feff861b51fe3f5f8a3b12d0f4712db80e919548a80839fc47c6a21e66d957e9c5d8cd108c7a2d2324bad71f9904ac0ae7336507d785b17a2c115e427a32fac00000000").unwrap();
        // let correct_some_block = hex::decode("010000004ddccd549d28f385ab457e98d1b11ce80bfea2c5ab93015ade4973e400000000bf4473e53794beae34e64fccc471dace6ae544180816f89591894e0f417a914cd74d6e49ffff001d323b3a7b0201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0804ffff001d026e04ffffffff0100f2052a0100000043410446ef0102d1ec5240f0d061a4246c1bdef63fc3dbab7733052fbbf0ecd8f41fc26bf049ebb4f9527f374280259e7cfa99c48b0e3f39c51347a19a5819651503a5ac00000000010000000321f75f3139a013f50f315b23b0c9a2b6eac31e2bec98e5891c924664889942260000000049483045022100cb2c6b346a978ab8c61b18b5e9397755cbd17d6eb2fe0083ef32e067fa6c785a02206ce44e613f31d9a6b0517e46f3db1576e9812cc98d159bfdaf759a5014081b5c01ffffffff79cda0945903627c3da1f85fc95d0b8ee3e76ae0cfdc9a65d09744b1f8fc85430000000049483045022047957cdd957cfd0becd642f6b84d82f49b6cb4c51a91f49246908af7c3cfdf4a022100e96b46621f1bffcf5ea5982f88cef651e9354f5791602369bf5a82a6cd61a62501fffffffffe09f5fe3ffbf5ee97a54eb5e5069e9da6b4856ee86fc52938c2f979b0f38e82000000004847304402204165be9a4cbab8049e1af9723b96199bfd3e85f44c6b4c0177e3962686b26073022028f638da23fc003760861ad481ead4099312c60030d4cb57820ce4d33812a5ce01ffffffff01009d966b01000000434104ea1feff861b51fe3f5f8a3b12d0f4712db80e919548a80839fc47c6a21e66d957e9c5d8cd108c7a2d2324bad71f9904ac0ae7336507d785b17a2c115e427a32fac00000000").unwrap();
        let cutoff_block = hex::decode("010000004ddccd549d28f385ab457e98d1b11ce80bfea2c5ab93015ade4973e400000000bf4473e53794beae34e64fccc471dace6ae544180816f89591894e0f417a914cd74d6e49ffff001d323b3a7b0201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0804ffff001d026e04ffffffff0100f2052a0100000043410446ef0102d1ec5240f0d061a4246c1bdef63fc3dbab7733052fbbf0ecd8f41fc26bf049ebb4f9527f374280259e7cfa99c48b0e3f39c51347a19a5819651503a5ac00000000010000000321f75f3139a013f50f315b23b0c9a2b6eac31e2bec98e5891c924664889942260000000049483045022100cb2c6b346a978ab8c61b18b5e9397755cbd17d6eb2fe0083ef32e067fa6c785a02206ce44e613f31d9a6b0517e46f3db1576e9812cc98d159bfdaf759a5014081b5c01ffffffff79cda0945903627c3da1f85fc95d0b8ee3e76ae0cfdc9a65d09744b1f8fc85430000000049483045022047957cdd957cfd0becd642f6b84d82f49b6cb4c51a91f49246908af7c3cfdf4a022100e96b46621f1bffcf5ea5982f88cef651e9354f5791602369bf5a82a6cd61a62501fffffffffe09f5fe3ffbf5ee97a54eb5e5069e9da6b4856ee86fc52938c2f979b0f38e82000000004847304402204165be9a4cbab8049e1af9723b96199bfd3e85f44c6b4c0177e3962686b26073022028f638da23fc003760861ad481ead4099312c60030d4cb57820ce4d33812a5ce01ffffffff01009d966b01000000434104ea1feff861b51fe3f5f8a3b12d0f4712db80e919548a80839fc47c6a21e66d957e9c5d8cd108c7a2d2324bad71f9904ac0ae7336507d785b17a2c115e427a32fac").unwrap();

        let prevhash = "4ddccd549d28f385ab457e98d1b11ce80bfea2c5ab93015ade4973e400000000";
        // let merkle =
        //     hex::decode("bf4473e53794beae34e64fccc471dace6ae544180816f89591894e0f417a914c")
        //         .unwrap();
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
        assert_eq!(
            hex::encode(decode.header().prev_hash().to_le_bytes()),
            prevhash
        );
        assert_eq!(
            decode.header().merkle_root(),
            &shared::MerkleRoot::from_vec(decode.txids())
        );
        assert_eq!(decode.header().raw_time(), 1231965655);
        // assert_eq!(real_decode.header.bits, 486604799);
        assert_eq!(decode.header().nonce(), 2067413810);
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
    use crate::{
        BitcoinCodec,
        Message::{self, *},
    };
    use bytes::{BufMut, BytesMut};
    use shared::{BlockHash, EncapsulatedAddr, MerkleRoot};
    use shared::{BlockHeader, Transaction};

    impl Message {
        fn to_bytes(&self) -> Result<BytesMut, std::io::Error> {
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
        let msg = Addr(addrs);
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn feefilter_serial_size() {
        let msg = FeeFilter(34567);
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn filteradd_serial_size() {
        let inner1 = Vec::from([1u8; 32]);
        let inner2 = Vec::from([7u8; 11]);
        let outer = Vec::from([inner1, inner2]);
        let msg = FilterAdd(outer);
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn filterclear_serial_size() {
        let msg = FilterClear;
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
        let msg = GetData(inventory);
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn headers_serial_size() {
        let h1 = BlockHeader::_test_header();
        let h2 = BlockHeader::new(
            0,
            BlockHash::from_u64(2),
            MerkleRoot::from_u64(88),
            2198321,
            shared::Nbits::new(shared::u256::from(0xf32231)),
            82,
        );

        let headers = Vec::from([h1, h2]);

        let msg = Headers(headers);
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
        let msg = Inv(inventory);
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
    #[test]
    fn mempool_serial_size() {
        let msg = MemPool;
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
        let msg = NotFound(inventory);
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn ping_serial_size() {
        let msg = Ping(34567);
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn pong_serial_size() {
        let msg = Pong(34567);
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn send_headers_serial_size() {
        let msg = SendHeaders;
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn tx_serial_size() {
        let tx = Transaction::_test_normal();
        let msg = Tx(tx);
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn verack_serial_size() {
        let msg = Verack;
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
}
