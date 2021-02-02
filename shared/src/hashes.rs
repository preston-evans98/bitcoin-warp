use crate::DeserializationError;
use byteorder::{LittleEndian, WriteBytesExt};
macro_rules! impl_hash_type {
    ($name: ident) => {
        // TODO: Impl hash
        #[derive(Debug, PartialEq, Hash, Eq)]
        pub struct $name([u8; 32]);

        impl $name {
            pub fn from(inner: [u8; 32]) -> $name {
                $name(inner)
            }
            pub fn from_u64(target: u64) -> $name {
                let mut contents = [0u8; 32];
                contents
                    .as_mut()
                    .write_u64::<LittleEndian>(target)
                    .expect("writing 8 bytes to [u8;32] should not fail");
                $name(contents)
            }
            pub fn inner(&self) -> &[u8; 32] {
                &self.0
            }
            pub fn to_le_bytes(&self) -> &[u8; 32] {
                &self.0
            }
        }

        impl crate::Serializable for $name {
            fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
            where
                W: std::io::Write,
            {
                self.0.serialize(target)
            }
        }
        impl crate::Deserializable for $name {
            fn deserialize<B: bytes::Buf>(target: B) -> Result<Self, DeserializationError>
            where
                Self: Sized,
            {
                Ok($name(<[u8; 32]>::deserialize(target)?))
            }
        }
    };
}

impl_hash_type!(BlockHash);
impl_hash_type!(MerkleRoot);
impl_hash_type!(TxID);
