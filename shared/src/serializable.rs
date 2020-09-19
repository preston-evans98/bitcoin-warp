use crate::CompactInt;
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::net::IpAddr;

pub trait Serializable {
    // fn serialize(&self, target: &mut Vec<u8>);
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write;
}
// pub trait BigEndianSerializable {
//     fn serialize(&self, target: &mut Vec<u8>);
// }

// impl Serializable for u8 {
//     fn serialize(&self, target: &mut Vec<u8>) {
//         target.push(*self);
//     }
// }
impl Serializable for bool {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        target.write_all(&[*self as u8])
    }
}

impl Serializable for u8 {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        target.write_all(&[*self])
    }
}
impl Serializable for u16 {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        target.write_u16::<LittleEndian>(*self)
    }
}
// impl BigEndianSerializable for u16 {
//     fn serialize(&self, target: &mut Vec<u8>) {
//         target.write_u16::<BigEndian>(*self).unwrap();
//     }
// }
impl Serializable for u32 {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        target.write_u32::<LittleEndian>(*self)
    }
}
impl Serializable for u64 {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        target.write_u64::<LittleEndian>(*self)
    }
}
impl Serializable for i64 {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        target.write_i64::<LittleEndian>(*self)
    }
}
impl Serializable for i32 {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,  
    {
        target.write_i32::<LittleEndian>(*self)
    }
}

impl Serializable for std::net::Ipv6Addr {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        target.write_all(&self.octets())
    }
}

impl Serializable for std::net::IpAddr {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        match self {
            IpAddr::V4(addr) => addr.to_ipv6_mapped().serialize(target),
            IpAddr::V6(addr) => addr.serialize(target),
        }
    }
}

impl Serializable for &std::net::SocketAddr {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        self.ip().serialize(target)?;
        target.write_u16::<BigEndian>(self.port())
    }
}

impl Serializable for std::net::SocketAddr {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        self.ip().serialize(target)?;
        target.write_u16::<BigEndian>(self.port())
    }
}

impl Serializable for &[u8] {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        target.write_all(self)
    }
}

impl Serializable for [u8; 4] {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        target.write_all(self)
    }
}

impl<T> Serializable for Vec<T>
where
    T: Serializable,
{
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        CompactInt::from(self.len()).serialize(target)?;
        for item in self.iter() {
            item.serialize(target)?
        }
        Ok(())
    }
}
