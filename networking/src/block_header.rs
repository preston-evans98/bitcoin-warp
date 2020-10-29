use byteorder::{LittleEndian, ReadBytesExt};
use serde_derive::{Deserializable, Serializable};
use shared::u256;
#[derive(Deserializable, Serializable)]
pub struct BlockHeader {
    version: u32,
    prev_hash: u256,
    merkle_root: u256,
    time: u32,
    target: Nbits,
    nonce: u32,
}

pub struct Nbits {
    target: u256,
}
impl Nbits {
    pub fn new(target: u256) -> Nbits {
        Nbits { target }
    }
}
impl shared::Deserializable for Nbits {
    fn deserialize<R>(target: &mut R) -> Result<Nbits, shared::DeserializationError>
    where
        R: std::io::Read,
    {
        let compressed_target = target.read_u32::<LittleEndian>()?;
        println!("{:x}", compressed_target);
        let mantissa: u32 = compressed_target & 0x00FFFFFF;
        println!("mantissa: {:x}", mantissa);
        // To replicate a bug in core: If the mantissa starts with 0b1, return 0.
        if mantissa & 0x00800000 != 0 {
            return Ok(Nbits {
                target: u256::new(),
            });
        }
        let mut exponent = (compressed_target & 0xFF000000) >> 24;
        println!("exponent: {}", exponent);
        let mut raw_target = [0u8; 32];
        for i in (0..=2).rev() {
            if exponent == 0 {
                break;
            }
            exponent -= 1;
            raw_target[exponent as usize] = (mantissa >> (8 * i)) as u8;
        }
        println!("{:?}", raw_target);
        Ok(Nbits {
            target: u256::deserialize(&mut std::io::Cursor::new(raw_target))?,
        })
        // ** Alternative Implementation. TODO: Compare performance
        // let mut mantissa = [0u8; 3];
        // target.read_exact(&mut mantissa)?;
        // // If the high order bit is set, return nbits 0;
        // if mantissa[0] & 0b10000000 != 0 {
        //     return Ok(Nbits {
        //         target: u256::new(),
        //     });
        // }
        // let exponent = target.read_u8()?;
        // let offset = (exponent as usize).saturating_sub(3);
        // let mut result = [0u8; 32];
        // result[offset] = mantissa[0];
        // result[offset + 1] = mantissa[1];
        // result[offset + 2] = mantissa[2];

        // Ok(Nbits {
        //     target: u256::deserialize(&mut std::io::Cursor::new(result))?,
        // })
    }
}
impl shared::Serializable for Nbits {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        // Dump the difficulty target to a vector as raw bytes
        let mut tempvec = Vec::with_capacity(32);
        self.target.serialize(&mut tempvec)?;
        // Initialize locals;
        let mut significand = 0u32;
        let mut hit_significand: bool = false;
        let mut exponent: u32 = 0;
        let mut remaining_slots = 3;

        // 1. Find the **last** non-zero byte. This is the MSB of the significand. Set the exponent accordingly
        // 2. After hitting the MSB, grab the next two bytes (recall that we're iterating in reverse) and place them into the significand using a bit shift
        // 3. After filling the significand, round based on the next byte if necessary
        for (rev_index, val) in tempvec.into_iter().rev().enumerate() {
            // Step 3.
            if remaining_slots == 0 {
                if val >= 0x80 {
                    significand += 1;
                }
                break;
            }

            // Step 1.
            if val != 0 && !hit_significand {
                hit_significand = true;
                exponent = 32 - (rev_index as u32);
                println!("Val: {}, Expt: {}", val, exponent);
            }

            // Setp 2.
            if hit_significand {
                remaining_slots -= 1;
                significand += (val as u32) << (remaining_slots * 8);
                println!(
                    "Significand: 0x{:x}, remaining slots: {}",
                    significand, remaining_slots
                );
            }
        }
        // println!();
        // Due to a bug in Bitcoin Core, significands are treated as signed numbers
        // If the significand has a 1 in the MSb, divide by 256 and increment the exponent
        if significand & 0x00800000 != 0 {
            significand >>= 8;
            exponent += 1;
        }
        println!("Significand: {}, Expt: {}", significand, exponent);
        // Store the exponent as the MSB and the
        let result = significand | (exponent << 24);
        println!("{:x}", result);
        println!("{:x}", exponent << 24);
        result.serialize(target)
    }
}

