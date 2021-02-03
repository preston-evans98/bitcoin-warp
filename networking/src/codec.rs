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
    use crate::Message::{self};
    use bytes::BytesMut;
    use shared::EncapsulatedAddr;
    use std::net::SocketAddr;
    use tokio_util::codec::{Decoder, Encoder};

    impl Codec {
        fn roundtrip(msg: Message) -> Message {
            let mut out = BytesMut::with_capacity(1000);

            let mut codec = Codec::new(config::Config::mainnet().magic());

            codec.encode(msg, &mut out).unwrap();
            codec.decode(&mut out).unwrap().unwrap()
        }
    }

    #[test]
    fn addr_roundtrip() {
        let a1 = EncapsulatedAddr::new(123, 4312, SocketAddr::from(([192, 168, 0, 1], 3030)));
        let a2 = EncapsulatedAddr::new(321, 2331, SocketAddr::from(([127, 0, 0, 1], 8333)));
        let expected = Message::Addr(vec![a1, a2]);
        let actual = Codec::roundtrip(expected.clone());
        match actual {
            Message::Addr(actual_vec) => {
                if let Message::Addr(expected_vec) = expected {
                    for (expected_addr, actual_addr) in expected_vec.iter().zip(actual_vec.iter()) {
                        assert_eq!(expected_addr.time(), actual_addr.time());
                        assert_eq!(expected_addr.services(), actual_addr.services());
                        assert_eq!(expected_addr.time(), actual_addr.time())
                    }
                } else {
                    panic!(
                        "Hardcoded message with wrong type! Expected to expect Addr, got {:?}",
                        expected
                    );
                }
            }
            _ => panic!("Wrong message! Expected Verack received {:?}", actual),
        }
    }

    #[test]
    fn version_roundtrip() {
        let v1 = Message::version(
            SocketAddr::from(([192, 168, 0, 1], 8333)),
            0,
            SocketAddr::from(([192, 168, 0, 0], 8333)),
            0,
            &config::Config::mainnet(),
        );
        let expected = v1.clone();
        let actual = Codec::roundtrip(v1);
        if let Message::Version(expected) = expected {
            if let Message::Version(actual) = actual {
                assert_eq!(expected.protocol_version(), actual.protocol_version());
                assert_eq!(expected.services(), actual.services());
                assert_eq!(expected.timestamp(), actual.timestamp());
                assert_eq!(expected.receiver_services(), actual.receiver_services());
                let receiver = match expected.receiver().ip() {
                    std::net::IpAddr::V4(addr) => addr.to_ipv6_mapped(),
                    std::net::IpAddr::V6(addr) => addr,
                };
                assert_eq!(receiver, actual.receiver().ip());
                assert_eq!(expected.receiver().port(), actual.receiver().port());
                assert_eq!(
                    expected.transmitter_services(),
                    actual.transmitter_services()
                );
                let transmitter = match expected.transmitter_ip().ip() {
                    std::net::IpAddr::V4(addr) => addr.to_ipv6_mapped(),
                    std::net::IpAddr::V6(addr) => addr,
                };
                assert_eq!(transmitter, actual.transmitter_ip().ip());
                assert_eq!(
                    expected.transmitter_ip().port(),
                    actual.transmitter_ip().port()
                );
                assert_eq!(expected.nonce(), actual.nonce());
                assert_eq!(expected.user_agent(), actual.user_agent());
                assert_eq!(expected.best_block(), actual.best_block());
                assert_eq!(expected.relay(), actual.relay());
            } else {
                panic!("Wrong Message: {:?}", actual)
            }
        }
    }
    #[test]
    fn verack_roundtrip() {
        let actual = Codec::roundtrip(Message::Verack);
        match actual {
            Message::Verack => {}
            _ => panic!("Wrong message! Expected Verack received {:?}", actual),
        }
    }
}

#[cfg(test)]
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
