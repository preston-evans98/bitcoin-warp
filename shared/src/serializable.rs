use byteorder::{LittleEndian, WriteBytesExt};

pub trait Serializable {
    fn serialize(&self, target: &mut Vec<u8>);
}

impl Serializable for u32 {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.write_u32::<LittleEndian>(*self).unwrap();
    }
}
impl Serializable for u16 {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.write_u16::<LittleEndian>(*self).unwrap();
    }
}
impl Serializable for u8 {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.push(*self);
    }
}
impl Serializable for u64 {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.write_u64::<LittleEndian>(*self).unwrap();
    }
}

impl Serializable for &[u8] {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.extend_from_slice(self)
    }
}
