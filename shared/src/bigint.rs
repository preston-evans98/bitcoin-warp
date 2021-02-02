use crate::deserializable::{Deserializable, DeserializationError};
use crate::serializable::Serializable;
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use bytes::Buf;
use std::convert::TryInto;

#[allow(non_camel_case_types)]
#[derive(Debug, Hash, Clone)]
pub struct u256([u8; 32]);

// fn substring(target: &str, start: usize, end: usize) -> &str {
//     if end > target.len() {
//         return &target[start..target.len()];
//     }
//     &target[start..end]
// }
impl u256 {
    pub fn new() -> u256 {
        u256([0u8; 32])
    }

    pub fn is_zero(&self) -> bool {
        self.0 == [0u8; 32]
    }

    pub fn from(target: u64) -> u256 {
        let mut contents = [0u8; 32];
        contents
            .as_mut()
            .write_u64::<LittleEndian>(target)
            .expect("writing 8 bytes to [u8;32] should not fail");
        u256(contents)
    }

    pub fn to_le_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    // pub fn from_bytes_be(target: [u8; 32]) -> Result<u256, DeserializationError> {
    //     let mut cursor = std::io::Cursor::new(target);
    //     u256::deserialize_be(&mut cursor)
    // }
    pub fn from_bytes(target: [u8; 32]) -> u256 {
        // let mut target = std::io::Cursor::new(target);
        // u256([
        //     target
        //         .read_u64::<LittleEndian>()
        //         .expect("Cursro must contain at least 32 bytes"),
        //     target
        //         .read_u64::<LittleEndian>()
        //         .expect("Cursro must contain at least 32 bytes"),
        //     target
        //         .read_u64::<LittleEndian>()
        //         .expect("Cursro must contain at least 32 bytes"),
        //     target
        //         .read_u64::<LittleEndian>()
        //         .expect("Cursro must contain at least 32 bytes"),
        // ])
        u256(target)
    }

    // // A big_endian counterpart to deser
    // pub fn deserialize_be<R>(target: &mut R) -> Result<u256, DeserializationError>
    // where
    //     R: std::io::Read,
    // {

    //     // let fourth = target.read_u64::<BigEndian>()?;
    //     // let third = target.read_u64::<BigEndian>()?;
    //     // let second = target.read_u64::<BigEndian>()?;
    //     // let first = target.read_u64::<BigEndian>()?;
    //     // Ok(u256([
    //     //     first.to_le_bytes().,
    //     //     second.to_le_bytes(),
    //     //     third.to_le_bytes(),
    //     //     fourth.to_le_bytes(),
    //     // ]))
    // }

    // // Read from big_endian hex
    // pub fn from_hex(target: &str) -> u256 {
    //     // TODO: Test
    //     let mut result = u256([0, 0, 0, 0]);
    //     match target.len() {
    //         0..=16 => {
    //             result.0[0] = u64::from_str_radix(substring(&target, 0, 16), 16).unwrap();
    //         }
    //         17..=32 => {
    //             result.0[0] = u64::from_str_radix(substring(&target, 16, 32), 16).unwrap();
    //             result.0[1] = u64::from_str_radix(substring(&target, 0, 16), 16).unwrap();
    //         }
    //         33..=48 => {
    //             result.0[0] = u64::from_str_radix(substring(&target, 32, 48), 16).unwrap();
    //             result.0[1] = u64::from_str_radix(substring(&target, 16, 32), 16).unwrap();
    //             result.0[2] = u64::from_str_radix(substring(&target, 0, 16), 16).unwrap();
    //         }
    //         _ => {
    //             result.0[0] = u64::from_str_radix(substring(&target, 48, 64), 16).unwrap();
    //             result.0[1] = u64::from_str_radix(substring(&target, 32, 48), 16).unwrap();
    //             result.0[2] = u64::from_str_radix(substring(&target, 16, 32), 16).unwrap();
    //             result.0[3] = u64::from_str_radix(substring(&target, 0, 16), 16).unwrap();
    //         }
    //     }
    //     result
    // }
    // writes contents to big_endian hex
    pub fn to_hex(&self) -> String {
        let first = u64::from_le_bytes(self.0[24..32].try_into().unwrap());
        let second = u64::from_le_bytes(self.0[16..24].try_into().unwrap());
        let third = u64::from_le_bytes(self.0[8..16].try_into().unwrap());
        let fourth = u64::from_le_bytes(self.0[0..8].try_into().unwrap());
        if first != 0 {
            return format!("{:x}{:x}{:x}{:x}", first, second, third, fourth);
        } else if third != 0 {
            return format!("{:x}{:x}{:x}", second, third, fourth);
        } else if second != 0 {
            return format!("{:x}{:x}", third, fourth);
        }
        format!("{:x}", fourth)
    }
}

impl Serializable for u256 {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        self.0.serialize(target)
    }
}

impl Serializable for &u256 {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        self.0.serialize(target)
    }
}

impl Deserializable for u256 {
    fn deserialize<B: Buf>(mut target: B) -> Result<u256, DeserializationError> {
        // if target.remaining() < (256 / 8) {
        //     return Err(DeserializationError::Parse(String::from(
        //         "Not enough data left in buffer to parse u256",
        //     )));
        // }
        // Ok(u256([
        //     target.get_u64_le(),
        //     target.get_u64_le(),
        //     target.get_u64_le(),
        //     target.get_u64_le(),
        // ]))
        let contents = <[u8; 32]>::deserialize(target)?;
        Ok(u256(contents))
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
    use bytes::BytesMut;
    use std::iter::FromIterator;
    let mut contents = [0u8; 32];
    contents[7] = 1;
    contents[15] = 2;
    contents[23] = 3;
    contents[31] = 4;
    let mut expected = u256(contents);
    // expe
    let mut serial: Vec<u8> = Vec::with_capacity(32);
    expected.serialize(&mut serial).unwrap();
    let mut de_target = BytesMut::from_iter(serial.iter());
    let actual = u256::deserialize(&mut de_target).unwrap();
    assert_eq!(expected, actual);
}

// #[test]
// fn test_u256_hex() {
//     use warp_crypto::double_sha256;

//     let from_hex =
//         u256::from_hex("9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50");
//     let from_bytes = u256::from_bytes_be(double_sha256(&b"hello".to_vec())).unwrap();
//     assert_eq!(from_bytes, from_hex);
//     assert_eq!(
//         &from_hex.to_hex(),
//         "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50"
//     )
// }
