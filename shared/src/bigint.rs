use crate::serializable::Serializable;
use byteorder::{LittleEndian, WriteBytesExt};

#[allow(non_camel_case_types)]
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