// impl shared::Serializable for Nbits {
//     fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
//     where
//         W: std::io::Write,
//     {
//         // TODO: improve efficiency
//         // 1. Convert target into raw bytes
//         // 2. find the index of the last non-zero byte
//         // 3.
//         let mut tempvec = Vec::with_capacity(32);
//         self.target.serialize(&mut tempvec)?;
//         let mut raw_result = [0u8; 4];
//         let mut offset = 2;
//         let mut hit_significand = false;
//         let mut exponent = 0;
//         // Loop:
//         for (rev_index, val) in tempvec.into_iter().rev().enumerate() {
//             // Once the significand is full, enter this block
//             if offset == 0 {
//                 // Check if we need to round up based on the contents of the next byte
//                 if val >= 0x80 {
//                     // If so, round up
//                     for index in 2..=0 {
//                         let (result, overflowed) = raw_result[index].overflowing_add(1);
//                         raw_result[index] = result;
//                         if !overflowed {
//                             break;
//                         }
//                         if index == 2 {
//                             // if index is 2 and we've overflowed,
//                             exponent += 1;
//                             raw_result = [0, 0, 1, exponent];
//                             target.write_all(&raw_result)?;
//                             return Ok(());
//                         }
//                     }
//                 }
//                 break;
//             }
//             if !hit_significand && val != 0 {
//                 exponent = 31 - rev_index as u8;
//                 hit_significand = true;
//             }
//             if hit_significand {
//                 raw_result[offset] = val;
//                 offset -= 1;
//             }
//         }
//         if raw_result[2] & 0x80 != 0 {
//             let mut cursor = std::io::Cursor::new(raw_result);
//             let significand = cursor.read_u32::<LittleEndian>()?;
//             significand -= 256;
//             exponent += 1;
//             significand = significand | (exponent >> 24);
//             return significand.serialize(target);
//         }
//         Ok(())
//     }
// }

#[cfg(test)]
#[test]
fn deser_nbits_zero() {
    use shared::{Deserializable, Serializable};
    let encoded: u32 = 0x01003456;
    let mut input = Vec::with_capacity(4);
    encoded.serialize(&mut input).unwrap();
    let mut cursor = std::io::Cursor::new(input);
    let nbits = Nbits::deserialize(&mut cursor).unwrap();
    // assert_eq!(format!("{:?}", nbits.target), "");
    assert_eq!(nbits.target.to_hex(), "0")
}

#[test]
fn deser_nbits_zero_2() {
    use shared::{Deserializable, Serializable};
    let encoded: u32 = 0;
    let mut input = Vec::with_capacity(4);
    encoded.serialize(&mut input).unwrap();
    let mut cursor = std::io::Cursor::new(input);
    let nbits = Nbits::deserialize(&mut cursor).unwrap();
    // assert_eq!(format!("{:?}", nbits.target), "");
    assert_eq!(nbits.target.to_hex(), "0")
}

#[test]
fn ser_nbits_zero() {
    use shared::Serializable;
    let target = u256::from(0x00);
    let nbits = Nbits::new(target);
    let mut out = Vec::with_capacity(4);
    nbits.serialize(&mut out).unwrap();
    let mut cursor = std::io::Cursor::new(out);
    let result = cursor.read_u32::<LittleEndian>().unwrap();
    assert_eq!(result, 0)
}

#[test]
fn deser_nbits_twelve() {
    use shared::Deserializable;
    use shared::Serializable;
    let encoded: u32 = 0x01123456;
    let mut input = Vec::with_capacity(4);
    encoded.serialize(&mut input).unwrap();
    let mut cursor = std::io::Cursor::new(input);
    let nbits = Nbits::deserialize(&mut cursor).unwrap();
    // assert_eq!(format!("{:?}", nbits.target), "");
    assert_eq!(nbits.target.to_hex(), "12")
}

