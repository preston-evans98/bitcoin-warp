use crate::deserializable::{Deserializable, DeserializationError};
use crate::serializable::Serializable;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

#[allow(non_camel_case_types)]
#[derive(Debug, Hash)]
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

    pub fn is_zero(&self) -> bool {
        self.0 == [0, 0, 0, 0]
    }

    pub fn from(target: u64) -> u256 {
        u256([target, 0, 0, 0])
    }

    pub fn from_bytes_be(target: [u8; 32]) -> Result<u256, DeserializationError> {
        let mut cursor = std::io::Cursor::new(target);
        u256::deserialize_be(&mut cursor)
    }

    // A big_endian counterpart to deser
    pub fn deserialize_be<R>(target: &mut R) -> Result<u256, DeserializationError>
    where
        R: std::io::Read,
    {
        let fourth = target.read_u64::<BigEndian>()?;
        let third = target.read_u64::<BigEndian>()?;
        let second = target.read_u64::<BigEndian>()?;
        let first = target.read_u64::<BigEndian>()?;
        Ok(u256([first, second, third, fourth]))
    }

    // Read from big_endian hex
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
    // writes contents to big_endian hex
    pub fn to_hex(&self) -> String {
        if self.0[3] != 0 {
            return format!(
                "{:x}{:x}{:x}{:x}",
                self.0[3], self.0[2], self.0[1], self.0[0]
            );
        } else if self.0[2] != 0 {
            return format!("{:x}{:x}{:x}", self.0[2], self.0[1], self.0[0]);
        } else if self.0[1] != 0 {
            return format!("{:x}{:x}", self.0[1], self.0[0]);
        }
        format!("{:x}", self.0[0])
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

impl Eq for u256 {}

#[test]
fn test_u256_ser_deser() {
    let expected = u256([1, 2, 3, 4]);
    let mut serial: Vec<u8> = Vec::with_capacity(32);
    expected.serialize(&mut serial).unwrap();
    let mut de_cursor = std::io::Cursor::new(serial);
    let actual = u256::deserialize(&mut de_cursor).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_u256_hex() {
    use warp_crypto::double_sha256;

    let from_hex =
        u256::from_hex("9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50");
    let from_bytes = u256::from_bytes_be(double_sha256(&b"hello".to_vec())).unwrap();
    assert_eq!(from_bytes, from_hex);
    assert_eq!(
        &from_hex.to_hex(),
        "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50"
    )
}
