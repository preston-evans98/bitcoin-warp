use crate::Serializable;
use byteorder::{LittleEndian, WriteBytesExt};

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
}

impl Serializable for CompactInt {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        if self.value() < 253 {
            target.write_all(&[self.value() as u8])
        } else if self.value() <= std::u16::MAX as u64 {
            target.write_all(&[253]);
            target.write_u16::<LittleEndian>(self.value() as u16)
        } else if self.value() <= std::u32::MAX as u64 {
            target.write_all(&[254]);
            target.write_u32::<LittleEndian>(self.value() as u32)
        } else {
            target.write_all(&[255]);
            target.write_u64::<LittleEndian>(self.value())
        }
    }
}