#[test]
fn ser_nbits_twelve() {
    use shared::Serializable;
    let target = u256::from(0x12);
    let nbits = Nbits::new(target);
    let mut out = Vec::with_capacity(4);
    nbits.serialize(&mut out).unwrap();
    let mut cursor = std::io::Cursor::new(out);
    let result = cursor.read_u32::<LittleEndian>().unwrap();
    println!("{:x}", result);
    assert_eq!(result, 0x01120000);
}

#[test]
fn deser_nbits_eighty() {
    use shared::Deserializable;
    use shared::Serializable;
    let encoded: u32 = 0x02008000;
    let mut input = Vec::with_capacity(4);
    encoded.serialize(&mut input).unwrap();
    let mut cursor = std::io::Cursor::new(input);
    let nbits = Nbits::deserialize(&mut cursor).unwrap();
    // assert_eq!(format!("{:?}", nbits.target), "");
    assert_eq!(nbits.target.to_hex(), "80")
}

#[test]
fn ser_nbits_eighty() {
    use shared::Serializable;
    let target = u256::from(0x80);
    let nbits = Nbits::new(target);
    let mut out = Vec::with_capacity(4);
    nbits.serialize(&mut out).unwrap();
    let mut cursor = std::io::Cursor::new(out);
    let result = cursor.read_u32::<LittleEndian>().unwrap();
    println!("{:x}", result);
    assert_eq!(result, 0x02008000);
}

// 0x05009234
#[test]
fn deser_nbits_big() {
    use shared::Deserializable;
    use shared::Serializable;
    let encoded: u32 = 0x05009234;
    let mut input = Vec::with_capacity(4);
    encoded.serialize(&mut input).unwrap();
    let mut cursor = std::io::Cursor::new(input);
    let nbits = Nbits::deserialize(&mut cursor).unwrap();
    // assert_eq!(format!("{:?}", nbits.target), "");
    assert_eq!(nbits.target.to_hex(), "92340000")
}

#[test]
fn ser_nbits_big() {
    use shared::Serializable;
    let target = u256::from(0x92340000);
    let nbits = Nbits::new(target);
    let mut out = Vec::with_capacity(4);
    nbits.serialize(&mut out).unwrap();
    let mut cursor = std::io::Cursor::new(out);
    let result = cursor.read_u32::<LittleEndian>().unwrap();
    println!("{:x}", result);
    assert_eq!(result, 0x05009234);
}

#[test]
fn deser_nbits_neg() {
    use shared::Deserializable;
    use shared::Serializable;
    let encoded: u32 = 0x04923456;
    let mut input = Vec::with_capacity(4);
    encoded.serialize(&mut input).unwrap();
    let mut cursor = std::io::Cursor::new(input);
    let nbits = Nbits::deserialize(&mut cursor).unwrap();
    // assert_eq!(format!("{:?}", nbits.target), "");
    assert_eq!(nbits.target.to_hex(), "0")
}

#[test]
fn deser_nbits_nonneg() {
    use shared::Deserializable;
    use shared::Serializable;
    let encoded: u32 = 0x04123456;
    let mut input = Vec::with_capacity(4);
    encoded.serialize(&mut input).unwrap();
    let mut cursor = std::io::Cursor::new(input);
    let nbits = Nbits::deserialize(&mut cursor).unwrap();
    // assert_eq!(format!("{:?}", nbits.target), "");
    assert_eq!(nbits.target.to_hex(), "12345600")
}

#[test]
fn ser_nbits_noneg() {
    use shared::Serializable;
    let target = u256::from(0x12345600);
    let nbits = Nbits::new(target);
    let mut out = Vec::with_capacity(4);
    nbits.serialize(&mut out).unwrap();
    let mut cursor = std::io::Cursor::new(out);
    let result = cursor.read_u32::<LittleEndian>().unwrap();
    println!("{:x}", result);
    assert_eq!(result, 0x04123456);
}