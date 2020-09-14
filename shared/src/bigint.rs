use crate::deserializable::{Deserializable, DeserializationError};
use crate::serializable::Serializable;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct u256([u64; 4]);

fn substring(target: &str, start: usize, end: usize) -> &str {
    if end > target.len() {
        return &target[start..target.len()];
    }
    &target[start..end]
}

impl u256 {
    pub fn new() -> u256 {
        u256([0, 0, 0, 0])
    }

    pub fn from(target: u64) -> u256 {
        u256([target, 0, 0, 0])
    }

    pub fn from_hex(target: &str) -> u256 {
        // TODO: Test
        let mut result = u256([0, 0, 0, 0]);
        match target.len() {
            0..=16 => {
                result.0[0] = u64::from_str_radix(substring(&target, 0, 16), 16).unwrap();
            }
            17..=32 => {
                result.0[0] = u64::from_str_radix(substring(&target, 16, 32), 16).unwrap();
                result.0[1] = u64::from_str_radix(substring(&target, 0, 16), 16).unwrap();
            }
            33..=48 => {
                result.0[0] = u64::from_str_radix(substring(&target, 32, 48), 16).unwrap();
                result.0[1] = u64::from_str_radix(substring(&target, 16, 32), 16).unwrap();
                result.0[2] = u64::from_str_radix(substring(&target, 0, 16), 16).unwrap();
            }
            _ => {
                result.0[0] = u64::from_str_radix(substring(&target, 48, 64), 16).unwrap();
                result.0[1] = u64::from_str_radix(substring(&target, 32, 48), 16).unwrap();
                result.0[2] = u64::from_str_radix(substring(&target, 16, 32), 16).unwrap();
                result.0[3] = u64::from_str_radix(substring(&target, 0, 16), 16).unwrap();
            }
        }
        result
    }
}

impl Serializable for u256 {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        for value in self.0.iter() {
            target.write_u64::<LittleEndian>(*value)?;
        }
        Ok(())
    }
}

impl Serializable for &u256 {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        for value in self.0.iter() {
            target.write_u64::<LittleEndian>(*value)?;
        }
        Ok(())
    }
}

impl Deserializable for u256 {
    fn deserialize<R>(target: &mut R) -> Result<u256, DeserializationError>
    where
        R: std::io::Read,
    {
        Ok(u256([
            target.read_u64::<LittleEndian>()?,
            target.read_u64::<LittleEndian>()?,
            target.read_u64::<LittleEndian>()?,
            target.read_u64::<LittleEndian>()?,
        ]))
    }
}

impl PartialEq for u256 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[test]
fn test_u256_ser_deser() {
    let expected = u256([1, 2, 3, 4]);
    let mut serial: Vec<u8> = Vec::with_capacity(32);
    // let mut ser_cursor = std::io::Cursor::new(&serial);
    expected.serialize(&mut serial).unwrap();
    let mut de_cursor = std::io::Cursor::new(serial);
    let actual = u256::deserialize(&mut de_cursor).unwrap();
    assert_eq!(expected, actual);
}
