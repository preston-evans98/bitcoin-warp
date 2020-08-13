use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub trait Serializable {
    fn serialize(&self, target: &mut Vec<u8>);
}
// pub trait BigEndianSerializable {
//     fn serialize(&self, target: &mut Vec<u8>);
// }

impl Serializable for u8 {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.push(*self);
    }
}
impl Serializable for u16 {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.write_u16::<LittleEndian>(*self).unwrap();
    }
}
// impl BigEndianSerializable for u16 {
//     fn serialize(&self, target: &mut Vec<u8>) {
//         target.write_u16::<BigEndian>(*self).unwrap();
//     }
// }
impl Serializable for u32 {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.write_u32::<LittleEndian>(*self).unwrap();
    }
}
impl Serializable for u64 {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.write_u64::<LittleEndian>(*self).unwrap();
    }
}
impl Serializable for std::net::Ipv6Addr {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.extend_from_slice(&self.octets());
    }
}

impl Serializable for std::net::IpAddr {
    fn serialize(&self, target: &mut Vec<u8>) {
        match self {
            IpAddr::V4(addr) => addr.to_ipv6_mapped().serialize(target),
            IpAddr::V6(addr) => addr.serialize(target),
        }
    }
}

impl Serializable for &std::net::SocketAddr {
    fn serialize(&self, target: &mut Vec<u8>) {
        self.ip().serialize(target);
        target.write_u16::<BigEndian>(self.port()).unwrap();
    }
}

impl Serializable for &[u8] {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.extend_from_slice(self)
    }
}
