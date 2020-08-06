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
    fn serialize(&self, target: &mut Vec<u8>) {
        if self.value() < 253 {
            target.push(self.value() as u8);
        } else if self.value() <= std::u16::MAX as u64 {
            target.push(253);
            target
                .write_u16::<LittleEndian>(self.value() as u16)
                .unwrap();
        } else if self.value() <= std::u32::MAX as u64 {
            target.push(254);
            target
                .write_u32::<LittleEndian>(self.value() as u32)
                .unwrap();
        } else {
            target.push(255);
            target.write_u64::<LittleEndian>(self.value()).unwrap();
        }
    }
}
