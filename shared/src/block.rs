use crate::block_header::BlockHeader;
pub use crate::hashes::BlockHash as Hash;
use crate::transaction::Transaction;
use crate::TxID;
use crate::{self as shared, Deserializable, DeserializationError, MerkleRoot};
use crate::{CompactInt, Serializable};
use bytes::BytesMut;
use serde_derive::Serializable;

#[derive(Serializable, Debug, Clone)]
pub struct Block {
    block_header: BlockHeader,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(header: BlockHeader, txs: Vec<Transaction>) -> Block {
        let message = Block {
            block_header: header,
            transactions: txs,
        };
        message
    }
    pub fn header(&self) -> &BlockHeader {
        &self.block_header
    }
    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
    pub fn txids(&self) -> Vec<&TxID> {
        self.transactions
            .iter()
            .map(|tx| tx.txid())
            .collect::<Vec<&TxID>>()
    }
    pub fn serialized_size(&self) -> usize {
        let mut size = CompactInt::size(self.transactions.len());
        size += BlockHeader::len();
        for transaction in self.transactions.iter() {
            size += transaction.len();
        }
        size
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut target = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut target)?;
        Ok(target)
    }
    /// Deserializes a block. Attempts to make structurally invalid blocks unrepresentable by enforcing that...
    /// 1. The block contains exactly one Coinbase transaction, and it's in the first position.
    /// 1. The block does not contain duplicate transactions
    /// 1. The transactions merkle-ize to the root in the block header
    pub fn deserialize(mut src: &mut BytesMut) -> Result<Self, DeserializationError> {
        let header = BlockHeader::deserialize(src.split_to(80))?;
        let tx_count = {
            let tx_count = CompactInt::deserialize(&mut src)?;
            tx_count.value()
        };

        // Reject empty blocks
        if tx_count == 0 {
            return Err(DeserializationError::Parse(String::from(
                "Block contains no transactions",
            )));
        }

        // Deserialize and structurally validate Coinbase
        let first_tx = Transaction::deserialize(&mut src)?;
        if !first_tx.is_coinbase() {
            return Err(DeserializationError::Parse(String::from(
                "Block did not contain Coinbase in first position",
            )));
        }
        // TODO: Parse block height
        if header.version() >= 2 {}
        let mut transactions = Vec::with_capacity(tx_count as usize);
        transactions.push(first_tx);

        // Parse and validate remaining transactions
        for _ in 1..tx_count {
            let next = Transaction::deserialize(&mut src)?;
            if next.is_coinbase() {
                return Err(DeserializationError::Parse(String::from(
                    "Block contained second Coinbase",
                )));
            }
            transactions.push(next);
        }
        let actual_merkle_root = MerkleRoot::from_iter(transactions.iter().map(|tx| tx.txid()));
        if !(&actual_merkle_root == header.merkle_root()) {
            return Err(DeserializationError::Parse(String::from(
                "Invalid Merkle Root",
            )));
        }
        Ok(Block::new(header, transactions))
    }

    // #[cfg(test)]
    pub fn _test_block() -> Block {
        let some_block = hex::decode("010000004ddccd549d28f385ab457e98d1b11ce80bfea2c5ab93015ade4973e400000000bf4473e53794beae34e64fccc471dace6ae544180816f89591894e0f417a914cd74d6e49ffff001d323b3a7b0201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0804ffff001d026e04ffffffff0100f2052a0100000043410446ef0102d1ec5240f0d061a4246c1bdef63fc3dbab7733052fbbf0ecd8f41fc26bf049ebb4f9527f374280259e7cfa99c48b0e3f39c51347a19a5819651503a5ac00000000010000000321f75f3139a013f50f315b23b0c9a2b6eac31e2bec98e5891c924664889942260000000049483045022100cb2c6b346a978ab8c61b18b5e9397755cbd17d6eb2fe0083ef32e067fa6c785a02206ce44e613f31d9a6b0517e46f3db1576e9812cc98d159bfdaf759a5014081b5c01ffffffff79cda0945903627c3da1f85fc95d0b8ee3e76ae0cfdc9a65d09744b1f8fc85430000000049483045022047957cdd957cfd0becd642f6b84d82f49b6cb4c51a91f49246908af7c3cfdf4a022100e96b46621f1bffcf5ea5982f88cef651e9354f5791602369bf5a82a6cd61a62501fffffffffe09f5fe3ffbf5ee97a54eb5e5069e9da6b4856ee86fc52938c2f979b0f38e82000000004847304402204165be9a4cbab8049e1af9723b96199bfd3e85f44c6b4c0177e3962686b26073022028f638da23fc003760861ad481ead4099312c60030d4cb57820ce4d33812a5ce01ffffffff01009d966b01000000434104ea1feff861b51fe3f5f8a3b12d0f4712db80e919548a80839fc47c6a21e66d957e9c5d8cd108c7a2d2324bad71f9904ac0ae7336507d785b17a2c115e427a32fac00000000").unwrap();
        Block::deserialize(&mut BytesMut::from(&some_block[..])).unwrap()
    }
}

#[test]
fn serial_size() {
    let previous_outpoint = crate::TxOutpoint::new(crate::u256::from(1), 438);
    let txin1 = crate::TxInput::new(previous_outpoint, Vec::from([8u8; 21]), 1);
    let txin2 = crate::TxInput::new(crate::TxOutpoint::new(crate::u256::new(), 0), Vec::new(), 2);
    let mut txins = Vec::new();
    txins.push(txin1);
    txins.push(txin2);
    let mut outputs = Vec::new();
    let out1 = crate::TxOutput::new(1, Vec::from([3u8; 11]));
    let out2 = crate::TxOutput::new(0, Vec::new());
    outputs.push(out1);
    outputs.push(out2);
    let tx1 = Transaction::new(0, txins, outputs);
    let tx2 = Transaction::new(1, Vec::new(), Vec::new());

    let mut txs = Vec::with_capacity(2);
    txs.push(tx1);
    txs.push(tx2);
    let block_header = BlockHeader::new(
        23,
        shared::block::Hash::from_u64(12345678),
        MerkleRoot::from_u64(9876543),
        2342,
        crate::block_header::Nbits::new(crate::u256::from(8719)),
        99,
    );

    let msg = Block {
        block_header,
        transactions: txs,
    };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}

mod consensus_deser_tests {

    #[test]
    // Adapted from https://github.com/rust-bitcoin/rust-bitcoin/blob/master/src/blockdata/block.rs
    fn deser_block_test() {
        use super::Block;
        use bytes::BytesMut;
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
            &super::MerkleRoot::from_vec(decode.txids())
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
