use crate::{Deserializable, DeserializationError, Serializable};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub struct CompactInt(u64);

impl CompactInt {
    pub fn new() -> CompactInt {
        CompactInt(0)
    }

    pub fn from(value: usize) -> CompactInt {
        CompactInt(value as u64)
    }

    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn size(value: usize) -> usize {
        if value < 253 {
            1
        } else if value < std::u16::MAX as usize {
            2
        } else if value < std::u32::MAX as usize {
            5
        } else {
            9
        }
    }
}

impl Serializable for CompactInt {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        if self.value() < 253 {
            target.write_all(&[self.value() as u8])
        } else if self.value() <= std::u16::MAX as u64 {
            target.write_all(&[253])?;
            target.write_u16::<LittleEndian>(self.value() as u16)
        } else if self.value() <= std::u32::MAX as u64 {
            target.write_all(&[254])?;
            target.write_u32::<LittleEndian>(self.value() as u32)
        } else {
            target.write_all(&[255])?;
            target.write_u64::<LittleEndian>(self.value())
        }
    }
}

impl Deserializable for CompactInt {
    fn deserialize<R>(target: &mut R) -> Result<CompactInt, DeserializationError>
    where
        R: std::io::Read,
    {
        let first = target.read_u8()?;
        if first < 253 {
            Ok(CompactInt::from(first as usize))
        } else if first == 253 {
            Ok(CompactInt::from(target.read_u16::<LittleEndian>()? as usize))
        } else if first == 254 {
            Ok(CompactInt::from(target.read_u32::<LittleEndian>()? as usize))
        } else {
            Ok(CompactInt(target.read_u64::<LittleEndian>()?))
        }
    }
}
